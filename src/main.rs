use std::io::{stderr, stdout, Write};

use colour::Colour;
mod colour;
mod vec3;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut stdout = stdout();
    let mut stderr = stderr();
    stdout
        .write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())
        .unwrap();

    for row in (0..image_height).rev() {
        stderr
            .write_all(format!("\rScanlines remaining: {row} ").as_bytes())
            .unwrap();

        for col in 0..image_width {
            let pixel_colour = Colour::new(
                col as f64 / (image_width - 1) as f64,
                row as f64 / (image_height - 1) as f64,
                0.25,
            );
            pixel_colour.write_to(&mut stdout);
        }
    }

    stderr.write_all("\r".as_bytes()).unwrap();
}
