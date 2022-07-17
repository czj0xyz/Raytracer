use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, fmax, fmin, Point3, Vec3};
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::sync::Arc;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        (*self).front_face = dot(r.get_dir(), outward_normal) < 0.0;
        (*self).normal = if (*self).front_face {
            outward_normal
        } else {
            Vec3 { e: [0.0; 3] } - outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool;
}

#[derive(Clone)]
pub struct Translate {
    pub ptr: Arc<dyn Hittable>,
    pub offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray {
            st: r.get_start() - (*self).offset,
            dir: r.get_dir(),
            tm: r.get_time(),
        };
        if !(*self).ptr.hit(moved_r, t_min, t_max, rec) {
            false
        } else {
            rec.p += (*self).offset;
            rec.set_face_normal(moved_r, rec.normal);
            true
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        if !(*self).ptr.bounding_box(t0, t1, output_box) {
            false
        } else {
            *output_box = Aabb {
                min: output_box.min() + (*self).offset,
                max: output_box.max() + (*self).offset,
            };
            true
        }
    }
}

#[derive(Clone)]
pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: Aabb,
}

impl RotateY {
    pub fn creat(p: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta_ = radians.sin();
        let cos_theta_ = radians.cos();
        let mut bbox_ = Default::default();
        let hasbox_ = p.bounding_box(0.0, 1.0, &mut bbox_);

        let mut min_: Point3 = Point3 { e: [INFINITY; 3] };
        let mut max_: Point3 = Point3 { e: [-INFINITY; 3] };

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox_.max().x() + (1.0 - i as f64) * bbox_.min().x();
                    let y = j as f64 * bbox_.max().y() + (1.0 - j as f64) * bbox_.min().y();
                    let z = k as f64 * bbox_.max().z() + (1.0 - k as f64) * bbox_.min().z();

                    let newx = cos_theta_ * x + sin_theta_ * z;
                    let newz = -sin_theta_ * x + cos_theta_ * z;

                    let tester = Vec3 { e: [newx, y, newz] };

                    for c in 0..3 {
                        min_.e[c] = fmin(min_.e[c], tester.e[c]);
                        max_.e[c] = fmax(max_.e[c], tester.e[c]);
                    }
                }
            }
        }
        bbox_ = Aabb {
            min: min_,
            max: max_,
        };

        RotateY {
            ptr: p,
            sin_theta: sin_theta_,
            cos_theta: cos_theta_,
            hasbox: hasbox_,
            bbox: bbox_,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.get_start();
        let mut dir_ = r.get_dir();

        origin.e[0] =
            (*self).cos_theta * r.get_start().e[0] - (*self).sin_theta * r.get_start().e[2];
        origin.e[2] =
            (*self).sin_theta * r.get_start().e[0] + (*self).cos_theta * r.get_start().e[2];

        dir_.e[0] = (*self).cos_theta * r.get_dir().e[0] - (*self).sin_theta * r.get_dir().e[2];
        dir_.e[2] = (*self).sin_theta * r.get_dir().e[0] + (*self).cos_theta * r.get_dir().e[2];

        let rotate_r = Ray {
            st: origin,
            dir: dir_,
            tm: r.get_time(),
        };

        if !(*self).ptr.hit(rotate_r, t_min, t_max, rec) {
            false
        } else {
            let mut p = rec.p;
            let mut normal = rec.normal;
            p.e[0] = (*self).cos_theta * rec.p.e[0] + (*self).sin_theta * rec.p.e[2];
            p.e[2] = (*self).cos_theta * rec.p.e[2] - (*self).sin_theta * rec.p.e[0];

            normal.e[0] = (*self).cos_theta * rec.normal.e[0] + (*self).sin_theta * rec.normal.e[2];
            normal.e[2] = (*self).cos_theta * rec.normal.e[2] - (*self).sin_theta * rec.normal.e[0];

            rec.p = p;
            rec.set_face_normal(rotate_r, normal);

            true
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut Aabb) -> bool {
        *output_box = (*self).bbox;
        true
    }
}
