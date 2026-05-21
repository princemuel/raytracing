use std::io;
use std::sync::Arc;

use rtc_core::material::{Lambertian, Metal};
use rtc_core::prelude::*;

fn main() -> io::Result<()> {
    // World

    let ground = Arc::new(Lambertian::new(color(0.8, 0.8, 0.0)));
    let center = Arc::new(Lambertian::new(color(0.1, 0.2, 0.5)));
    let left = Arc::new(Metal::new(color(0.8, 0.8, 0.8)));
    let right = Arc::new(Metal::new(color(0.8, 0.6, 0.2)));

    let world = Hittables::from(vec![ground as Arc<dyn Material>, center, left, right]);

    // Camera
    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    camera.render(&world)
}
