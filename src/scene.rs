use crate::{
    aarect::{XYRect, XZRect, YZRect},
    camera::Camera,
    colour::Colour,
    cuboid::Cuboid,
    hittable::{Hittable, RotateY, Translate},
    hittable_list::HittableList,
    image::Image,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    vec3::{Point3, Vec3},
    world::World,
};
use clap::ValueEnum;
use rand::{random, thread_rng, Rng};
use std::{path::Path, sync::Arc};

#[derive(ValueEnum, Clone)]
pub enum Scene {
    Random,
    TwoSpheres,
    TwoPerlinSpheres,
    Earth,
    SimpleLight,
    CornellBox,
}

impl Scene {
    pub fn get(self) -> (Image, Camera, World) {
        // Defaults
        let camera = Camera::new(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            0.0,
            10.0,
            0.0,
            1.0,
        );
        let image = Image::new(16.0 / 9.0, 400, 200, 50);
        let blueish_white = Colour::new(0.7, 0.8, 1.0);
        let black = Colour::default();

        // Scenes
        match self {
            Self::Random => (
                image,
                camera.set_aperature(0.1),
                World::new(blueish_white, Self::random_scene()),
            ),
            Self::TwoSpheres => (
                image,
                camera,
                World::new(blueish_white, Self::two_spheres()),
            ),
            Self::TwoPerlinSpheres => (
                image,
                camera,
                World::new(blueish_white, Self::two_perlin_spheres()),
            ),
            Self::Earth => (image, camera, World::new(blueish_white, Self::earth())),
            Self::SimpleLight => (
                image.set_samples_per_pixel(400),
                camera
                    .set_look_from(Point3::new(26.0, 3.0, 6.0))
                    .set_look_at(Point3::new(0.0, 2.0, 0.0)),
                World::new(black, Self::simple_light()),
            ),
            Self::CornellBox => (
                image.set_aspect_ratio(1.0).set_width(600),
                camera
                    .set_aspect_ratio(1.0)
                    .set_look_from(Point3::new(278.0, 278.0, -800.0))
                    .set_look_at(Point3::new(278.0, 278.0, 0.0))
                    .set_vertical_field_of_view_degrees(40.0),
                World::new(black, Self::cornell_box()),
            ),
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
                let chosen_material = random::<f64>();
                let center = Point3::new(
                    a as f64 + 0.9 * random::<f64>(),
                    0.2,
                    b as f64 + 0.9 * random::<f64>(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if chosen_material < 0.8 {
                        // diffuse
                        let albedo = Colour::random();
                        let center2 =
                            center + Vec3::new(0.0, thread_rng().gen_range(0.0..=0.5), 0.0);
                        world.push(Arc::new(MovingSphere::new(
                            center,
                            center2,
                            0.0,
                            1.0,
                            0.2,
                            Arc::new(Lambertian::from(albedo)),
                        )));
                    } else if chosen_material < 0.95 {
                        // metal
                        let albedo = Colour::random_within_range(0.5, 1.0);
                        let fuzz = thread_rng().gen_range(0.0..=0.5);
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

        let material_diffuse = Arc::new(Lambertian::from(Colour::new(0.4, 0.2, 0.1)));
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

    fn two_perlin_spheres() -> HittableList {
        let mut objects = HittableList::new();

        let noise = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4.0))));

        objects.push(Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            noise.clone(),
        )));
        objects.push(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, noise)));

        objects
    }

    fn earth() -> HittableList {
        let earth_surface = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(Path::new(
            "earthmap.jpg",
        )))));
        let globe = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

        let mut objects = HittableList::new();
        objects.push(globe);
        objects
    }

    fn simple_light() -> HittableList {
        let mut objects = HittableList::new();

        let noise = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4.0))));
        objects.push(Arc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            noise.clone(),
        )));
        objects.push(Arc::new(Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            noise,
        )));

        let diffuse_light = Arc::new(DiffuseLight::new(Colour::new(4.0, 4.0, 4.0)));
        objects.push(Arc::new(Sphere::new(
            Point3::new(0.0, 7.0, 0.0),
            2.0,
            diffuse_light.clone(),
        )));
        objects.push(Arc::new(XYRect::new(
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            diffuse_light,
        )));

        objects
    }

    fn cornell_box() -> HittableList {
        let mut objects = HittableList::new();

        let red = Arc::new(Lambertian::from(Colour::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::from(Colour::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::from(Colour::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::new(Colour::new(15.0, 15.0, 15.0)));

        objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
        objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
        objects.push(Arc::new(XZRect::new(
            213.0, 343.0, 227.0, 332.0, 554.0, light,
        )));
        objects.push(Arc::new(XZRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )));
        objects.push(Arc::new(XZRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        objects.push(Arc::new(XYRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));

        let mut cuboid_1: Arc<dyn Hittable> = Arc::new(Cuboid::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(165.0, 330.0, 165.0),
            white.clone(),
        ));
        cuboid_1 = Arc::new(RotateY::new(cuboid_1, 15.0));
        cuboid_1 = Arc::new(Translate::new(cuboid_1, Vec3::new(265.0, 0.0, 295.0)));
        objects.push(cuboid_1);

        let mut cuboid_2: Arc<dyn Hittable> = Arc::new(Cuboid::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(165.0, 165.0, 165.0),
            white,
        ));
        cuboid_2 = Arc::new(RotateY::new(cuboid_2, -18.0));
        cuboid_2 = Arc::new(Translate::new(cuboid_2, Vec3::new(130.0, 0.0, 65.0)));
        objects.push(cuboid_2);

        objects
    }
}
