#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_ops)]
#![feature(sized_hierarchy)]

use core::f64;
use core::marker::PointeeSized;

use rand::prelude::*;

pub type Real = f64;
pub const EPSILON: Real = <Real as FuzzyEq>::EPSILON;

pub const PI: Real = f64::consts::PI;

/// Returns a random real in [0,1).
#[must_use]
pub fn random(rng: &mut impl Rng) -> Real { rng.random() }

/// Returns a random real in [min,max).
#[must_use]
pub fn random_w_range(rng: &mut impl Rng, min: Real, max: Real) -> Real {
    debug_assert!(min <= max);
    min + (max - min) * rng.random::<Real>()
}

pub const trait FuzzyEq<Rhs: PointeeSized = Self>: PointeeSized {
    type Float;
    const EPSILON: Self::Float;

    fn fuzzy_eq(&self, other: &Rhs) -> bool;
    fn fuzzy_ne(&self, other: &Rhs) -> bool { !self.fuzzy_eq(other) }
}

impl const FuzzyEq for f32 {
    type Float = f32;

    const EPSILON: f32 = 1e-5_f32;

    fn fuzzy_eq(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        if self.is_nan() || other.is_nan() {
            return false;
        }

        if self.is_infinite() || other.is_infinite() {
            return false;
        }

        let scale = self.abs().max(other.abs());
        let delta = (self - other).abs();

        if scale < 1.0 { delta <= Self::EPSILON } else { delta <= scale * Self::EPSILON }
    }
}

impl const FuzzyEq for f64 {
    type Float = f64;

    const EPSILON: f64 = 1e-4;

    fn fuzzy_eq(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        if self.is_nan() || other.is_nan() {
            return false;
        }

        if self.is_infinite() || other.is_infinite() {
            return false;
        }

        let scale = self.abs().max(other.abs());
        let delta = (self - other).abs();

        if scale < 1.0 { delta <= Self::EPSILON } else { delta <= scale * Self::EPSILON }
    }
}

/// Approximate equality. mirrors `assert_eq!` ergonomics.
#[macro_export]
macro_rules! fuzzy_eq {
    ($left:expr, $right:expr) => {
        $crate::FuzzyEq::fuzzy_eq(&$left, &$right)
    };
}

/// Approximate inequality. mirrors `assert_ne!` ergonomics.
#[macro_export]
macro_rules! fuzzy_ne {
    ($left:expr, $right:expr) => {
        $crate::FuzzyEq::fuzzy_ne(&$left, &$right)
    };
}

/// Asserts two values are approximately equal.
#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($left:expr, $right:expr) => {
        assert!(
            $crate::fuzzy_eq!($left, $right),
            "assertion `fuzzy_eq` failed\n  left: {:?}\n right: {:?}",
            $left,
            $right
        )
    };
}

/// Asserts two values are not approximately equal.
#[macro_export]
macro_rules! assert_fuzzy_ne {
    ($left:expr, $right:expr) => {
        assert!(
            $crate::fuzzy_ne!($left, $right),
            "assertion `fuzzy_ne` failed\n  left: {:?}\n right: {:?}",
            $left,
            $right
        )
    };
}

/// Approximate equality by absolute tolerance.
///
/// Returns `true` if `|x - y| <= tolerance`.
///
/// Best for values near zero. For larger values, prefer [`approx_eq_rel`].
/// NaN is never equal to anything.
#[must_use]
pub const fn approx_eq_abs(x: Real, y: Real, tolerance: Real) -> bool {
    debug_assert!(tolerance >= 0.0);

    // handle ±0, ±∞, exact matches
    #[expect(clippy::float_cmp)]
    if x == y {
        return true;
    }

    if x.is_nan() || y.is_nan() {
        return false;
    }

    (x - y).abs() <= tolerance
}

/// Approximate equality by relative tolerance.
///
/// Returns `true` if `|x - y| <= max(|x|, |y|) * tolerance`.
///
/// The `tolerance` should be positive; `sqrt(EPSILON)` is a common choice.
/// Not meaningful near zero — use [`approx_eq_abs`] there instead.
/// NaN is never equal to anything.
#[must_use]
pub const fn approx_eq_rel(x: Real, y: Real, tolerance: Real) -> bool {
    debug_assert!(tolerance > 0.0);

    // handle ±0, ±∞, exact matches
    #[expect(clippy::float_cmp)]
    if x == y {
        return true;
    }
    if x.is_nan() || y.is_nan() {
        return false;
    }

    let scale = x.abs().max(y.abs());
    (x - y).abs() <= scale * tolerance
}

/// Approximate equality using an adaptive epsilon.
///
/// Uses absolute epsilon for values near zero (`|x|, |y| < 1`),
/// and relative epsilon for larger values. This handles the full range
/// of `Real` correctly without manual tolerance selection.
///
/// NaN is never equal to anything.
#[must_use]
pub const fn approx_eq(x: Real, y: Real) -> bool { approx_eq_eps(x, y, EPSILON) }

