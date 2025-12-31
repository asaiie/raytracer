use crate::{interval::Interval, vec3::Vec3};
use std::io::{self, Write};

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    const INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };
    let rbyte = (256.0 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
