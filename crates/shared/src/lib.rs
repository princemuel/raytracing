#![warn(clippy::pedantic)]
#![warn(clippy::ptr_arg)]
#![warn(clippy::use_self)]
#![warn(clippy::suspicious)]
#![warn(clippy::perf)]
// #![feature(const_trait_impl)]
// #![feature(const_cmp)]
// #![feature(const_ops)]

use core::f64;

pub type Real = f64;

pub const EPSILON: Real = 1e-4;

pub const INFINITY: Real = Real::INFINITY;
pub const PI: Real = f64::consts::PI;

// pub const trait FloatLike:
//     Copy
//     + [const] core::cmp::PartialOrd
//     + [const] core::cmp::PartialEq
//     + core::ops::Sub<Output = Self>
//     + core::ops::Mul<Output = Self>
// {
//     #[must_use]
//     fn zero() -> Self;
//     #[must_use]
//     fn epsilon() -> Self;
//     #[must_use]
//     fn one() -> Self;
//     #[must_use]
//     fn abs(self) -> Self;
//     #[must_use]
//     fn max(self, other: Self) -> Self;

//     fn is_nan(self) -> bool;
//     fn is_infinite(self) -> bool;
// }

// impl const FloatLike for Real {
//     fn zero() -> Self { 0.0 }

//     fn one() -> Self { 1.0 }

//     fn epsilon() -> Self { Self::EPSILON }

//     fn abs(self) -> Self { Self::abs(self) }

//     fn max(self, other: Self) -> Self { Self::max(self, other) }

//     fn is_nan(self) -> bool { Self::is_nan(self) }

//     fn is_infinite(self) -> bool { Self::is_infinite(self) }
// }

// impl const FloatLike for f32 {
//     fn zero() -> Self { 0.0 }

//     fn one() -> Self { 1.0 }

//     fn epsilon() -> Self { Self::EPSILON }

//     fn abs(self) -> Self { Self::abs(self) }

//     fn max(self, other: Self) -> Self { Self::max(self, other) }

//     fn is_nan(self) -> bool { Self::is_nan(self) }

//     fn is_infinite(self) -> bool { Self::is_infinite(self) }
// }
