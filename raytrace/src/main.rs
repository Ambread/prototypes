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

use crate::render::{render, ImageInfo};

#[derive(Debug, Clone, Parser)]
struct Args {
    width: u32,
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
    image_info: ImageInfo,
    receiver: Receiver<(Vec<u8>, u32)>,
    texture: TextureHandle,
}

impl App {
    fn new(ctx: &CreationContext<'_>) -> Self {
        let args = Args::parse();
        let image_info = ImageInfo::new(args.width, args.samples_per_pixel);
        let buffer = vec![0; (image_info.height * image_info.width * 4) as usize];

        let size = [image_info.width as usize, image_info.height as usize];
        let image = ColorImage::from_rgba_unmultiplied(size, buffer.as_slice());
        let texture = ctx.egui_ctx.load_texture("render", image);

        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || render(sender, image_info));

        Self {
            image_info,
            receiver,
            texture,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(&self.texture, self.texture.size_vec2());

            if let Ok((line, current_height)) = self.receiver.try_recv() {
                dbg!(current_height);

                let size = [self.image_info.width as usize, 1];
                let image = ColorImage::from_rgba_unmultiplied(size, &line);

                self.texture
                    .set_partial([0, current_height as usize - 1], image);
            }
        });
    }
}
