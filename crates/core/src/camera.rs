use std::io::{self, BufWriter, Write as _};

use rand::prelude::*;
use rayon::prelude::*;
use rtc_shared::random;

use crate::prelude::*;

/// All user-facing camera parameters.
///
/// Construct with `Camera { ..Default::default() }` and override the fields
/// you care about. The `render()` method will pre-compute the derived geometry
/// and then render the scene.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// Image width / height ratio.
    pub aspect_ratio: f64,
    /// Rendered image width in pixels.
    pub image_width: u32,
    /// Number of random samples per pixel (anti-aliasing).
    pub samples_per_pixel: u32,
    /// Maximum ray-bounce depth.
    pub max_depth: u32,
    /// Vertical field of view in degrees.
    pub vfov: f64,
    /// Camera position.
    pub lookfrom: Point3,
    /// Point the camera aims at.
    pub lookat: Point3,
    /// World-space "up" direction (need not be unit length).
    pub vup: Vec3,
    /// Half-angle of the defocus cone in degrees (0 = pinhole / no blur).
    pub defocus_angle: f64,
    /// Distance from `lookfrom` to the plane of perfect focus.
    pub focus_dist: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::ZERO,
            lookat: Point3::NEG_Z,
            vup: Vec3::Y,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

/// Internal rendering state derived from [`Camera`] parameters.
///
/// Separated from `Camera` so that `initialize()` is called exactly once per
/// render and its result can be safely shared across threads.
#[derive(Clone, Copy, Debug)]
struct CameraState {
    image_height: u32,
    /// 1 / `samples_per_pixel`. multiply rather than
    /// divide in the inner loop.
    pixel_samples_scale: f64,
    center: Point3,
    /// World-space location of the (0, 0) pixel centre.
    pixel00_loc: Point3,
    /// Per-pixel horizontal step vector.
    pixel_du: Vec3,
    /// Per-pixel vertical step vector.
    pixel_dv: Vec3,
    /// Defocus disk basis vectors (zero-length when `defocus_angle <= 0`).
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

// ---------------------------------------------------------------------------
// Camera implementation
// ---------------------------------------------------------------------------

impl Camera {
    /// Renders the scene and writes PPM output to **stdout**.
    ///
    /// Row-level parallelism via `rayon` — each scanline is independent.
    /// The pixel buffer is collected into a `Vec` first so I/O stays serial
    /// (writing to stdout from multiple threads would require a Mutex).
    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let state = self.initialize();

        let mut out = BufWriter::new(io::stdout().lock());
        writeln!(out, "P3\n{} {}\n255", self.image_width, state.image_height)?;

        // Parallelise over rows. Each row creates its own RNG so there are no
        // shared mutable state or lock contention issues.
        let pixels: Vec<Color3> = (0..state.image_height)
            .into_par_iter()
            .flat_map(|row| {
                let mut rng = rand::rng();
                (0..self.image_width)
                    .map(|col| {
                        // Accumulate `samples_per_pixel` jittered rays, then scale.
                        let pixel_color: Color3 = core::iter::repeat_with(|| {
                            let ray = self.get_ray(&state, &mut rng, col, row);
                            Self::ray_color(&mut rng, &ray, self.max_depth, world)
                        })
                        .take(self.samples_per_pixel.try_into().unwrap_or(0))
                        .sum();
                        state.pixel_samples_scale * pixel_color
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for pixel in pixels {
            writeln!(out, "{pixel}")?;
        }

        eprintln!("\rDone.        ");
        Ok(())
    }

    /// Computes a ray from the camera through the pixel at `(col, row)`.
    ///
    /// Adds a random sub-pixel offset for anti-aliasing, and samples the
    /// defocus disk for depth-of-field.
    fn get_ray(&self, s: &CameraState, rng: &mut dyn Rng, col: u32, row: u32) -> Ray {
        // Sub-pixel jitter in [−0.5, +0.5).
        let offset = vec3(random(rng) - 0.5, random(rng) - 0.5, 0.0);

        let pixel_sample = s.pixel00_loc
            + ((f64::from(col) + offset.x) * s.pixel_du)
            + ((f64::from(row) + offset.y) * s.pixel_dv);

        let origin = if self.defocus_angle <= 0.0 {
            s.center
        } else {
            // Sample a random point on the defocus disk instead of the exact eye point.
            let p = Vec3::random_in_unit_disk(rng);
            s.center + (p.x * s.defocus_disk_u) + (p.y * s.defocus_disk_v)
        };

        Ray::new(origin, pixel_sample - origin)
    }

    /// Recursively traces `ray` and returns the accumulated radiance.
    ///
    /// The recursion terminates either at `depth == 0` (absorb all light) or
    /// when a ray escapes to the sky gradient.
    fn ray_color(rng: &mut dyn Rng, ray: &Ray, depth: u32, world: &dyn Hittable) -> Color3 {
        if depth == 0 {
            return Color3::BLACK;
        }

        // t_min = 0.001 avoids "shadow acne": self-intersection due to the hit
        // point floating slightly inside the surface.
        if let Some(rec) = world.hit(ray, interval(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.material.scatter(rng, ray, &rec) {
                return attenuation * Self::ray_color(rng, &scattered, depth - 1, world);
            }
            return Color3::BLACK;
        }

        // Sky gradient: white at the horizon, light blue at the top.
        let a = 0.5 * (ray.direction.unit().y + 1.0);
        (1.0 - a) * Color3::WHITE + a * color(0.5, 0.7, 1.0)
    }

    /// Pre-computes all camera geometry from the user-facing parameters.
    ///
    /// Called once at the start of [`Self::render`]. The separation keeps
    /// `Camera` fields clean and avoids re-deriving geometry for every
    /// pixel.
    #[expect(clippy::similar_names)]
    fn initialize(&self) -> CameraState {
        let iw = f64::from(self.image_width);
        // At least 1 pixel tall.
        let ih = (iw / self.aspect_ratio).max(1.0).round();

        #[expect(
            clippy::cast_possible_truncation,
            clippy::as_conversions,
            clippy::cast_sign_loss
        )]
        let image_height = ih as u32;

        let pixel_samples_scale = 1.0 / f64::from(self.samples_per_pixel);
        let center = self.lookfrom;

        // Camera basis (right-handed, -z into screen).
        let w = (self.lookfrom - self.lookat).unit(); // points *away* from scene
        let u = self.vup.cross(w).unit(); // points right
        let v = w.cross(u); // points up

        // Viewport in world space.
        let h = (self.vfov.to_radians() / 2.0).tan();
        let vh = 2.0 * h * self.focus_dist;
        let vw = vh * (iw / ih);

        let viewport_u = vw * u; // horizontal edge vector
        let viewport_v = vh * -v; // vertical edge vector (down)

        let pixel_du = viewport_u / iw;
        let pixel_dv = viewport_v / ih;

        let viewport_upper_left =
            center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_du + pixel_dv);

        // Defocus disk.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        CameraState {
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_du,
            pixel_dv,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
