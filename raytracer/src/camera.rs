use crate::ray::Ray;
use crate::vec3::{cross, unit_vector, Point3, Vec3};
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
}

impl Default for Camera {
    fn default() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin_ = Point3 { e: [0.0; 3] };
        let horizontal_ = Vec3 {
            e: [viewport_width, 0.0, 0.0],
        };
        let vertical_ = Vec3 {
            e: [0.0, viewport_height, 0.0],
        };
        let lower_left_corner_ = origin_
            - horizontal_ / 2.0
            - vertical_ / 2.0
            - Vec3 {
                e: [0.0, 0.0, focal_length],
            };

        Camera {
            origin: origin_,
            lower_left_corner: lower_left_corner_,
            horizontal: horizontal_,
            vertical: vertical_,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            st: (*self).origin,
            dir: (*self).lower_left_corner + (*self).horizontal * s + (*self).vertical * t
                - (*self).origin,
        }
    }
    pub fn creat(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        // let focal_length = 1.0;
        let origin_ = lookfrom;
        let horizontal_ = u * viewport_width;
        let vertical_ = v * viewport_height;
        let lower_left_corner_ = origin_ - horizontal_ / 2.0 - vertical_ / 2.0 - w;

        Camera {
            origin: origin_,
            lower_left_corner: lower_left_corner_,
            horizontal: horizontal_,
            vertical: vertical_,
        }
    }
}
