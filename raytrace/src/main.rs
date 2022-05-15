mod hittable;
mod vec3;

use std::io::Write;

use hittable::{HitRecord, Hittable, Ray};
use rand::{distributions::Uniform, prelude::Distribution};
use rayon::{
    current_num_threads,
    iter::{IntoParallelIterator, ParallelIterator},
};
use vec3::{Color, Point3, Scalar, Vec3};

use crate::hittable::Sphere;

#[derive(Debug, Clone, Copy)]
struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    fn get_ray(&self, u: Scalar, v: Scalar) -> Ray {
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;

        Ray {
            origin: self.origin,
            direction,
        }
    }
}

fn ray_color(ray: Ray, world: &impl Hittable) -> Color {
    let mut hit_record = HitRecord::default();
    if world.hit(ray, 0.0, Scalar::INFINITY, &mut hit_record) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let direction = ray.direction.unit_length();
    let t = 0.5 * (direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080;
    let image_height = (image_width as Scalar / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // World
    let world = vec![
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{image_width} {image_height}\n255");

    eprintln!("Using {} threads", current_num_threads());
    for j in (0..image_height).rev() {
        let percent_left = (j as Scalar / image_height as Scalar) * 100.0;
        eprint!("\rProgress: {:.2}%", 100.0 - percent_left);
        std::io::stderr().flush().unwrap();

        for i in 0..image_width {
            let uniform = Uniform::from(0.0..1.0);

            let pixel_color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u =
                        (i as Scalar + uniform.sample(&mut rng)) / (image_width as Scalar - 1.0);
                    let v =
                        (j as Scalar + uniform.sample(&mut rng)) / (image_height as Scalar - 1.0);

                    let ray = camera.get_ray(u, v);
                    ray_color(ray, &world)
                })
                .sum();

            write_color(pixel_color, samples_per_pixel);
        }
    }
}

fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as Scalar;

    let r = ((pixel_color.x * scale).clamp(0.0, 0.999) * 256.0) as u8;
    let g = ((pixel_color.y * scale).clamp(0.0, 0.999) * 256.0) as u8;
    let b = ((pixel_color.z * scale).clamp(0.0, 0.999) * 256.0) as u8;

    println!("{r} {g} {b}");
}
