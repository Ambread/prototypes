use crate::{assets::Assets, chunk::Chunk};
use luminance::{
    context::GraphicsContext,
    pixel::{NormRGB8UI, R8UI},
    texture::{Dim2, Dim2Array, GenMipmaps, MagFilter, MinFilter, Sampler, Texture, TextureError},
};
use luminance_gl::GL33;
use luminance_glfw::GlfwSurface;

/// Texture array for all the tile sprites
pub type TileTexture = Texture<GL33, Dim2Array, NormRGB8UI>;

/// Texture array for all the tile sprites
pub fn create_tile_texture(
    surface: &mut GlfwSurface,
    assets: &Assets,
) -> Result<TileTexture, TextureError> {
    let mut tile_texture = surface.context.new_texture::<Dim2Array, NormRGB8UI>(
        (
            assets.tile_data.texture_size.cast().unwrap().into(),
            assets.tile_data.texture_count as u32,
        ),
        2,
        Sampler {
            mag_filter: MagFilter::Nearest,
            ..Default::default()
        },
    )?;

    // Upload the texture array
    tile_texture.upload_raw(GenMipmaps::Yes, &assets.tile_sprites)?;

    Ok(tile_texture)
}

/// Texture to hold the tiles of the world
pub type WorldTexture = Texture<GL33, Dim2, R8UI>;

/// Texture to hold the tiles of the world
pub fn create_world_texture(surface: &mut GlfwSurface) -> Result<WorldTexture, TextureError> {
    surface.context.new_texture::<Dim2, R8UI>(
        [Chunk::SIZE as u32, Chunk::SIZE as u32],
        0,
        Sampler {
            // Integer textures must use `Nearest` otherwise it'll read all zeros
            mag_filter: MagFilter::Nearest,
            min_filter: MinFilter::Nearest,
            ..Default::default()
        },
    )
}
