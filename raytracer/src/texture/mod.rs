pub mod perlin;

use crate::basic::{
    clamp,
    vec3::{Color, Point3},
};
use image::*;
use perlin::Perlin;
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[derive(Default, Clone, Copy)]
pub struct SolidColor {
    pub color_value: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        (*self).color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture<T:Texture,U:Texture> {
    pub odd: T,
    pub even: U,
}

impl<T:Texture,U:Texture> CheckerTexture<T,U> {
    pub fn creat(a: Color, b: Color) -> CheckerTexture<SolidColor,SolidColor> {
        CheckerTexture {
            odd: SolidColor { color_value: b },
            even: SolidColor { color_value: a },
        }
    }
}

impl<T:Texture,U:Texture> Texture for CheckerTexture<T,U> {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            (*self).odd.value(u, v, p)
        } else {
            (*self).even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color { e: [1.0; 3] }
            * 0.5
            * (1.0 + ((*self).scale * p.z() + 10.0 * (*self).noise.turb(p, 7)).sin())
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

#[derive(Clone)]
pub struct ImageTexture {
    data: DynamicImage,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn creat(file: &str) -> ImageTexture {
        let ret = image::open(file).unwrap();
        let w = ret.width() as usize;
        let h = ret.height() as usize;
        ImageTexture {
            data: ret,
            width: w,
            height: h,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u * (*self).width as f64) as usize;
        let mut j = (v * (*self).height as f64) as usize;

        i = if i >= (*self).width {
            (*self).width - 1
        } else {
            i
        };
        j = if j >= (*self).height {
            (*self).height - 1
        } else {
            j
        };

        let color_scale = 1.0 / 255.0;

        let pixel = (*self).data.get_pixel(i as u32, j as u32);
        Color {
            e: [
                pixel.0[0] as f64 * color_scale,
                pixel.0[1] as f64 * color_scale,
                pixel.0[2] as f64 * color_scale,
            ],
        }
    }
}
