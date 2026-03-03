use std::sync::Arc;

use rtc_shared::Real;

use crate::prelude::{Point3, Vec3};
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: Ray, tmin: Real, tmax: Real, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HittableList(Vec<Arc<dyn Hittable>>);

impl HittableList {
    pub const fn new() -> Self { Self(Vec::new()) }

    pub fn objects(&self) -> Vec<Arc<dyn Hittable>> { self.0.clone() }

    pub fn add(&mut self, value: Arc<dyn Hittable>) { self.0.push(value); }

    pub fn clear(&mut self) { self.0.clear(); }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, tmin: Real, tmax: Real, hit_record: &mut HitRecord) -> bool {
        let mut record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = tmax;

        for object in self.objects() {
            if object.hit(ray, tmin, closest_so_far, &mut record) {
                hit_anything = true;
                closest_so_far = record.t();
                *hit_record = record.clone()
            }
        }

        hit_anything
    }
}

#[derive(Clone, Debug)]
pub struct HitRecord(Point3, Vec3, Real, bool);

impl HitRecord {
    pub const fn new() -> Self { Self(Point3::ZERO, Vec3::ZERO, 0.0, false) }

    pub const fn p(&self) -> Point3 { self.0 }

    pub const fn normal(&self) -> Vec3 { self.1 }

    pub const fn t(&self) -> Real { self.2 }

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
