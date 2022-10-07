use std::f64::consts::PI;

use rand::random;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a random real in [0, 1)
pub fn random_double() -> f64 {
    random()
}

/// Returns a random real in [min, max)
pub fn random_double_within_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
