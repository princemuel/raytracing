use rtc_shared::Real;

use crate::prelude::{Color3, HitRecord, Ray, Vec3};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color3,
}
impl Lambertian {
    pub const fn new(albedo: Color3) -> Self { Self { albedo } }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        let mut direction = rec.normal + Vec3::random_unit();
        if direction.near_zero() {
            direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Color3,
    pub fuzz: Real,
}
impl Metal {
    pub const fn new(albedo: Color3, fuzz: Real) -> Self { Self { albedo, fuzz } }
}
impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray_in.direction.reflect(rec.normal);
        let direction = reflected.unit() + (self.fuzz * Vec3::random_unit());
        *scattered = Ray::new(rec.p, direction);
        *attenuation = self.albedo;

        scattered.direction.dot(rec.normal) > 0.0
    }
}
