mod camera;
mod hittable;
mod material;
mod render;
mod vec3;
mod world;

use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread,
};

use clap::Parser;
use eframe::{
    epaint::{ColorImage, TextureHandle},
    CreationContext,
};
use image::{ColorType, ImageFormat};

use crate::render::{render, ImageInfo};

#[derive(Debug, Clone, Parser)]
struct Args {
    width: usize,
    samples_per_pixel: u32,
    output: PathBuf,
}

fn main() {
    eframe::run_native(
        "Raytrace",
        eframe::NativeOptions::default(),
        Box::new(|ctx| Box::new(App::new(ctx))),
    );
}

struct App {
    args: Args,
    image_info: ImageInfo,
    receiver: Receiver<(Vec<u8>, usize)>,
    texture: TextureHandle,
    buffer: Vec<u8>,
}

impl App {
    fn new(ctx: &CreationContext<'_>) -> Self {
        let args = Args::parse();
        let image_info = ImageInfo::new(args.width, args.samples_per_pixel);

        let capacity = image_info.height * image_info.width * 4;
        let blank = vec![0; capacity];
        let buffer = Vec::with_capacity(capacity);

        let size = [image_info.width, image_info.height];
        let image = ColorImage::from_rgba_unmultiplied(size, blank.as_slice());
        let texture = ctx.egui_ctx.load_texture("render", image);

        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || render(sender, image_info));

        Self {
            args,
            image_info,
            receiver,
            texture,
            buffer,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(&self.texture, self.texture.size_vec2());

            if let Ok((mut line, current_height)) = self.receiver.try_recv() {
                dbg!(current_height);

                let size = [self.image_info.width, 1];
                let image = ColorImage::from_rgba_unmultiplied(size, &line);

                self.texture.set_partial([0, current_height - 1], image);

                self.buffer.append(&mut line);

                if current_height == self.image_info.height {
                    println!("Saving...");
                    image::save_buffer_with_format(
                        &self.args.output,
                        &self.buffer,
                        self.image_info.width as _,
                        self.image_info.height as _,
                        ColorType::Rgba8,
                        ImageFormat::Png,
                    )
                    .unwrap();
                    println!("Done!");
                }
            }
        });
    }
}
