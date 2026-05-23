//! final scene: hundreds of random spheres.
use core::ops::Range;
use std::io;
use std::sync::Arc;

use rtc_core::prelude::*;
use rtc_shared::{random, random_range};
const GRID: Range<i8> = -11..11;

fn main() -> io::Result<()> {
    let mut world = Hittables::new();

    // Ground
    world.add(Arc::new(Sphere::new(
        point3(0, -1000, 0),
        1000.0,
        Arc::new(Lambertian::new(color(0.5, 0.5, 0.5))),
    )));

    // Random small spheres on a 22×22 grid
    for sphere in GRID.flat_map(|a| {
        let a = f64::from(a);
        GRID.filter_map(move |b| {
            let b = f64::from(b);
            let mut rng = rand::rng();

            let choose_mat = random(&mut rng);
            let center = point3(a + 0.9 * random(&mut rng), 0.2, b + 0.9 * random(&mut rng));

            // Skip spheres that would overlap the three large centrepiece spheres.
            ((center - point3(4, 0.2, 0)).length() > 0.9).then(|| {
                let mat: Arc<dyn Hittable> = if choose_mat < 0.8 {
                    // Diffuse — colour is the product of two random colours
                    let albedo = Color3::random(&mut rng) * Color3::random(&mut rng);
                    Arc::new(Sphere::new(center, 0.2, Arc::new(Lambertian::new(albedo))))
                } else if choose_mat < 0.95 {
                    // Metal — slightly tinted and mildly fuzzy
                    let albedo = Color3::random_range(&mut rng, 0.5, 1.0);
                    let fuzz = random_range(&mut rng, 0.0, 0.5);
                    Arc::new(Sphere::new(center, 0.2, Arc::new(Metal::new(albedo, fuzz))))
                } else {
                    // Glass
                    Arc::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))))
                };
                mat
            })
        })
    }) {
        world.add(sphere);
    }

    // Three large centrepiece spheres
    world.add(Arc::new(Sphere::new(Point3::Y, 1.0, Arc::new(Dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(
        point3(-4, 1, 0),
        1.0,
        Arc::new(Lambertian::new(color(0.4, 0.2, 0.1))),
    )));
    world.add(Arc::new(Sphere::new(
        point3(4, 1, 0),
        1.0,
        Arc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0)),
    )));

    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200, // book's final-scene resolution
        samples_per_pixel: 500,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: point3(13, 2, 3),
        lookat: Point3::ZERO,
        vup: Vec3::Y,
        defocus_angle: 0.6,
        focus_dist: 10.0,
    }
    .render(&world)
}
