use rtc_shared::Real;

use crate::prelude::{HitRecord, Hittable, Interval, Point3};
use crate::ray::Ray;

pub struct Sphere(Point3, Real);

impl Sphere {
    #[must_use]
    pub const fn new(center: Point3, radius: Real) -> Self { Self(center, radius.max(0.0)) }

    #[must_use]
    pub const fn center(&self) -> Point3 { self.0 }

    #[must_use]
    pub const fn radius(&self) -> Real { self.1 }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t: Interval, hit_record: &mut HitRecord) -> bool {
        let origin_center = self.center() - ray.origin();

        let a = ray.direction().length_squared();
        let h = ray.direction().dot(origin_center);
        let c = origin_center.length_squared() - self.radius() * self.radius();

        let discriminant: Real = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t.surrounds(root) {
                return false;
            }
        }

        hit_record.set_t(root);
        hit_record.set_p(ray.at(hit_record.t()));
        let outward_normal = (hit_record.p() - self.center()) / self.radius();
        hit_record.set_face_normal(ray, outward_normal);

        true
    }
}
