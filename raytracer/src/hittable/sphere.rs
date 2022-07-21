use crate::basic::{
    ray::Ray,
    vec3::{dot, Point3},
};
use crate::bvh::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.get_start() - (*self).center;
        let a = r.get_dir().length_squared();
        let half_b = dot(oc, r.get_dir());
        let c = oc.length_squared() - (*self).radius * (*self).radius;
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return false
                }
            }
            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal = (rec.p - (*self).center) / (*self).radius;
            rec.set_face_normal(r, outward_normal);
            Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
            rec.mat_ptr = (*self).mat_ptr.clone();
        }
        true
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
