use ray::Ray;
use std::io::{stderr, stdout, Write};
use vec3::{Point3, Vec3};

mod colour;
mod ray;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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
            let u = col as f64 / (image_width - 1) as f64;
            let v = row as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_colour = ray.ray_colour();
            pixel_colour.write_to(&mut stdout);
        }
    }

    stderr.write_all("\r".as_bytes()).unwrap();
}
