use std::ops::{Add, AddAssign, Mul};

pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn write(&mut self, samples_per_pixel: i32) -> String {
        let scale = 1.0 / samples_per_pixel as f64;

        self.r *= scale;
        self.g *= scale;
        self.b *= scale;

        format!(
            "{} {} {}",
            (256.0 * self.r.clamp(0.0, 0.999)) as i32,
            (256.0 * self.g.clamp(0.0, 0.999)) as i32,
            (256.0 * self.b.clamp(0.0, 0.999)) as i32,
        )
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        rhs * self
    }
}
