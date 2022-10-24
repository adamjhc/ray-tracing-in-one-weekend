use crate::{
    colour::Colour,
    vec3::{Point3, Vec3},
    world::World,
};

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_colour(&self, world: &World, depth: i32) -> Colour {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Colour::default();
        }

        if let Some(hit_record) = world.objects.hit(self, 0.001, f64::INFINITY) {
            let emitted = hit_record.material.as_ref().unwrap().emitted(
                hit_record.u,
                hit_record.v,
                &hit_record.p,
            );

            if let Some((attenuation, scattered)) = hit_record
                .material
                .as_ref()
                .unwrap()
                .scatter(self, &hit_record)
            {
                emitted + attenuation * scattered.ray_colour(world, depth - 1)
            } else {
                emitted
            }
        } else {
            world.background
        }
    }
}
