use camera::Camera;
use colour::Colour;
use hittable_list::HittableList;
use indicatif::ParallelProgressIterator;
use material::{Dielectric, Lambertian, Metal};
use moving_sphere::MovingSphere;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use sphere::Sphere;
use std::sync::Arc;
use utils::{random_double, random_double_within_range};
use vec3::{Point3, Vec3};

mod aabb;
mod bvh;
mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
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
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

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
                        let u = (col as f64 + random_double()) / (image_width - 1) as f64;
                        let v = (row as f64 + random_double()) / (image_height - 1) as f64;
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let chosen_material = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if chosen_material < 0.8 {
                    // diffuse
                    let albedo = Colour::random();
                    let center2 =
                        center + Vec3::new(0.0, random_double_within_range(0.0, 0.5), 0.0);
                    world.push(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new(albedo)),
                    )));
                } else if chosen_material < 0.95 {
                    // metal
                    let albedo = Colour::random_within_range(0.5, 1.0);
                    let fuzz = random_double_within_range(0.0, 0.5);
                    world.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                };
            }
        }
    }

    let material_glass = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_glass,
    )));

    let material_diffuse = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_diffuse,
    )));

    let material_metal = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.push(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_metal,
    )));

    world
}
