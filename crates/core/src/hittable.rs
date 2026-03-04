use std::sync::Arc;

use rtc_shared::Real;

use crate::prelude::{Interval, Point3, Ray, Vec3, interval};

pub trait Hittable {
    fn hit(&self, ray: Ray, t: Interval, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HittableList(Vec<Arc<dyn Hittable>>);

impl HittableList {
    #[must_use]
    pub const fn new() -> Self { Self(Vec::new()) }

    #[must_use]
    pub fn objects(&self) -> Vec<Arc<dyn Hittable>> { self.0.clone() }

    pub fn add(&mut self, value: Arc<dyn Hittable>) { self.0.push(value); }

    pub fn clear(&mut self) { self.0.clear(); }
}

impl Default for HittableList {
    fn default() -> Self { Self::new() }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t.max();

        for object in self.objects() {
            if object.hit(ray, interval(t.min(), closest_so_far), &mut record) {
                hit_anything = true;
                closest_so_far = record.t();
                *hit_record = record.clone();
            }
        }

        hit_anything
    }
}

#[derive(Clone, Debug)]
pub struct HitRecord(Point3, Vec3, Real, bool);

impl HitRecord {
    #[must_use]
    pub const fn new() -> Self { Self(Point3::ZERO, Vec3::ZERO, 0.0, false) }

    #[must_use]
    pub const fn p(&self) -> Point3 { self.0 }

    #[must_use]
    pub const fn normal(&self) -> Vec3 { self.1 }

    #[must_use]
    pub const fn t(&self) -> Real { self.2 }

    #[must_use]
    pub const fn is_front_face(&self) -> bool { self.3 }

    pub const fn set_p(&mut self, value: Point3) { self.0 = value }

    pub const fn set_normal(&mut self, value: Vec3) { self.1 = value }

    pub const fn set_t(&mut self, value: Real) { self.2 = value }

    pub const fn set_is_front_face(&mut self, value: bool) { self.3 = value }

    /// Sets the hit record normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have a unit length
    pub const fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.set_is_front_face(ray.direction().dot(outward_normal) < 0.0);
        self.set_normal(if self.is_front_face() { outward_normal } else { -outward_normal });
    }
}

impl Default for HitRecord {
    fn default() -> Self { Self::new() }
}
