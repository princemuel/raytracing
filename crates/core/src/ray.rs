use crate::prelude::{Point3, Vec3};

/// A ray **P**(*t*) = origin + *t* × direction.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    #[inline]
    #[must_use]
    pub const fn new(origin: Point3, direction: Vec3, time: Option<f64>) -> Self {
        Self { origin, direction, time: if let Some(time) = time { time } else { 0.0 } }
    }

    /// Evaluates **P**(*t*) = origin + *t* × direction.
    #[inline]
    #[must_use]
    pub const fn at(&self, t: f64) -> Point3 { self.origin + t * self.direction }
}
