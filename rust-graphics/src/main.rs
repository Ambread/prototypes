#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod assets;
pub mod chunk;
pub mod input;
pub mod render;

use anyhow::Result;
use assets::Assets;
use cgmath::Vector2;
use chunk::Chunk;
use glfw::{Action, Key, MouseButtonLeft, WindowEvent};
use input::Input;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use render::Renderer;
use std::env::current_dir;

fn main() -> Result<()> {
    let mut main = Main::new()?;

    loop {
        let should_quit = main.handle_events()?;

        if should_quit {
            return Ok(());
        }

        main.handle_input()?;
        main.render()?;
    }
}

struct Main {
    assets: Assets,
    renderer: Renderer,
    chunk: Chunk,
    input: Input,

    // LIBRARY BUG: `surface` must drop after `renderer` to prevent segfault
    // https://github.com/phaazon/luminance-rs/issues/304
    surface: GlfwSurface,
}

impl Main {
    const WINDOW_SIZE: u32 = 768;

    fn new() -> Result<Self> {
        let assets = Self::load_assets()?;

        const WINDOW_SIZE: u32 = 768;

        let mut surface = GlfwSurface::new_gl33(
            "Rust Graphics Test",
            WindowOpt::default().set_dim(WindowDim::Windowed {
                width: WINDOW_SIZE,
                height: WINDOW_SIZE,
            }),
        )?;

        // TODO: Add proper resizing logic
        // For now just lock it
        surface.context.window.set_size_limits(
            Some(WINDOW_SIZE),
            Some(WINDOW_SIZE),
            Some(WINDOW_SIZE),
            Some(WINDOW_SIZE),
        );

        let renderer = Renderer::new(&mut surface, &assets)?;

        let chunk = Chunk::new(Vector2::new(0, 0));

        let input = Input::new();

        let mut this = Self {
            assets,
            renderer,
            chunk,
            input,
            surface,
        };

        this.generate()?;

        Ok(this)
    }

    fn load_assets() -> Result<Assets> {
        Assets::from_path(current_dir()?.join("assets"))
    }

    fn reload(&mut self) -> Result<()> {
        self.assets = Self::load_assets()?;

        self.renderer
            .reload_assets(&mut self.surface, &self.assets)?;
        self.generate()
    }

    fn generate(&mut self) -> Result<()> {
        self.chunk.generate(&self.assets)?;

        self.renderer.upload_world_texture(self.chunk.tiles())?;

        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if self.input.was_key_pressed(Key::Space) {
            self.reload()?;
        }

        if self.input.was_key_pressed(Key::P) {
            println!("Chunk = {:?}", self.chunk.position);
        }
        if self.input.was_key_pressed(Key::O) {
            println!("{:?}", self.chunk);
        }

        if self.input.was_key_pressed(Key::W) {
            self.chunk.position.y += 1;
            self.generate()?;
        } else if self.input.was_key_pressed(Key::A) {
            self.chunk.position.x -= 1;
            self.generate()?;
        } else if self.input.was_key_pressed(Key::S) {
            self.chunk.position.y -= 1;
            self.generate()?;
        } else if self.input.was_key_pressed(Key::D) {
            self.chunk.position.x += 1;
            self.generate()?;
        }

        if self.input.was_key_pressed(Key::L) {
            println!("Mouse = {:?}", self.input.mouse_position());
        }

        if self.input.was_key_pressed(Key::K) {
            println!("Current = {:?}", self.current_tile());
        }

        if self.input.is_mouse_held(MouseButtonLeft) {
            if let Some(current_tile) = self.current_tile() {
                self.chunk.set_tile(current_tile, "water", &self.assets);
                self.renderer.upload_world_texture(self.chunk.tiles())?;
            }
        }

        Ok(())
    }

    fn current_tile(&self) -> Option<Vector2<usize>> {
        self.input
            .mouse_position()
            .map(|it| it * (Chunk::SIZE as f64 / Self::WINDOW_SIZE as f64))
            .map(|it| it.map(|it| it.trunc()).cast().unwrap())
    }

    fn handle_events(&mut self) -> Result<bool> {
        self.surface.context.window.glfw.poll_events();

        // HACK: Can't borrow self inside the loop so use flags and do things afterwards
        let mut should_refresh_back_buffer = false;

        for (_, event) in glfw::flush_messages(&self.surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    return Ok(true);
                }

                WindowEvent::FramebufferSize(..) => should_refresh_back_buffer = true,

                _ => {}
            }

            self.input.handle(&event);
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
