use std::sync::Arc;

use rtc_shared::Real;

use crate::prelude::{Interval, Point3, Ray, Vec3, interval};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Real,
    pub is_front_face: bool,
}

impl HitRecord {
    /// Sets the normal relative to the ray direction.
    /// `outward_normal` must be unit length.
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.is_front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.is_front_face { outward_normal } else { -outward_normal };
    }
}

#[derive(Clone, Default)]
pub struct HittableList(Vec<Arc<dyn Hittable>>);

impl HittableList {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    pub fn objects(&self) -> impl Iterator<Item = &dyn Hittable> {
        self.0.iter().map(AsRef::as_ref)
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) { self.0.push(object); }

    pub fn clear(&mut self) { self.0.clear(); }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord> {
        let mut closest = t.max;
        let mut result = None;

        for object in self.objects() {
            if let Some(record) = object.hit(ray, interval(t.min, closest)) {
                closest = record.t;
                result = Some(record);
            }
        }

        result
    }
}
