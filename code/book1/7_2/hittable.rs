use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Default, Copy, Clone, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        (*self).front_face = dot(r.get_dir(), outward_normal) < 0.0;
        (*self).normal = if (*self).front_face {
            outward_normal
        } else {
            Vec3 { e: [0.0; 3] } - outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
