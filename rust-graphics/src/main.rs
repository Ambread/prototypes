#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod assets;
pub mod chunk;

use anyhow::Result;
use cgmath::Vector2;
use chunk::Chunk;
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _,
    pipeline::{PipelineState, TextureBinding},
    pixel::{NormRGB8UI, NormUnsigned, Unsigned, R8UI},
    render_state::RenderState,
    shader::Uniform,
    tess::Mode,
    texture::{Dim2, Dim2Array, GenMipmaps, MagFilter, MinFilter, Sampler},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::env::current_dir;

use assets::Assets;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

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

fn main() -> Result<()> {
    let assets = Assets::from_path(current_dir()?.join("assets"))?;

    let mut surface = GlfwSurface::new_gl33(
        "Rust Graphics Test",
        WindowOpt::default().set_dim(WindowDim::Windowed {
            width: 960,
            height: 540,
        }),
    )?;

    let mut back_buffer = surface.context.back_buffer()?;

    let mut program = surface
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

    let mut texture = surface.context.new_texture::<Dim2Array, NormRGB8UI>(
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

    texture.upload_raw(GenMipmaps::Yes, &assets.tile_sprites)?;

    let mut world = surface.context.new_texture::<Dim2, R8UI>(
        [Chunk::SIZE as u32, Chunk::SIZE as u32],
        0,
        Sampler {
            mag_filter: MagFilter::Nearest,
            min_filter: MinFilter::Nearest,
            ..Default::default()
        },
    )?;

    let mut chunk = Chunk::new(Vector2::new(0, 0));

    chunk.generate(&assets)?;
    world.upload(GenMipmaps::No, chunk.tiles())?;

    'main: loop {
        surface.context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'main
                }

                WindowEvent::Key(key, _, Action::Press, _) => {
                    match key {
                        Key::W => chunk.position.y += 1,
                        Key::A => chunk.position.x -= 1,
                        Key::S => chunk.position.y -= 1,
                        Key::D => chunk.position.x += 1,

                        Key::P => println!("{:?}", chunk),

                        _ => {}
                    }

                    if let Key::W | Key::A | Key::S | Key::D = key {
                        chunk.generate(&assets)?;
                        world.upload(GenMipmaps::No, chunk.tiles())?;
                    }
                }

                WindowEvent::FramebufferSize(..) => {
                    back_buffer = surface.context.back_buffer()?;
                }

                _ => {}
            }
        }

        let render = surface
            .context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |pipeline, mut shade_gate| {
                    let bound_texture = pipeline.bind_texture(&mut texture)?;
                    let bound_world = pipeline.bind_texture(&mut world)?;

                    shade_gate.shade(&mut program, |mut interface, uniforms, mut render_gate| {
                        interface.set(&uniforms.texles, bound_texture.binding());
                        interface.set(&uniforms.world, bound_world.binding());
                        interface.set(&uniforms.world_size, Chunk::SIZE as u32);

                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&quad)
                        })
                    })
                },
            )
            .assume();

        if render.is_ok() {
            surface.context.window.swap_buffers();
        } else {
            break 'main;
        }
    }

    Ok(())
}
