use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use rtc_shared::Real;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    pub const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];

    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::Z,
            Self::Z => Self::X,
        }
    }
}

/// Creates a 3-dimensional vector.
#[inline(always)]
#[must_use]
pub fn vec3<X, Y, Z>(x: X, y: Y, z: Z) -> Vec3
where
    X: Into<Real>,
    Y: Into<Real>,
    Z: Into<Real>,
{
    Vec3::new(x.into(), y.into(), z.into())
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3(Real, Real, Real);
impl Vec3 {
    #[must_use]
    pub const fn new(e0: Real, e1: Real, e2: Real) -> Self { Self(e0, e1, e2) }

    /// Creates a vector with all elements set to `value`.
    #[must_use]
    pub const fn splat(value: Real) -> Self { Self(value, value, value) }

    /// Alternative to using the index
    pub const fn get(&self, axis: Axis) -> Real {
        match axis {
            Axis::X => self.0,
            Axis::Y => self.1,
            Axis::Z => self.2,
        }
    }

    pub const fn x(&self) -> Real { self.0 }

    pub const fn y(&self) -> Real { self.1 }

    pub const fn z(&self) -> Real { self.2 }
}

impl Vec3 {
    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    /// All `Real::INFINITY`.
    pub const INFINITY: Self = Self::splat(Real::INFINITY);
    /// All `Real::MAX`.
    pub const MAX: Self = Self::splat(Real::MAX);
    /// All `Real::MIN`.
    pub const MIN: Self = Self::splat(Real::MIN);
    /// All `Real::NAN`.
    pub const NAN: Self = Self::splat(Real::NAN);
    /// All `Real::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(Real::NEG_INFINITY);
    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);
    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);
    /// All ones.
    pub const ONE: Self = Self::splat(1.0);
    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);
}

impl Vec3 {
    pub fn length(&self) -> Real { Real::sqrt(self.length_squared()) }

    pub const fn length_squared(&self) -> Real { self.0 * self.0 + self.1 * self.1 + self.2 * self.2 }

    pub const fn dot(&self, rhs: Self) -> Real { self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 }

    #[must_use]
    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    #[must_use]
    pub fn unit(&self) -> Self { *self / self.length() }
}

/// Create a 3-dimensional point
#[inline]
pub fn point3<X, Y, Z>(x: X, y: Y, z: Z) -> Point3
where
    X: Into<Real>,
    Y: Into<Real>,
    Z: Into<Real>,
{
    Point3::new(x.into(), y.into(), z.into())
}
pub type Point3 = Vec3;

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "{:.p$} {:.p$} {:.p$}]", self.x(), self.y(), self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = Real;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => unreachable!("index out of bounds"),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2) }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2) }
}

impl const Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.0, -self.1, -self.2) }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output { Self::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2) }
}

impl Mul<Vec3> for Real {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output { Vec3::new(self * rhs.0, self * rhs.1, self * rhs.2) }
}

impl Mul<Real> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output { rhs * self }
}

impl MulAssign<Real> for Vec3 {
    fn mul_assign(&mut self, rhs: Real) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output { Self::new(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2) }
}

impl Div<Real> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Real) -> Self { (1.0 / rhs) * self }
}

impl DivAssign<Real> for Vec3 {
    fn div_assign(&mut self, rhs: Real) { *self *= 1.0 / rhs; }
}
