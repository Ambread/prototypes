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

    let all_colors: HashSet<_> = rules.keys().copied().collect();
    let mut output = vec![all_colors; (img.width() * img.height()).try_into()?];

    while let Some(tile) = find_lowest_entropy(&mut output, &rules) {
        let pixels: Vec<_> = tile.iter().collect();
        let weights = pixels.iter().map(|pixel| rules[pixel].count);
        let dist = WeightedIndex::new(weights)?;
        let decided = *pixels[dist.sample(&mut rng)];
        tile.retain(|pixel| *pixel == decided);
    }

    Ok(())
}

fn find_lowest_entropy<'a>(
    output: &'a mut [HashSet<Rgba<u8>>],
    rules: &'a HashMap<Rgba<u8>, Rule>,
) -> Option<&'a mut HashSet<Rgba<u8>>> {
    Some(
        output
            .iter_mut()
            .filter(|tile| tile.len() > 1)
            .map(|tile| {
                let entropy = shannon_entropy_for_tile(tile, rules);
                (tile, entropy)
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())?
            .0,
    )
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
