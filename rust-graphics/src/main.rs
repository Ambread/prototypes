#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod assets;
pub mod chunk;
pub mod renderer;

use anyhow::Result;
use assets::Assets;
use cgmath::Vector2;
use chunk::Chunk;
use glfw::{Action, Key, WindowEvent};
use luminance::texture::GenMipmaps;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use renderer::Renderer;
use std::env::current_dir;

struct Main {
    pub assets: Assets,
    pub renderer: Renderer,
    pub chunk: Chunk,
    pub should_quit: bool,

    // LIBRARY BUG: `surface` must drop after `renderer` to prevent segfault
    // https://github.com/phaazon/luminance-rs/issues/304
    pub surface: GlfwSurface,
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
            surface,
            renderer,
            chunk,
            should_quit: false,
        };

        this.generate()?;

        Ok(this)
    }

    fn generate(&mut self) -> Result<()> {
        self.chunk.generate(&self.assets)?;

        self.renderer
            .world_texture
            .upload(GenMipmaps::No, self.chunk.tiles())?;

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        self.surface.context.window.glfw.poll_events();

        let mut should_regenerate = false;

        for (_, event) in glfw::flush_messages(&self.surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    self.should_quit = true;
                    return Ok(());
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

                WindowEvent::FramebufferSize(..) => {
                    self.renderer.back_buffer = self.surface.context.back_buffer()?;
                }

                _ => {}
            }
        }

        if should_regenerate {
            self.generate()?;
        }

        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        self.renderer.render(&mut self.surface)
    }
}

fn main() -> Result<()> {
    let mut main = Main::new()?;

    loop {
        main.handle_events()?;

        if main.should_quit {
            break;
        }

        main.render()?;
    }

    return dbg!(Ok(()));
}
