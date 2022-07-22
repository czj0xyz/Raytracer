use super::{HitRecord, Hittable};
use crate::basic::{ray::Ray, vec3::Vec3};
use crate::bvh::aabb::Aabb;

#[derive(Clone)]
pub struct Translate<T:Hittable> {
    pub ptr: T,//Hittable
    pub offset: Vec3,
}

impl<T:Hittable> Hittable for Translate<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray {
            st: r.get_start() - (*self).offset,
            dir: r.get_dir(),
            tm: r.get_time(),
        };
        let rec = (*self).ptr.hit(moved_r, t_min, t_max);
        if rec.is_none() {
            None
        } else {
            let mut rec = rec.unwrap();
            rec.p += (*self).offset;
            rec.set_face_normal(moved_r, rec.normal);
            Some(rec)
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
