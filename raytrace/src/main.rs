mod camera;
mod hittable;
mod material;
mod render;
mod vec3;
mod world;

use std::{
    io::Write,
    path::PathBuf,
    sync::mpsc::{self},
    thread,
};

use anyhow::Result;
use clap::Parser;
use image::{ColorType, ImageFormat};
use rayon::current_num_threads;

use crate::{
    render::{render, ImageInfo},
    vec3::Scalar,
};

#[derive(Debug, Parser)]
struct Args {
    width: u32,
    samples_per_pixel: u32,
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let image_info = ImageInfo::new(args.width, args.samples_per_pixel);

    println!("Using {} threads", current_num_threads());

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || render(sender, image_info));

    let mut buffer = Vec::new();

    while let Ok((mut line, current_line)) = receiver.recv() {
        buffer.append(&mut line);

        let percent_left = (current_line as Scalar / image_info.height as Scalar) * 100.0;
        print!("\rProgress: {:.2}%", 100.0 - percent_left);
        std::io::stderr().flush()?;
    }

    println!("\nSaving...");
    image::save_buffer_with_format(
        args.output,
        &buffer,
        image_info.width,
        image_info.height,
        ColorType::Rgb8,
        ImageFormat::Png,
    )?;
    println!("Done!");

    Ok(())
}
