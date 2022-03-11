use anyhow::Result;
use glfw::{Context as _, Key, SwapInterval, WindowEvent, WindowMode};
use luminance::{
    context::GraphicsContext, pipeline::PipelineState, render_state::RenderState, tess::Mode,
};
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::{GlfwSurface, GlfwSurfaceError};

use crate::GameChannels;

pub fn render(channels: GameChannels, title: &str) -> Result<()> {
    let surface = GlfwSurface::new(|glfw| {
        let (mut window, events) = glfw
            .create_window(400, 400, title, WindowMode::Windowed)
            .ok_or(GlfwSurfaceError::UserError(()))?;

        window.make_current();
        window.set_all_polling(true);
        glfw.set_swap_interval(SwapInterval::Sync(1));

        Ok((window, events))
    })
    .unwrap();

    let mut context = surface.context;
    let events = surface.events_rx;
    let back_buffer = context.back_buffer().expect("back buffer");

    let mut current_color = [0.0; 4];

    let triangle = context
        .new_tess()
        .set_vertices(VERTICES)
        .set_mode(Mode::Triangle)
        .build()?;

    let mut program = context
        .new_shader_program::<VertexSemantics, (), ()>()
        .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)?
        .ignore_warnings();

    while !context.window.should_close() {
        context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let WindowEvent::Key(key, _, _, _) = event {
                let color = match key {
                    Key::R => [1.0, 0.0, 0.0, 1.0],
                    Key::G => [0.0, 1.0, 0.0, 1.0],
                    Key::B => [0.0, 0.0, 1.0, 1.0],
                    _ => continue,
                };

                channels
                    .to_net
                    .send(crate::Message::SetColor { color })
                    .unwrap();
            }
        }

        if let Ok(crate::Message::SetColor { color }) = channels.from_net.try_recv() {
            current_color = color;
        }

        context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(current_color),
                |_, mut shade_gate| {
                    shade_gate.shade(&mut program, |_, _, mut render_gate| {
                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&triangle)
                        })
                    })
                },
            )
            .assume()
            .into_result()?;

        context.window.swap_buffers();
    }

    Ok(())
}

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
#[allow(dead_code)]
pub struct Vertex {
    position: VertexPosition,
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

const VERTICES: &[Vertex] = &[
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
