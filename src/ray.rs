use crate::{
    colour::Colour,
    hittable_list::HittableList,
    vec3::{Point3, Vec3},
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

    pub fn ray_colour(&self, world: &HittableList, depth: i32) -> Colour {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Colour::default();
        }

        if let Some(hit_record) = world.hit(self, 0.001, f64::INFINITY) {
            assert!(hit_record.material.is_some());

            let mut scattered = Ray::default();
            let mut attenuation = Colour::default();
            return if hit_record.material.as_ref().unwrap().scatter(
                self,
                &hit_record,
                &mut attenuation,
                &mut scattered,
            ) {
                attenuation * scattered.ray_colour(world, depth - 1)
            } else {
                Colour::default()
            };
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}
