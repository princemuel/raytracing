use rtc_shared::Real;

use crate::prelude::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray(Point3, Vec3);

impl Ray {
    #[must_use]
    pub const fn new(origin: Point3, direction: Vec3) -> Self { Self(origin, direction) }

    pub const fn origin(&self) -> Point3 { self.0 }

    pub const fn direction(&self) -> Vec3 { self.1 }

    pub fn at(&self, t: Real) -> Point3 { self.origin() + t * self.direction() }
}
