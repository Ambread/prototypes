use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pack {
    tiles: Vec<Tile>,
    world_generator: WorldGenerator,
    texture_atlases: Vec<TextureAtlas>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tile {
    name: String,
    sprite: Sprite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Sprite {
    atlas: usize,
    index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum WorldGenerator {
    Flat {
        tile: String,
    },
    Noise {
        seed: f64,
        scale: f64,
        tiles: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TextureAtlas {
    path: PathBuf,
    texture_size: usize,
    texture_count: usize,
}
