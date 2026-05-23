//! Example 22 — dielectric / metal / Lambertian scene with depth-of-field.
use std::io;
use std::sync::Arc;

use engine::prelude::*;

fn main() -> io::Result<()> {
    let ground = Arc::new(Lambertian::new(color(0.8, 0.8, 0.0)));
    let center = Arc::new(Lambertian::new(color(0.1, 0.2, 0.5)));
    let left = Arc::new(Dielectric::new(1.5));
    let bubble = Arc::new(Dielectric::new(1.0 / 1.5)); // hollow glass
    let right = Arc::new(Metal::new(color(0.8, 0.6, 0.2), 1.0));

    let world = Hittables::from(vec![
        Sphere::new(point3(0.0, -100.5, -1.0), 100.0, ground),
        Sphere::new(point3(0.0, 0.0, -1.2), 0.5, center),
        Sphere::new(point3(-1.0, 0.0, -1.0), 0.5, left),
        Sphere::new(point3(-1.0, 0.0, -1.0), 0.4, bubble),
        Sphere::new(point3(1.0, 0.0, -1.0), 0.5, right),
    ]);

    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 90.0,
        lookfrom: point3(-2, 2, 1),
        lookat: Point3::NEG_Z,
        vup: Vec3::Y,
        defocus_angle: 10.0,
        focus_dist: 3.4,
    }
    .render(&world)
}
