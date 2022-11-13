use crate::image::Image;
use crate::scene::Scene;
use camera::Camera;
use clap::{arg, Parser};
use colour::Colour;
use minifb::{Key, Window, WindowOptions};
use rand::random;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::sync::mpsc;
use world::World;

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

    let (width, height) = (image.width as usize, image.height as usize);

    let mut window = Window::new(
        &format!("Raytracer - {:?}", args.scene),
        width,
        height,
        WindowOptions::default(),
    )
    .expect("Unable to open window");

    let (sender, receiver) = mpsc::channel();

    rayon::spawn(move || {
        (0..height)
            .into_par_iter()
            .for_each_with(sender, |sender, row| {
                sender
                    .send((
                        row,
                        render_row(row, &image, &camera, &world)
                            .iter()
                            .map(|colour| colour.to_u32(image.samples_per_pixel))
                            .collect::<Vec<u32>>(),
                    ))
                    .unwrap();
            })
    });

    let mut pixels = vec![0; width * height];
    let count = pixels.len();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Ok((row_num, row_pixels)) = receiver.try_recv() {
            let row_start = (count - width) - row_num * width;
            pixels.splice(row_start..row_start + width, row_pixels);
        }
        window.update_with_buffer(&pixels, width, height).unwrap();
    }
}

fn render_row(row: usize, image: &Image, camera: &Camera, world: &World) -> Vec<Colour> {
    (0..image.width as usize)
        .map(|col| render_pixel(row, col, image, camera, world))
        .collect::<Vec<Colour>>()
}

fn render_pixel(row: usize, col: usize, image: &Image, camera: &Camera, world: &World) -> Colour {
    (0..image.samples_per_pixel).fold(Colour::default(), |pixel_colour, _| {
        let u = (col as f64 + random::<f64>()) / (image.width - 1) as f64;
        let v = (row as f64 + random::<f64>()) / (image.height - 1) as f64;
        let ray = camera.get_ray(u, v);
        pixel_colour + ray.ray_colour(world, image.max_depth)
    })
}
