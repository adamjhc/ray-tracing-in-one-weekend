use crate::{
    colour::Colour,
    hittable::{HitRecord, Hittable},
    material::{Isotropic, Material},
    ray::Ray,
    vec3::Vec3,
};
use rand::random;
use std::{f64::INFINITY, sync::Arc};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    negative_inverse_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, albedo: Colour) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new(albedo)),
            negative_inverse_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let mut rec_1 = self.boundary.hit(ray, -INFINITY, INFINITY)?;
        let mut rec_2 = self.boundary.hit(ray, rec_1.t + 0.0001, INFINITY)?;

        if rec_1.t < t_min {
            rec_1.t = t_min;
        }

        if rec_2.t > t_max {
            rec_2.t = t_max;
        }

        if rec_1.t >= rec_2.t {
            return None;
        }

        if rec_1.t < 0.0 {
            rec_1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec_2.t - rec_1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec_1.t + hit_distance / ray_length;
        Some(HitRecord::new(
            ray.at(t),
            t,
            0.0,
            0.0,
            &Ray::default(),
            Vec3::default(),
            Some(self.phase_function.clone()),
        ))
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<crate::aabb::Aabb> {
        self.boundary.bounding_box(time_0, time_1)
    }
}
