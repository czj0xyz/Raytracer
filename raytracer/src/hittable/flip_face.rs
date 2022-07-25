use super::{HitRecord, Hittable};
use crate::basic::ray::Ray;
use crate::bvh::aabb::Aabb;

#[derive(Clone)]
pub struct FlipFace<T: Hittable> {
    pub ptr: T,
}

#[allow(clippy::unnecessary_unwrap)]
impl<T: Hittable> Hittable for FlipFace<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rec = (*self).ptr.hit(r, t_min, t_max);
        if rec.is_none() {
            None
        } else {
            let mut rec = rec.unwrap();
            rec.front_face = !rec.front_face;
            Some(rec)
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        (*self).ptr.bounding_box(t0, t1, output_box)
    }
}
