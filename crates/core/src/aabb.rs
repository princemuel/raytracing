//! Axis-Aligned Bounding Boxes (AABBs)
//! Real Name: axis-aligned bounding rectangular parallelepipeds

use shared::TOLERANCE;

use crate::prelude::{Axis, Interval, Point3, Ray, interval};

#[derive(Clone, Copy, Debug, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub const EMPTY: Self = Self::splat(Interval::EMPTY);
    pub const UNIVERSE: Self = Self::splat(Interval::UNIVERSE);

    #[inline]
    #[must_use]
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self::pad_to_minimum(x, y, z)
    }

    #[must_use]
    pub const fn get(self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    #[inline]
    #[must_use]
    pub const fn splat(v: Interval) -> Self { Self { x: v, y: v, z: v } }
}

impl AABB {
    /// Returns the index of the longest axis of the bounding box.
    #[must_use]
    pub const fn longest_axis(&self) -> Axis {
        if self.x.size() >= self.y.size() && self.x.size() >= self.z.size() {
            Axis::X
        } else if self.y.size() >= self.z.size() {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    /// Adjusts [`AABB`] so that no side is narrower than some delta, padding it
    /// if necessary.
    const fn pad_to_minimum(x: Interval, y: Interval, z: Interval) -> Self {
        const DELTA: f64 = TOLERANCE;

        let x = if x.size() < DELTA { x.expand(DELTA) } else { x };
        let y = if y.size() < DELTA { y.expand(DELTA) } else { y };
        let z = if z.size() < DELTA { z.expand(DELTA) } else { z };
        Self { x, y, z }
    }

    #[must_use]
    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let ax = self.get(axis);
            let adinv = ray.direction.get(axis).recip();

            let (t0, t1) = {
                let t0 = (ax.min - ray.origin.get(axis)) * adinv;
                let t1 = (ax.max - ray.origin.get(axis)) * adinv;
                if adinv < 0.0 { (t1, t0) } else { (t0, t1) }
            };

            if t0 > ray_t.min {
                ray_t.min = t0;
            }

            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}

impl From<(Point3, Point3)> for AABB {
    /// Treat the two points a and b as extrema for the bounding box, so we
    /// don't require a particular minimum/maximum coordinate order.
    fn from((a, b): (Point3, Point3)) -> Self {
        let x = if a.x <= b.x { interval(a.x, b.x) } else { interval(b.x, a.x) };
        let y = if a.y <= b.y { interval(a.y, b.y) } else { interval(b.y, a.y) };
        let z = if a.z <= b.z { interval(a.z, b.z) } else { interval(b.z, a.z) };
        Self::pad_to_minimum(x, y, z)
    }
}

impl From<(AABB, AABB)> for AABB {
    /// Create the AABB tightly enclosing the two input aAABBs.
    fn from((box0, box1): (AABB, AABB)) -> Self {
        let x = Interval::from((box0.x, box1.x));
        let y = Interval::from((box0.y, box1.y));
        let z = Interval::from((box0.z, box1.z));
        Self { x, y, z }
    }
}
