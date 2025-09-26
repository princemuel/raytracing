use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub const fn new(e0: f64, e1: f64, e2: f64) -> Self { Self(e0, e1, e2) }

    pub const fn zero() -> Self { Self(0.0, 0.0, 0.0) }

    pub const fn x(&self) -> f64 { self.0 }

    pub const fn y(&self) -> f64 { self.1 }

    pub const fn z(&self) -> f64 { self.2 }

    pub fn len(&self) -> f64 { self.length_squared().sqrt() }

    const fn length_squared(&self) -> f64 { self.0 * self.0 + self.1 * self.1 + self.2 * self.2 }

    pub fn dot(&self, other: Self) -> f64 { self.0 * other.0 + self.1 * other.1 + self.2 * other.2 }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit(&self) -> Self { *self / self.len() }
}

pub type Point3 = Vec3;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

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

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output { Vec3::new(self * rhs.0, self * rhs.1, self * rhs.2) }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output { rhs * self }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self { (1.0 / rhs) * self }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) { *self *= 1.0 / rhs; }
}
