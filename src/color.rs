use std::io::{self, Write};

use crate::vector::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> io::Result<()> {
    let (r, g, b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());
    // Translate the [0.0, 1.0] component values to the byte range [0,255].
    let r_byte = (r * 255.0) as u8;
    let g_byte = (g * 255.0) as u8;
    let b_byte = (b * 255.0) as u8;
    // Write out the pixel color components.
    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
}

pub fn write_colors_batch<W: Write>(out: &mut W, colors: &[Color]) -> io::Result<()> {
    let mut buffer = String::with_capacity(colors.len() * 12); // ~12 chars per color

    for color in colors {
        let (r, g, b) = (color.x(), color.y(), color.z());
        let r_byte = (r * 255.0) as u8;
        let g_byte = (g * 255.0) as u8;
        let b_byte = (b * 255.0) as u8;

        use std::fmt::Write as FmtWrite;
        writeln!(&mut buffer, "{} {} {}", r_byte, g_byte, b_byte).unwrap();
    }

    out.write_all(buffer.as_bytes())
}
