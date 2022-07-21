use super::{HitRecord, Hittable};
use crate::basic::{
    ray::Ray,
    vec3::{Point3, Vec3},
    {degrees_to_radians, fmax, fmin},
};
use crate::bvh::aabb::Aabb;
use std::f64::INFINITY;
use std::sync::Arc;

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
