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

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        (*self).front_face = dot(r.get_dir(), outward_normal) < 0.0;
        (*self).normal = if (*self).front_face {
            outward_normal
        } else {
            Vec3 { e: [0.0; 3] } - outward_normal
        };
    }
    #[allow(clippy::redundant_field_names)]
    #[allow(clippy::many_single_char_names)]
    pub fn creat(
        u: f64,
        v: f64,
        t: f64,
        outward_normal: Vec3,
        r: Ray,
        p: Point3,
        mat_ptr: &'a dyn Material,
    ) -> HitRecord<'a> {
        let mut ret = HitRecord {
            p: p,
            t: t,
            u: u,
            v: v,
            normal: Vec3 { e: [0.0; 3] },
            front_face: false,
            mat_ptr: mat_ptr,
        };
        ret.set_face_normal(r, outward_normal);
        ret
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool;
}
