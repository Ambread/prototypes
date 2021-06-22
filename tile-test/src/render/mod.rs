mod fields;
mod shader;
mod uniforms;

use crate::{assets::Assets, chunk::Chunk};
use anyhow::Result;
use fields::Quad;
use glfw::Context as _;
use luminance::{
    context::GraphicsContext as _, pipeline::PipelineState, render_state::RenderState,
    texture::GenMipmaps,
};
use luminance_glfw::GlfwSurface;

use self::fields::{
    create_back_buffer, create_program, create_quad, create_tile_texture, create_world_texture,
    BackBuffer, Program, TileTexture, WorldTexture,
};

pub struct Renderer {
    back_buffer: BackBuffer,
    program: Program,
    quad: Quad,
    tile_texture: TileTexture,
    world_texture: WorldTexture,
}

impl Renderer {
    pub fn new(surface: &mut GlfwSurface, assets: &Assets) -> Result<Self> {
        Ok(Self {
            back_buffer: create_back_buffer(surface)?,
            program: create_program(surface)?,
            quad: create_quad(surface)?,
            tile_texture: create_tile_texture(surface, assets)?,
            world_texture: create_world_texture(surface)?,
        })
    }

    pub fn reload_assets(&mut self, surface: &mut GlfwSurface, assets: &Assets) -> Result<()> {
        self.tile_texture = create_tile_texture(surface, assets)?;
        Ok(())
    }

    // Needed when the screen is resized
    pub fn refresh_back_buffer(&mut self, surface: &mut GlfwSurface) -> Result<()> {
        self.back_buffer = surface.context.back_buffer()?;
        Ok(())
    }

    // How `Renderer` accepts `Chunk`'s tiles
    pub fn upload_world_texture(&mut self, tiles: &[u8]) -> Result<()> {
        self.world_texture.upload(GenMipmaps::No, tiles)?;
        Ok(())
    }

    pub fn render(&mut self, surface: &mut GlfwSurface) -> Result<()> {
        // HACK: Borrowing issues, so just split to begin with
        // Rust somehow likes this better
        let Self {
            program,
            quad,
            tile_texture,
            world_texture,
            ..
        } = self;

        surface
            .context
            .new_pipeline_gate()
            .pipeline(
                &self.back_buffer,
                &PipelineState::default(),
                |pipeline, mut shade_gate| {
                    // Any textures you have to bind here
                    let bound_texture = pipeline.bind_texture(tile_texture)?;
                    let bound_world = pipeline.bind_texture(world_texture)?;

                    shade_gate.shade(program, |mut interface, uniforms, mut render_gate| {
                        // Assign uniforms
                        interface.set(&uniforms.tile_texture, bound_texture.binding());
                        interface.set(&uniforms.world_texture, bound_world.binding());
                        interface.set(&uniforms.world_size, Chunk::SIZE as u32);

                        // Render some stuff
                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&*quad)
                        })
                    })
                },
            )
            .assume()
            .into_result()?;

        // Put the new stuff onto the screen
        surface.context.window.swap_buffers();

        Ok(())
    }
}