/// Approximate equality using an adaptive epsilon.
///
/// Uses absolute epsilon for values near zero (`|x|, |y| < 1`),
/// and relative epsilon for larger values. This handles the full range
/// of `Real` correctly without manual tolerance selection.
///
/// NaN is never equal to anything.
#[must_use]
pub const fn approx_eq_eps(x: Real, y: Real, epsilon: Real) -> bool {
    #[expect(clippy::float_cmp)]
    if x == y {
        return true;
    }

    if x.is_nan() || y.is_nan() {
        return false;
    }

    if x.is_infinite() || y.is_infinite() {
        return false;
    }

    let scale = x.abs().max(y.abs());

    if scale < 1.0 { approx_eq_abs(x, y, epsilon) } else { approx_eq_rel(x, y, epsilon) }
}

#[cfg(test)]
mod tests {
    use super::*;

    // approx_eq_abs

    #[test]
    fn abs_zeros() {
        assert_fuzzy_eq!(0.0, 0.0);
        assert_fuzzy_eq!(-0.0, -0.0);
        assert_fuzzy_eq!(0.0, -0.0);
    }

    #[test]
    fn abs_within_tolerance() {
        // testing the boundary of a specific tolerance, not fuzzy equality
        assert!(approx_eq_abs(1.0 + EPSILON, 1.0, EPSILON));
    }

    #[test]
    fn abs_outside_tolerance() {
        assert_fuzzy_ne!(1.0 + 2.0 * EPSILON, 1.0);
    }

    #[test]
    fn abs_opposite_signs_near_zero() {
        assert_fuzzy_ne!(1e-8, -1e-8);
    }

    #[test]
    fn abs_min_positive_near_zero() {
        // MIN_POSITIVE is ~2.2e-308, well within any reasonable epsilon
        assert_fuzzy_eq!(Real::MIN_POSITIVE, 0.0);
    }

    // approx_eq_rel

    #[test]
    fn rel_exact_equality() {
        assert_fuzzy_eq!(1.0, 1.0);
        assert_fuzzy_eq!(Real::INFINITY, Real::INFINITY);
    }

    #[test]
    fn rel_nan_not_equal_to_nan() {
        assert_fuzzy_ne!(Real::NAN, Real::NAN);
    }

    #[test]
    fn rel_not_equal() {
        assert_fuzzy_ne!(1.0, 0.0);
    }

    #[test]
    fn rel_nan_not_equal_to_zero() {
        assert_fuzzy_ne!(Real::NAN, 0.0);
    }

    #[test]
    fn rel_large_values() {
        let a = 1_000_000.0;
        assert!(approx_eq_rel(a, a + a * EPSILON * 0.5, EPSILON));
        assert!(!approx_eq_rel(a, a + a * EPSILON * 2.0, EPSILON));
    }
    // approx_eq (adaptive)

    #[test]
    fn adaptive_exact_equality() {
        assert_fuzzy_eq!(1.0, 1.0);
        assert_fuzzy_eq!(-42.5, -42.5);
        assert_fuzzy_eq!(0.0, -0.0);
    }

    #[test]
    fn adaptive_infinity_equal_to_itself() {
        assert_fuzzy_eq!(Real::INFINITY, Real::INFINITY);
        assert_fuzzy_eq!(Real::NEG_INFINITY, Real::NEG_INFINITY);
    }

    #[test]
    fn adaptive_opposite_infinities_not_equal() {
        assert_fuzzy_ne!(Real::INFINITY, Real::NEG_INFINITY);
        assert_fuzzy_ne!(Real::INFINITY, 1.0);
    }

    #[test]
    fn adaptive_nan_never_equal() {
        let nan = Real::NAN;
        assert_fuzzy_ne!(nan, nan);
        assert_fuzzy_ne!(nan, 0.0);
        assert_fuzzy_ne!(0.0, nan);
    }

    #[test]
    fn adaptive_near_zero_uses_absolute() {
        assert_fuzzy_eq!(0.0, EPSILON * 0.5);
        assert_fuzzy_ne!(0.0, EPSILON * 2.0);
    }

    #[test]
    fn adaptive_large_values_use_relative() {
        let a = 1_000_000.0;
        assert!(approx_eq_rel(a, a + a * EPSILON * 0.5, EPSILON));
        assert!(!approx_eq_rel(a, a + a * EPSILON * 2.0, EPSILON));
    }

    #[test]
    fn adaptive_common_rounding_error() {
        assert_fuzzy_eq!(0.1 + 0.2, 0.3);
    }

    #[test]
    fn adaptive_symmetric() {
        let pairs =
            [(0.0, EPSILON * 0.5), (1.0, 1.0 + EPSILON), (1000.0, 1000.0 + EPSILON * 500.0)];
        for (a, b) in pairs {
            assert_eq!(fuzzy_eq!(a, b), fuzzy_eq!(b, a));
        }
    }

    #[test]
    fn adaptive_opposite_large_values_not_equal() {
        assert_fuzzy_ne!(1e6, -1e6);
    }
}
