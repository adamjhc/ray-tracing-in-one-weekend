use std::{f64::INFINITY, rc::Rc};

use crate::{
    colour::Colour,
    hittable::{HitRecord, Hittable},
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

    pub fn ray_colour(&self, world: &HittableList) -> Colour {
        let mut hit_record = HitRecord::default();
        if world.hit(self, 0.0, INFINITY, &mut hit_record) {
            return 0.5 * (hit_record.normal + Colour::new(1.0, 1.0, 1.0));
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}
