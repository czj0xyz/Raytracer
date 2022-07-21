pub mod aarect;
pub mod constant_medium;
pub mod hittable_list;
pub mod moving_sphere;
pub mod mybox;
pub mod rotate_y;
pub mod sphere;
pub mod translate;

use crate::basic::{
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};
use crate::bvh::aabb::Aabb;
use crate::material::Material;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: Option<Arc<dyn Material>>,
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

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool;
}