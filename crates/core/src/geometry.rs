use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::prelude::*;
use shared::{random, random_range};

/// This accepts anything `Into<f64>` so you can write
/// `vec3(0, 1, -2)` instead of `vec3(0.0, 1.0, -2.0)`.
#[inline]
#[must_use]
pub fn vec3(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Vec3 {
    Vec3::new(x.into(), y.into(), z.into())
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    // Named unit-axis vectors and common sentinels
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    pub const INFINITY: Self = Self::splat(f64::INFINITY);
    pub const NAN: Self = Self::splat(f64::NAN);
    pub const NEG_INFINITY: Self = Self::splat(f64::NEG_INFINITY);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const ZERO: Self = Self::splat(0.0);

    #[inline]
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }

    #[inline]
    #[must_use]
    pub const fn splat(v: f64) -> Self { Self { x: v, y: v, z: v } }
}

// ---------------------------------------------------------------------------
// Geometry operations
// ---------------------------------------------------------------------------

impl Vec3 {
    #[inline]
    #[must_use]
    pub fn length(self) -> f64 { self.length_squared().sqrt() }

    #[inline]
    #[must_use]
    pub const fn length_squared(self) -> f64 { self.dot(self) }

    #[inline]
    #[must_use]
    pub const fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// Reflects `self` around the surface `normal` (must be unit length).
    ///
    /// Formula: **v** − 2(**v**·**n**)**n**
    #[inline]
    #[must_use]
    pub const fn reflect(self, normal: Self) -> Self {
        self - (2.0 * self.dot(normal)) * normal
    }

    /// Refracts `self` through a surface whose normal is `normal` (unit
    /// length).
    ///
    /// `etai_over_etat` is `η_i` `η_t`_t (incident / transmitted index of
    /// refraction).
    #[inline]
    #[must_use]
    pub fn refract(self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }

    /// Returns the unit vector (normalised to length 1).
    #[inline]
    #[must_use]
    pub fn unit(self) -> Self {
        // Multiply by reciprocal — one division instead of three.
        (1.0 / self.length()) * self
    }

    /// Returns `true` if all components are within 1e-8 of zero.
    ///
    /// Used after diffuse scatter to avoid degenerate zero-length rays.
    #[inline]
    #[must_use]
    pub const fn near_zero(self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}

// ---------------------------------------------------------------------------
// Random sampling helpers
// ---------------------------------------------------------------------------

impl Vec3 {
    #[inline]
    #[must_use]
    pub fn random(rng: &mut dyn Rng) -> Self {
        Self::new(random(rng), random(rng), random(rng))
    }

    #[inline]
    #[must_use]
    pub fn random_range(rng: &mut dyn Rng, min: f64, max: f64) -> Self {
        Self::new(
            random_range(rng, min, max),
            random_range(rng, min, max),
            random_range(rng, min, max),
        )
    }

    /// Returns a random unit vector (uniform over the unit sphere).
    ///
    /// Uses rejection sampling: pick a random point in the [-1,1]³ cube,
    /// reject if outside the unit ball (avoids polar bunching), then normalise.
    /// The `BLACKHOLE` guard prevents a division by a subnormal length.
    #[must_use]
    pub fn random_unit(rng: &mut dyn Rng) -> Self {
        // Any point with |p|² < 1e-160 is so close to the origin that
        // normalising would produce a NaN or infinity; reject it.
        const BLACKHOLE: f64 = 1e-160;
        loop {
            let p = Self::random_range(rng, -1.0, 1.0);
            let len_sq = p.length_squared();
            if BLACKHOLE < len_sq && len_sq <= 1.0 {
                return p * (1.0 / len_sq.sqrt());
            }
        }
    }

    /// Returns a random unit vector in the same hemisphere as `normal`.
    #[inline]
    #[must_use]
    pub fn random_on_hemisphere(rng: &mut dyn Rng, normal: Self) -> Self {
        let v = Self::random_unit(rng);
        if v.dot(normal) > 0.0 { v } else { -v }
    }

    /// Returns a random point inside the unit disk (z = 0).
    ///
    /// Used for defocus / depth-of-field sampling.
    #[must_use]
    pub fn random_in_unit_disk(rng: &mut dyn Rng) -> Self {
        loop {
            let p = Self::new(random_range(rng, -1.0, 1.0), random_range(rng, -1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Point3 alias
// ---------------------------------------------------------------------------

/// A point in 3-D space. Alias for [`Vec3`] — not a distinct newtype.
///
/// Keeping it as an alias (rather than a newtype) preserves all `Vec3`
/// arithmetic without needing `From` conversions, matching the book's style.
pub type Point3 = Vec3;

#[inline]
#[must_use]
pub fn point3(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Point3 {
    Point3::new(x.into(), y.into(), z.into())
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "[{:.p$} {:.p$} {:.p$}]", self.x, self.y, self.z)
    }
}

// ---------------------------------------------------------------------------
// Operator impls
// ---------------------------------------------------------------------------

impl const Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) }
}

impl const Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl const AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl const Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl const SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/// Component-wise multiply (Hadamard product).
impl const Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl const Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self { rhs * self }
}

/// Scalar-on-the-left: `2.0 * v`.
impl const Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 { Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z) }
}

impl const MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl const Div<f64> for Vec3 {
    type Output = Self;

    /// Divides by scalar via multiplication by its reciprocal. one division
    /// instead of three, which is a meaningful saving in a hot path.
    #[inline]
    fn div(self, rhs: f64) -> Self { (1.0 / rhs) * self }
}

impl const DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) { *self *= 1.0 / rhs; }
}
