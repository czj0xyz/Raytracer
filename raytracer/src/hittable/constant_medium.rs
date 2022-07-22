use std::f64::INFINITY;

use super::{HitRecord, Hittable};
use crate::basic::{
    ray::Ray,
    vec3::{random_double, Color, Vec3},
};
use crate::bvh::aabb::Aabb;
use crate::material::{Isotropic, Material};
use crate::texture::{Texture,SolidColor};

#[derive(Clone)]
pub struct ConstantMedium<T:Hittable, U:Material> {
    pub boundary: T,//Hittbale
    pub phase_function: U,//Material
    pub neg_inv_density: f64,
}

impl<T:Hittable, M:Texture> ConstantMedium<T,Isotropic<M>> {
    #[allow(dead_code)]
    pub fn creat(b: T, d: f64, a: M) -> ConstantMedium<T,Isotropic<M> > {
        ConstantMedium {
            boundary: b,
            phase_function: Isotropic { albedo: a },
            neg_inv_density: (-1.0 / d),
        }
    }
}

impl<T:Hittable> ConstantMedium<T,Isotropic<SolidColor> > {
    pub fn creat2(b: T, d: f64, c: Color) -> ConstantMedium<T,Isotropic<SolidColor> > {
        ConstantMedium {
            boundary: b,
            phase_function: Isotropic { albedo: SolidColor{ color_value: c} },
            neg_inv_density: (-1.0 / d),
        }
    }
}

impl<T:Hittable, U:Material> Hittable for ConstantMedium<T,U> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rec1= (*self).boundary.hit(r, -INFINITY, INFINITY);
        if rec1.is_none(){return None}
        let mut rec1 = rec1.unwrap();

        let rec2 = (*self).boundary.hit(r, rec1.t + 0.0001, INFINITY);
        if rec2.is_none(){return None}
        let mut rec2= rec2.unwrap();

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {return None}

        if rec1.t < 0.0 {
            rec1.t = 0.0
        }

        let ray_length = r.get_dir().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = (*self).neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {return None}
        
        let rec = HitRecord{
            t: rec1.t + hit_distance / ray_length,
            p: r.at( rec1.t + hit_distance / ray_length ),

            normal: Vec3 { e: [1.0, 0.0, 0.0] },
            front_face: true,
            mat_ptr: &(*self).phase_function,

            u:0.0,
            v:0.0,
        };
        Some(rec)
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut Aabb) -> bool {
        (*self).boundary.bounding_box(t0, t1, output_box)
    }
}
