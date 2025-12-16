use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const ALL: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::Z,
            Self::Z => Self::X,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    #[must_use]
    pub const fn new(e0: f32, e1: f32, e2: f32) -> Self { Self(e0, e1, e2) }

    /// Creates a vector with all elements set to `value`.
    #[must_use]
    pub const fn splat(value: f32) -> Self { Self(value, value, value) }

    /// Alternative to using the i
    #[must_use]
    pub const fn get(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.0,
            Axis::Y => self.1,
            Axis::Z => self.2,
        }
    }

    #[must_use]
    pub const fn x(&self) -> f32 { self.0 }

    #[must_use]
    pub const fn y(&self) -> f32 { self.1 }

    #[must_use]
    pub const fn z(&self) -> f32 { self.2 }
}

impl Vec3 {
    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    /// All `f32::INFINITY`.
    pub const INFINITY: Self = Self::splat(f32::INFINITY);
    /// All `f32::MAX`.
    pub const MAX: Self = Self::splat(f32::MAX);
    /// All `f32::MIN`.
    pub const MIN: Self = Self::splat(f32::MIN);
    /// All `f32::NAN`.
    pub const NAN: Self = Self::splat(f32::NAN);
    /// All `f32::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);
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
    #[must_use]
    pub fn length(&self) -> f32 { f32::sqrt(self.length_squared()) }

    const fn length_squared(&self) -> f32 { self.0 * self.0 + self.1 * self.1 + self.2 * self.2 }

    #[must_use]
    pub fn dot(&self, rhs: Self) -> f32 { self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 }

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

pub type Point3 = Vec3;

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "[{:.p$}, {:.p$}, {:.p$}]", self.x(), self.y(), self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

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

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.0, -self.1, -self.2) }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output { Self::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2) }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output { Vec3::new(self * rhs.0, self * rhs.1, self * rhs.2) }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output { rhs * self }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output { Self::new(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2) }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self { (1.0 / rhs) * self }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) { *self *= 1.0 / rhs; }
}
