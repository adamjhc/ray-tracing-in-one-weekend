use rand::{random, thread_rng, Rng};
use std::ops::{Add, AddAssign, Mul};

#[derive(Default, Clone, Copy)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn write_to_rgb(&self, samples_per_pixel: i32) -> String {
        let (r, g, b) = self.to_rgb(samples_per_pixel);

        format!("{r} {g} {b}")
    }

    pub fn to_u32(self, samples_per_pixel: i32) -> u32 {
        let (r, g, b) = self.to_rgb(samples_per_pixel);

        ((r as u32) << 16) | ((g as u32) << 8) | b as u32
    }

    fn to_rgb(self, samples_per_pixel: i32) -> (u32, u32, u32) {
        // Divide the colour by the number of samples and gamma-correct for gamma=2.0
        let scale = 1.0 / samples_per_pixel as f64;

        let r = (scale * self.r).sqrt();
        let g = (scale * self.g).sqrt();
        let b = (scale * self.b).sqrt();

        // Write the translated [0, 255] value of each colour component

        (
            (256.0 * r.clamp(0.0, 0.999)) as u32,
            (256.0 * g.clamp(0.0, 0.999)) as u32,
            (256.0 * b.clamp(0.0, 0.999)) as u32,
        )
    }

    pub fn random() -> Self {
        Self::new(random(), random(), random())
    }

    pub fn random_within_range(min: f64, max: f64) -> Self {
        let mut thread_rng = thread_rng();
        Self::new(
            thread_rng.gen_range(min..=max),
            thread_rng.gen_range(min..=max),
            thread_rng.gen_range(min..=max),
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

impl Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
