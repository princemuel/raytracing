use core::ops::Range;
use std::io;
use std::sync::Arc;

use rtc_core::prelude::*;
use rtc_shared::{Real, random, random_w_range};

const RANGE: Range<i8> = -11..11;

fn main() -> io::Result<()> {
    let mut world = Hittables::new();

    let ground_material = Arc::new(Lambertian::new(color(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(point3(0, -1000, 0), 1000.0, ground_material)));

    // Build all random spheres in parallel, collecting into a Vec to avoid mutex
    // contention
    for sphere in RANGE.flat_map(|a| {
        let a = Real::from(a);
        RANGE.filter_map(move |b| {
            let b = Real::from(b);
            let mut rng = rand::rng();

            let choose_mat = random(&mut rng);
            let center = point3(a + 0.9 * random(&mut rng), 0.2, b + 0.9 * random(&mut rng));

            ((center - point3(4, 0.2, 0)).length() > 0.9).then(|| {
                let sphere: Arc<dyn Hittable> = if choose_mat < 0.8 {
                    let albedo = Color3::random(&mut rng) * Color3::random(&mut rng);
                    Arc::new(Sphere::new(center, 0.2, Arc::new(Lambertian::new(albedo))))
                } else if choose_mat < 0.95 {
                    let albedo = Color3::random_w_range(&mut rng, 0.5, 1.0);
                    let fuzz = random_w_range(&mut rng, 0.0, 0.5);
                    Arc::new(Sphere::new(center, 0.2, Arc::new(Metal::new(albedo, fuzz))))
                } else {
                    Arc::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))))
                };

                sphere
            })
        })
    }) {
        world.add(sphere);
    }

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

    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 50,
        max_depth: 10,
        vfov: 20.0,
        lookfrom: point3(13, 2, 3),
        lookat: Point3::ZERO,
        vup: Vec3::Y,
        defocus_angle: 0.6,
        focus_dist: 10.0,
    };

    camera.render(&world)
}
