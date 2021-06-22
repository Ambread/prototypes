use crate::{assets::Assets, chunk::Chunk};
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
use luminance_derive::{UniformInterface, Vertex};
use luminance_gl::gl33::GL33;
use luminance_glfw::GlfwSurface;

// Just include the shaders as static string
const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

pub struct Renderer {
    back_buffer: Framebuffer<GL33, Dim2, (), ()>,
    program: Program<GL33, VertexSemantics, (), ShaderInterface>,
    quad: Tess<GL33, Vertex, (), (), Interleaved>,
    tile_texture: Texture<GL33, Dim2Array, NormRGB8UI>,
    world_texture: Texture<GL33, Dim2, R8UI>,
}

impl Renderer {
    pub fn new(surface: &mut GlfwSurface, assets: &Assets) -> Result<Self> {
        // Use the back buffer for rendering
        let back_buffer = surface.context.back_buffer()?;

        // Shader program
        let program = surface
            .context
            .new_shader_program::<VertexSemantics, (), ShaderInterface>()
            .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)?
            .ignore_warnings();

        // Quad that covers the entire viewport
        let quad = surface
            .context
            .new_tess()
            .set_vertices(&VERTICES[..])
            .set_mode(Mode::Triangle)
            .build()?;

        // Texture array for all the tile sprites
        let tile_texture = Self::create_tile_textures(surface, assets)?;

        // Texture to hold the tiles of the world
        let world_texture = surface.context.new_texture::<Dim2, R8UI>(
            [Chunk::SIZE as u32, Chunk::SIZE as u32],
            0,
            Sampler {
                // Integer textures must use `Nearest` otherwise it'll read all zeros
                mag_filter: MagFilter::Nearest,
                min_filter: MinFilter::Nearest,
                ..Default::default()
            },
        )?;

        Ok(Self {
            back_buffer,
            program,
            quad,
            tile_texture,
            world_texture,
        })
    }

    pub fn create_tile_textures(
        surface: &mut GlfwSurface,
        assets: &Assets,
    ) -> Result<Texture<GL33, Dim2Array, NormRGB8UI>> {
        // Texture array for all the tile sprites
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

        // Upload the texture array
        tile_texture.upload_raw(GenMipmaps::Yes, &assets.tile_sprites)?;

        Ok(tile_texture)
    }

    pub fn reload_assets(&mut self, surface: &mut GlfwSurface, assets: &Assets) -> Result<()> {
        self.tile_texture = Self::create_tile_textures(surface, assets)?;
        Ok(())
    }

    // Needed when the screen is resized
    pub fn refresh_back_buffer(&mut self, surface: &mut GlfwSurface) -> Result<()> {
        self.back_buffer = surface.context.back_buffer()?;
        Ok(())
    }

    // How `Renderer` accepts `Chunk`'s tiles
    pub fn upload_world_texture(&mut self, tiles: &[u8]) -> Result<()> {
        self.world_texture.upload(GenMipmaps::No, tiles)?;
        Ok(())
    }

    pub fn render(&mut self, surface: &mut GlfwSurface) -> Result<()> {
        // HACK: Borrowing issues, so just split to begin with
        // Rust somehow likes this better
        let Self {
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
                    // Any textures you have to bind here
                    let bound_texture = pipeline.bind_texture(tile_texture)?;
                    let bound_world = pipeline.bind_texture(world_texture)?;

                    shade_gate.shade(program, |mut interface, uniforms, mut render_gate| {
                        // Assign uniforms
                        interface.set(&uniforms.tile_texture, bound_texture.binding());
                        interface.set(&uniforms.world_texture, bound_world.binding());
                        interface.set(&uniforms.world_size, Chunk::SIZE as u32);

                        // Render some stuff
                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&*quad)
                        })
                    })
                },
            )
            .assume()
            .into_result()?;

        // Put the new stuff onto the screen
        surface.context.window.swap_buffers();

        Ok(())
    }
}

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    tile_texture: Uniform<TextureBinding<Dim2Array, NormUnsigned>>,
    #[uniform(unbound)]
    world_texture: Uniform<TextureBinding<Dim2, Unsigned>>,
    #[uniform(unbound)]
    world_size: Uniform<u32>,
}

// HACK: The `Semantics` derive macro insists on it being public when we don't want it public
// Wrap it in a module to actually keep them private
mod vertex_semantics {
    use luminance_derive::Semantics;

    #[derive(Debug, Clone, Copy, Semantics)]
    pub enum VertexSemantics {
        #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
        Position,
    }
}
use vertex_semantics::*;

#[derive(Debug, Clone, Copy, Vertex)]
#[vertex(sem = "VertexSemantics")]
struct Vertex {
    position: VertexPosition,
}

// Vertices for a quad that covers the entire viewport
const VERTICES: [Vertex; 6] = [
    Vertex::new(VertexPosition::new([1.0, 1.0])),
    Vertex::new(VertexPosition::new([1.0, -1.0])),
    Vertex::new(VertexPosition::new([-1.0, 1.0])),
    Vertex::new(VertexPosition::new([-1.0, -1.0])),
    Vertex::new(VertexPosition::new([-1.0, 1.0])),
    Vertex::new(VertexPosition::new([1.0, -1.0])),
];
