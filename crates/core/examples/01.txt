#![expect(clippy::as_conversions)]
#![expect(clippy::cast_possible_truncation)]
use std::io;
use std::io::Write as _;

use rtc_shared::Real;

fn main() -> io::Result<()> {
    // Image dimensions
    let image_width = 256;
    let image_height = 256;

    // Render PPM header
    println!("P3\n{image_width} {image_height}\n255");

    // Generate pixel data
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush()?;

        for i in 0..image_width {
            let r = Real::from(i) / Real::from(image_width - 1);
            let g = Real::from(j) / Real::from(image_height - 1);
            let b = 0.0;

            // Translate the [0,1] component values to the byte range [0,255]
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            // Write out the pixel color components
            println!("{ir} {ig} {ib}");
        }
    }

    eprint!("\rDone.                 \n");
    Ok(())
}
