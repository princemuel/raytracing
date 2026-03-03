use std::io;
use std::io::Write;

use rtc_shared::{INFINITY, Real};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Camera {
    /// Ratio of the image width over height
    pub aspect_ratio: Real,
    /// Rendered image width in pixel count
    pub image_width:  i32,
    /// Rendered image height
    image_height:     i32,
    /// Camera center
    center:           Point3,
    /// Location of pixel 0, 0
    pixel00_loc:      Point3,
    /// Offset to the pixel to the right
    pixel_du:         Vec3,
    /// Offset to the pixel below
    pixel_dv:         Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            ..Default::default()
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            io::stderr().flush()?;

            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel00_loc + (i as Real * self.pixel_du) + (j as Real * self.pixel_dv);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = self.ray_color(ray, world);
                writeln!(&mut io::stdout(), "{color}")?;
            }
        }

        eprint!("\rDone.                 \n");
        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as Real / self.aspect_ratio).max(1.0) as i32;
        self.center = Point3::ZERO;

        let image_width = self.image_width as Real;
        let image_height = self.image_height as Real;

        // Determine viewport dimensions
        let focal_length = 1.0;
        let vh = 2.0;
        let vw = vh * (image_width / image_height);

        // Calculate the vectors across the horizontal and down the vertical viewport
        // edges
        let viewport_u = vec3(vw, 0, 0);
        let viewport_v = vec3(0, -vh, 0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_du = viewport_u / image_width;
        self.pixel_dv = viewport_v / image_height;

        // Calculate the location of the upper left pixel
        let viewport_top_left =
            self.center - vec3(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_top_left + 0.5 * (self.pixel_du + self.pixel_dv);
    }

    fn ray_color(&self, ray: Ray, world: &dyn Hittable) -> Color3 {
        let mut hit_record = HitRecord::new();

        if world.hit(ray, interval(0, INFINITY), &mut hit_record) {
            return 0.5 * (Color3::WHITE + hit_record.normal());
        }

        let unit_direction = ray.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color3::WHITE + a * color(0.5, 0.7, 1.0)
    }
}
