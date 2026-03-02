#![warn(clippy::pedantic)]
#![warn(clippy::ptr_arg)]
#![warn(clippy::use_self)]
#![warn(clippy::suspicious)]
#![warn(clippy::perf)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_ops)]

use rtc_shared::{EPSILON, Real};

/// Performs an approximate comparison of two floating point values `x` and `y`.
/// Returns true if the absolute difference between them is less or equal than
/// the specified tolerance.
///
/// The `tolerance` parameter is the absolute tolerance used when determining if
/// the two numbers are close enough; a good value for this parameter is a small
/// multiple of `floatEps(T)`.
///
/// Note that this function is recommended for comparing small numbers
/// around zero; using `approx_eq_rel` is suggested otherwise.
///
/// NaN values are never considered equal to any value.
#[must_use]
pub const fn approx_eq_abs(x: Real, y: Real, tolerance: Real) -> bool {
    debug_assert!(tolerance >= 0.0);

    // Fast path for equal values (and signed zeros and infinites).
    // This is intentional - we want exact equality for special values
    #[allow(clippy::float_cmp)]
    if x == y {
        return true;
    }

    if x.is_nan() || y.is_nan() {
        return false;
    }

    let delta = (x - y).abs();
    delta <= tolerance
}

/// Performs an approximate comparison of two floating point values `x` and `y`.
/// Returns true if the absolute difference between them is less or equal than
/// `max(|x|, |y|) * tolerance`, where `tolerance` is a positive number greater
/// than zero.
///
/// The `tolerance` parameter is the relative tolerance used when determining if
/// the two numbers are close enough; a good value for this parameter is usually
/// `sqrt(floatEps(T))`, meaning that the two numbers are considered equal if at
/// least half of the digits are equal.
///
/// Note that for comparisons of small numbers around zero this function won't
/// give meaningful results, use `approx_eq_abs` instead.
///
/// NaN values are never considered equal to any value.
#[must_use]
pub const fn approx_eq_rel(x: Real, y: Real, tolerance: Real) -> bool {
    debug_assert!(tolerance > 0.0);
    // Fast path for equal values (and signed zeros and infinites).
    // This is intentional - we want exact equality for special values
    #[allow(clippy::float_cmp)]
    if x == y {
        return true;
    }

    if x.is_nan() || y.is_nan() {
        return false;
    }

    let scale = x.abs().max(y.abs());
    let delta = (x - y).abs();

    delta <= scale * tolerance
}

/// Compares two Real values for approximate equality using adaptive epsilon
///
/// Uses relative epsilon for larger values and absolute epsilon near zero.
/// This handles the full range of Real values correctly.
#[must_use]
pub const fn approx_equal_fl(x: Real, y: Real) -> bool {
    // Fast path: exact equality (handles infinities, zeros, and exact matches)
    // This is intentional - we want exact equality for special values
    #[allow(clippy::float_cmp)]
    if x == y {
        return true;
    }

    // Handle NaN cases - NaN should never equal anything
    if x.is_nan() || y.is_nan() {
        return false;
    }

    // Handle infinite cases that aren't exactly equal
    if x.is_infinite() || y.is_infinite() {
        return false; // Different infinities or one infinite, one finite
    }

    let delta = (x - y).abs();
    let scale = x.abs().max(y.abs());

    // For very small numbers near zero, use absolute epsilon
    if scale < 1.0 {
        return delta < EPSILON;
    }

    // For larger numbers, use relative epsilon to maintain precision
    // This prevents issues when comparing large coordinate values
    let relative_epsilon = EPSILON * scale;

    // Use the larger of absolute and relative epsilon
    // This handles edge cases around 1.0 and ensures consistent behavior
    delta < EPSILON.max(relative_epsilon)
}

#[cfg(test)]
mod tests {
    use rtc_shared::{EPSILON, Real};

    use super::*;

    #[test]
    fn scenario_exact_equality() {
        assert!(approx_equal_fl(1.0, 1.0));
        assert!(approx_equal_fl(-42.5, -42.5));
        assert!(approx_equal_fl(-5.0, -5.0));
        assert!(approx_equal_fl(0.0, 0.0));
        assert!(approx_equal_fl(-0.0, 0.0));
    }

    #[test]
    fn scenario_exact_infinity_equality() {
        assert!(approx_equal_fl(Real::INFINITY, Real::INFINITY));
        assert!(approx_equal_fl(Real::NEG_INFINITY, Real::NEG_INFINITY));
    }

    #[test]
    fn scenario_nan_never_equals_anything() {
        let nan = Real::NAN;

        assert!(!approx_equal_fl(nan, nan));
        assert!(!approx_equal_fl(nan, 0.0));
        assert!(!approx_equal_fl(0.0, nan));
        assert!(!approx_equal_fl(nan, Real::INFINITY));
    }

    #[test]
    fn scenario_infinities_do_not_mix() {
        assert!(!approx_equal_fl(Real::INFINITY, Real::NEG_INFINITY));
        assert!(!approx_equal_fl(Real::INFINITY, 1.0));
        assert!(!approx_equal_fl(-1.0, Real::NEG_INFINITY));
    }

    #[test]
    fn scenario_near_zero_absolute_epsilon() {
        assert!(approx_equal_fl(0.0, EPSILON * 0.5));
        assert!(!approx_equal_fl(0.0, EPSILON * 2.0));
    }

