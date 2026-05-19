use rtc_shared::Real;

use crate::prelude::{Interval, Point3, Ray, Vec3};

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
