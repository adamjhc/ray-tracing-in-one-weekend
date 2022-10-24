use crate::scene::Scene;
use colour::Colour;
use indicatif::ParallelProgressIterator;
use rand::random;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

mod aabb;
mod bvh;
mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod scene;
mod sphere;
mod texture;
mod utils;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World and camera
    let (world, camera) = Scene::get(3, aspect_ratio);

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    let colours = (0..image_height)
        .into_par_iter()
        .rev()
        .progress_count(image_height as u64)
        .flat_map(|row| {
            (0..image_width)
                .map(|col| {
                    (0..samples_per_pixel).fold(Colour::default(), |pixel_colour, _| {
                        let u = (col as f64 + random::<f64>()) / (image_width - 1) as f64;
                        let v = (row as f64 + random::<f64>()) / (image_height - 1) as f64;
                        let ray = camera.get_ray(u, v);
                        pixel_colour + ray.ray_colour(&world, max_depth)
                    })
                })
                .collect::<Vec<Colour>>()
        })
        .collect::<Vec<Colour>>();

    eprint!("\rWriting to file...");
    for mut pixel in colours {
        println!("{}", pixel.write_to_rgb(samples_per_pixel));
    }
    eprint!("\r")
}
