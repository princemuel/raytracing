use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::prelude::Real;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3(Real, Real, Real);

impl Vec3 {
    #[must_use]
    pub const fn new(e0: Real, e1: Real, e2: Real) -> Self { Self(e0, e1, e2) }

    #[must_use]
    pub const fn zero() -> Self { Self(0.0, 0.0, 0.0) }

    #[must_use]
    pub const fn x(&self) -> Real { self.0 }

    #[must_use]
    pub const fn y(&self) -> Real { self.1 }

    #[must_use]
    pub const fn z(&self) -> Real { self.2 }

    #[must_use]
    pub fn len(&self) -> Real { self.length_squared().sqrt() }

    const fn length_squared(&self) -> Real { self.0 * self.0 + self.1 * self.1 + self.2 * self.2 }

    #[must_use]
    pub fn dot(&self, other: Self) -> Real { self.0 * other.0 + self.1 * other.1 + self.2 * other.2 }

    #[must_use]
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[must_use]
    pub fn unit(&self) -> Self { *self / self.len() }
}

pub type Point3 = Vec3;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
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

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.0, -self.1, -self.2) }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
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

impl Div<Real> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Real) -> Self { (1.0 / rhs) * self }
}

impl DivAssign<Real> for Vec3 {
    fn div_assign(&mut self, rhs: Real) { *self *= 1.0 / rhs; }
}
