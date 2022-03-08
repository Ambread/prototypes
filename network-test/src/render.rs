use anyhow::Result;
use glfw::{Context as _, SwapInterval, WindowMode};
use luminance::{context::GraphicsContext, pipeline::PipelineState};
use luminance_glfw::{GlfwSurface, GlfwSurfaceError};
use std::time::Instant;

pub fn render() -> Result<()> {
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

    let start_time = Instant::now();

    while !context.window.should_close() {
        context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {}

        let t = start_time.elapsed().as_secs_f32();
        let color = [t.cos(), t.sin(), 0.5, 1.];

        context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(color),
                |_, _| Ok(()),
            )
            .assume();

        context.window.swap_buffers();
    }

    Ok(())
}
