use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

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
}
