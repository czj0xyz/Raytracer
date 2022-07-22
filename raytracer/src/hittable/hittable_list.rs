use super::{HitRecord, Hittable};
use crate::basic::ray::Ray;
use crate::bvh::aabb::{surrounding_box, Aabb};
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        (*self).objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut ret : Option<HitRecord>  = Default::default();
        for object in &(*self).objects {
            let rec = object.hit(r, t_min, closest_so_far);
            if rec.is_some() {
                let tmp = rec.unwrap();
                closest_so_far = tmp.t;
                ret=Some(tmp);
            }
        }
        ret
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        if (*self).objects.is_empty() {
            false
        } else {
            let mut temp_box: Aabb = Default::default();
            let mut first = true;
            for obj in &(*self).objects {
                if !obj.bounding_box(t0, t1, &mut temp_box) {
                    return false
                }
                *output_box = if first {
                    temp_box
                } else {
                    surrounding_box(*output_box, temp_box)
                };
                first = false;
            }
            true
        }
    }
}
