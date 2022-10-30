use crate::{ray::Ray, vec3::Point3};

#[derive(Clone, Copy)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        // for a in 0..3 {
        //     let inverse_direction = 1.0 / ray.direction[a];
        //     let mut t0 = (self.minimum[a] - ray.origin[a]) * inverse_direction;
        //     let mut t1 = (self.maximum[a] - ray.origin[a]) * inverse_direction;
        //     if inverse_direction < 0.0 {
        //         swap(&mut t0, &mut t1);
        //     }

        //     t_min = t0.max(t_min);
        //     t_max = t1.min(t_max);
        //     if t_max <= t_min {
        //         return false;
        //     }
        // }

        for a in 0..3 {
            let t0 = ((self.minimum[a] - ray.origin[a]) / ray.direction[a])
                .min((self.maximum[a] - ray.origin[a]) / ray.direction[a]);
            let t1 = ((self.minimum[a] - ray.origin[a]) / ray.direction[a])
                .max((self.maximum[a] - ray.origin[a]) / ray.direction[a]);

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    /// Computes the bounding box of two boxes
    pub fn surrounding_box(&self, bounding_box: &Aabb) -> Aabb {
        let small = Point3::new(
            self.minimum.x.min(bounding_box.minimum.x),
            self.minimum.y.min(bounding_box.minimum.y),
            self.minimum.z.min(bounding_box.minimum.z),
        );
        let big = Point3::new(
            self.maximum.x.max(bounding_box.maximum.x),
            self.maximum.y.max(bounding_box.maximum.y),
            self.maximum.z.max(bounding_box.maximum.z),
        );

        Aabb::new(small, big)
    }
}
