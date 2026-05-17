use rtc_shared::Real;

use crate::prelude::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Point3, direction: Vec3) -> Self { Self { origin, direction } }

    #[must_use]
    pub fn at(&self, t: Real) -> Point3 { self.origin + t * self.direction }
}
