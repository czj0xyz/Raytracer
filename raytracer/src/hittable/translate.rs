use super::{HitRecord, Hittable};
use crate::basic::{ray::Ray, vec3::Vec3};
use crate::bvh::aabb::Aabb;
use std::sync::Arc;

#[derive(Clone)]
pub struct Translate {
    pub ptr: Arc<dyn Hittable>,
    pub offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray {
            st: r.get_start() - (*self).offset,
            dir: r.get_dir(),
            tm: r.get_time(),
        };
        if !(*self).ptr.hit(moved_r, t_min, t_max, rec) {
            false
        } else {
            rec.p += (*self).offset;
            rec.set_face_normal(moved_r, rec.normal);
            true
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        if !(*self).ptr.bounding_box(t0, t1, output_box) {
            false
        } else {
            *output_box = Aabb {
                min: output_box.min() + (*self).offset,
                max: output_box.max() + (*self).offset,
            };
            true
        }
    }
}
