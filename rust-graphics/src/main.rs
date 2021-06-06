use cgmath::{prelude::*, Vector2, Vector3};
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _, pipeline::PipelineState, render_state::RenderState,
    shader::Uniform, tess::Mode,
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    color: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    view: Uniform<[f32; 2]>,
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

const VERTICES: [Vertex; 3] = [
    Vertex::new(VertexPosition::new([-0.5, -0.5])),
    Vertex::new(VertexPosition::new([0.5, -0.5])),
    Vertex::new(VertexPosition::new([0., 0.5])),
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

    let back_buffer = surface.context.back_buffer().unwrap();

    let mut program = surface
        .context
        .new_shader_program::<VertexSemantics, (), ShaderInterface>()
        .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)
        .unwrap()
        .ignore_warnings();

    let triangle = surface
        .context
        .new_tess()
        .set_vertices(&VERTICES[..])
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    let mut color = Vector3::new(1.0, 1.0, 1.0);
    let mut view = Vector2::new(0.0, 0.0);

    const SPEED: f32 = 0.05;

    'main: loop {
        surface.context.window.glfw.poll_events();

        let mut position = Vector2::new(0.0, 0.0);

        for (_, event) in glfw::flush_messages(&surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'main
                }

                WindowEvent::Key(key, _, Action::Release, _) => match key {
                    // Color
                    Key::R => color[0] = 1.0 - color[0],
                    Key::G => color[1] = 1.0 - color[1],
                    Key::B => color[2] = 1.0 - color[2],

                    // View
                    Key::W => position.y += 1.0,
                    Key::A => position.x -= 1.0,
                    Key::S => position.y -= 1.0,
                    Key::D => position.x += 1.0,

                    _ => {}
                },

                _ => {}
            }
        }

        if !position.is_zero() {
            view += position.normalize() * SPEED;
        }

        let render = surface
            .context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |_, mut shade_gate| {
                    shade_gate.shade(&mut program, |mut interface, uniforms, mut render_gate| {
                        interface.set(&uniforms.color, color.into());
                        interface.set(&uniforms.view, view.into());

                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&triangle)
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
