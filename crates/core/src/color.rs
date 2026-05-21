use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, Sub};
use core::str::FromStr;

use rtc_shared::{FuzzyEq as _, Real};

use crate::prelude::{Interval, Vec3};

#[must_use]
pub fn color(r: impl Into<Real>, g: impl Into<Real>, b: impl Into<Real>) -> Color3 {
    Color3 { r: r.into(), g: g.into(), b: b.into() }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Color3 {
    pub r: Real,
    pub g: Real,
    pub b: Real,
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

    #[must_use]
    pub const fn new(r: Real, g: Real, b: Real) -> Self { Self { r, g, b } }

    #[must_use]
    pub const fn splat(v: Real) -> Self { Self { r: v, g: v, b: v } }
}

impl PartialEq for Color3 {
    fn eq(&self, other: &Self) -> bool {
        self.r.fuzzy_eq(&other.r) && self.g.fuzzy_eq(&other.g) && self.b.fuzzy_eq(&other.b)
    }
}

#[must_use]
pub fn linear_to_gamma(component: Real) -> Real {
    if component > 0.0 { component.sqrt() } else { 0.0 }
}

const BYTE_TO_FLOAT: Real = 1.0 / 255.0;
const FLOAT_TO_BYTE: Real = 256.0;
const INTENSITY: Interval = Interval::new(0.0, 0.999);

#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::as_conversions)]
impl From<Color3> for [u8; 3] {
    fn from(c: Color3) -> Self {
        [c.r, c.g, c.b].map(|v| {
            // Applies a linear to gamma  transform for gamma 2
            let c = linear_to_gamma(v);
            // INTENSITY.clamp(v) is in [0.0, 0.999], so * 256.0 = [0.0, 255.744].
            (FLOAT_TO_BYTE * INTENSITY.clamp(c)) as u8
        })
    }
}

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

impl From<[u8; 3]> for Color3 {
    fn from([r, g, b]: [u8; 3]) -> Self {
        let [r, g, b] = [r, g, b].map(|v| Real::from(v) * BYTE_TO_FLOAT);
        Self::new(r, g, b)
    }
}

impl From<(u8, u8, u8)> for Color3 {
    fn from((r, g, b): (u8, u8, u8)) -> Self { Self::from([r, g, b]) }
}

impl FromStr for Color3 {
    type Err = String;

    /// Parses a CSS-style hex colour string e.g. `#FF8000` or `#ff8000`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s
            .strip_prefix('#')
            .filter(|h| h.len() == 6 && h.is_ascii())
            .ok_or_else(|| format!("expected '#RRGGBB', got: {s}"))?;

        let parse_channel = |slice, name| {
            u8::from_str_radix(slice, 16).map_err(|e| {
                format!("invalid {name} component '{slice}' (expected 00-FF): {e}")
            })
        };

        let (r, g, b) = (
            parse_channel(&hex[..2], "red")?,
            parse_channel(&hex[2..4], "green")?,
            parse_channel(&hex[4..6], "blue")?,
        );

        Ok(Self::from([r, g, b]))
    }
}

impl const Add for Color3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}
impl const AddAssign for Color3 {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

/// Allows mapping a surface normal (Vec3 in [-1,1]) to a color.
/// See "Ray Tracing in One Weekend", §6.1.
impl const Add<Vec3> for Color3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        Self::new(self.r + rhs.x, self.g + rhs.y, self.b + rhs.z)
    }
}

impl const Sub for Color3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl const Mul for Color3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl const Mul<Real> for Color3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self { rhs * self }
}

impl const Mul<Color3> for Real {
    type Output = Color3;

    fn mul(self, rhs: Color3) -> Color3 {
        Color3::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl const Div for Color3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b)
    }
}

impl Sum for Color3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::BLACK, Self::add)
    }
}

impl Product for Color3 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::WHITE, Self::mul)
    }
}
