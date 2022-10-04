use crate::{
    colour::Colour,
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

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_colour(self) -> Colour {
        if self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5) {
            return Colour::new(1.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }

    pub fn hit_sphere(&self, center: Point3, radius: f64) -> bool {
        let oc = self.origin - center;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }
}
