use crate::{
    aarect::{XYRect, XZRect, YZRect},
    bvh::BVHNode,
    camera::Camera,
    colour::Colour,
    constant_medium::ConstantMedium,
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
    CornellSmoke,
    FinalScene,
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
            Self::CornellSmoke => (
                image.set_aspect_ratio(1.0).set_width(600),
                camera
                    .set_aspect_ratio(1.0)
                    .set_look_from(Point3::new(278.0, 278.0, -800.0))
                    .set_look_at(Point3::new(278.0, 278.0, 0.0))
                    .set_vertical_field_of_view_degrees(40.0),
                World::new(black, Self::cornell_smoke()),
            ),
            Self::FinalScene => (
                image
                    .set_aspect_ratio(1.0)
                    .set_width(800)
                    .set_samples_per_pixel(10000),
                camera
                    .set_aspect_ratio(1.0)
                    .set_look_from(Point3::new(478.0, 278.0, -600.0))
                    .set_look_at(Point3::new(278.0, 278.0, 0.0))
                    .set_vertical_field_of_view_degrees(40.0),
                World::new(black, Self::final_scene()),
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

    fn cornell_smoke() -> HittableList {
        let mut objects = HittableList::new();

        let red = Arc::new(Lambertian::from(Colour::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::from(Colour::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::from(Colour::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::new(Colour::new(7.0, 7.0, 7.0)));

        objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
        objects.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
        objects.push(Arc::new(XZRect::new(
            113.0, 443.0, 127.0, 432.0, 554.0, light,
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
        cuboid_1 = Arc::new(ConstantMedium::new(
            cuboid_1,
            0.01,
            Colour::new(0.0, 0.0, 0.0),
        ));
        objects.push(cuboid_1);

        let mut cuboid_2: Arc<dyn Hittable> = Arc::new(Cuboid::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(165.0, 165.0, 165.0),
            white,
        ));
        cuboid_2 = Arc::new(RotateY::new(cuboid_2, -18.0));
        cuboid_2 = Arc::new(Translate::new(cuboid_2, Vec3::new(130.0, 0.0, 65.0)));
        cuboid_2 = Arc::new(ConstantMedium::new(
            cuboid_2,
            0.01,
            Colour::new(1.0, 1.0, 1.0),
        ));
        objects.push(cuboid_2);

        objects
    }

    fn final_scene() -> HittableList {
        let mut objects = HittableList::new();

        // Green ground boxes
        let mut boxes_1 = HittableList::new();
        let ground = Arc::new(Lambertian::from(Colour::new(0.48, 0.83, 0.53)));
        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let width = 100.0;
                let x0 = -1000.0 + i as f64 * width;
                let x1 = x0 + width;
                let y0 = 0.0;
                let y1 = thread_rng().gen_range(1.0..101.0);
                let z0 = -1000.0 + j as f64 * width;
                let z1 = z0 + width;

                boxes_1.push(Arc::new(Cuboid::new(
                    Point3::new(x0, y0, z0),
                    Point3::new(x1, y1, z1),
                    ground.clone(),
                )));
            }
        }
        objects.push(Arc::new(BVHNode::new(boxes_1, 0.0, 1.0)));

        // Light
        let light = Arc::new(DiffuseLight::new(Colour::new(7.0, 7.0, 7.0)));
        objects.push(Arc::new(XZRect::new(
            123.0, 423.0, 147.0, 412.0, 554.0, light,
        )));

        // Brown moving sphere
        let center_1 = Point3::new(400.0, 400.0, 200.0);
        let center_2 = center_1 + Vec3::new(30.0, 0.0, 0.0);
        let moving_sphere_material = Arc::new(Lambertian::from(Colour::new(0.7, 0.3, 0.1)));
        objects.push(Arc::new(MovingSphere::new(
            center_1,
            center_2,
            0.0,
            1.0,
            50.0,
            moving_sphere_material,
        )));

        // Glass
        objects.push(Arc::new(Sphere::new(
            Point3::new(260.0, 150.0, 45.0),
            50.0,
            Arc::new(Dielectric::new(1.5)),
        )));

        // Grey fuzz
        objects.push(Arc::new(Sphere::new(
            Point3::new(0.0, 150.0, 145.0),
            50.0,
            Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.9), 1.0)),
        )));

        // Blue
        let boundary = Arc::new(Sphere::new(
            Point3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(Dielectric::new(1.5)),
        ));
        objects.push(boundary.clone());
        objects.push(Arc::new(ConstantMedium::new(
            boundary,
            0.2,
            Colour::new(0.2, 0.4, 0.9),
        )));

        // Big thin mist
        let boundary = Arc::new(Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            5000.0,
            Arc::new(Dielectric::new(1.5)),
        ));
        objects.push(Arc::new(ConstantMedium::new(
            boundary,
            0.0001,
            Colour::new(1.0, 1.0, 1.0),
        )));

        // Earth
        let earth_map = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(Path::new(
            "earthmap.jpg",
        )))));
        objects.push(Arc::new(Sphere::new(
            Point3::new(400.0, 200.0, 400.0),
            100.0,
            earth_map,
        )));

        // Noise
        let pertext = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(0.1))));
        objects.push(Arc::new(Sphere::new(
            Point3::new(220.0, 280.0, 300.0),
            80.0,
            pertext,
        )));

        // Bubbly
        let mut boxes_2 = HittableList::new();
        let white = Arc::new(Lambertian::from(Colour::new(0.73, 0.73, 0.73)));
        for _ in 0..1000 {
            boxes_2.push(Arc::new(Sphere::new(
                Point3::random_within(0.0, 165.0),
                10.0,
                white.clone(),
            )));
        }
        objects.push(Arc::new(Translate::new(
            Arc::new(RotateY::new(
                Arc::new(BVHNode::new(boxes_2, 0.0, 1.0)),
                15.0,
            )),
            Vec3::new(-100.0, 270.0, 395.0),
        )));

        objects
    }
}
