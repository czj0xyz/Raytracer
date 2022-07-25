pub mod camera;
pub mod onb;
pub mod ray;
pub mod vec3;
use std::f64::consts::PI;

pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn fmax(a: f64, b: f64) -> f64 {
    if a < b {
        b
    } else {
        a
    }
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
