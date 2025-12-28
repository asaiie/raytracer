use crate::{interval::Interval, vec3::Vec3};
use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    const INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };
    let rbyte = (256.0 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
