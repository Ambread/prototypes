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

    let (rules, width, height) = {
        let img = image::io::Reader::open(args.input)?.decode()?.to_rgba8();
        let rules = generate_rules(&img);
        (rules, img.width(), img.height())
    };

    let mut tiles = {
        let initial = rules.keys().copied().collect();
        vec![vec![initial; height as usize]; height as usize]
    };
    let mut output = Image::new(width, height);

    let mut rng = thread_rng();

    while let Some((x, y, tile)) = find_lowest_entropy(&mut tiles, &rules) {
        let collapsed = {
            let pixels: Vec<_> = tile.iter().collect();

            let weights = pixels.iter().map(|pixel| rules[pixel].count);
            let dist = WeightedIndex::new(weights)?;

            *pixels[dist.sample(&mut rng)]
        };

        tile.retain(|pixel| *pixel == collapsed);
        output.put_pixel(x as u32, y as u32, collapsed);
    }

    output.save("wave-collapse-output.png")?;

    Ok(())
}

fn find_lowest_entropy<'a>(
    tiles: &'a mut [Vec<HashSet<Pixel>>],
    rules: &'a HashMap<Pixel, Rule>,
) -> Option<(usize, usize, &'a mut HashSet<Pixel>)> {
    tiles
        .iter_mut()
        .enumerate()
        .flat_map(|(x, column)| {
            column
                .iter_mut()
                .enumerate()
                .map(move |(y, tile)| (x, y, tile))
        })
        .filter(|(_, _, tile)| tile.len() > 1)
        .map(|(x, y, tile)| {
            let entropy = shannon_entropy_for_tile(tile, rules);
            (x, y, tile, entropy)
        })
        .min_by(|a, b| a.3.partial_cmp(&b.3).unwrap())
        .map(|(x, y, tile, _)| (x, y, tile))
}

fn shannon_entropy_for_tile(tile: &HashSet<Pixel>, rules: &HashMap<Pixel, Rule>) -> f64 {
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
