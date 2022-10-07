use camera::Camera;
use colour::Colour;
use hittable_list::HittableList;
use sphere::Sphere;
use std::{
    io::{stderr, stdout, Write},
    rc::Rc,
};
use utils::random_double;
use vec3::Point3;

mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

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
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (col as f64 + random_double()) / (image_width - 1) as f64;
                let v = (row as f64 + random_double()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray.ray_colour(&world, max_depth);
            }
            stdout
                .write_all(format!("{}\n", pixel_colour.write(samples_per_pixel)).as_bytes())
                .unwrap()
        }
    }

    stderr.write_all("\r".as_bytes()).unwrap();
}
