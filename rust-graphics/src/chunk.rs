use crate::assets::{Assets, FlatWorldGenerator, NoiseWorldGenerator, WorldGenerator};
use cgmath::Vector2;
use noise::{NoiseFn, Perlin, Seedable};

#[derive(Clone)]
pub struct Chunk {
    tiles: [u8; Self::SIZE * Self::SIZE],
    // DEBUG: Only public for debug controls
    pub position: Vector2<isize>,
}

impl Chunk {
    pub const SIZE: usize = 32;
    const INVERT: bool = false;

    pub fn new(position: Vector2<isize>) -> Self {
        Self {
            position,
            tiles: [0; Self::SIZE * Self::SIZE],
        }
    }

    pub fn tiles(&self) -> &[u8] {
        &self.tiles
    }

    pub fn generate(&mut self, assets: &Assets) {
        match &assets.world_data {
            WorldGenerator::Flat(_) => todo!(),
            WorldGenerator::Noise(gen) => self.generate_noise(gen, assets),
        }
    }

    fn generate_flat(&mut self, gen: &FlatWorldGenerator, assets: &Assets) {}

    fn generate_noise(&mut self, gen: &NoiseWorldGenerator, assets: &Assets) {
        let noise = Perlin::new().set_seed(gen.seed);
        let tiles = gen
            .tiles
            .iter()
            .map(|it| assets.tile_data.tiles.get(it))
            .collect::<Option<Vec<_>>>()
            .unwrap();

        for (i, tile) in self.tiles.iter_mut().enumerate() {
            let i = [
                ((i % Self::SIZE) as isize + self.position.x * Self::SIZE as isize) as f64
                    / gen.scale,
                ((i / Self::SIZE) as isize + self.position.y * Self::SIZE as isize) as f64
                    / gen.scale,
            ];

            // Get noise value for position
            let output = noise.get(i);

            // Map from `-1.0..1.0` to `0..tile.len()`
            let output = output * tiles.len() as f64;
            let output = output.trunc() as usize;
            let output = output.min(tiles.len() - 1);

            // Retrieve the sprite index for the tile
            let output = tiles[output].sprite;

            *tile = output as u8;

            if Self::INVERT {
                *tile = tiles.len() as u8 - *tile;
            }
        }
    }
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, tiles) in self.tiles.chunks(Chunk::SIZE).enumerate() {
            write!(f, "{}  ", i)?;
            for tile in tiles {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
