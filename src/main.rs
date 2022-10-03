use std::io::{stderr, stdout, Write};

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut stdout = stdout();
    let mut stderr = stderr();
    stdout
        .write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .unwrap();

    for row in (0..image_height).rev() {
        stderr
            .write_all(format!("\rScanlines remaining: {} ", row).as_bytes())
            .unwrap();

        for col in 0..image_width {
            let r = col as f32 / (image_width - 1) as f32;
            let g = row as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            stdout
                .write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }

    stderr.write_all("\r".as_bytes()).unwrap();
}
