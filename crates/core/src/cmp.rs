const EPSILON: f32 = 1e-8;

/// Compares two f32 values for approximate equality using adaptive epsilon
///
/// Uses relative epsilon for larger values and absolute epsilon near zero.
/// This handles the full range of f32 values correctly.
pub const fn is_equal(a: f32, b: f32) -> bool {
    // Fast path: exact equality (handles infinities, zeros, and exact matches)
    // This is intentional - we want exact equality for special values
    #[allow(clippy::float_cmp)]
    if a == b {
        return true;
    }

    // Handle NaN cases - NaN should never equal anything
    if a.is_nan() || b.is_nan() {
        return false;
    }

    // Handle infinite cases that aren't exactly equal
    if a.is_infinite() || b.is_infinite() {
        return false; // Different infinities or one infinite, one finite
    }

    let diff = f32::abs(a - b);

    // For very small numbers near zero, use absolute epsilon
    if f32::max(f32::abs(a), f32::abs(b)) < 1.0 {
        return diff < EPSILON;
    }

    // For larger numbers, use relative epsilon to maintain precision
    // This prevents issues when comparing large coordinate values
    let relative_epsilon = EPSILON * f32::max(f32::abs(a), f32::abs(b));

    // Use the larger of absolute and relative epsilon
    // This handles edge cases around 1.0 and ensures consistent behavior
    diff < f32::max(EPSILON, relative_epsilon)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // Assuming EPSILON is 1e-6 for f32
//     const TEST_EPSILON: f32 = 1e-6;

//     #[test]
//     fn test_exact_equality() {
//         assert!(is_equal(1.0, 1.0));
//         assert!(is_equal(0.0, 0.0));
//         assert!(is_equal(-5.0, -5.0));
//     }

//     #[test]
//     fn test_near_zero() {
//         // Small numbers should use absolute epsilon
//         assert!(is_equal(0.0, TEST_EPSILON * 0.5));
//         assert!(!is_equal(0.0, TEST_EPSILON * 2.0));
//     }

//     #[test]
//     fn test_large_numbers() {
//         // Large numbers should use relative epsilon
//         let large = 1_000_000.0;
//         let diff = large * TEST_EPSILON * 0.5;
//         assert!(is_equal(large, large + diff));

//         let diff_too_large = large * TEST_EPSILON * 2.0;
//         assert!(!is_equal(large, large + diff_too_large));
//     }

//     #[test]
//     fn test_infinity() {
//         assert!(is_equal(f32::INFINITY, f32::INFINITY));
//         assert!(is_equal(f32::NEG_INFINITY, f32::NEG_INFINITY));
//         assert!(!is_equal(f32::INFINITY, f32::NEG_INFINITY));
//         assert!(!is_equal(f32::INFINITY, 1.0));
//     }

//     #[test]
//     fn test_nan() {
//         assert!(!is_equal(f32::NAN, f32::NAN));
//         assert!(!is_equal(f32::NAN, 0.0));
//         assert!(!is_equal(0.0, f32::NAN));
//     }

//     #[test]
//     fn test_common_float_errors() {
//         // Classic floating point error
//         let a = 0.1 + 0.2;
//         let b = 0.3;
//         assert!(is_equal(a, b));
//     }

//     #[test]
//     fn test_around_one() {
//         // Edge case around 1.0 where absolute and relative epsilon meet
//         assert!(is_equal(1.0, 1.0 + TEST_EPSILON * 0.5));
//         assert!(!is_equal(1.0, 1.0 + TEST_EPSILON * 2.0));
//     }
// }
