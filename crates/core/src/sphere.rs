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
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord> {
        let origin_center = self.center - ray.origin;

        let a = ray.direction.length_squared();
        let h = ray.direction.dot(origin_center);
        let c = origin_center.length_squared() - self.radius * self.radius;

        let discriminant: Real = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let root = {
            let r1 = (h - sqrtd) / a;
            if t.surrounds(r1) {
                r1
            } else {
                let r2 = (h + sqrtd) / a;
                if t.surrounds(r2) {
                    r2
                } else {
                    return None;
                }
            }
        };

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord { p, t: root, ..Default::default() };
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
