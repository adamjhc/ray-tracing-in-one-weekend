use crate::scene::Scene;
use clap::{arg, Parser};
use colour::Colour;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rand::random;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod colour;
mod constant_medium;
mod cuboid;
mod hittable;
mod hittable_list;
mod image;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod scene;
mod sphere;
mod texture;
mod utils;
mod vec3;
mod world;

#[derive(Parser)]
struct Args {
    #[arg(value_enum)]
    pub scene: Scene,
}

fn main() {
    let args = Args::parse();

    let (image, camera, world) = args.scene.get();

    // Render
    println!("P3");
    println!("{} {}", image.width, image.height);
    println!("255");

    let colours = (0..image.height)
        .into_par_iter()
        .rev()
        .progress_count(image.height as u64)
        .flat_map(|row| {
            (0..image.width)
                .map(|col| {
                    (0..image.samples_per_pixel).fold(Colour::default(), |pixel_colour, _| {
                        let u = (col as f64 + random::<f64>()) / (image.width - 1) as f64;
                        let v = (row as f64 + random::<f64>()) / (image.height - 1) as f64;
                        let ray = camera.get_ray(u, v);
                        pixel_colour + ray.ray_colour(&world, image.max_depth)
                    })
                })
                .collect::<Vec<Colour>>()
        })
        .collect::<Vec<Colour>>();

    colours
        .iter()
        .progress()
        .for_each(|pixel| println!("{}", pixel.write_to_rgb(image.samples_per_pixel)));
}
