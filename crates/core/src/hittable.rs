use rtc_shared::Real;

use crate::prelude::{Point3, Vec3};
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: Ray, tmin: Real, tmax: Real, hit_record: &mut HitRecord) -> bool;
}

pub struct HitRecord(Point3, Vec3, Real);

impl HitRecord {
    pub const fn p(&self) -> Point3 { self.0 }

    pub const fn normal(&self) -> Vec3 { self.1 }

    pub const fn t(&self) -> Real { self.2 }

    pub const fn set_p(&mut self, value: Point3) { self.0 = value }

    pub const fn set_normal(&mut self, value: Vec3) { self.1 = value }

    pub const fn set_t(&mut self, value: Real) { self.2 = value }
}
