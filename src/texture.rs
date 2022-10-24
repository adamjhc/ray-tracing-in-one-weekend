use crate::{colour::Colour, perlin::Perlin, vec3::Point3};
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour;
}

pub struct SolidColour {
    colour_value: Colour,
}

impl SolidColour {
    pub fn from(colour: Colour) -> Self {
        Self {
            colour_value: colour,
        }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Colour {
        self.colour_value
    }
}

pub struct CheckerTexture {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { even, odd }
    }

    pub fn from_colour(colour1: Colour, colour2: Colour) -> Self {
        Self::new(
            Arc::new(SolidColour::from(colour1)),
            Arc::new(SolidColour::from(colour2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Colour {
        Colour::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(&(self.scale * p), 7)).sin())
    }
}
