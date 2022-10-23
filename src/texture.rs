use std::sync::Arc;

use crate::{colour::Colour, vec3::Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour;
}

pub struct SolidColour {
    colour_value: Colour,
}

impl SolidColour {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            colour_value: Colour::new(red, green, blue),
        }
    }

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
