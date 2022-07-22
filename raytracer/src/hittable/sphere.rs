use crate::basic::{
    ray::Ray,
    vec3::{dot, Point3},
};
use crate::bvh::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use std::f64::consts::PI;

#[derive(Default, Clone)]
pub struct Sphere<T:Material+Clone> {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: T,//Material
}

pub fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;
    *u = phi / (2.0 * PI);
    *v = theta / PI;
}

impl<T:Material+Clone> Hittable for Sphere<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.get_start() - (*self).center;
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
                    return None
                }
            }
            let outward_normal_=(r.at(root) - (*self).center) / (*self).radius;
            let mut u_ =0.0;
            let mut v_=0.0;
            get_sphere_uv(outward_normal_,&mut u_,&mut v_);
            let rec = HitRecord::creat(u_,v_,root,
                outward_normal_,r,r.at(root),& (*self).mat_ptr);
            Some(rec)
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            min: (*self).center
                - Point3 {
                    e: [(*self).radius; 3],
                },
            max: (*self).center
                + Point3 {
                    e: [(*self).radius; 3],
                },
        };
        true
    }
}
