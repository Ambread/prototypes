use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _, pipeline::PipelineState, render_state::RenderState, tess::Mode,
};
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::time::Instant;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

#[derive(Debug, Clone, Copy, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Debug, Clone, Copy, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    position: VertexPosition,
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

const VERTICES: [Vertex; 3] = [
    Vertex::new(
        VertexPosition::new([-0.5, -0.5]),
        VertexRGB::new([255, 0, 0]),
    ),
    Vertex::new(
        VertexPosition::new([0.5, -0.5]),
        VertexRGB::new([0, 255, 0]),
    ),
    Vertex::new(VertexPosition::new([0., 0.5]), VertexRGB::new([0, 0, 255])),
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

    let start_time = Instant::now();
    let back_buffer = surface.context.back_buffer().unwrap();

    let mut program = surface
        .context
        .new_shader_program::<VertexSemantics, (), ()>()
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

    'main: loop {
        surface.context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'main
                }

                _ => {}
            }
        }

        let time = start_time.elapsed().as_millis() as f32 * 1e-3;
        let background_color = [time.cos(), time.sin(), 0.5, 1.0];

        let render = surface
            .context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(background_color),
                |_, mut shade_gate| {
                    shade_gate.shade(&mut program, |_, _, mut render_gate| {
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
