use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct XyRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: Arc<dyn Material>,
}

impl Hittable for XyRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = ((*self).k - r.get_start().z()) / r.get_dir().z();
        if t < t_min || t > t_max {
            false
        } else {
            let x = r.get_start().x() + t * r.get_dir().x();
            let y = r.get_start().y() + t * r.get_dir().y();
            if x < (*self).x0 || x > (*self).x1 || y < (*self).y0 || y > (*self).y1 {
                false
            } else {
                rec.u = (x - (*self).x0) / ((*self).x1 - (*self).x0);
                rec.v = (y - (*self).y0) / ((*self).y1 - (*self).y0);
                rec.t = t;
                let outward_normal = Vec3 { e: [0.0, 0.0, 1.0] };
                rec.set_face_normal(r, outward_normal);
                rec.mat_ptr = Some((*self).mp.clone());
                rec.p = r.at(t);
                true
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            min: Point3 {
                e: [(*self).x0, (*self).y0, (*self).k - 0.0001],
            },
            max: Point3 {
                e: [(*self).x1, (*self).y1, (*self).k + 0.0001],
            },
        };
        true
    }
}
