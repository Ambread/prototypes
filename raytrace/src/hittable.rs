use crate::vec3::{Color, Point3, Scalar, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, time: Scalar) -> Point3 {
        self.origin + time * self.direction
    }

    pub fn color(self, world: &impl Hittable, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_record) = world.hit(self, 0.001, Scalar::INFINITY) {
            let target = hit_record.point + hit_record.normal + Vec3::random_unit_vector();
            let ray = Ray {
                origin: hit_record.point,
                direction: target - hit_record.point,
            };
            return 0.5 * ray.color(world, depth - 1);
        }

        let direction = self.direction.unit_length();
        let t = 0.5 * (direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: Scalar,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, t: Scalar, ray: Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: Scalar, t_max: Scalar) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: Scalar,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: Scalar, t_max: Scalar) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant = discriminant.sqrt();

        let mut root = (-half_b - discriminant) / a;
        if root < t_min || t_max < root {
            root = (-half_b - discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        };

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;

        Some(HitRecord::new(point, root, ray, outward_normal))
    }
}

impl<T> Hittable for Vec<T>
where
    T: Hittable,
{
    fn hit(&self, r: Ray, t_min: Scalar, t_max: Scalar) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(object_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = object_record.t;
                hit_record = Some(object_record);
            }
        }

        hit_record
    }
}
