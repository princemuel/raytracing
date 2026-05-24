use std::sync::Arc;

use crate::prelude::{AABB, HitRecord, Hittable, Interval, Material, Point3, Ray, Vec3};

/// A sphere — the only primitive in Book 1.
pub struct Sphere {
    pub center: Ray,
    /// Always ≥ 0. negative radii are clamped on construction.
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    #[inline]
    #[must_use]
    pub const fn new(
        center_a: Point3,
        center_b: Option<Point3>,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let center = if let Some(center_b) = center_b {
            Ray::new(center_a, center_b - center_a, None)
        } else {
            Ray::new(center_a, Vec3::ZERO, None)
        };

        Self { center, radius: radius.max(0.0), material }
    }
}

impl Hittable for Sphere {
    fn bounding_box(&self) -> AABB {
        let rvec = Vec3::splat(self.radius);
        // Enclose both the start and end positions (handles moving spheres;
        // for stationary ones center_b == center_a so box0 == box1).
        let box0 = AABB::from((self.center.origin - rvec, self.center.origin + rvec));
        let box1 = AABB::from((self.center.at(1.0) - rvec, self.center.at(1.0) + rvec));
        AABB::from((box0, box1))
    }

    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let center = self.center.at(ray.time);
        // Vector from ray origin to sphere centre: **oc** = C − O
        let oc = center - ray.origin;

        // Quadratic coefficients (simplified half-b form from §6.2):
        //   a  = |d|²
        //   h  = d · oc       (h = −b/2 in the full form)
        //   c  = |oc|² − r²
        // discriminant = h² − a·c
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Try the nearer root first, then the farther one.
        let t_hit = {
            let r1 = (h - sqrtd) / a;
            let r2 = (h + sqrtd) / a;
            if t.surrounds(r1) {
                r1
            } else if t.surrounds(r2) {
                r2
            } else {
                return None;
            }
        };

        let p = ray.at(t_hit);
        let outward_normal = (p - center) / self.radius;

        let mut record = HitRecord {
            p,
            t: t_hit,
            normal: outward_normal, // overwritten below
            is_front_face: false,   // overwritten below
            material: Arc::clone(&self.material),
        };
        record.set_face_normal(ray, outward_normal);

        Some(record)
    }
}
