use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rtc_shared::{Real, random, random_w_range};

#[must_use]
pub fn vec3(x: impl Into<Real>, y: impl Into<Real>, z: impl Into<Real>) -> Vec3 {
    Vec3::new(x.into(), y.into(), z.into())
}

// macro_rules! assert_equal_len {
//     // The `tt` (token tree) designator is used for
//     // operators and tokens.
//     ($a:expr, $b:expr, $func:ident, $op:tt) => {
//         assert!(
//             $a.len() == $b.len(),
//             "{:?}: dimension mismatch: {:?} {:?} {:?}",
//             stringify!($func),
//             ($a.len(),),
//             stringify!($op),
//             ($b.len(),)
//         );
//     };
// }

// macro_rules! op {
//     ($func:ident, $bound:ident, $op:tt, $method:ident) => {
//         fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys:
// &Vec<T>) {             assert_equal_len!(xs, ys, $func, $op);

//             for (x, y) in xs.iter_mut().zip(ys.iter()) {
//                 *x = $bound::$method(*x, *y);
//                 // *x = x.$method(*y);
//             }
//         }
//     };
// }

// op!(add_assign, Add, +=, add);
// op!(mul_assign, Mul, *=, mul);
// op!(sub_assign, Sub, -=, sub);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Vec3 {
    /// The three unit axis vectors [X, Y, Z].
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    pub const INFINITY: Self = Self::splat(Real::INFINITY);
    pub const MAX: Self = Self::splat(Real::MAX);
    pub const MIN: Self = Self::splat(Real::MIN);
    pub const NAN: Self = Self::splat(Real::NAN);
    pub const NEG_INFINITY: Self = Self::splat(Real::NEG_INFINITY);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const ZERO: Self = Self::splat(0.0);

    #[must_use]
    pub const fn new(x: Real, y: Real, z: Real) -> Self { Self { x, y, z } }

    #[must_use]
    pub const fn splat(v: Real) -> Self { Self { x: v, y: v, z: v } }
}

impl Vec3 {
    #[must_use]
    pub fn length(self) -> Real { self.length_squared().sqrt() }

    #[must_use]
    pub const fn length_squared(self) -> Real {
        // ? can i do this self.dot(self)
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[must_use]
    pub const fn dot(self, rhs: Self) -> Real {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[must_use]
    pub const fn reflect(self, normal: Self) -> Self {
        self - (normal * (2.0 * self.dot(normal)))
    }

    #[must_use]
    pub fn refract(self, normal: Self, etai_over_etat: Real) -> Self {
        let cos_theta = (-self.dot(normal)).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * normal;

        r_out_perp + r_out_parallel
    }

    #[must_use]
    pub fn unit(self) -> Self { self / self.length() }

    #[must_use]
    pub const fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        const EPSILON: Real = 1e-8;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    #[must_use]
    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self::new(random(&mut rng), random(&mut rng), random(&mut rng))
    }

    #[must_use]
    pub fn random_w_range(min: Real, max: Real) -> Self {
        let mut rng = rand::rng();
        Self::new(
            random_w_range(&mut rng, min, max),
            random_w_range(&mut rng, min, max),
            random_w_range(&mut rng, min, max),
        )
    }

    #[must_use]
    pub fn random_unit() -> Self {
        const BLACKHOLE: Real = 1e-160;
        loop {
            let p = Self::random_w_range(-1.0, 1.0);
            let len_sq = p.length_squared();
            // ? NOTE: using < 2 instead of <= 1
            if BLACKHOLE < len_sq && len_sq < 2.0 {
                return p / len_sq.sqrt();
            }
        }
    }

    #[must_use]
    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit();
        if on_unit_sphere.dot(normal) > 0.0 { on_unit_sphere } else { -on_unit_sphere }
    }
}

/// A point in 3D space. Alias for [`Vec3`]. not a distinct newtype.
/// See "Ray Tracing in One Weekend" §3.
pub type Point3 = Vec3;

#[inline]
pub fn point3(x: impl Into<Real>, y: impl Into<Real>, z: impl Into<Real>) -> Point3 {
    Point3::new(x.into(), y.into(), z.into())
}

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "[{:.p$} {:.p$} {:.p$}]", self.x, self.y, self.z)
    }
}

impl const Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) }
}

impl const Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl const AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl const Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl const Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl const Mul<Real> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self { rhs * self }
}

impl const Mul<Vec3> for Real {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 { Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z) }
}

impl const MulAssign<Real> for Vec3 {
    fn mul_assign(&mut self, rhs: Real) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl const Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl const Div<Real> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Real) -> Self { (1.0 / rhs) * self }
}

impl const DivAssign<Real> for Vec3 {
    fn div_assign(&mut self, rhs: Real) { *self *= 1.0 / rhs; }
}
