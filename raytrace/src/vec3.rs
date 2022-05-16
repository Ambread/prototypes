use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Neg, Range, Sub},
};

use rand::{distributions::Uniform, prelude::Distribution};

pub type Scalar = f64;
pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vec3 {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    pub fn length(self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_length(self) -> Self {
        self / self.length()
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_length()
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0..1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random(range: Range<Scalar>) -> Self {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(range);

        Self {
            x: uniform.sample(&mut rng),
            y: uniform.sample(&mut rng),
            z: uniform.sample(&mut rng),
        }
    }

    pub fn dot(self, rhs: Self) -> Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::default(), |a, b| a + b)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Scalar> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Scalar {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<Scalar> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
