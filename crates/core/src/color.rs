use core::iter::{Product, Sum};
use core::ops::{Add, Div, Mul, Sub};
use core::str::FromStr;

use rtc_shared::Real;

use crate::prelude::Vec3;

#[inline]
pub fn color<R, G, B>(r: R, g: G, b: B) -> Color3
where
    R: Into<Real>,
    G: Into<Real>,
    B: Into<Real>,
{
    Color3 { r: r.into(), g: g.into(), b: b.into() }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color3 {
    pub r: Real,
    pub g: Real,
    pub b: Real,
}

const BYTE_TO_FLOAT: Real = 1.0 / 255.0;
const FLOAT_TO_BYTE: Real = 256.0;

impl Color3 {
    pub const BLACK: Self = Self::splat(0.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    pub const PINK: Self = Self::new(1.0, 0.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const WHITE: Self = Self::splat(1.0);
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0);

    pub const fn new(r: Real, g: Real, b: Real) -> Self { Self { r, g, b } }

    pub const fn splat(v: Real) -> Self { Self { r: v, g: v, b: v } }
}

impl From<Color3> for [u8; 3] {
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from(c: Color3) -> Self { [c.r, c.g, c.b].map(|v| (v.clamp(0.0, 0.999) * FLOAT_TO_BYTE) as u8) }
}

impl core::fmt::Display for Color3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let [r, g, b] = <[u8; 3]>::from(*self);
        write!(f, "{r} {g} {b}")
    }
}

impl From<[u8; 3]> for Color3 {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::new(
            Real::from(r) * BYTE_TO_FLOAT,
            Real::from(g) * BYTE_TO_FLOAT,
            Real::from(b) * BYTE_TO_FLOAT,
        )
    }
}

impl From<(u8, u8, u8)> for Color3 {
    fn from((r, g, b): (u8, u8, u8)) -> Self { Self::from([r, g, b]) }
}

impl FromStr for Color3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex =
            s.strip_prefix('#').ok_or_else(|| format!("hex color must start with '#', got: {s}"))?;

        if hex.len() != 6 {
            return Err(format!("hex color must be 7 chars (#RRGGBB), got: {s}"));
        }

        let parse = |slice: &str, ch| {
            u8::from_str_radix(slice, 16)
                .map_err(|_| format!("invalid {ch} component '{slice}' (expected 00-FF)"))
        };

        Ok(Self::from([
            parse(&hex[0..2], "red")?,
            parse(&hex[2..4], "green")?,
            parse(&hex[4..6], "blue")?,
        ]))
    }
}

impl Add for Color3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self { Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b) }
}

/// Allows mapping a surface normal (Vec3 in [-1,1]) to a color.
/// See "Ray Tracing in One Weekend", §6.1.
impl Add<Vec3> for Color3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self { Self::new(self.r + rhs.x, self.g + rhs.y, self.b + rhs.z) }
}

impl Sub for Color3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self { Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b) }
}

impl Mul for Color3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self { Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b) }
}

impl Mul<Real> for Color3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self { rhs * self }
}

impl Mul<Color3> for Real {
    type Output = Color3;

    fn mul(self, rhs: Color3) -> Color3 { Color3::new(self * rhs.r, self * rhs.g, self * rhs.b) }
}

impl Div for Color3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self { Self::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b) }
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
