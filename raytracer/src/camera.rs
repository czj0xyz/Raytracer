use crate::ray::Ray;
use crate::vec3::{cross, random_double_lr, random_in_unit_disk, unit_vector, Point3, Vec3};
use std::f64::consts::PI;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * (*self).lens_radius;
        let offset = (*self).u * rd.x() + (*self).v * rd.y();
        Ray {
            st: (*self).origin + offset,
            dir: (*self).lower_left_corner + (*self).horizontal * s + (*self).vertical * t
                - (*self).origin
                - offset,
            tm: random_double_lr((*self).time0, (*self).time1),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn creat(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w_ = unit_vector(lookfrom - lookat);
        let u_ = unit_vector(cross(vup, w_));
        let v_ = cross(w_, u_);

        // let focal_length = 1.0;
        let origin_ = lookfrom;
        let horizontal_ = u_ * viewport_width * focus_dist;
        let vertical_ = v_ * viewport_height * focus_dist;
        let lower_left_corner_ = origin_ - horizontal_ / 2.0 - vertical_ / 2.0 - w_ * focus_dist;

        Camera {
            origin: origin_,
            lower_left_corner: lower_left_corner_,
            horizontal: horizontal_,
            vertical: vertical_,
            u: u_,
            v: v_,
            w: w_,
            lens_radius: aperture / 2.0,
            time0: _time0,
            time1: _time1,
        }
    }
}
