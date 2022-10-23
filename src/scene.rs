use std::sync::Arc;

use crate::{
    camera::Camera,
    colour::Colour,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    texture::CheckerTexture,
    utils::{random_double, random_double_within_range},
    vec3::{Point3, Vec3},
};

pub struct Scene;

impl Scene {
    pub fn get(index: usize, aspect_ratio: f64) -> (HittableList, Camera) {
        match index {
            1 => (
                Self::random_scene(),
                Camera::new(
                    Point3::new(13.0, 2.0, 2.0),
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    20.0,
                    aspect_ratio,
                    0.1,
                    10.0,
                    0.0,
                    1.0,
                ),
            ),
            2 => (
                Self::two_spheres(),
                Camera::new(
                    Point3::new(13.0, 2.0, 2.0),
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    20.0,
                    aspect_ratio,
                    0.0,
                    10.0,
                    0.0,
                    1.0,
                ),
            ),
            _ => panic!("Scene {index} doesn't exist"),
        }
    }

    fn random_scene() -> HittableList {
        let mut world = HittableList::new();

        let material_ground = Arc::new(Lambertian::new(Arc::new(CheckerTexture::from_colour(
            Colour::new(0.2, 0.3, 0.1),
            Colour::new(0.9, 0.9, 0.9),
        ))));
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
                            Arc::new(Lambertian::from_colour(albedo)),
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

        let material_diffuse = Arc::new(Lambertian::from_colour(Colour::new(0.4, 0.2, 0.1)));
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

    fn two_spheres() -> HittableList {
        let mut objects = HittableList::new();

        let checker = Arc::new(Lambertian::new(Arc::new(CheckerTexture::from_colour(
            Colour::new(0.2, 0.3, 0.1),
            Colour::new(0.9, 0.9, 0.9),
        ))));

        objects.push(Arc::new(Sphere::new(
            Vec3::new(0.0, -10.0, 0.0),
            10.0,
            checker.clone(),
        )));
        objects.push(Arc::new(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            checker,
        )));

        objects
    }
}
