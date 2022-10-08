use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use crate::{
    colour::Colour,
    utils::{random_double, random_double_within_range},
};

pub type Point3 = Vec3;

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        Self::new(random_double(), random_double(), random_double())
    }

    pub fn random_within_range(min: f64, max: f64) -> Self {
        Self::new(
            random_double_within_range(min, max),
            random_double_within_range(min, max),
            random_double_within_range(min, max),
        )
    }

    pub fn length_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_within_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(&self) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(self) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Return true if the vector is close to zero in all dimensions
    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
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

impl Add<Colour> for Vec3 {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour::new(self.x + rhs.r, self.y + rhs.g, self.z + rhs.b)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
