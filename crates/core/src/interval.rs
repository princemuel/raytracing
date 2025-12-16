use core::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq)]
pub struct Interval(RangeInclusive<f32>);
impl Interval {
    /// Creates a new interval with the given bounds (inclusive)
    pub const fn new(min: f32, max: f32) -> Self { Self(min..=max) }

    /// Creates an empty interval (min > max)
    pub const fn empty() -> Self { Self(f32::INFINITY..=f32::NEG_INFINITY) }

    /// Creates a universe interval (all real numbers)
    pub const fn universe() -> Self { Self(f32::NEG_INFINITY..=f32::INFINITY) }

    /// Returns the minimum bound
    pub const fn min(&self) -> f32 { *self.0.start() }

    /// Returns the maximum bound
    pub const fn max(&self) -> f32 { *self.0.end() }

    /// Clamps `x` to be within the interval bounds
    pub const fn clamp(&self, x: f32) -> f32 { x.clamp(self.min(), self.max()) }

    /// Returns the size of the interval
    pub fn size(&self) -> f32 { self.max() - self.min() }

    /// Checks if the interval contains `x` (inclusive)
    pub fn contains(&self, x: f32) -> bool { self.0.contains(&x) }

    /// Checks if the interval surrounds `x` (exclusive)
    pub fn surrounds(&self, x: f32) -> bool { self.min() < x && x < self.max() }
}

impl Default for Interval {
    fn default() -> Self { Self::empty() }
}
// Convert from RangeInclusive<f32> to Interval
impl From<RangeInclusive<f32>> for Interval {
    fn from(range: RangeInclusive<f32>) -> Self { Self(range) }
}

// Convert from Interval to RangeInclusive<f32>
impl From<Interval> for RangeInclusive<f32> {
    fn from(interval: Interval) -> Self { interval.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::is_equal;

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
        assert!(interval.contains(f32::MAX));
        assert!(interval.contains(f32::MIN));
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
        assert!(is_equal(interval.clamp(-5.0), 0.0));
        assert!(is_equal(interval.clamp(5.0), 5.0));
        assert!(is_equal(interval.clamp(15.0), 10.0));
    }

    #[test]
    fn test_from_range() {
        let range = 0.0..=10.0;
        let interval = Interval::from(range);
        assert!(is_equal(interval.min(), 0.0));
        assert!(is_equal(interval.max(), 10.0));
    }
}
