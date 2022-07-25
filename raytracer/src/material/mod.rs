use crate::hittable::HitRecord;
use crate::texture::*;

use crate::basic::{
    fmax, fmin,
    onb::Onb,
    ray::Ray,
    vec3::{
        dot, random_double, random_in_unit_sphere, reflect, refract, unit_vector, Color, Point3,
        Vec3,
    },
};
use crate::pdf::random_cosine_direction;
use std::f64::consts::PI;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: HitRecord,
        _albedo: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emitted(&self, _r_in: Ray, _rec: HitRecord, _u: f64, _v: f64, _p: Point3) -> Color {
        Color { e: [0.0; 3] }
    }

    fn scattering_pdf(&self, _r_in: Ray, _rec: HitRecord, _scattered: Ray) -> f64 {
        0.0
    }
}

#[derive(Default, Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Lambertian<SolidColor> {
    pub fn creat(c: Color) -> Lambertian<SolidColor> {
        Lambertian {
            albedo: SolidColor { color_value: c },
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        alb: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let mut uvw: Onb = Default::default();
        uvw.build_from_w(rec.normal);
        let dir = uvw.local_vec(random_cosine_direction());
        *scattered = Ray {
            st: rec.p,
            dir: unit_vector(dir),
            tm: r_in.get_time(),
        };
        *alb = (*self).albedo.value(rec.u, rec.v, rec.p);
        *pdf = dot(uvw.w(), scattered.get_dir()) / PI;
        true
    }
    fn scattering_pdf(&self, _r_in: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        let cosine = dot(rec.normal, unit_vector(scattered.get_dir()));
        fmax(0.0, cosine / PI)
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
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

#[derive(Default, Clone)]
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
        _pdf: &mut f64,
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
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl DiffuseLight<SolidColor> {
    pub fn creat_color(c: Color) -> DiffuseLight<SolidColor> {
        DiffuseLight {
            emit: SolidColor { color_value: c },
        }
    }
}

impl<T: Texture> DiffuseLight<T> {
    #[allow(dead_code)]
    pub fn creat_ptr(c: T) -> DiffuseLight<T> {
        DiffuseLight { emit: c }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emitted(&self, _r_in: Ray, rec: HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if rec.front_face {
            (*self).emit.value(u, v, p)
        } else {
            Color { e: [0.0; 3] }
        }
    }
}

pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn creat(c: Color) -> Isotropic<SolidColor> {
        Isotropic {
            albedo: SolidColor { color_value: c },
        }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
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
