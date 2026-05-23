use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, Sub};

use rand::prelude::*;
use shared::{FuzzyEq as _, random, random_range};

use crate::prelude::{Axis, Interval, Vec3};

#[inline]
#[must_use]
pub fn color(r: impl Into<f64>, g: impl Into<f64>, b: impl Into<f64>) -> Color3 {
    Color3 { r: r.into(), g: g.into(), b: b.into() }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Color3 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color3 {
    pub const BLACK: Self = Self::splat(0.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    pub const PINK: Self = Self::new(1.0, 0.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const WHITE: Self = Self::splat(1.0);
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0);

    #[inline]
    #[must_use]
    pub const fn new(r: f64, g: f64, b: f64) -> Self { Self { r, g, b } }

    #[inline]
    #[must_use]
    pub const fn splat(v: f64) -> Self { Self { r: v, g: v, b: v } }

    #[must_use]
    pub const fn get(self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.r,
            Axis::Y => self.g,
            Axis::Z => self.b,
        }
    }
}

impl Color3 {
    #[inline]
    #[must_use]
    pub fn random(rng: &mut dyn Rng) -> Self { Self::new(random(rng), random(rng), random(rng)) }

    #[inline]
    #[must_use]
    pub fn random_range(rng: &mut dyn Rng, min: f64, max: f64) -> Self {
        Self::new(
            random_range(rng, min, max),
            random_range(rng, min, max),
            random_range(rng, min, max),
        )
    }
}

impl PartialEq for Color3 {
    fn eq(&self, other: &Self) -> bool {
        self.r.fuzzy_eq(&other.r) && self.g.fuzzy_eq(&other.g) && self.b.fuzzy_eq(&other.b)
    }
}

// ---------------------------------------------------------------------------
// Gamma correction
// ---------------------------------------------------------------------------

/// Applies a gamma-2 (square-root) transfer function.
///
/// Linear values below zero are clamped to 0. they can arise from floating-
/// point error and have no physical meaning.
#[inline]
#[must_use]
pub fn linear_to_gamma(component: f64) -> f64 {
    if component > 0.0 { component.sqrt() } else { 0.0 }
}

// ---------------------------------------------------------------------------
// Conversion to/from [u8; 3]
// ---------------------------------------------------------------------------

/// Output intensity clamped to [0.0, 0.999] before scaling to [0, 255].
///
/// The 0.999 upper bound prevents `(v * 256.0) as u8` from wrapping to 0
/// when v rounds to exactly 1.0.
const INTENSITY: Interval = Interval::new(0.0, 0.999);
const FLOAT_TO_BYTE: f64 = 256.0;
const BYTE_TO_FLOAT: f64 = 1.0 / 255.0;

#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::as_conversions)]
impl From<Color3> for [u8; 3] {
    fn from(c: Color3) -> Self {
        // Apply gamma correction then map [0, 0.999] → [0, 255].
        [c.r, c.g, c.b].map(|v| (FLOAT_TO_BYTE * INTENSITY.clamp(linear_to_gamma(v))) as u8)
    }
}

impl From<[u8; 3]> for Color3 {
    fn from([r, g, b]: [u8; 3]) -> Self {
        let [r, g, b] = [r, g, b].map(|v| f64::from(v) * BYTE_TO_FLOAT);
        Self::new(r, g, b)
    }
}

impl From<(u8, u8, u8)> for Color3 {
    fn from((r, g, b): (u8, u8, u8)) -> Self { Self::from([r, g, b]) }
}

// ---------------------------------------------------------------------------
// Display / formatting
// ---------------------------------------------------------------------------

impl fmt::Display for Color3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b] = <[u8; 3]>::from(*self);
        write!(f, "{r} {g} {b}")
    }
}

impl fmt::LowerHex for Color3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b] = <[u8; 3]>::from(*self);
        write!(f, "{r:02x}{g:02x}{b:02x}")
    }
}

// ---------------------------------------------------------------------------
// FromStr — CSS hex colours (#RRGGBB)
// ---------------------------------------------------------------------------

impl core::str::FromStr for Color3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s
            .strip_prefix('#')
            .filter(|h| h.len() == 6 && h.is_ascii())
            .ok_or_else(|| format!("expected '#RRGGBB', got: {s}"))?;

        let chan = |slice: &str, name: &str| {
            u8::from_str_radix(slice, 16)
                .map_err(|e| format!("invalid {name} component '{slice}' (expected 00–FF): {e}"))
        };

        Ok(Self::from([
            chan(&hex[..2], "red")?,
            chan(&hex[2..4], "green")?,
            chan(&hex[4..], "blue")?,
        ]))
    }
}

// ---------------------------------------------------------------------------
// Arithmetic operators
// ---------------------------------------------------------------------------

impl const Add for Color3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self { Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b) }
}

impl const AddAssign for Color3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

/// Map a surface normal (Vec3 in [−1, 1]) to a colour.
impl const Add<Vec3> for Color3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vec3) -> Self { Self::new(self.r + rhs.x, self.g + rhs.y, self.b + rhs.z) }
}

impl const Sub for Color3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self { Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b) }
}

/// Component-wise multiply (albedo tinting).
impl const Mul for Color3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self { Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b) }
}

impl const Mul<f64> for Color3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self { rhs * self }
}

/// Scalar on the left: `0.5 * color`.
impl const Mul<Color3> for f64 {
    type Output = Color3;

    #[inline]
    fn mul(self, rhs: Color3) -> Color3 { Color3::new(self * rhs.r, self * rhs.g, self * rhs.b) }
}

impl const Div for Color3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self { Self::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b) }
}

impl Sum for Color3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::BLACK, Self::add) }
}

impl Product for Color3 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::WHITE, Self::mul) }
}
