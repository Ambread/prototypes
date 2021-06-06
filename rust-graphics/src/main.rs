use cgmath::{prelude::*, Vector2};
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _,
    pipeline::{PipelineState, TextureBinding},
    pixel::{Floating, R32F},
    render_state::RenderState,
    shader::Uniform,
    tess::Mode,
    texture::{Dim2, GenMipmaps, MagFilter, Sampler},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
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

fn main() {
    let mut surface = GlfwSurface::new_gl33(
        "Tile Test",
        WindowOpt::default().set_dim(WindowDim::Windowed {
            width: 960,
            height: 540,
        }),
    )
    .unwrap();

    let mut back_buffer = surface.context.back_buffer().unwrap();

    let mut program = surface
        .context
        .new_shader_program::<VertexSemantics, (), ShaderInterface>()
        .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)
        .unwrap()
        .ignore_warnings();

    let quad = surface
        .context
        .new_tess()
        .set_vertices(&VERTICES[..])
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    let mut texture = surface
        .context
        .new_texture::<Dim2, R32F>(
            [16, 16],
            0,
            Sampler {
                mag_filter: MagFilter::Nearest,
                ..Default::default()
            },
        )
        .unwrap();

    let mut texles = [0.0; 16 * 16];

    for (i, texle) in texles.iter_mut().enumerate() {
        let is_even_x = i % 2 == 0;
        let is_even_y = (i / 16) % 2 == 0;
        *texle = if is_even_x ^ is_even_y { 1.0 } else { 0.0 };
    }

    texture.upload_raw(GenMipmaps::No, &texles).unwrap();

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

                WindowEvent::Key(key, _, Action::Press, _) => {
                    pressed_keys.insert(key);
                }
                WindowEvent::Key(key, _, Action::Release, _) => {
                    pressed_keys.remove(&key);
                }

                WindowEvent::FramebufferSize(..) => {
                    back_buffer = surface.context.back_buffer().unwrap();
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
                    let bound_texture = pipeline.bind_texture(&mut texture).unwrap();

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
}
