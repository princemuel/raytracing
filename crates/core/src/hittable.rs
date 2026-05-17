use std::sync::Arc;

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

#[derive(Clone, Default)]
pub struct HittableList(Vec<Arc<dyn Hittable>>);

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) { self.0.push(object); }

    pub fn clear(&mut self) { self.0.clear(); }

    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn iter(&self) -> impl Iterator<Item = &dyn Hittable> {
        self.0.iter().map(AsRef::as_ref)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord> {
        let mut closest = t.max;
        let mut result = None;

        for object in &self.0 {
            if let Some(rec) = object.hit(ray, Interval { min: t.min, max: closest }) {
                closest = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}

impl FromIterator<Arc<dyn Hittable>> for HittableList {
    fn from_iter<I: IntoIterator<Item = Arc<dyn Hittable>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Hittable for Arc<dyn Hittable> {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord> { self.as_ref().hit(ray, t) }
}

#[expect(clippy::as_conversions, trivial_casts)]
impl<T: Hittable + 'static> From<Vec<T>> for HittableList {
    fn from(v: Vec<T>) -> Self {
        Self(v.into_iter().map(|h| Arc::new(h) as Arc<dyn Hittable>).collect())
    }
}

impl IntoIterator for HittableList {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Arc<dyn Hittable>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a HittableList {
    type Item = &'a dyn Hittable;

    type IntoIter = impl Iterator<Item = Self::Item> + 'a;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
