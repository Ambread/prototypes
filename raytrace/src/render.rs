use std::{mem, sync::mpsc::Sender};

use anyhow::Result;
use rand::{distributions::Uniform, prelude::Distribution};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    camera::Camera,
    vec3::{Color, Scalar},
    world::world,
};

pub fn render(sender: Sender<(Vec<u8>, usize)>, image_info: ImageInfo) -> Result<()> {
    let camera = Camera::new(image_info.aspect_ratio);
    let world = world();

    let mut line = Vec::with_capacity(image_info.width * 4);

    for current_line in (0..image_info.height).rev() {
        for current_column in 0..image_info.width {
            let uniform = Uniform::from(0.0..1.0);

            let pixel_color: Color = (0..image_info.samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u = (current_column as Scalar + uniform.sample(&mut rng))
                        / (image_info.width as Scalar - 1.0);
                    let v = (current_line as Scalar + uniform.sample(&mut rng))
                        / (image_info.height as Scalar - 1.0);

                    camera.get_ray(u, v).color(&world, image_info.max_depth)
                })
                .sum();

            let scale = 1.0 / image_info.samples_per_pixel as Scalar;

            let r = ((pixel_color.x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
            let g = ((pixel_color.y * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
            let b = ((pixel_color.z * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;

            line.extend([r, g, b, 255]);
        }

        sender.send((mem::take(&mut line), image_info.height - current_line))?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct ImageInfo {
    pub aspect_ratio: Scalar,
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl ImageInfo {
    pub fn new(width: usize, samples_per_pixel: u32) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let height = (width as Scalar / aspect_ratio) as _;
        let max_depth = 50;

        Self {
            aspect_ratio,
            width,
            height,
            samples_per_pixel,
            max_depth,
        }
    }
}
