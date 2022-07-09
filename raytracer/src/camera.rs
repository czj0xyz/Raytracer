use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

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

        let origin = Point3 { e: [0.0; 3] };
        let horizontal = Vec3 {
            e: [viewport_width, 0.0, 0.0],
        };
        let vertical = Vec3 {
            e: [0.0, viewport_height, 0.0],
        };
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3 {
                e: [0.0, 0.0, focal_length],
            };

        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            st: (*self).origin,
            dir: (*self).lower_left_corner + (*self).horizontal * u + (*self).vertical * v
                - (*self).origin,
        }
    }
}
