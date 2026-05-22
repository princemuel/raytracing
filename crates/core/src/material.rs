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

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color3,
}
impl Lambertian {
    #[must_use]
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

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color3,
    pub fuzz: Real,
}
impl Metal {
    #[must_use]
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

#[derive(Clone, Copy)]
pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's
    /// refractive index over the refractive index of the enclosing media
    pub refract_idx: Real,
}
impl Dielectric {
    #[must_use]
    pub const fn new(refract_idx: Real) -> Self { Self { refract_idx } }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color3::WHITE;
        let refract_idx =
            if rec.is_front_face { self.refract_idx.recip() } else { self.refract_idx };

        let unit_direction = ray_in.direction.unit();
        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refract_idx * sin_theta > 1.0;
        let direction = if cannot_refract {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refract_idx)
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }
}
