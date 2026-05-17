use std::io::{self, BufWriter, Write};

use rtc_shared::{INFINITY, Real};

use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// Ratio of image width over height
    pub aspect_ratio: Real,
    /// Rendered image width in pixels
    pub image_width: i32,
}

impl Default for Camera {
    fn default() -> Self { Self { aspect_ratio: 1.0, image_width: 100 } }
}

struct CameraConfig {
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let cfg = self.build_config();

        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(out, "P3\n{} {}\n255", self.image_width, cfg.image_height)?;

        for j in 0..cfg.image_height {
            eprint!("\rScanlines remaining: {} ", cfg.image_height - j);
            io::stderr().flush()?;

            for i in 0..self.image_width {
                let pixel_center =
                    cfg.pixel00_loc + (i as Real * cfg.pixel_du) + (j as Real * cfg.pixel_dv);
                let ray = Ray::new(cfg.center, pixel_center - cfg.center);
                let color = Self::ray_color(ray, world);
                writeln!(out, "{color}")?;
            }
        }

        eprintln!("\rDone.");
        Ok(())
    }

    fn build_config(&self) -> CameraConfig {
        let image_width = self.image_width as Real;
        let image_height = (image_width / self.aspect_ratio).max(1.0) as i32;
        let image_height_f = image_height as Real;

        let center = Point3::ZERO;
        let focal_length: Real = 1.0;
        let vh: Real = 2.0;
        let vw = vh * (image_width / image_height_f);

        let viewport_u = vec3(vw, 0, 0);
        let viewport_v = vec3(0, -vh, 0);

        let pixel_du = viewport_u / image_width;
        let pixel_dv = viewport_v / image_height_f;

        let viewport_top_left = center - vec3(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_top_left + 0.5 * (pixel_du + pixel_dv);

        CameraConfig { image_height, center, pixel00_loc, pixel_du, pixel_dv }
    }

    fn ray_color(ray: Ray, world: &dyn Hittable) -> Color3 {
        if let Some(record) = world.hit(ray, interval(0, INFINITY)) {
            return 0.5 * (Color3::WHITE + record.normal);
        }

        let unit_direction = ray.direction().unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color3::WHITE + a * color(0.5, 0.7, 1.0)
    }
}
