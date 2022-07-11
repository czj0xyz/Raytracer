use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.get_start() - (*self).center;
        let a = r.get_dir().length_squared();
        let half_b = dot(oc, r.get_dir());
        let c = oc.length_squared() - (*self).radius * (*self).radius;
        let mut ret = true;
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            ret = false;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    ret = false;
                }
            }
            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal = (rec.p - (*self).center) / (*self).radius;
            rec.set_face_normal(r, outward_normal);
            rec.mat_ptr = (*self).mat_ptr.clone();
        }
        ret
    }
}
