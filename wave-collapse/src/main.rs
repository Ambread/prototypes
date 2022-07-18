use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use image::{GenericImageView, Rgba};

#[derive(Debug, Clone, Parser)]
struct Args {
    input: PathBuf,
    #[clap(short, long, default_value = "wave-collapse-output.png")]
    output: PathBuf,
}

type Pixel = Rgba<u8>;

#[derive(Debug, Clone, Default)]
struct Relations {
    up: HashSet<Pixel>,
    down: HashSet<Pixel>,
    left: HashSet<Pixel>,
    right: HashSet<Pixel>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let img = image::io::Reader::open(args.input)?.decode()?.to_rgba8();

    let mut relations = HashMap::<Pixel, Relations>::new();

    for (x, y, pixel) in img.enumerate_pixels() {
        let relation = relations.entry(*pixel).or_default();

        let directions = [(&mut relation.up, Some(x), y.checked_sub(1))];

        for (side, x, y) in directions {
            if let Some(pixel) = x.zip(y).and_then(|(x, y)| img.get_pixel_checked(x, y)) {
                side.insert(*pixel);
            }
        }
    }

    img.save(args.output)?;
    Ok(())
}
