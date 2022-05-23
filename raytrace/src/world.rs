use crate::{
    hittable::{Hittable, Sphere},
    material::Material,
    vec3::{Color, Point3},
};

pub fn world() -> impl Hittable {
    let ground = Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        },
    };

    let center = Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5),
        },
    };

    let left = Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Dielectric {
            index_of_refraction: 1.5,
        },
    };

    let right = Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal {
            albedo: Color::new(0.8, 0.6, 0.2),
            fuzz: 0.0,
        },
    };

    vec![ground, center, left, right]
}
