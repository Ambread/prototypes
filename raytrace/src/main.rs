mod camera;
mod hittable;
mod material;
mod vec3;
mod world;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use eventuals::{Closed, EventualWriter};
use futures::never::Never;
use rand::{distributions::Uniform, prelude::Distribution};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vec3::{Color, Scalar};

use crate::{camera::Camera, world::world};

#[derive(Debug, Parser)]
struct Args {
    width: u32,
    samples: u32,
    output: PathBuf,
}

#[tokio::main]
async fn main() {
    let mut reader = eventuals::Eventual::spawn(aaaa).subscribe();
    while let Ok(value) = reader.next().await {
        dbg!(value.len());
    }
}

async fn aaaa(mut writer: EventualWriter<Vec<u8>>) -> Result<Never, Closed> {
    let args = Args::parse();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = args.width;
    let image_height = (image_width as Scalar / aspect_ratio) as u32;
    let samples_per_pixel = args.samples;
    let max_depth = 50;

    // Camera
    let camera = Camera::new();
    let world = world();

    // Render
    let mut image = Vec::with_capacity((image_width * image_height * 3) as usize);

    for j in (0..image_height).rev() {
        writer.write(image.clone());
        for i in 0..image_width {
            let uniform = Uniform::from(0.0..1.0);

            let pixel_color: Color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u =
                        (i as Scalar + uniform.sample(&mut rng)) / (image_width as Scalar - 1.0);
                    let v =
                        (j as Scalar + uniform.sample(&mut rng)) / (image_height as Scalar - 1.0);

                    camera.get_ray(u, v).color(&world, max_depth)
                })
                .sum();

            let scale = 1.0 / samples_per_pixel as Scalar;

            image.push(((pixel_color.x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8);
            image.push(((pixel_color.y * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8);
            image.push(((pixel_color.z * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8);
        }
    }

    Err(Closed)
}
