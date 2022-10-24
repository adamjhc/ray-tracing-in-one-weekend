use crate::{colour::Colour, perlin::Perlin, vec3::Point3};
use std::{path::Path, sync::Arc};

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

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: usize = 3;

    pub fn new(filename: &Path) -> Self {
        let image = image::open(filename)
            .expect("Could not load texture image file")
            .to_rgb8();

        let (width, height) = image.dimensions();

        Self {
            data: image.into_raw(),
            width: width as usize,
            height: height as usize,
            bytes_per_scanline: Self::BYTES_PER_PIXEL * width as usize,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: Point3) -> Colour {
        u = u.clamp(0.0, 1.0);
        v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let colour_scale = 1.0 / 255.0;
        let pixel_location = j * self.bytes_per_scanline + i * Self::BYTES_PER_PIXEL;

        Colour::new(
            colour_scale * self.data[pixel_location] as f64,
            colour_scale * self.data[pixel_location + 1] as f64,
            colour_scale * self.data[pixel_location + 2] as f64,
        )
    }
}
