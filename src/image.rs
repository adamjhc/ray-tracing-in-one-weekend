pub struct Image {
    pub aspect_ratio: f64,
    pub width: i32,
    pub height: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

impl Image {
    pub fn new(aspect_ratio: f64, width: i32, samples_per_pixel: i32, max_depth: i32) -> Self {
        Self {
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as i32,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn set_aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self.set_height()
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self.width = width;
        self.set_height()
    }

    fn set_height(mut self) -> Self {
        self.height = (self.width as f64 / self.aspect_ratio) as i32;
        self
    }

    pub fn set_samples_per_pixel(mut self, samples_per_pixel: i32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }
}
