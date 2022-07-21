use crate::hittable::HitRecord;
use crate::texture::*;

use crate::basic::{
    fmin,
    ray::Ray,
    vec3::{
        dot, random_double, random_in_unit_sphere, random_unit_vector, reflect, refract,
        unit_vector, Color, Point3, Vec3,
    },
};

use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color { e: [0.0; 3] }
    }
}

#[derive(Default, Clone)]
pub struct Lambertian {
    pub albedo: Option<Arc<dyn Texture>>,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Lambertian {
    pub fn creat(c: Color) -> Lambertian {
        Lambertian {
            albedo: Some(Arc::new(SolidColor { color_value: c })),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray {
            st: rec.p,
            dir: scatter_direction,
            tm: _r_in.get_time(),
        };
        *attenuation = match (*self).albedo.clone() {
            Some(x) => x.value(rec.u, rec.v, rec.p),
            None => Color { e: [0.0; 3] },
        };
        true
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.get_dir()), rec.normal);
        *scattered = Ray {
            st: rec.p,
            dir: reflected + random_in_unit_sphere() * (*self).fuzz,
            tm: r_in.get_time(),
        };
        *attenuation = (*self).albedo;
        dot(scattered.get_dir(), rec.normal) > 0.0
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color { e: [1.0; 3] };
        let refraction_ratio = if rec.front_face {
            1.0 / ((*self).ir)
        } else {
            (*self).ir
        };
        let unit_direction = unit_vector(r_in.get_dir());

        let cos_theta = fmin(dot(Vec3 { e: [0.0; 3] } - unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray {
            st: rec.p,
            dir: direction,
            tm: r_in.get_time(),
        };
        true
    }
}

impl Metal {
    pub fn creat(a: Color, f: f64) -> Metal {
        let ff = if f < 1.0 { f } else { 1.0 };
        Metal {
            albedo: a,
            fuzz: ff,
        }
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn creat_color(c: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor { color_value: c }),
        }
    }
    #[allow(dead_code)]
    pub fn creat_ptr(c: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit: c }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        (*self).emit.value(u, v, p)
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn creat(c: Color) -> Isotropic {
        Isotropic {
            albedo: Arc::new(SolidColor { color_value: c }),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray {
            st: rec.p,
            dir: random_in_unit_sphere(),
            tm: r_in.get_time(),
        };
        *attenuation = (*self).albedo.value(rec.u, rec.v, rec.p);
        true
    }
}
