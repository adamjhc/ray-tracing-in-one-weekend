use std::f64::INFINITY;

use crate::{
    colour::Colour,
    hittable::HitRecord,
    hittable_list::HittableList,
    vec3::{Point3, Vec3},
};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_colour(&self, world: &HittableList, depth: i32) -> Colour {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Colour::default();
        }

        let mut hit_record = HitRecord::default();
        if world.hit(self, 0.001, INFINITY, &mut hit_record) {
            // let target = hit_record.p + hit_record.normal + Vec3::random_unit_vector();
            let target = hit_record.p + hit_record.normal.random_in_hemisphere();

            return 0.5
                * Ray::new(hit_record.p, target - hit_record.p).ray_colour(world, depth - 1);
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}
