use std::sync::Arc;

use crate::prelude::{Interval, Material, Point3, Ray, Vec3, interval};

/// All information about a ray–surface intersection.
pub struct HitRecord {
    /// Intersection point in world space.
    pub p: Point3,
    /// Outward-facing surface normal at `p` (unit length).
    pub normal: Vec3,
    /// Ray parameter *t* at the intersection.
    pub t: f64,
    /// The material of the intersected surface.
    pub material: Arc<dyn Material>,
    /// `true` if the ray hit the front face of the surface.
    pub is_front_face: bool,
}

impl HitRecord {
    /// Sets `normal` and `is_front_face` from the ray direction and the
    /// geometry's outward normal.  `outward_normal` must be unit length.
    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.is_front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.is_front_face { outward_normal } else { -outward_normal };
    }
}

// ---------------------------------------------------------------------------
// Hittable trait
// ---------------------------------------------------------------------------

pub trait Hittable: Send + Sync {
    /// Returns the closest hit in the interval `t`, or `None`.
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord>;
}

// ---------------------------------------------------------------------------
// Hittables — a flat list of hittable objects
// ---------------------------------------------------------------------------

/// A scene is just an ordered list of [`Hittable`] objects.
///
/// Stores each object behind an `Arc<dyn Hittable>` so that objects can be
/// cheaply shared between the scene and, e.g., motion-blur or instancing.
#[derive(Default)]
pub struct Hittables(Vec<Arc<dyn Hittable>>);

impl Hittables {
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    #[inline]
    pub fn add(&mut self, object: Arc<dyn Hittable>) { self.0.push(object); }

    #[inline]
    pub fn clear(&mut self) { self.0.clear(); }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &dyn Hittable> { self.0.iter().map(Arc::as_ref) }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        // Walk every object, shrinking the far-plane as we find closer hits.
        // This is O(n) but sufficient for Book 1; Book 2 adds a BVH.
        self.0
            .iter()
            .fold((t.max, None), |(closest, best), obj| {
                if let Some(rec) = obj.hit(ray, interval(t.min, closest)) {
                    (rec.t, Some(rec))
                } else {
                    (closest, best)
                }
            })
            .1
    }
}

// ---------------------------------------------------------------------------
// Blanket impl so Arc<dyn Hittable> is itself Hittable
// ---------------------------------------------------------------------------

impl Hittable for Arc<dyn Hittable> {
    #[inline]
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> { self.as_ref().hit(ray, t) }
}

// ---------------------------------------------------------------------------
// Conversions
// ---------------------------------------------------------------------------

impl FromIterator<Arc<dyn Hittable>> for Hittables {
    fn from_iter<I: IntoIterator<Item = Arc<dyn Hittable>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
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
