use std::io;
use std::sync::Arc;

use rtc_core::material::{Lambertian, Metal};
use rtc_core::prelude::*;

fn main() -> io::Result<()> {
    let world = Hittables::from(vec![
        Arc::new(Lambertian::new(color(0.8, 0.8, 0.0))) as Arc<dyn Material>, // ground
        Arc::new(Lambertian::new(color(0.1, 0.2, 0.5))),                      // center
        Arc::new(Metal::new(color(0.8, 0.8, 0.8), 0.3)),                      // left
        Arc::new(Metal::new(color(0.8, 0.6, 0.2), 1.0)),                      // right
    ]);

    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    camera.render(&world)
}
