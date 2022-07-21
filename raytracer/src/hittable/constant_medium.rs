use std::f64::INFINITY;
use std::sync::Arc;

use super::{HitRecord, Hittable};
use crate::basic::{
    ray::Ray,
    vec3::{random_double, Color, Vec3},
};
use crate::bvh::aabb::Aabb;
use crate::material::{Isotropic, Material};
use crate::texture::Texture;

#[derive(Clone)]
pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    #[allow(dead_code)]
    pub fn creat(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Isotropic { albedo: a }),
            neg_inv_density: (-1.0 / d),
        }
    }
    pub fn creat2(b: Arc<dyn Hittable>, d: f64, c: Color) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Isotropic::creat(c)),
            neg_inv_density: (-1.0 / d),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rec1: HitRecord = Default::default();
        let mut rec2: HitRecord = Default::default();

        if !(*self).boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }
        if !(*self)
            .boundary
            .hit(r, rec1.t + 0.0001, INFINITY, &mut rec2)
        {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0
        }

        let ray_length = r.get_dir().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = (*self).neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3 { e: [1.0, 0.0, 0.0] };
        rec.front_face = true;
        rec.mat_ptr = Some((*self).phase_function.clone());

        true
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        (*self).boundary.bounding_box(t0, t1, output_box)
    }
}
