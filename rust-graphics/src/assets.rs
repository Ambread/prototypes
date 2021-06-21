use anyhow::Result;
use cgmath::Vector2;
use image::{GenericImageView, RgbImage};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone)]
pub struct Assets {
    pub tile_data: Tiles,
    pub tile_sprites: Vec<u8>,
    pub world_data: WorldGenerator,
}

impl Assets {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let tile_data = fs::read_to_string(path.join("tiles.json"))?;
        let tile_sprites = image::open(path.join("tiles.png"))?;
        let world_data = fs::read_to_string(path.join("world.json"))?;

        let tile_data = serde_json::from_str(&tile_data)?;
        let tile_sprites = Self::parse_atlas(tile_sprites.to_rgb8(), &tile_data);
        let world_data = serde_json::from_str(&world_data)?;

        Ok(Self {
            tile_data,
            tile_sprites,
            world_data,
        })
    }

    fn parse_atlas(atlas: RgbImage, tile_data: &Tiles) -> Vec<u8> {
        let (atlas_width, atlas_height) = atlas.dimensions();
        let (texture_width, texture_height) = tile_data.texture_size.into();

        (0..atlas_height)
            .step_by(texture_height)
            .flat_map(|y| (0..atlas_width).step_by(texture_width).map(move |x| (x, y)))
            .take(tile_data.texture_count)
            .flat_map(|(x, y)| {
                atlas
                    .view(x, y, texture_width as u32, texture_height as u32)
                    .to_image()
                    .into_raw()
                    .into_iter()
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tiles {
    pub texture_size: Vector2<usize>,
    pub texture_count: usize,
    pub tiles: HashMap<String, Tile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub sprites: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorldGenerator {
    Flat(FlatWorldGenerator),
    Noise(NoiseWorldGenerator),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatWorldGenerator {
    pub tile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseWorldGenerator {
    pub seed: u32,
    pub scale: f64,
    pub tiles: Vec<NoiseTileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseTileEntry {
    pub name: String,
    pub bias: usize,
}
