#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod assets;
pub mod chunk;
pub mod renderer;

use anyhow::Result;
use assets::Assets;
use cgmath::Vector2;
use chunk::Chunk;
use glfw::{Action, Key, WindowEvent};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use renderer::Renderer;
use std::env::current_dir;

struct Main {
    assets: Assets,
    renderer: Renderer,
    chunk: Chunk,

    // LIBRARY BUG: `surface` must drop after `renderer` to prevent segfault
    // https://github.com/phaazon/luminance-rs/issues/304
    surface: GlfwSurface,
}

impl Main {
    fn new() -> Result<Self> {
        let assets = Assets::from_path(current_dir()?.join("assets"))?;

        let mut surface = GlfwSurface::new_gl33(
            "Rust Graphics Test",
            WindowOpt::default().set_dim(WindowDim::Windowed {
                width: 768,
                height: 768,
            }),
        )?;

        let renderer = Renderer::new(&mut surface, &assets)?;

        let chunk = Chunk::new(Vector2::new(0, 0));

        let mut this = Self {
            assets,
            renderer,
            chunk,
            surface,
        };

        this.generate()?;

        Ok(this)
    }

    fn generate(&mut self) -> Result<()> {
        self.chunk.generate(&self.assets)?;

        self.renderer.upload_world_texture(self.chunk.tiles())?;

        Ok(())
    }

    fn handle_events(&mut self) -> Result<bool> {
        self.surface.context.window.glfw.poll_events();

        // HACK: Can't borrow self inside the loop so use flags and do things afterwards
        let mut should_regenerate = false;
        let mut should_refresh_back_buffer = false;

        for (_, event) in glfw::flush_messages(&self.surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    return Ok(true);
                }

                WindowEvent::Key(key, _, Action::Press, _) => {
                    match key {
                        Key::W => self.chunk.position.y += 1,
                        Key::A => self.chunk.position.x -= 1,
                        Key::S => self.chunk.position.y -= 1,
                        Key::D => self.chunk.position.x += 1,

                        Key::P => println!("{:?}", self.chunk),

                        _ => {}
                    }

                    if let Key::W | Key::A | Key::S | Key::D = key {
                        should_regenerate = true;
                    }
                }

                WindowEvent::FramebufferSize(..) => should_refresh_back_buffer = true,

                _ => {}
            }
        }

        if should_regenerate {
            self.generate()?;
        }

        if should_refresh_back_buffer {
            self.renderer.refresh_back_buffer(&mut self.surface)?;
        }

        Ok(false)
    }

    fn render(&mut self) -> Result<()> {
        self.renderer.render(&mut self.surface)
    }
}

fn main() -> Result<()> {
    let mut main = Main::new()?;

    loop {
        let should_quit = main.handle_events()?;

        if should_quit {
            return Ok(());
        }

        main.render()?;
    }
}
