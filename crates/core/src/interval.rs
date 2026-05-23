use core::ops::RangeInclusive;

/// Convenience constructor.
#[inline]
#[must_use]
pub fn interval(min: impl Into<f64>, max: impl Into<f64>) -> Interval {
    Interval::new(min.into(), max.into())
}

/// A closed interval [min, max] over the floats.
///
/// `Interval::EMPTY` (min > max) is the canonical "no values" sentinel.
///
/// `Interval::UNIVERSE` covers all finite and infinite floats.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// Contains no values (`min > max`).
    pub const EMPTY: Self = Self { min: f64::INFINITY, max: f64::NEG_INFINITY };
    /// Contains every float.
    pub const UNIVERSE: Self = Self { min: f64::NEG_INFINITY, max: f64::INFINITY };

    #[inline]
    #[must_use]
    pub const fn new(min: f64, max: f64) -> Self { Self { min, max } }

    /// Signed size (`max − min`). Negative for empty intervals.
    #[inline]
    #[must_use]
    pub const fn size(self) -> f64 { self.max - self.min }

    /// Clamps `x` into `[min, max]`.
    #[inline]
    #[must_use]
    pub const fn clamp(self, x: f64) -> f64 { x.clamp(self.min, self.max) }

    /// `true` if `min <= x <= max` (inclusive).
    #[inline]
    #[must_use]
    pub const fn contains(self, x: f64) -> bool { self.min <= x && x <= self.max }

    /// `true` if `min < x < max` (exclusive). used for hit-record filtering.
    #[inline]
    #[must_use]
    pub const fn surrounds(self, x: f64) -> bool { self.min < x && x < self.max }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool { self.min > self.max }

    /// Note: uses exact float equality against the sentinel constants.
    #[inline]
    #[must_use]
    pub const fn is_universe(self) -> bool {
        self.min == f64::NEG_INFINITY && self.max == f64::INFINITY
    }
}

/// Defaults to `EMPTY`. Use `Interval::UNIVERSE` explicitly to accept all
/// values.
impl Default for Interval {
    fn default() -> Self { Self::EMPTY }
}

impl From<(f64, f64)> for Interval {
    fn from((min, max): (f64, f64)) -> Self { Self::new(min, max) }
}

impl From<RangeInclusive<f64>> for Interval {
    fn from(r: RangeInclusive<f64>) -> Self { Self::new(*r.start(), *r.end()) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use rtc_shared::assert_fuzzy_eq;

    use super::*;

    #[test]
    fn scenario_empty_interval() {
        let i = Interval::EMPTY;
        assert!(i.is_empty());
        assert!(i.size() < 0.0);
        assert!(!i.contains(0.0));
    }

    #[test]
    fn scenario_default_is_empty() {
        assert_eq!(Interval::default(), Interval::EMPTY);
    }

    #[test]
    fn scenario_universe_interval() {
        let i = Interval::UNIVERSE;
        assert!(i.contains(0.0));
        assert!(i.contains(f64::MAX));
        assert!(i.contains(f64::MIN));
        assert!(i.is_universe());
    }

    #[test]
    fn scenario_contains() {
        let i = Interval::new(0.0, 10.0);
        assert!(i.contains(0.0));
        assert!(i.contains(5.0));
        assert!(i.contains(10.0));
        assert!(!i.contains(-1.0));
        assert!(!i.contains(11.0));
    }

    #[test]
    fn scenario_surrounds() {
        let i = Interval::new(0.0, 10.0);
        assert!(!i.surrounds(0.0));
        assert!(i.surrounds(5.0));
        assert!(!i.surrounds(10.0));
        assert!(!i.surrounds(-1.0));
    }

    #[test]
    fn scenario_clamp() {
        let i = Interval::new(0.0, 10.0);
        assert_fuzzy_eq!(i.clamp(-5.0), 0.0);
        assert_fuzzy_eq!(i.clamp(5.0), 5.0);
        assert_fuzzy_eq!(i.clamp(15.0), 10.0);
    }

    #[test]
    fn scenario_from_tuple() {
        let i = Interval::from((0.0, 10.0));
        assert_fuzzy_eq!(i.min, 0.0);
        assert_fuzzy_eq!(i.max, 10.0);
    }

    #[test]
    fn scenario_from_inclusive_range() {
        let i = Interval::from(0.0..=10.0);
        assert_fuzzy_eq!(i.min, 0.0);
        assert_fuzzy_eq!(i.max, 10.0);
    }
}
