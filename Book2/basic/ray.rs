use super::vec3::{Point3, Vec3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Ray {
    pub st: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn get_start(&self) -> Point3 {
        (*self).st
    }
    pub fn get_dir(&self) -> Vec3 {
        (*self).dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        (*self).st + (*self).dir * t
    }
    pub fn get_time(&self) -> f64 {
        (*self).tm
    }
}
