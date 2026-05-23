use crate::prelude::{Point3, Vec3};

/// A ray: **P**(*t*) = origin + *t* × direction.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    #[inline]
    #[must_use]
    pub const fn new(origin: Point3, direction: Vec3) -> Self { Self { origin, direction } }

    /// Evaluates **P**(*t*) = origin + *t* × direction.
    #[inline]
    #[must_use]
    pub const fn at(&self, t: f64) -> Point3 { self.origin + t * self.direction }
}
