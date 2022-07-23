use crate::basic::{
    ray::Ray,
    vec3::{dot, Point3},
};
use crate::bvh::aabb::{surrounding_box, Aabb};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

#[derive(Clone)]
pub struct MovingSphere<T: Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: T,
}

impl<T: Material> MovingSphere<T> {
    pub fn center(&self, time: f64) -> Point3 {
        (*self).center0
            + ((*self).center1 - (*self).center0) * (time - (*self).time0)
                / ((*self).time1 - (*self).time0)
    }
}

impl<T: Material> Hittable for MovingSphere<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.get_start() - (*self).center(r.get_time());
        let a = r.get_dir().length_squared();
        let half_b = dot(oc, r.get_dir());
        let c = oc.length_squared() - (*self).radius * (*self).radius;
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }
            let rec = HitRecord::creat(
                0.0,
                0.0,
                root,
                (r.at(root) - (*self).center(r.get_time())) / (*self).radius,
                r,
                r.at(root),
                &(*self).mat_ptr,
            );
            Some(rec)
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        let box0 = Aabb {
            min: (*self).center(t0)
                - Point3 {
                    e: [(*self).radius; 3],
                },
            max: (*self).center(t0)
                + Point3 {
                    e: [(*self).radius; 3],
                },
        };

        let box1 = Aabb {
            min: (*self).center(t1)
                - Point3 {
                    e: [(*self).radius; 3],
                },
            max: (*self).center(t1)
                + Point3 {
                    e: [(*self).radius; 3],
                },
        };

        *output_box = surrounding_box(box0, box1);
        true
    }
}
