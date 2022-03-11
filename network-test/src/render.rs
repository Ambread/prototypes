use std::collections::HashSet;

use anyhow::Result;
use glfw::{Context as _, Key, SwapInterval, WindowEvent, WindowMode};
use luminance::{
    context::GraphicsContext,
    pipeline::PipelineState,
    render_state::RenderState,
    tess::{Mode, Tess},
};
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::{GlfwSurface, GlfwSurfaceError};

use crate::{message::Message, GameChannels};

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

    let mut program = context
        .new_shader_program::<VertexSemantics, (), ()>()
        .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)?
        .ignore_warnings();

    let mut tess = context
        .new_tess()
        .set_vertices([])
        .set_mode(Mode::Triangle)
        .build()?;

    let mut current_color = [0.0; 4];
    let mut self_id = 0;
    let mut players = HashSet::new();

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

                channels.to_net.send(Message::SetColor { color }).unwrap();
            }
        }

        while let Ok(message) = channels.from_net.try_recv() {
            match message {
                Message::SelfJoined { id } => self_id = id,
                Message::SetColor { color } => current_color = color,
                Message::PlayerJoined { id } => {
                    println!("Player {id} joined");
                    players.insert(id);

                    let vertices: Vec<_> = players
                        .iter()
                        .flat_map(|id| quad(*id as f32 * 0.1, [255, 0, 0]))
                        .collect();

                    tess = context
                        .new_tess()
                        .set_vertices(vertices.as_slice())
                        .set_mode(Mode::Triangle)
                        .build()?;
                }
            }
        }

        context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(current_color),
                |_, mut shade_gate| {
                    shade_gate.shade(&mut program, |_, _, mut render_gate| {
                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&tess)
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

fn quad(size: f32, color: [u8; 3]) -> Vec<Vertex> {
    [
        [1.0, 1.0],
        [1.0, -1.0],
        [-1.0, 1.0],
        [-1.0, -1.0],
        [-1.0, 1.0],
        [1.0, -1.0],
    ]
    .into_iter()
    .map(|position| {
        Vertex::new(
            VertexPosition::new(position.map(|sign| size * sign)),
            VertexRGB::new(color),
        )
    })
    .collect()
}
