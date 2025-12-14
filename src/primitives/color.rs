use std::io::{self, Write};

use crate::prelude::Vec3;

pub type Color3 = Vec3;

pub fn write_color<W: Write>(out: &mut W, color: Color3) -> io::Result<()> {
    let (r, g, b) = (color.x(), color.y(), color.z());
    // Translate the [0.0, 1.0] component values to the byte range [0,255].

    let r = (r * 255.0) as u8;
    let g = (g * 255.0) as u8;
    let b = (b * 255.0) as u8;
    // Write out the pixel color components.
    writeln!(out, "{r} {g} {b}")
}

#[allow(clippy::cast_possible_truncation)]
pub fn write_colors_batch<W: Write>(out: &mut W, colors: &[Color3]) -> io::Result<()> {
    let mut buffer = String::with_capacity(colors.len() * 12); // ~12 chars per color

    for color in colors {
        let (r, g, b) = (color.x(), color.y(), color.z());
        let r = (r * 255.0) as u8;
        let g = (g * 255.0) as u8;
        let b = (b * 255.0) as u8;

        use std::fmt::Write as _;
        writeln!(&mut buffer, "{r} {g} {b}").unwrap();
    }

    out.write_all(buffer.as_bytes())
}
