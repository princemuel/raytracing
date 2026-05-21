use std::io::{self, BufWriter, Write as _};

use rand::prelude::Rng;
use rtc_shared::{Real, random};

use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// The ratio of image width over height
    pub aspect_ratio: Real,
    /// The rendered image width in pixel count
    pub image_width: i32,
    /// The count of random samples for each pixel
    pub samples_per_pixel: i32,
    /// The maximum number of ray bounces into a scene
    pub max_depth: i32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { aspect_ratio: 1.0, image_width: 100, samples_per_pixel: 10, max_depth: 10 }
    }
}

struct CameraConfig {
    /// The rendered image height
    image_height: i32,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: Real,
    /// The camera center
    center: Point3,
    // The location of pixel 0, 0
    pixel00_loc: Point3,
    ///    Offset to pixel to the right
    pixel_du: Vec3,
    /// Offset to pixel below
    pixel_dv: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let cfg = self.initialize();

        let stdout = io::stdout();
        let mut rng = rand::rng();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(&mut out, "P3\n{} {}\n255", self.image_width, cfg.image_height)?;

        for j in 0..cfg.image_height {
            eprint!("\rScanlines remaining: {} ", cfg.image_height - j);
            io::stderr().flush()?;

            for i in 0..self.image_width {
                let mut pixel_color = Color3::BLACK;

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(&mut rng, i.into(), j.into());
                    pixel_color += Self::ray_color(ray, self.max_depth, world);
                }

                writeln!(&mut out, "{}", cfg.pixel_samples_scale * pixel_color)?;
            }
        }

        eprintln!("\rDone.");
        Ok(())
    }

    /// Returns a ray from the camera center to the pixel at (u, v) with a
    /// random offset within the pixel for anti-aliasing.
    #[must_use]
    pub fn get_ray(&self, rng: &mut impl Rng, u: Real, v: Real) -> Ray {
        let cfg = self.initialize();
        let offset = self.sample_square(rng);

        let pixel_sample =
            cfg.pixel00_loc + ((u + offset.x) * cfg.pixel_du) + ((v + offset.y) * cfg.pixel_dv);

        Ray::new(cfg.center, pixel_sample - cfg.center)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit
    /// square.
    ///
    /// This is used for anti-aliasing by jittering the ray direction within a
    /// pixel.
    pub fn sample_square(&self, rng: &mut impl Rng) -> Vec3 {
        vec3(random(rng) - 0.5, random(rng) - 0.5, 0)
    }

    #[expect(clippy::similar_names)]
    fn initialize(&self) -> CameraConfig {
        let image_width = Real::from(self.image_width);
        let samples_per_pixel = Real::from(self.samples_per_pixel);
        let image_height = (image_width / self.aspect_ratio).max(1.0).round();

        let pixel_samples_scale = 1.0 / samples_per_pixel;

        let center = Point3::ZERO;
        // Determine viewport dimensions.
        let focal_length = 1.0;
        let vh = 2.0;
        let vw = vh * (image_width / image_height);

        // vector across viewport horizontal edge
        let viewport_u = vec3(vw, 0, 0);
        // vector down viewport vertical edge
        let viewport_v = vec3(0, -vh, 0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_du = viewport_u / image_width;
        let pixel_dv = viewport_v / image_height;

        // Calcuralte the location of the upper left pixel
        // vec3(0, 0, focal_length) == Vec3::Z when focal_length = 1.0
        let viewport_top_left =
            center - vec3(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_top_left + 0.5 * (pixel_du + pixel_dv);

        #[expect(clippy::cast_possible_truncation, clippy::as_conversions)]
        // max(1.0) and round() guarantee this is a positive integer. cast is safe
        let image_height = image_height as i32;

        CameraConfig {
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_du,
            pixel_dv,
        }
    }

    fn ray_color(ray: Ray, depth: i32, world: &dyn Hittable) -> Color3 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth < 1 {
            return Color3::BLACK;
        }

        if let Some(record) = world.hit(ray, interval(0, Real::INFINITY)) {
            let direction = Vec3::random_on_hemi_vec(record.normal);
            return 0.5 * Self::ray_color(Ray::new(record.p, direction), depth - 1, world);
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color3::WHITE + a * color(0.5, 0.7, 1.0)
    }
}
