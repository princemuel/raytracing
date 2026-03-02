use std::io;
use std::io::Write;

use rtc_core::prelude::*;
use rtc_shared::Real;

fn main() -> io::Result<()> {
    // Image dimensions

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1
    let image_height = (image_width as Real / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let vh = 2.0;
    let vw = vh * (image_width as Real / image_height as Real);
    let camera_center = Point3::ZERO;

    // Calculate the vectors across the horizontal and down the vertical viewport
    // edges
    let viewport_u = vec3(vw, 0, 0);
    let viewport_v = vec3(0, -vh, 0);

    // Calcualte the horizontal and the vertical delta vectors from pixel to pixel
    let pixel_du = viewport_u / image_width as Real;
    let pixel_dv = viewport_v / image_height as Real;

    // Calculate the location of the upper left pixel
    let viewport_top_left = camera_center - vec3(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_top_left + 0.5 * (pixel_du + pixel_dv);

    // Render PPM header
    println!("P3\n{image_width} {image_height}\n255");

    // Generate pixel data
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush()?;

        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as Real * pixel_du) + (j as Real * pixel_dv);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = {
                let unit_direction = ray.direction().unit();
                let a = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - a) * Color3::WHITE + a * color(0.5, 0.7, 1.0)
            };

            writeln!(&mut io::stdout(), "{color}")?;
        }
    }

    eprint!("\rDone.                 \n");
    Ok(())
}
