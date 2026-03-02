use core::iter::{Product, Sum};
use core::ops::{Add, Div, Mul, Sub};
use std::io::{self, Write};

use rtc_shared::Real;

/// Creates a color
#[inline]
pub fn color<R, G, B>(r: R, g: G, b: B) -> Color3
where
    R: Into<Real>,
    G: Into<Real>,
    B: Into<Real>,
{
    Color3::new(r.into(), g.into(), b.into())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color3(Real, Real, Real);

impl Color3 {
    /// Creates a new color.
    #[must_use]
    pub const fn new(e0: Real, e1: Real, e2: Real) -> Self { Self(e0, e1, e2) }

    /// Creates a color with all elements set to `value`.
    #[must_use]
    pub const fn splat(value: Real) -> Self { Self(value, value, value) }

    #[must_use]
    pub const fn r(&self) -> Real { self.0 }

    #[must_use]
    pub const fn g(&self) -> Real { self.1 }

    #[must_use]
    pub const fn b(&self) -> Real { self.2 }
}

impl Color3 {
    /// All zeroes.
    // #000
    pub const BLACK: Self = Self::splat(0.0);
    /// A unit color with the `blue` component set to 1
    // #00f
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);
    /// A unit color with the `green` and `blue` components set to 1
    // #0ff
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0);
    /// A unit color with the `green` component set to 1
    // #0f0
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    /// A unit color with the `red` and `blue` components set to 1
    // #f0f
    pub const PINK: Self = Self::new(1.0, 0.0, 1.0);
    /// A unit color with the `red` component set to 1
    // #f00
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    /// All ones.
    // #fff
    pub const WHITE: Self = Self::splat(1.0);
    /// A unit color with the `red` and `green` components set to 1
    // #ff0
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0);
}

impl Default for Color3 {
    fn default() -> Self { Self::BLACK }
}

impl core::fmt::Display for Color3 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const FLOAT_TO_BYTE_SCALE: Real = 256.0;

        let r = (self.r().clamp(0.0, 0.999) * FLOAT_TO_BYTE_SCALE) as u8;
        let g = (self.g().clamp(0.0, 0.999) * FLOAT_TO_BYTE_SCALE) as u8;
        let b = (self.b().clamp(0.0, 0.999) * FLOAT_TO_BYTE_SCALE) as u8;

        write!(f, "{r} {g} {b}")
    }
}

impl Add for Color3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2) }
}

impl Sub for Color3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2) }
}

impl Mul<Real> for Color3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output { rhs * self }
}

impl Mul<Color3> for Real {
    type Output = Color3;

    fn mul(self, rhs: Color3) -> Self::Output { Color3::new(self * rhs.0, self * rhs.1, self * rhs.2) }
}

impl Mul for Color3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output { Self::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2) }
}

impl Div for Color3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output { Self::new(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2) }
}

impl Sum for Color3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::BLACK, Self::add)
    }
}

impl Product for Color3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::WHITE, Self::mul)
    }
}

const BYTE_TO_FLOAT_SCALE: Real = 1.0 / 255.0;
const FLOAT_TO_BYTE_SCALE: Real = 256.0;

impl From<(u8, u8, u8)> for Color3 {
    fn from((r, g, b): (u8, u8, u8)) -> Self { Self::from([r, g, b]) }
}

impl From<[u8; 3]> for Color3 {
    fn from(rgb: [u8; 3]) -> Self {
        let [r, g, b] = rgb.map(|c| Real::from(c) * BYTE_TO_FLOAT_SCALE);
        Self::new(r, g, b)
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl From<Color3> for [u8; 3] {
    fn from(color: Color3) -> Self {
        [color.r(), color.g(), color.b()].map(|c| (c.clamp(0.0, 0.999) * FLOAT_TO_BYTE_SCALE) as u8)
    }
}

impl TryFrom<[u8; 7]> for Color3 {
    type Error = String;

    fn try_from(value: [u8; 7]) -> Result<Self, Self::Error> {
        let src = str::from_utf8(&value).map_err(|_| "Invalid UTF-8 in hex color")?;

        if !src.starts_with('#') {
            return Err(format!("Hex color must start with '#', got: {src}"));
        }

        let parse_hex = |s, c| {
            u8::from_str_radix(s, 16)
                .map(|v| Real::from(v) / 255.0)
                .map_err(|_| format!("Invalid {c} component '{s}' (expected 00-FF)"))
        };

        Ok(Self(
            parse_hex(&src[1..3], "red")?,
            parse_hex(&src[3..5], "green")?,
            parse_hex(&src[5..7], "blue")?,
        ))
    }
}

/// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.
#[allow(clippy::cast_possible_truncation)]
pub fn _write_colors_batch<W: Write>(out: &mut W, colors: &[Color3]) -> io::Result<()> {
    let mut buffer = String::with_capacity(colors.len() * 12); // ~12 chars per color

    for color in colors {
        use std::fmt::Write as _;
        writeln!(&mut buffer, "{color}").unwrap();
    }

    out.write_all(buffer.as_bytes())
}
