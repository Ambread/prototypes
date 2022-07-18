use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgba};
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};

#[derive(Debug, Clone, Parser)]
struct Args {
    input: PathBuf,
    #[clap(short, long, default_value = "wave-collapse-output.png")]
    output: PathBuf,
}

type Pixel = Rgba<u8>;
type Image = ImageBuffer<Pixel, Vec<u8>>;

fn main() -> Result<()> {
    let args = Args::parse();
    let img = image::io::Reader::open(args.input)?.decode()?.to_rgba8();
    let rules = generate_rules(&img);
    let mut rng = thread_rng();

    let initial = Tile::Potential(rules.keys().copied().collect());
    let mut tiles = vec![vec![initial; img.height() as usize]; img.width() as usize];
    let mut output = Image::new(img.width(), img.height());

    while let Some((x, y, tile)) = find_lowest_entropy(&mut tiles, &rules) {
        dbg!((x, y));
        let pixels: Vec<_> = tile.iter().collect();
        let weights = pixels.iter().map(|pixel| rules[pixel].count);
        let dist = WeightedIndex::new(weights)?;
        let pixel = *pixels[dist.sample(&mut rng)];
        tiles[x][y] = Tile::Collapsed(pixel);
        output.put_pixel(x as u32, y as u32, pixel);
    }

    output.save("wave-collapse-output.png")?;

    Ok(())
}

#[derive(Debug, Clone)]
enum Tile {
    Collapsed(Pixel),
    Potential(HashSet<Pixel>),
}

impl Tile {
    fn as_potential(&self) -> Option<&HashSet<Pixel>> {
        if let Self::Potential(tile) = self {
            Some(tile)
        } else {
            None
        }
    }
}

fn find_lowest_entropy<'a>(
    tiles: &'a mut [Vec<Tile>],
    rules: &'a HashMap<Rgba<u8>, Rule>,
) -> Option<(usize, usize, &'a HashSet<Rgba<u8>>)> {
    tiles
        .iter()
        .enumerate()
        .flat_map(|(x, column)| column.iter().enumerate().map(move |(y, tile)| (x, y, tile)))
        .filter_map(|(x, y, tile)| {
            let tile = tile.as_potential()?;
            let entropy = shannon_entropy_for_tile(tile, rules);
            Some((x, y, tile, entropy))
        })
        .min_by(|a, b| a.3.partial_cmp(&b.3).unwrap())
        .map(|(x, y, tile, _)| (x, y, tile))
}

fn shannon_entropy_for_tile(tile: &HashSet<Rgba<u8>>, rules: &HashMap<Rgba<u8>, Rule>) -> f64 {
    let sum = tile.iter().map(|pixel| rules[pixel].count).sum::<f64>();
    -tile
        .iter()
        .map(|pixel| {
            let weight = rules[pixel].count / sum;
            weight * weight.log10()
        })
        .sum::<f64>()
}

#[derive(Debug, Clone, Default)]
struct Rule {
    count: f64,
    up: HashSet<Pixel>,
    down: HashSet<Pixel>,
    left: HashSet<Pixel>,
    right: HashSet<Pixel>,
}

fn generate_rules(img: &Image) -> HashMap<Pixel, Rule> {
    let mut rules = HashMap::<Pixel, Rule>::new();

    for (x, y, pixel) in img.enumerate_pixels() {
        let rule = rules.entry(*pixel).or_default();
        rule.count += 1.0;

        let directions = [
            (&mut rule.up, Some(x), y.checked_sub(1)),
            (&mut rule.down, Some(x), y.checked_add(1)),
            (&mut rule.left, x.checked_sub(1), Some(y)),
            (&mut rule.right, x.checked_add(1), Some(y)),
        ];

        for (side, x, y) in directions {
            x.zip(y)
                .and_then(|(x, y)| img.get_pixel_checked(x, y))
                .map(|pixel| side.insert(*pixel));
        }
    }

    rules
}
