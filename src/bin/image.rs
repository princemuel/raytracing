use std::io::{self, Write as _};

//
use rtc::prelude::*;

fn main() {
    // // Image dimensions
    // let image_width = 256;
    // let image_height = 256;

    // // Render PPM header
    // println!("P3\n{} {}\n255", image_width, image_height);

    // // Generate pixel data
    // for j in 0..image_height {
    //     eprint!("\rScanlines remaining: {} ", image_height - j);
    //     io::stderr().flush().unwrap();

    //     for i in 0..image_width {
    //         let r = i as Real / (image_width - 1) as Real;
    //         let g = j as Real / (image_height - 1) as Real;
    //         let b = 0.0;

    //         // let ir = (255.999 * r) as i32;
    //         // let ig = (255.999 * g) as i32;
    //         // let ib = (255.999 * b) as i32;

    //         // println!("{} {} {}", ir, ig, ib);
    //         let color = Color::new(r, g, b);
    //         write_color(&mut std::io::stdout(), color).unwrap();
    //     }
    // }

    // eprint!("\rDone.                 \n");
}
