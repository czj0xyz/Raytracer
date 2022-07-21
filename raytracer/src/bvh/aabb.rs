use crate::basic::{
    ray::Ray,
    vec3::*,
    {fmax, fmin},
};

#[derive(Default, Clone, Copy)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    #[allow(dead_code)]
    pub fn max(&self) -> Point3 {
        (*self).max
    }
    #[allow(dead_code)]
    pub fn min(&self) -> Point3 {
        (*self).min
    }
    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let invd = 1.0 / r.get_dir().e[i];
            let mut t0 = ((*self).min.e[i] - r.get_start().e[i]) * invd;
            let mut t1 = ((*self).max.e[i] - r.get_start().e[i]) * invd;
            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if fmin(t1, t_max) <= fmax(t0, t_min) {
                return false
            }
        }
        true
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Point3 {
        e: [
            fmin(box0.min.x(), box1.min.x()),
            fmin(box0.min.y(), box1.min.y()),
            fmin(box0.min.z(), box1.min.z()),
        ],
    };

    let big = Point3 {
        e: [
            fmax(box0.max.x(), box1.max.x()),
            fmax(box0.max.y(), box1.max.y()),
            fmax(box0.max.z(), box1.max.z()),
        ],
    };

    Aabb {
        min: small,
        max: big,
    }
}
