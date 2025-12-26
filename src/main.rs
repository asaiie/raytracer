mod vec3;
mod color;

use vec3::Vec3;
use color::{Color, write_color};

use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let file = File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", image_width, image_height)?;
    writeln!(out, "255")?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        std::io::stdout().flush()?;

        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0
            );
            write_color(&mut out, &pixel_color)?;
        }
    }

    println!("\rDone.                 ");
    Ok(())
}
