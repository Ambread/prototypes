#![windows_subsystem = "windows"]

use anyhow::Result;
use cgmath::{prelude::*, Vector2};
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _,
    pipeline::{PipelineState, TextureBinding},
    pixel::{Floating, RGB32F},
    render_state::RenderState,
    shader::Uniform,
    tess::Mode,
    texture::{Dim2, GenMipmaps, MagFilter, Sampler},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use noise::{NoiseFn, Perlin, Seedable};
use rand::random;
use std::collections::HashSet;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    view: Uniform<[f32; 2]>,
    #[uniform(unbound)]
    texles: Uniform<TextureBinding<Dim2, Floating>>,
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

const TEXTURE_SIZE: usize = 128;

fn fill_texles() -> [(f32, f32, f32); TEXTURE_SIZE * TEXTURE_SIZE] {
    let mut texles = [(0.0, 0.0, 0.0); TEXTURE_SIZE * TEXTURE_SIZE];
    let noise = (
        Perlin::new().set_seed(random()),
        Perlin::new().set_seed(random()),
        Perlin::new().set_seed(random()),
    );

    for (i, texle) in texles.iter_mut().enumerate() {
        let i = [
            (i % TEXTURE_SIZE) as f64 / TEXTURE_SIZE as f64,
            (i / TEXTURE_SIZE) as f64 / TEXTURE_SIZE as f64,
        ];

        texle.0 = noise.0.get(i) as f32;
        texle.1 = noise.1.get(i) as f32;
        texle.2 = noise.2.get(i) as f32;
    }

    texles
}

fn main() -> Result<()> {
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

    let mut texture = surface.context.new_texture::<Dim2, RGB32F>(
        [TEXTURE_SIZE as u32, TEXTURE_SIZE as u32],
        0,
        Sampler {
            mag_filter: MagFilter::Nearest,
            ..Default::default()
        },
    )?;

    texture.upload(GenMipmaps::No, &fill_texles())?;

    let mut view = Vector2::new(0.0, 0.0);

    let mut pressed_keys = HashSet::new();

    const SPEED: f32 = 0.05;

    'main: loop {
        surface.context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'main
                }

                WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    texture.upload(GenMipmaps::No, &fill_texles())?
                }

                WindowEvent::Key(Key::R, _, Action::Release, _) => {
                    view = Vector2::new(0.0, 0.0);
                }

                WindowEvent::Key(key, _, Action::Press, _) => {
                    pressed_keys.insert(key);
                }
                WindowEvent::Key(key, _, Action::Release, _) => {
                    pressed_keys.remove(&key);
                }

                WindowEvent::FramebufferSize(..) => {
                    back_buffer = surface.context.back_buffer()?;
                }

                _ => {}
            }
        }

        {
            let mut position = Vector2::new(0.0, 0.0);

            if pressed_keys.contains(&Key::W) {
                position.y += 1.0;
            }
            if pressed_keys.contains(&Key::A) {
                position.x -= 1.0;
            }
            if pressed_keys.contains(&Key::S) {
                position.y -= 1.0;
            }
            if pressed_keys.contains(&Key::D) {
                position.x += 1.0;
            }

            if !position.is_zero() {
                view += position.normalize() * SPEED;
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

                    shade_gate.shade(&mut program, |mut interface, uniforms, mut render_gate| {
                        interface.set(&uniforms.view, view.into());
                        interface.set(&uniforms.texles, bound_texture.binding());

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
