use crate::basic::{
    onb::Onb,
    vec3::{dot, random_double, unit_vector, Vec3},
};
use std::f64::consts::PI;

pub trait Pdf {
    fn value(&self, dir: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3 { e: [x, y, z] }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn creat(w: Vec3) -> CosinePdf {
        let mut ret: Onb = Default::default();
        ret.build_from_w(w);

        CosinePdf { uvw: ret }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, dir: Vec3) -> f64 {
        let cosine = dot(unit_vector(dir), (*self).uvw.w());

        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        (*self).uvw.local_vec(random_cosine_direction())
    }
}
