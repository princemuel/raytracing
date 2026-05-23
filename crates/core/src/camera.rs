use std::io::{self, BufWriter, Write as _};

use rand::prelude::Rng;
use rayon::prelude::{IntoParallelIterator as _, ParallelIterator as _};
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
    /// The vertical view angle (field of view) in degrees
    pub vfov: Real,
    /// The point the camera is looking from
    pub lookfrom: Point3,
    /// The point the camera is looking at
    pub lookat: Point3,
    /// The camera-relative "up" direction
    pub vup: Vec3,
    /// The variation angle of rays through each pixel
    pub defocus_angle: Real,
    /// The distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: Real,
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

#[derive(Clone, Copy, Debug)]
pub struct CameraConfig {
    /// The rendered image height
    image_height: i32,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: Real,
    /// The camera center
    center: Point3,
    // The location of pixel 0, 0
    pixel00_loc: Point3,
    /// The offset to pixel to the right
    pixel_du: Vec3,
    /// The offset to pixel below
    pixel_dv: Vec3,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let cfg = self.initialize();

        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(&mut out, "P3\n{} {}\n255", self.image_width, cfg.image_height)?;

        // Each row is independent — parallelise over scanlines
        let pixels: Vec<_> = (0..cfg.image_height)
            .into_par_iter()
            .flat_map(|j| {
                let mut rng = rand::rng();

                (0..self.image_width)
                    .map(move |i| {
                        let mut pixel_color = Color3::BLACK;
                        for _sample in 0..self.samples_per_pixel {
                            let ray = self.get_ray(&cfg, &mut rng, i.into(), j.into());
                            pixel_color += Self::ray_color(&ray, self.max_depth, world);
                        }
                        cfg.pixel_samples_scale * pixel_color
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for pixel in pixels {
            writeln!(&mut out, "{pixel}")?;
        }

        eprintln!("\rDone         ");
        Ok(())
    }

    /// Returns a ray from the camera center to the pixel at (u, v) with a
    /// random offset within the pixel for anti-aliasing.
    #[must_use]
    pub fn get_ray(&self, cfg: &CameraConfig, mut rng: &mut impl Rng, u: Real, v: Real) -> Ray {
        let offset = self.sample_square(&mut rng);

        let pixel_sample = {
            cfg.pixel00_loc + ((u + offset.x) * cfg.pixel_du) + ((v + offset.y) * cfg.pixel_dv)
        };

        let origin = if self.defocus_angle <= 0.0 {
            cfg.center
        } else {
            self.defocus_disk_sample(&mut rng)
        };
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit
    /// square.
    ///
    /// This is used for anti-aliasing by jittering the ray direction within a
    /// pixel.
    pub fn sample_square(&self, rng: &mut impl Rng) -> Vec3 {
        vec3(random(rng) - 0.5, random(rng) - 0.5, 0)
    }

    /// Returns a random point in the unit (radius 0.5) disk centered at the
    /// origin.
    pub fn sample_disk(&self, mut rng: &mut impl Rng, radius: Real) -> Vec3 {
        radius * Vec3::random_in_unit_disk(&mut rng)
    }

    /// Returns the vector to a random point in the camera's defocus disk
    pub fn defocus_disk_sample(&self, mut rng: &mut impl Rng) -> Vec3 {
        let cfg = self.initialize();
        let p = Vec3::random_in_unit_disk(&mut rng);
        cfg.center + (p.x * cfg.defocus_disk_u) + (p.y * cfg.defocus_disk_v)
    }

    #[expect(clippy::similar_names)]
    fn initialize(&self) -> CameraConfig {
        let Self {
            image_width,
            focus_dist,
            samples_per_pixel,
            defocus_angle,
            aspect_ratio,
            lookfrom,
            lookat,
            vup,
            vfov,
            ..
        } = *self;

        let image_width = Real::from(image_width);
        let samples_per_pixel = Real::from(samples_per_pixel);
        let image_height = (image_width / aspect_ratio).max(1.0).round();

        let pixel_samples_scale = 1.0 / samples_per_pixel;

        let center = lookfrom;

        // Determine viewport dimensions.
        // let focal_len = (self.lookfrom - self.lookat).length();
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let vh = 2.0 * h * focus_dist;
        let vw = vh * (image_width / image_height);

        // Calculate the u,v,w unit basis vectors for the camera's coordinate frame.
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        // vector across viewport horizontal edge
        let viewport_u = vw * u;
        // vector down viewport vertical edge
        let viewport_v = vh * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_du = viewport_u / image_width;
        let pixel_dv = viewport_v / image_height;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - (focus_dist * w) - (viewport_u / 2.0) - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_du + pixel_dv);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn ray_color(ray: &Ray, depth: i32, world: &dyn Hittable) -> Color3 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color3::BLACK;
        }

        if let Some(rec) = world.hit(ray, interval(0.001, Real::INFINITY)) {
            let mut scattered = Ray::default();
            let mut attenuation = Color3::default();

            if rec.material.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }

            return Color3::BLACK;
        }

        let direction = ray.direction.unit();
        let a = 0.5 * (direction.y + 1.0);
        (1.0 - a) * Color3::WHITE + a * color(0.5, 0.7, 1.0)
    }
}
