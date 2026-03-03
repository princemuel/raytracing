use core::ops::RangeInclusive;

use rtc_shared::Real;

/// Creates a color
#[inline]
pub fn interval<MIN, MAX>(min: MIN, max: MAX) -> Interval
where
    MIN: Into<Real>,
    MAX: Into<Real>,
{
    Interval::new(min.into(), max.into())
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interval(RangeInclusive<Real>);
impl Interval {
    /// Creates a new interval with the given bounds (inclusive)
    pub const fn new(min: Real, max: Real) -> Self { Self(min..=max) }

    /// Creates an empty interval (min > max)
    pub const fn empty() -> Self { Self(Real::INFINITY..=Real::NEG_INFINITY) }

    /// Creates a universe interval (all real numbers)
    pub const fn universe() -> Self { Self(Real::NEG_INFINITY..=Real::INFINITY) }

    /// Returns the minimum bound
    pub const fn min(&self) -> Real { *self.0.start() }

    /// Returns the maximum bound
    pub const fn max(&self) -> Real { *self.0.end() }

    /// Clamps `x` to be within the interval bounds
    pub const fn clamp(&self, x: Real) -> Real { x.clamp(self.min(), self.max()) }

    /// Returns the raw size of the interval (`max - min`).
    ///
    /// The returned value is only meaningful for non-empty, finite intervals
    /// where `min <= max`. For empty intervals (`min > max`), the result is
    /// negative (typically `-∞`). For the universe interval, the result is
    /// `+∞`.
    /// # Safety
    /// This method performs no validation or clamping and should not be used
    /// to test emptiness. Prefer `is_empty`, `size_checked`, or
    /// `size_nonnegative` when safety is required.
    pub const fn size(&self) -> Real { self.max() - self.min() }

    pub const fn size_checked(&self) -> Option<Real> {
        if self.is_empty() || !self.is_valid() {
            None
        } else {
            Some(self.max() - self.min())
        }
    }

    pub const fn size_nonnegative(&self) -> Real {
        if self.is_empty() || !self.is_valid() {
            0.0
        } else {
            (self.max() - self.min()).max(0.0)
        }
    }

    /// Checks if the interval contains `x` (inclusive)
    pub const fn contains(&self, x: Real) -> bool { self.0.contains(&x) }

    /// Checks if the interval surrounds `x` (exclusive)
    pub const fn surrounds(&self, x: Real) -> bool { self.min() < x && x < self.max() }

    pub const fn is_empty(&self) -> bool { self.min() > self.max() }

    pub const fn is_universe(&self) -> bool {
        self.min() == Real::NEG_INFINITY && self.max() == Real::INFINITY
    }

    pub const fn is_valid(&self) -> bool { !self.min().is_nan() && !self.max().is_nan() }

    pub const fn is_finite(&self) -> bool { self.min().is_finite() && self.max().is_finite() }
}

impl Default for Interval {
    fn default() -> Self { Self::empty() }
}
// Convert from RangeInclusive<Real> to Interval
impl From<RangeInclusive<Real>> for Interval {
    fn from(range: RangeInclusive<Real>) -> Self { Self(range) }
}

// Convert from Interval to RangeInclusive<Real>
impl From<Interval> for RangeInclusive<Real> {
    fn from(interval: Interval) -> Self { interval.0 }
}

#[cfg(test)]
mod tests {
    use rtc_cmp::approx_eq_abs;
    use rtc_shared::EPSILON;

    use super::*;

    #[test]
    fn test_empty_interval() {
        let interval = Interval::empty();
        assert!(interval.size().is_nan() || interval.size() < 0.0);
        assert!(!interval.contains(0.0));
    }

    #[test]
    fn test_default_interval() {
        let interval = Interval::default();
        assert!(interval.size().is_nan() || interval.size() < 0.0);
        assert!(!interval.contains(0.0));
    }

    #[test]
    fn test_universe_interval() {
        let interval = Interval::universe();
        assert!(interval.contains(0.0));
        assert!(interval.contains(Real::MAX));
        assert!(interval.contains(Real::MIN));
    }

    #[test]
    fn test_contains() {
        let interval = Interval::new(0.0, 10.0);
        assert!(interval.contains(0.0));
        assert!(interval.contains(5.0));
        assert!(interval.contains(10.0));
        assert!(!interval.contains(-1.0));
        assert!(!interval.contains(11.0));
    }

    #[test]
    fn test_surrounds() {
        let interval = Interval::new(0.0, 10.0);
        assert!(!interval.surrounds(0.0)); // Boundary excluded
        assert!(interval.surrounds(5.0));
        assert!(!interval.surrounds(10.0)); // Boundary excluded
        assert!(!interval.surrounds(-1.0));
    }

    #[test]
    fn test_clamp() {
        let interval = Interval::new(0.0, 10.0);
        assert!(approx_eq_abs(interval.clamp(-5.0), 0.0, EPSILON));
        assert!(approx_eq_abs(interval.clamp(5.0), 5.0, EPSILON));
        assert!(approx_eq_abs(interval.clamp(15.0), 10.0, EPSILON));
    }

    #[test]
    fn test_from_range() {
        let range = 0.0..=10.0;
        let interval = Interval::from(range);
        assert!(approx_eq_abs(interval.min(), 0.0, EPSILON));
        assert!(approx_eq_abs(interval.max(), 10.0, EPSILON));
    }
}