    #[test]
    fn scenario_tiny_denormals_within_epsilon() {
        let a = Real::MIN_POSITIVE;
        let b = a + EPSILON * 0.25;

        assert!(approx_equal_fl(a, b));
    }

    #[test]
    fn scenario_large_values_relative_epsilon() {
        let a = 1_000_000.0;

        assert!(approx_equal_fl(a, a + a * EPSILON * 0.5));
        assert!(!approx_equal_fl(a, a + a * EPSILON * 2.0));
    }

    #[test]
    fn scenario_around_one_boundary() {
        assert!(approx_equal_fl(1.0, 1.0 + EPSILON * 0.5));
        assert!(!approx_equal_fl(1.0, 1.0 + EPSILON * 2.0));
    }

    #[test]
    fn scenario_comparison_is_symmetric() {
        let pairs = [
            (0.0, EPSILON * 0.5),
            (1.0, 1.0 + EPSILON),
            (1000.0, 1000.0 + EPSILON * 500.0),
        ];

        for &(a, b) in &pairs {
            assert_eq!(approx_equal_fl(a, b), approx_equal_fl(b, a));
        }
    }

    #[test]
    fn scenario_increasing_difference_breaks_equality() {
        let a = 10.0;

        assert!(approx_equal_fl(a, a + a * EPSILON * 0.5));
        assert!(!approx_equal_fl(a, a + a * EPSILON * 5.0));
    }

    #[test]
    fn scenario_small_opposite_signs_are_not_equal() {
        assert!(!approx_eq_rel(1e-8, -1e-8, EPSILON));
    }

    #[test]
    fn scenario_opposite_large_values_are_not_equal() {
        assert!(!approx_equal_fl(1e6, -1e6));
    }

    #[test]
    fn scenario_common_float_rounding_error() {
        let a = 0.1 + 0.2;
        let b = 0.3;

        assert!(approx_equal_fl(a, b));
    }

    #[test]
    fn scenario_approximately_eq_to_abs_val() {
        //  for  f16, f32, Real, f128 } {
        //     const EPSILON = comptime floatEps(T);
        //     const min_value = comptime floatMin(T);

        //     assert!(approx_eq_abs(T, 0.0, 0.0, EPSILON));
        //     assert!(approx_eq_abs(T, -0.0, -0.0, EPSILON));
        //     assert!(approx_eq_abs(T, 0.0, -0.0, EPSILON));
        //     assert!(!approx_eq_abs(T, 1.0 + 2 * EPSILON, 1.0, EPSILON));
        //     assert!(approx_eq_abs(T, 1.0 + 1 * EPSILON, 1.0, EPSILON));
        //     assert!(approx_eq_abs(T, min_value, 0.0, EPSILON * 2));
        //     assert!(approx_eq_abs(T, -min_value, 0.0, EPSILON * 2));
        // }

        // `` is guaranteed to have the same precision and operations of
        // the largest other floating point type, which is f128 but it doesn't have a
        // defined layout so we can't rely on `@bitCast` to construct the smallest
        // possible epsilon value like we do in the tests above. In the same vein, we
        // also can't represent a max/min, `NaN` or `Inf` values.

        assert!(approx_eq_abs(0.0, 0.0, EPSILON));
        assert!(approx_eq_abs(-0.0, -0.0, EPSILON));
        assert!(approx_eq_abs(0.0, -0.0, EPSILON));
        assert!(!approx_eq_abs(1.0 + 2.0 * EPSILON, 1.0, EPSILON));
        assert!(approx_eq_abs(1.0 + 1.0 * EPSILON, 1.0, EPSILON));
    }

    #[test]
    fn scenario_approximately_eq_to_rel_val() {
        // inline for ([_]type{ f16, f32, Real, f128 }) |T| {
        //     const EPSILON = comptime floatEps(T);
        //     const sqrt_EPSILON = comptime sqrt(EPSILON);
        //     const nan_value = comptime nan(T);
        //     const inf_value = comptime inf(T);
        //     const min_value = comptime floatMin(T);

        //     assert!(approx_eq_rel(T, 1.0, 1.0, sqrt_EPSILON));
        //     assert!(!approx_eq_rel(T, 1.0, 0.0, sqrt_EPSILON));
        //     assert!(!approx_eq_rel(T, 1.0, nan_value, sqrt_EPSILON));
        //     assert!(!approx_eq_rel(T, nan_value, nan_value, sqrt_EPSILON));
        //     assert!(approx_eq_rel(T, inf_value, inf_value, sqrt_EPSILON));
        //     assert!(approx_eq_rel(T, min_value, min_value, sqrt_EPSILON));
        //     assert!(approx_eq_rel(T, -min_value, -min_value, sqrt_EPSILON));
        // }

        // `` is guaranteed to have the same precision and operations of
        // the largest other floating point type, which is f128 but it doesn't
        // have a defined layout so we can't rely on `@bitCast` to
        // construct the smallest possible epsilon value like we do in
        // the tests above. In the same vein, we also can't represent a
        // max/min, `NaN` or `Inf` values. const EPSILON: Real = 1e-4;
        // const SQRT_EPSILON: Real = EPSILON.algebraic_div(rhs);

        // assert!(approx_eq_rel(, 1.0, 1.0, SQRT_EPSILON));
        // assert!(!approx_eq_rel(, 1.0, 0.0, SQRT_EPSILON));
    }
}
