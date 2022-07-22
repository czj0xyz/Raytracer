use crate::hittable::{HitRecord, Hittable};

use crate::basic::{
    ray::Ray,
    vec3::{Point3, Vec3},
};
use crate::bvh::aabb::Aabb;
use crate::material::Material;

#[derive(Clone)]
pub struct XyRect<T:Material>{
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: T,
}

impl<T:Material> Hittable for XyRect<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = ((*self).k - r.get_start().z()) / r.get_dir().z();
        if t < t_min || t > t_max {
            None
        } else {
            let x = r.get_start().x() + t * r.get_dir().x();
            let y = r.get_start().y() + t * r.get_dir().y();
            if x < (*self).x0 || x > (*self).x1 || y < (*self).y0 || y > (*self).y1 {
                None
            } else {
                let u = (x - (*self).x0) / ((*self).x1 - (*self).x0);
                let v = (y - (*self).y0) / ((*self).y1 - (*self).y0);
                let outward_normal = Vec3 { e: [0.0, 0.0, 1.0] };
                let rec = HitRecord::creat(u,v,t,outward_normal,r,r.at(t),&(*self).mp);
                Some(rec)
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

#[derive(Clone)]
pub struct XzRect<T> {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: T,
}

impl<T:Material> Hittable for XzRect<T>{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = ((*self).k - r.get_start().y()) / r.get_dir().y();
        if t < t_min || t > t_max {
            None
        } else {
            let x = r.get_start().x() + t * r.get_dir().x();
            let z = r.get_start().z() + t * r.get_dir().z();
            if x < (*self).x0 || x > (*self).x1 || z < (*self).z0 || z > (*self).z1 {
                None
            } else {
                let u = (x - (*self).x0) / ((*self).x1 - (*self).x0);
                let v = (z - (*self).z0) / ((*self).z1 - (*self).z0);
                let outward_normal = Vec3 { e: [0.0, 1.0, 0.0] };
                let rec = HitRecord::creat(u,v,t,outward_normal,r,r.at(t),&(*self).mp);
                Some(rec)
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            min: Point3 {
                e: [(*self).x0, (*self).k - 0.0001, (*self).z0],
            },
            max: Point3 {
                e: [(*self).x1, (*self).k + 0.0001, (*self).z1],
            },
        };
        true
    }
}

#[derive(Clone)]
pub struct YzRect<T> {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: T,
}

impl<T:Material> Hittable for YzRect<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = ((*self).k - r.get_start().x()) / r.get_dir().x();
        if t < t_min || t > t_max {
            None
        } else {
            let y = r.get_start().y() + t * r.get_dir().y();
            let z = r.get_start().z() + t * r.get_dir().z();
            if z < (*self).z0 || z > (*self).z1 || y < (*self).y0 || y > (*self).y1 {
                None
            } else {
                let u = (y - (*self).y0) / ((*self).y1 - (*self).y0);
                let v = (z - (*self).z0) / ((*self).z1 - (*self).z0);
                let outward_normal = Vec3 { e: [1.0, 0.0, 0.0] };
                let rec = HitRecord::creat(u,v,t,outward_normal,r,r.at(t),&(*self).mp);
                Some(rec)
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            min: Point3 {
                e: [(*self).k - 0.0001, (*self).y0, (*self).z0],
            },
            max: Point3 {
                e: [(*self).k + 0.0001, (*self).y1, (*self).z1],
            },
        };
        true
    }
}
