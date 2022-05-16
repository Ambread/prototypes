use crate::{
    hittable::{HitRecord, Ray},
    vec3::{Color, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian { albedo } => {
                let origin = hit_record.point;
                let mut direction = hit_record.normal + Vec3::random_unit_vector();

                if direction.near_zero() {
                    direction = origin;
                }

                let scattered = Ray { origin, direction };
                Some((scattered, *albedo))
            }

            Material::Metal { albedo } => {
                let origin = hit_record.point;
                let direction = ray_in.direction.unit_length().reflect(hit_record.normal);
                let scattered = Ray { origin, direction };

                (scattered.direction.dot(hit_record.normal) > 0.0).then(|| (scattered, *albedo))
            }
        }
    }
}
