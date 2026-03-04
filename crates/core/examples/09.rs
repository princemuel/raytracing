use std::io;
use std::sync::Arc;

use rtc_core::prelude::*;

fn main() -> io::Result<()> {
    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::NEG_Z, 0.5)));
    world.add(Arc::new(Sphere::new(point3(0, -100.5, -1), 100.0)));

    // Camera
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.render(&world)?;

    Ok(())
}
