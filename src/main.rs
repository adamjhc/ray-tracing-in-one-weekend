use camera::Camera;
use colour::Colour;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use sphere::Sphere;
use std::rc::Rc;
use utils::{random_double, random_double_within_range};
use vec3::{Point3, Vec3};

mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
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
    );

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for row in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {row} ");

        for col in 0..image_width {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (col as f64 + random_double()) / (image_width - 1) as f64;
                let v = (row as f64 + random_double()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray.ray_colour(&world, max_depth);
            }

            println!("{}", pixel_colour.write(samples_per_pixel));
        }
    }

    eprint!("\r");
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
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
                let material_sphere: Rc<dyn Material> = if chosen_material < 0.8 {
                    // diffuse
                    let albedo = Colour::random();
                    Rc::new(Lambertian::new(albedo))
                } else if chosen_material < 0.95 {
                    // metal
                    let albedo = Colour::random_within_range(0.5, 1.0);
                    let fuzz = random_double_within_range(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Rc::new(Sphere::new(center, 0.2, material_sphere)));
            }
        }
    }

    let material_glass = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_glass,
    )));

    let material_diffuse = Rc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_diffuse,
    )));

    let material_metal = Rc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_metal,
    )));

    world
}
