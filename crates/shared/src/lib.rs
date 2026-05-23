use core::f64;

use rand::prelude::*;

/// Adaptive fuzzy-equality epsilon; also the default for [`approx_eq`].
pub const TOLERANCE: f64 = 1e-6;
pub const PI: f64 = f64::consts::PI;

// ---------------------------------------------------------------------------
// RNG helpers
// ---------------------------------------------------------------------------

/// Returns a random real in [0,1).
#[inline]
#[must_use]
pub fn random(rng: &mut impl Rng) -> f64 { rng.random() }

/// Returns a random real in [min,max).
///
/// The upper bound is exclusive: `rng.random::<f64>()` is in `[0,1)`,
/// so the result is in `[min, max)`.
#[inline]
#[must_use]
pub fn random_range(rng: &mut impl Rng, min: f64, max: f64) -> f64 {
    debug_assert!(min <= max, "random_range: min ({min}) must be <= max ({max})");
    min + (max - min) * rng.random::<f64>()
}

// ---------------------------------------------------------------------------
// FuzzyEq trait
// ---------------------------------------------------------------------------

pub trait FuzzyEq<Rhs = Self>
where
    Self: Sized,
{
    type Float;
    const TOLERANCE: Self::Float;

    fn fuzzy_eq(&self, other: &Rhs) -> bool;

    #[inline]
    fn fuzzy_ne(&self, other: &Rhs) -> bool { !self.fuzzy_eq(other) }
}

impl FuzzyEq for f32 {
    type Float = f32;

    const TOLERANCE: f32 = 1e-6;

    fn fuzzy_eq(&self, other: &Self) -> bool {
        approx_eq_eps(f64::from(*self), f64::from(*other), f64::from(Self::TOLERANCE))
    }
}

impl FuzzyEq for f64 {
    type Float = f64;

    const TOLERANCE: f64 = 1e-6;

    fn fuzzy_eq(&self, other: &Self) -> bool { approx_eq_eps(*self, *other, Self::TOLERANCE) }
}

// ---------------------------------------------------------------------------
// Convenience macros
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Stand-alone approximate-equality functions
// ---------------------------------------------------------------------------

