use std::sync::Arc;

use crate::prelude::{AABB, Interval, Material, Point3, Ray, Vec3, interval};

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
    /// Always returns a valid AABB. Use `AABB::EMPTY` for unbounded objects.
    fn bounding_box(&self) -> AABB { AABB::EMPTY }

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
pub struct Hittables {
    objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl Hittables {
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    #[inline]
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::from((self.bbox, object.bounding_box()));
        self.objects.push(object);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = AABB::EMPTY;
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.objects.len() }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.objects.is_empty() }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &dyn Hittable> {
        self.objects.iter().map(Arc::as_ref)
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let (_, record) = self.objects.iter().fold((t.max, None), |(closest, best), obj| match obj
            .hit(ray, interval(t.min, closest))
        {
            Some(rec) => (rec.t, Some(rec)),
            None => (closest, best),
        });

        record
    }

    fn bounding_box(&self) -> AABB { self.bbox } // AABB is Copy — free
}

// ---------------------------------------------------------------------------
// Blanket impl so Arc<dyn Hittable> is itself Hittable
// ---------------------------------------------------------------------------
impl Hittable for Arc<dyn Hittable> {
    #[inline]
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> { self.as_ref().hit(ray, t) }

    #[inline]
    fn bounding_box(&self) -> AABB { self.as_ref().bounding_box() }
}

// ---------------------------------------------------------------------------
// Conversions
// ---------------------------------------------------------------------------

impl FromIterator<Arc<dyn Hittable>> for Hittables {
    fn from_iter<I: IntoIterator<Item = Arc<dyn Hittable>>>(iter: I) -> Self {
        Self { objects: iter.into_iter().collect(), bbox: AABB::EMPTY }
    }
}

#[expect(clippy::as_conversions, trivial_casts)]
impl<T: Hittable + 'static> From<Vec<T>> for Hittables {
    fn from(v: Vec<T>) -> Self {
        Self {
            objects: v.into_iter().map(|h| Arc::new(h) as Arc<dyn Hittable>).collect(),
            bbox: AABB::EMPTY,
        }
    }
}

impl IntoIterator for Hittables {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Arc<dyn Hittable>;

    fn into_iter(self) -> Self::IntoIter { self.objects.into_iter() }
}

impl<'a> IntoIterator for &'a Hittables {
    type Item = &'a dyn Hittable;

    type IntoIter = impl Iterator<Item = Self::Item> + 'a;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
