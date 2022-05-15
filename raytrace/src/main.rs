mod hittable;
mod vec3;

use std::io::Write;

use hittable::{HitRecord, Hittable, Ray};
use vec3::{Color, Point3, Scalar, Vec3};

use crate::hittable::Sphere;

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
    let aspect_ratio = 16.0 / 8.0;
    let image_width = 400;
    let image_height = (image_width as Scalar / aspect_ratio) as u32;

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
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j:03}");
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = f64::from(i) / f64::from(image_width - 1);
            let v = f64::from(j) / f64::from(image_height - 1);

            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray { origin, direction };

            let color = ray_color(ray, &world);
            write_color(color);
        }
    }
}

fn write_color(color: Color) {
    let r = (255.999 * color.x) as u8;
    let g = (255.999 * color.y) as u8;
    let b = (255.999 * color.z) as u8;

    println!("{r} {g} {b}");
}