/// Approximate equality by absolute tolerance.
///
/// Returns `true` if `|x - y| <= tolerance`.
///
/// Best for values near zero. For larger values, prefer [`approx_eq_rel`].
///
/// - `±∞ == ±∞` returns `true` (via the `x == y` fast path — IEEE semantics).
/// - `NaN` is never equal to anything.
#[must_use]
pub const fn approx_eq_abs(x: f64, y: f64, tolerance: f64) -> bool {
    debug_assert!(tolerance >= 0.0_f64);

    // Handles ±0 == ±0, ±∞ == ±∞, and exact matches.
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
/// The `tolerance` should be positive; `sqrt(TOLERANCE)` is a common choice.
/// Not meaningful near zero. use [`approx_eq_abs`] there instead.
///
/// - `±∞ == ±∞` returns `true` (via the `x == y` fast path).
/// - Mixed `∞`/finite comparisons return `false` (the infinity guard below).
///   Without this guard, `scale` would be `∞`, making `∞ * tolerance = ∞`, and
///   any finite difference would be `<= ∞` — incorrectly returning `true`.
/// - `NaN` is never equal to anything.
#[must_use]
pub const fn approx_eq_rel(x: f64, y: f64, tolerance: f64) -> bool {
    debug_assert!(tolerance > 0.0_f64);

    // Handles ±0 == ±0, ±∞ == ±∞, and exact matches.
    #[expect(clippy::float_cmp)]
    if x == y {
        return true;
    }
    if x.is_nan() || y.is_nan() {
        return false;
    }
    // Without this, approx_eq_rel(INFINITY, any_finite, t) returns true
    // because scale = ∞ and (∞ - finite).abs() = ∞ <= ∞ * t = ∞.
    if x.is_infinite() || y.is_infinite() {
        return false;
    }

    let scale = x.abs().max(y.abs());
    (x - y).abs() <= scale * tolerance
}

/// Approximate equality using an adaptive epsilon equal to [`TOLERANCE`].
///
/// Convenience wrapper for [`approx_eq_eps`]`(x, y, TOLERANCE)`.
/// See that function for full semantics.
#[must_use]
pub const fn approx_eq(x: f64, y: f64) -> bool { approx_eq_eps(x, y, TOLERANCE) }

/// Approximate equality using an adaptive epsilon.
///
/// Uses [`approx_eq_abs`] for values near zero (`max(|x|, |y|) < 1`) and
/// [`approx_eq_rel`] for larger values. This handles the full range of `f64`
/// correctly without manual tolerance selection.
///
/// - `±∞ == ±∞` returns `true` (via the `x == y` fast path).
/// - Mixed `∞`/finite inputs return `false` (the infinity guard below exists
///   only as a safety net; in practice, `approx_eq_rel` already rejects them,
///   but the guard makes the intent explicit and avoids relying on delegation).
/// - `NaN` is never equal to anything.
#[must_use]
pub const fn approx_eq_eps(x: f64, y: f64, epsilon: f64) -> bool {
    // Handles ±0 == ±0, ±∞ == ±∞, and exact matches.
    #[expect(clippy::float_cmp)]
    if x == y {
        return true;
    }

    if x.is_nan() || y.is_nan() {
        return false;
    }

    // Fires only for mixed ∞/finite inputs. Equal infinities are already
    // handled by the x == y fast path above, so this is NOT dead code.
    // It catches e.g. (INFINITY, 1.0) before the scale computation below
    // would produce a meaningless result.
    if x.is_infinite() || y.is_infinite() {
        return false;
    }

    let scale = x.abs().max(y.abs());

    if scale < 1.0 { approx_eq_abs(x, y, epsilon) } else { approx_eq_rel(x, y, epsilon) }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- approx_eq_abs ---

    #[test]
    fn abs_zeros() {
        assert_fuzzy_eq!(0.0_f64, 0.0_f64);
        assert_fuzzy_eq!(-0.0_f64, -0.0_f64);
        assert_fuzzy_eq!(0.0_f64, -0.0_f64);
    }

    #[test]
    fn abs_within_tolerance() {
        // x and y differ by exactly `tolerance` should be equal.
        assert!(approx_eq_abs(1.0 + TOLERANCE, 1.0, TOLERANCE));
    }

    #[test]
    fn abs_outside_tolerance() {
        // x and y differ by 2*tol — should not be equal.
        let tol = TOLERANCE;
        assert!(!approx_eq_abs(1.0 + 2.0 * tol, 1.0, tol));
    }

    #[test]
    fn abs_opposite_signs_near_zero() {
        // delta = 2e-8, far smaller than any sane epsilon → equal.
        assert_fuzzy_eq!(1e-8_f64, -1e-8_f64);
        // delta = 2 * TOLERANCE, always outside tolerance → not equal.
        let t = <f64 as FuzzyEq>::TOLERANCE;
        assert_fuzzy_ne!(t, -t);
    }

    #[test]
    fn abs_min_positive_near_zero() {
        // MIN_POSITIVE (~2.2e-308) is always within any reasonable tolerance.
        assert_fuzzy_eq!(f64::MIN_POSITIVE, 0.0_f64);
    }

    // --- approx_eq_rel ---

    #[test]
    fn rel_exact_equality() {
        assert!(approx_eq_rel(1.0, 1.0, TOLERANCE));
        assert!(approx_eq_rel(f64::INFINITY, f64::INFINITY, TOLERANCE));
    }

    #[test]
    fn rel_nan_not_equal_to_nan() {
        assert!(!approx_eq_rel(f64::NAN, f64::NAN, TOLERANCE));
    }

    #[test]
    fn rel_not_equal() {
        assert!(!approx_eq_rel(1.0, 0.0_f64, TOLERANCE));
    }

    #[test]
    fn rel_nan_not_equal_to_zero() {
        assert!(!approx_eq_rel(f64::NAN, 0.0_f64, TOLERANCE));
    }

    #[test]
    fn rel_large_values() {
        // Values differing by half the relative tolerance → equal.
        // Values differing by twice the relative tolerance → not equal.
        let a = 1_000_000.0_f64;
        assert!(approx_eq_rel(a, a + a * TOLERANCE * 0.5, TOLERANCE));
        assert!(!approx_eq_rel(a, a + a * TOLERANCE * 2.0, TOLERANCE));
    }

    #[test]
    fn rel_infinity_not_equal_to_finite() {
        assert!(!approx_eq_rel(f64::INFINITY, 1.0, TOLERANCE));
        assert!(!approx_eq_rel(1.0, f64::INFINITY, TOLERANCE));
        assert!(!approx_eq_rel(f64::NEG_INFINITY, 1.0, TOLERANCE));
        assert!(!approx_eq_rel(1.0, f64::NEG_INFINITY, TOLERANCE));
        assert!(!approx_eq_rel(f64::INFINITY, 0.0_f64, TOLERANCE));
    }

    #[test]
    fn rel_opposite_infinities_not_equal() {
        assert!(!approx_eq_rel(f64::INFINITY, f64::NEG_INFINITY, TOLERANCE));
    }

    // --- FuzzyEq trait (via macros) ---

    #[test]
    fn adaptive_exact_equality() {
        assert_fuzzy_eq!(1.0_f64, 1.0_f64);
        assert_fuzzy_eq!(-42.5_f64, -42.5_f64);
        assert_fuzzy_eq!(0.0_f64, -0.0_f64);
    }

    #[test]
    fn adaptive_infinity_equal_to_itself() {
        assert_fuzzy_eq!(f64::INFINITY, f64::INFINITY);
        assert_fuzzy_eq!(f64::NEG_INFINITY, f64::NEG_INFINITY);
    }

    #[test]
    fn adaptive_opposite_infinities_not_equal() {
        assert_fuzzy_ne!(f64::INFINITY, f64::NEG_INFINITY);
        assert_fuzzy_ne!(f64::INFINITY, 1.0_f64);
    }

    #[test]
    fn adaptive_nan_never_equal() {
        assert_fuzzy_ne!(f64::NAN, f64::NAN);
        assert_fuzzy_ne!(f64::NAN, 0.0_f64);
        assert_fuzzy_ne!(0.0_f64, f64::NAN);
    }

    #[test]
    fn adaptive_near_zero_uses_absolute() {
        // Use the trait's own TOLERANCE so the test stays valid for any epsilon.
        let t = <f64 as FuzzyEq>::TOLERANCE;
        // delta = t*0.5, within tolerance → equal.
        assert_fuzzy_eq!(0.0_f64, t * 0.5);
        // delta = t*2.0, outside tolerance → not equal.
        assert_fuzzy_ne!(0.0_f64, t * 2.0);
    }

    #[test]
    fn adaptive_large_values_use_relative() {
        let a = 1_000_000.0_f64;
        let t = <f64 as FuzzyEq>::TOLERANCE;
        assert!(approx_eq_rel(a, a + a * t * 0.5, t));
        assert!(!approx_eq_rel(a, a + a * t * 2.0, t));
    }

    #[test]
    fn adaptive_common_rounding_error() {
        // 0.1 + 0.2 != 0.3 exactly in IEEE 754; fuzzy equality should absorb it.
        assert_fuzzy_eq!(0.1_f64 + 0.2_f64, 0.3_f64);
    }

    #[test]
    fn adaptive_symmetric() {
        let t = <f64 as FuzzyEq>::TOLERANCE;
        let pairs = [
            (0.0_f64, t * 0.5),
            (1.0_f64, 1.0_f64 + t),
            (1000.0_f64, 1000.0_f64 + t * 500.0_f64),
        ];
        for (a, b) in pairs {
            assert_eq!(fuzzy_eq!(a, b), fuzzy_eq!(b, a));
        }
    }

    #[test]
    fn adaptive_opposite_large_values_not_equal() {
        // 1e6 and -1e6 differ by 2e6, always outside any sane tolerance.
        assert_fuzzy_ne!(1e6_f64, -1e6_f64);
    }

    // --- random_range ---

    #[test]
    fn random_range_stays_below_max() {
        let mut rng = rand::rng();
        for _ in 0..10_000 {
            let v = random_range(&mut rng, 0.0_f64, 1.0_f64);
            assert!((0.0_f64..1.0_f64).contains(&v), "out of [0,1): {v}");
        }
    }
}
