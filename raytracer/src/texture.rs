use crate::perlin::Perlin;
use crate::vec3::{Color, Point3};
use std::sync::Arc;
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[derive(Default, Clone)]
pub struct SolidColor {
    pub color_value: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        (*self).color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn creat(a: Color, b: Color) -> CheckerTexture {
        CheckerTexture {
            odd: Arc::new(SolidColor { color_value: b }),
            even: Arc::new(SolidColor { color_value: a }),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            (*self).odd.value(u, v, p)
        } else {
            (*self).even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color { e: [1.0; 3] } * (*self).noise.noise(p * (*self).scale)
    }
}

impl NoiseTexture {
    pub fn creat(sc: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Default::default(),
            scale: sc,
        }
    }
}
