#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod assets;
pub mod chunk;
pub mod renderer;

use anyhow::Result;
use assets::Assets;
use cgmath::Vector2;
use chunk::Chunk;
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _,
    framebuffer::Framebuffer,
    pipeline::{PipelineState, TextureBinding},
    pixel::{NormRGB8UI, NormUnsigned, Unsigned, R8UI},
    render_state::RenderState,
    shader::{Program, Uniform},
    tess::{Interleaved, Mode, Tess},
    texture::{Dim2, Dim2Array, GenMipmaps, MagFilter, MinFilter, Sampler, Texture},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_gl::gl33::GL33;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::env::current_dir;

struct Main {
    assets: Assets,
    surface: GlfwSurface,
    chunk: Chunk,
    should_quit: bool,
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

        let chunk = Chunk::new(Vector2::new(0, 0));

        let mut this = Self {
            assets,
            surface,
            chunk,
            should_quit: false,
        };

        this.generate()?;

        Ok(this)
    }

    fn generate(&mut self) -> Result<()> {
        self.chunk.generate(&self.assets)?;

        self.world_texture
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
                    self.back_buffer = self.surface.context.back_buffer()?;
                }

                _ => {}
            }
        }

        if should_regenerate {
            self.generate()?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let mut main = Main::new()?;

    loop {
        main.handle_events()?;

        if main.should_quit {
            return Ok(());
        }

        main.render()?;
    }
}
