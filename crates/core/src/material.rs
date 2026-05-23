use rand::prelude::Rng;
use shared::random;

use crate::prelude::{Color3, HitRecord, Ray, Vec3};

/// A material decides whether (and how) an incoming ray scatters.
///
/// Returns `Some((attenuation, scattered_ray))` on scatter, `None` on full
/// absorption.
///
/// `attenuation` is the fraction of the ray's colour that is
/// absorbed by the material; the rest is scattered.
///
/// For example, a pure mirror
/// would return `Some((Color3::WHITE, reflected_ray))`, while a pure black hole
/// would return `None`.
pub trait Material: Send + Sync {
    fn scatter(&self, rng: &mut dyn Rng, ray_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)>;
}

// ---------------------------------------------------------------------------
// Lambertian (diffuse)
// ---------------------------------------------------------------------------

/// Perfectly diffuse (Lambertian) surface.
///
/// Scatters in a direction near the surface normal with cosine weighting,
/// giving physically correct attenuation without an explicit PDF term.
#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color3,
}

impl Lambertian {
    #[inline]
    #[must_use]
    pub const fn new(albedo: Color3) -> Self { Self { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut dyn Rng, ray_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)> {
        // Add a random unit vector to the surface normal.
        // If that accidentally produces a near-zero direction (very rare),
        // fall back to the normal itself to avoid NaNs downstream.
        let direction = {
            let d = rec.normal + Vec3::random_unit(rng);
            if d.near_zero() { rec.normal } else { d }
        };

        let scattered = Ray::new(rec.p, direction, Some(ray_in.time));
        Some((self.albedo, scattered))
    }
}

// ---------------------------------------------------------------------------
// Metal (specular)
// ---------------------------------------------------------------------------

/// Polished-metal surface with optional blur (`fuzz` in [0, 1]).
///
/// `fuzz = 0` gives a perfect mirror; `fuzz = 1` gives a rough matte metal.
#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Color3,
    /// Fuzz radius in [0, 1]. Values > 1 are treated as 1.
    pub fuzz: f64,
}

impl Metal {
    #[inline]
    #[must_use]
    pub const fn new(albedo: Color3, fuzz: f64) -> Self {
        // Clamp fuzz to [0, 1]: fuzz > 1 would make the scattered ray go
        // *through* the surface, producing strange darkening artefacts.
        Self { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut dyn Rng, ray_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)> {
        let reflected = ray_in.direction.reflect(rec.normal);
        // Normalise before adding fuzz so the magnitude of `reflected` doesn't
        // scale the blur radius — fuzz is specified in unit-sphere radii.
        let direction = reflected.unit() + self.fuzz * Vec3::random_unit(rng);

        // Rays that scatter *into* the surface (dot ≤ 0) are absorbed.
        let scattered = Ray::new(rec.p, direction, Some(ray_in.time));
        (direction.dot(rec.normal) > 0.0).then_some((self.albedo, scattered))
    }
}

// ---------------------------------------------------------------------------
// Dielectric (glass / water)
// ---------------------------------------------------------------------------

/// Transparent dielectric material (glass, water, etc.).
///
/// Uses Schlick's approximation for the Fresnel reflectance term.
#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    /// Refractive index `η` (relative to the surrounding medium, typically air
    /// ≈ 1).
    pub refract_idx: f64,
}

impl Dielectric {
    #[inline]
    #[must_use]
    pub const fn new(refract_idx: f64) -> Self { Self { refract_idx } }
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut dyn Rng, ray_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)> {
        // η_i / η_t: flip ratio when hitting the back face.
        let ri = if rec.is_front_face { 1.0 / self.refract_idx } else { self.refract_idx };

        let v = ray_in.direction.unit();
        let cos_theta = (-v).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Total internal reflection OR Schlick probabilistic reflection.
        let direction = if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > random(rng) {
            v.reflect(rec.normal)
        } else {
            v.refract(rec.normal, ri)
        };
        let scattered = Ray::new(rec.p, direction, Some(ray_in.time));
        Some((Color3::WHITE, scattered))
    }
}

// ---------------------------------------------------------------------------
// Schlick reflectance approximation
// ---------------------------------------------------------------------------

/// Schlick's polynomial approximation for the Fresnel reflectance.
///
/// Returns the probability that a ray reflects (rather than refracts) at a
/// dielectric interface. `cosine` is the angle of incidence; `ri` is `η_i/η_t`.
#[inline]
#[must_use]
pub fn reflectance(cosine: f64, ri: f64) -> f64 {
    let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
