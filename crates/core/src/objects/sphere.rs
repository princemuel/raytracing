use rtc_shared::Real;

use crate::prelude::{HitRecord, Hittable, Interval, Point3};
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: Real,
}

impl Sphere {
    #[must_use]
    pub const fn new(center: Point3, radius: Real) -> Self {
        Self { center, radius: radius.max(0.0) }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: Ray, t: Interval) -> Option<HitRecord> {
        let origin_center = self.center - r.origin;

        let a = r.direction.length_squared();
        let h = r.direction.dot(origin_center);
        let c = origin_center.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let t = {
            let r1 = (h - sqrtd) / a;
            t.surrounds(r1).then_some(r1).or_else(|| {
                let r2 = (h + sqrtd) / a;
                t.surrounds(r2).then_some(r2)
            })?
        };

        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let record =
            HitRecord { p, t, ..Default::default() }.with_face_normal(&r, outward_normal);
        Some(record)
    }
}
