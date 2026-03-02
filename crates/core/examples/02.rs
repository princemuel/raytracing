use std::io;
use std::io::Write;

use rtc_core::prelude::*;
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
            let r = i as Real / (image_width - 1) as Real;
            let g = j as Real / (image_height - 1) as Real;
            let b = 0.0;

            let color = color(r, g, b);

            writeln!(&mut io::stdout(), "{color}")?;
        }
    }

    eprint!("\rDone.                 \n");
    Ok(())
}
