use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgba};

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
    let relations = generate_rules(&img);
    dbg!(relations);

    img.save(args.output)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
struct Rule {
    count: u32,
    up: HashSet<Pixel>,
    down: HashSet<Pixel>,
    left: HashSet<Pixel>,
    right: HashSet<Pixel>,
}

fn generate_rules(img: &Image) -> HashMap<Pixel, Rule> {
    let mut rules = HashMap::<Pixel, Rule>::new();

    for (x, y, pixel) in img.enumerate_pixels() {
        let rule = rules.entry(*pixel).or_default();
        rule.count += 1;

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
