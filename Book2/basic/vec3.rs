use super::fmin;
use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub fn random_double() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn random_double_lr(min: f64, max: f64) -> f64 {
    min + random_double() * (max - min)
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [
                (*self).e[0] + other.e[0],
                (*self).e[1] + other.e[1],
                (*self).e[2] + other.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = f64;
    fn mul(self, other: Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, x: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * x, self.e[1] * x, self.e[2] * x],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [
                (*self).e[0] * other,
                (*self).e[1] * other,
                (*self).e[2] * other,
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [
                (*self).e[0] - other.e[0],
                (*self).e[1] - other.e[1],
                (*self).e[2] - other.e[2],
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, x: f64) -> Vec3 {
        self * (1.0 / x)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [
                (*self).e[0] / other,
                (*self).e[1] / other,
                (*self).e[2] / other,
            ],
        }
    }
}

impl Vec3 {
    pub fn mul(&self, a: Vec3) -> Vec3 {
        Vec3 {
            e: [
                (*self).e[0] * a.e[0],
                (*self).e[1] * a.e[1],
                (*self).e[2] * a.e[2],
            ],
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        ((*self).e[0]).abs() < s && ((*self).e[1]).abs() < s && ((*self).e[2]).abs() < s
    }

    pub fn length(&self) -> f64 {
        ((*self).e[0] * (*self).e[0] + (*self).e[1] * (*self).e[1] + (*self).e[2] * (*self).e[2])
            .sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (*self).e[0] * (*self).e[0] + (*self).e[1] * (*self).e[1] + (*self).e[2] * (*self).e[2]
    }

    pub fn x(&self) -> f64 {
        (*self).e[0]
    }

    pub fn y(&self) -> f64 {
        (*self).e[1]
    }

    pub fn z(&self) -> f64 {
        (*self).e[2]
    }

    pub fn random() -> Vec3 {
        Vec3 {
            e: [random_double(), random_double(), random_double()],
        }
    }

    pub fn random_lr(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random_double_lr(min, max),
                random_double_lr(min, max),
                random_double_lr(min, max),
            ],
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.e[0] * b.e[0] + a.e[1] * b.e[1] + a.e[2] * b.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_lr(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3 {
            e: [
                random_double_lr(-1.0, 1.0),
                random_double_lr(-1.0, 1.0),
                0.0,
            ],
        };
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn random_int_lr(min: isize, max: isize) -> isize {
    random_double_lr(min as f64, (max + 1) as f64) as isize
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * dot(v, n))
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = fmin(dot(Vec3 { e: [0.0; 3] } - uv, n), 1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * (-(((1.0 - r_out_perp.length_squared()).abs()).sqrt()));
    r_out_perp + r_out_parallel
}

pub type Color = Vec3;

pub type Point3 = Vec3;
