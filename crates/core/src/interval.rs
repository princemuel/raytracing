use core::ops::RangeInclusive;

use rtc_shared::Real;

#[inline]
pub fn interval(min: impl Into<Real>, max: impl Into<Real>) -> Interval {
    Interval::new(min.into(), max.into())
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Interval {
    pub min: Real,
    pub max: Real,
}

impl Interval {
    /// An interval containing no values (`min > max`).
    pub const EMPTY: Self = Self { min: Real::INFINITY, max: Real::NEG_INFINITY };
    /// An interval containing all real numbers.
    pub const UNIVERSE: Self = Self { min: Real::NEG_INFINITY, max: Real::INFINITY };

    #[must_use]
    pub const fn new(min: Real, max: Real) -> Self { Self { min, max } }

    /// Raw size (`max - min`). Negative for empty intervals.
    #[must_use]
    pub const fn size(&self) -> Real { self.max - self.min }

    /// Clamps `x` to `[min, max]`.
    #[must_use]
    pub const fn clamp(&self, x: Real) -> Real { x.clamp(self.min, self.max) }

    /// Returns `true` if `x` is within `[min, max]` (inclusive).
    #[must_use]
    pub const fn contains(&self, x: Real) -> bool { self.min <= x && x <= self.max }

    /// Returns `true` if `x` is strictly inside `(min, max)` (exclusive).
    #[must_use]
    pub const fn surrounds(&self, x: Real) -> bool { self.min < x && x < self.max }

    #[must_use]
    pub const fn is_empty(&self) -> bool { self.min > self.max }

    /// Note: uses exact float equality against sentinel constants.
    #[must_use]
    pub const fn is_universe(&self) -> bool {
        self.min == Real::NEG_INFINITY && self.max == Real::INFINITY
    }
}

/// Defaults to `EMPTY`.
/// Use `Interval::UNIVERSE` explicitly to accept all values.
impl Default for Interval {
    fn default() -> Self { Self::EMPTY }
}

impl From<(Real, Real)> for Interval {
    fn from((min, max): (Real, Real)) -> Self { Self::new(min, max) }
}

impl From<RangeInclusive<Real>> for Interval {
    fn from(r: RangeInclusive<Real>) -> Self { Self::new(*r.start(), *r.end()) }
}

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
        assert!(i.contains(Real::MAX));
        assert!(i.contains(Real::MIN));
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
