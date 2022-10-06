use std::f64::consts::PI;

use rand::{random, thread_rng, Rng};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    random()
}

pub fn random_double_within_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}
