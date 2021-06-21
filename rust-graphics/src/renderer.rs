use anyhow::Result;
use glfw::Context as _;
use luminance::{
    context::GraphicsContext as _,
    framebuffer::Framebuffer,
    pipeline::{PipelineState, TextureBinding},
    pixel::{NormRGB8UI, NormUnsigned, Unsigned, R8UI},
    render_state::RenderState,
    shader::{Program, Uniform},
    tess::{Interleaved, Mode, Tess},
    texture::{Dim2, Dim2Array, GenMipmaps, MagFilter, MinFilter, Sampler, Texture},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_gl::gl33::GL33;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

use crate::{assets::Assets, chunk::Chunk};

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

pub struct Renderer {
    surface: GlfwSurface,
    back_buffer: Framebuffer<GL33, Dim2, (), ()>,
    program: Program<GL33, VertexSemantics, (), ShaderInterface>,
    quad: Tess<GL33, Vertex, (), (), Interleaved>,
    tile_texture: Texture<GL33, Dim2Array, NormRGB8UI>,
    world_texture: Texture<GL33, Dim2, R8UI>,
}

impl Renderer {
    pub fn new(assets: Assets) -> Result<Self> {
        let mut surface = GlfwSurface::new_gl33(
            "Rust Graphics Test",
            WindowOpt::default().set_dim(WindowDim::Windowed {
                width: 768,
                height: 768,
            }),
        )?;

        let back_buffer = surface.context.back_buffer()?;

        let program = surface
            .context
            .new_shader_program::<VertexSemantics, (), ShaderInterface>()
            .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)?
            .ignore_warnings();

        let quad = surface
            .context
            .new_tess()
            .set_vertices(&VERTICES[..])
            .set_mode(Mode::Triangle)
            .build()?;

        let mut tile_texture = surface.context.new_texture::<Dim2Array, NormRGB8UI>(
            (
                assets.tile_data.texture_size.cast().unwrap().into(),
                assets.tile_data.texture_count as u32,
            ),
            2,
            Sampler {
                mag_filter: MagFilter::Nearest,
                ..Default::default()
            },
        )?;

        tile_texture.upload_raw(GenMipmaps::Yes, &assets.tile_sprites)?;

        let world_texture = surface.context.new_texture::<Dim2, R8UI>(
            [Chunk::SIZE as u32, Chunk::SIZE as u32],
            0,
            Sampler {
                mag_filter: MagFilter::Nearest,
                min_filter: MinFilter::Nearest,
                ..Default::default()
            },
        )?;

        Ok(Self {
            surface,
            back_buffer,
            program,
            quad,
            tile_texture,
            world_texture,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        let Self {
            surface,
            program,
            quad,
            tile_texture,
            world_texture,
            ..
        } = self;

        surface
            .context
            .new_pipeline_gate()
            .pipeline(
                &self.back_buffer,
                &PipelineState::default(),
                |pipeline, mut shade_gate| {
                    let bound_texture = pipeline.bind_texture(tile_texture)?;
                    let bound_world = pipeline.bind_texture(world_texture)?;

                    shade_gate.shade(program, |mut interface, uniforms, mut render_gate| {
                        interface.set(&uniforms.texles, bound_texture.binding());
                        interface.set(&uniforms.world, bound_world.binding());
                        interface.set(&uniforms.world_size, Chunk::SIZE as u32);

                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&*quad)
                        })
                    })
                },
            )
            .assume()
            .into_result()?;

        self.surface.context.window.swap_buffers();

        Ok(())
    }
}

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    texles: Uniform<TextureBinding<Dim2Array, NormUnsigned>>,
    #[uniform(unbound)]
    world: Uniform<TextureBinding<Dim2, Unsigned>>,
    #[uniform(unbound)]
    world_size: Uniform<u32>,
}

#[derive(Debug, Clone, Copy, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
}

#[derive(Debug, Clone, Copy, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    position: VertexPosition,
}

const VERTICES: [Vertex; 6] = [
    Vertex::new(VertexPosition::new([1.0, 1.0])),
    Vertex::new(VertexPosition::new([1.0, -1.0])),
    Vertex::new(VertexPosition::new([-1.0, 1.0])),
    Vertex::new(VertexPosition::new([-1.0, -1.0])),
    Vertex::new(VertexPosition::new([-1.0, 1.0])),
    Vertex::new(VertexPosition::new([1.0, -1.0])),
];
