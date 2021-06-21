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
            WorldGenerator::Flat(gen) => self.generate_flat(gen, assets),
            WorldGenerator::Noise(gen) => self.generate_noise(gen, assets),
        }
    }

    fn generate_flat(&mut self, gen: &FlatWorldGenerator, assets: &Assets) {
        // Retrieve the sprite index for the tile
        let tile = assets.tile_data.tiles.get(&gen.tile).unwrap();

        // Set the entire array to it
        self.tiles = [tile.sprite; Self::SIZE * Self::SIZE];
    }

    fn generate_noise(&mut self, gen: &NoiseWorldGenerator, assets: &Assets) {
        // Create noise from seed
        let noise = Perlin::new().set_seed(gen.seed);

        // Map each tile id to their data
        let tiles = gen
            .tiles
            .iter()
            .map(|it| assets.tile_data.tiles.get(it))
            .collect::<Option<Vec<_>>>()
            .unwrap();

        // For every tile
        for (index, tile) in self.tiles.iter_mut().enumerate() {
            // Map chunk index into (X, Y) pair
            let index = Vector2::new(index % Self::SIZE, index / Self::SIZE);

            // Relate to global grid
            let index = index.cast().unwrap() + self.position * Self::SIZE as isize;

            // Scale by world gen settings
            let index = index.cast().unwrap() / gen.scale;

            // Get noise value for position
            let index: [f64; 2] = index.into();
            let output = noise.get(index);

            // Map from `-1.0..1.0` to `0..tile.len()`
            let output = output * tiles.len() as f64;
            let output = output.trunc() as usize;
            let output = output.min(tiles.len() - 1);

            // Retrieve the sprite index for the tile
            let output = tiles[output].sprite;

            // Update buffer with new sprite id
            *tile = output as u8;
        }
    }
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, tiles) in self.tiles.chunks(Chunk::SIZE).enumerate() {
            // Write current index as sidebar
            write!(f, "{}  ", i)?;

            // Write all sprite ids in row
            for tile in tiles {
                write!(f, "{}", tile)?;
            }

            // Newline
            writeln!(f)?;
        }
        Ok(())
    }
}
