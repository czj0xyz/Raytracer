use std::sync::Arc;

use super::{HitRecord, Hittable};
use crate::basic::ray::Ray;
use crate::bvh::aabb::{surrounding_box, Aabb};

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        (*self).objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec: HitRecord = Default::default();
        for object in &(*self).objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = (temp_rec.clone()).t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        if (*self).objects.is_empty() {
            false
        } else {
            let mut temp_box: Aabb = Default::default();
            let mut first = true;
            let mut ret = true;
            for obj in &(*self).objects {
                if obj.bounding_box(t0, t1, &mut temp_box) {
                    ret = false;
                }
                *output_box = if first {
                    temp_box
                } else {
                    surrounding_box(*output_box, temp_box)
                };
                first = false;
            }
            ret
        }
    }
}
