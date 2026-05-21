use std::sync::Arc;

use rtc_shared::Real;

use crate::prelude::{Interval, Material, Point3, Ray, Vec3, interval};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Real,
    pub material: Arc<dyn Material>,
    pub is_front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self { Self::new() }
}

impl HitRecord {
    #[must_use]
    pub fn new() -> Self {
        Self {
            p: Point3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            is_front_face: false,
            material: Arc::new(data),
        }
    }

    /// Sets the normal relative to the ray direction.
    ///
    /// The parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        (self.is_front_face, self.normal) = face_normal(r, outward_normal);
    }

    /// Sets the normal relative to the ray direction.
    ///
    /// The parameter `outward_normal` is assumed to have unit length.
    #[must_use]
    pub fn with_face_normal(mut self, ray: &Ray, outward_normal: Vec3) -> Self {
        (self.is_front_face, self.normal) = face_normal(ray, outward_normal);
        self
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord>;
}

#[derive(Clone, Default)]
pub struct Hittables(Vec<Arc<dyn Hittable>>);

impl Hittables {
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

impl Hittable for Hittables {
    fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord> {
        let mut closest = t.max;
        let mut hit_record = None;

        for object in &self.0 {
            if let Some(record) = object.hit(r, interval(t.min, closest)) {
                closest = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}

impl FromIterator<Arc<dyn Hittable>> for Hittables {
    fn from_iter<I: IntoIterator<Item = Arc<dyn Hittable>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Hittable for Arc<dyn Hittable> {
    fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord> { self.as_ref().hit(r, t) }
}

#[expect(clippy::as_conversions, trivial_casts)]
impl<T: Hittable + 'static> From<Vec<T>> for Hittables {
    fn from(v: Vec<T>) -> Self {
        Self(v.into_iter().map(|h| Arc::new(h) as Arc<dyn Hittable>).collect())
    }
}

impl IntoIterator for Hittables {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Arc<dyn Hittable>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a Hittables {
    type Item = &'a dyn Hittable;

    type IntoIter = impl Iterator<Item = Self::Item> + 'a;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

/// Returns `(is_front_face, normal)` given a ray and outward normal.
///
/// The parameter `outward_normal` is assumed to have unit length.
#[must_use]
pub fn face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let is_front_face = ray.direction.dot(outward_normal) < 0.0;
    let normal = if is_front_face { outward_normal } else { -outward_normal };
    (is_front_face, normal)
}
