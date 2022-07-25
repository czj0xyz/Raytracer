use super::vec3::{cross, unit_vector, Vec3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Onb {
    pub axis: [Vec3; 3],
}

impl Onb {
    pub fn u(&self) -> Vec3 {
        (*self).axis[0]
    }
    pub fn v(&self) -> Vec3 {
        (*self).axis[1]
    }
    pub fn w(&self) -> Vec3 {
        (*self).axis[2]
    }
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        (*self).axis[0] * a + (*self).axis[1] * b + (*self).axis[2] * c
    }
    pub fn local_vec(&self, v: Vec3) -> Vec3 {
        (*self).axis[0] * v.e[0] + (*self).axis[1] * v.e[1] + (*self).axis[2] * v.e[2]
    }
    pub fn build_from_w(&mut self, n: Vec3) {
        (*self).axis[2] = unit_vector(n);
        let a = if self.w().x().abs() > 0.9 {
            Vec3 { e: [0.0, 1.0, 0.0] }
        } else {
            Vec3 { e: [1.0, 0.0, 0.0] }
        };
        (*self).axis[1] = unit_vector(cross(self.w(), a));
        (*self).axis[0] = cross(self.w(), self.v());
    }
}
