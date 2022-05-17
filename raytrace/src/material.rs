use crate::{
    hittable::{HitRecord, Ray},
    vec3::{Color, Scalar, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: Scalar },
    Dielectric { index_of_refraction: Scalar },
}

impl Material {
    pub fn scatter(self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian { albedo } => {
                let origin = hit_record.point;
                let mut direction = hit_record.normal + Vec3::random_unit_vector();

                if direction.near_zero() {
                    direction = origin;
                }

                let scattered = Ray { origin, direction };
                Some((scattered, albedo))
            }

            Material::Metal { albedo, fuzz } => {
                let origin = hit_record.point;
                let direction = ray_in.direction.unit_length().reflect(hit_record.normal)
                    + fuzz * Vec3::random_in_unit_sphere();
                let scattered = Ray { origin, direction };

                (scattered.direction.dot(hit_record.normal) > 0.0).then(|| (scattered, albedo))
            }

            Material::Dielectric {
                index_of_refraction,
            } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if hit_record.front_face {
                    1.0 / index_of_refraction
                } else {
                    index_of_refraction
                };

                let direction = ray_in
                    .direction
                    .unit_length()
                    .refract(hit_record.normal, refraction_ratio);

                let scattered = Ray {
                    origin: hit_record.point,
                    direction,
                };

                Some((scattered, attenuation))
            }
        }
    }
}
