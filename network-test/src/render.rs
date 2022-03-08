use anyhow::Result;
use glfw::{Context as _, Key, SwapInterval, WindowEvent, WindowMode};
use luminance::{context::GraphicsContext, pipeline::PipelineState};
use luminance_glfw::{GlfwSurface, GlfwSurfaceError};

use crate::GameChannels;

pub fn render(channels: GameChannels) -> Result<()> {
    let surface = GlfwSurface::new(|glfw| {
        let (mut window, events) = glfw
            .create_window(90, 90, "Wew", WindowMode::Windowed)
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

    while !context.window.should_close() {
        context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let WindowEvent::Key(key, _, _, _) = event {
                let color = match key {
                    Key::R => [1.0, 0.0, 0.0, 1.0],
                    Key::G => [1.0, 0.0, 0.0, 1.0],
                    Key::B => [1.0, 0.0, 0.0, 1.0],
                    _ => continue,
                };

                channels.to_net.send(crate::Message::SetColor { color })?;
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
                |_, _| Ok(()),
            )
            .assume();

        context.window.swap_buffers();
    }

    Ok(())
}
