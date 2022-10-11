use std::sync::Arc;

use crate::{hittable::Hittable, material::Material, vec3::Point3};

pub struct MovingSphere {
    center_start: Point3,
    center_end: Point3,
    time_start: f64,
    time_end: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center_start: Point3,
        center_end: Point3,
        time_start: f64,
        time_end: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> MovingSphere {
        Self {
            center_start,
            center_end,
            time_start,
            time_end,
            radius,
            material,
        }
    }

    pub fn center_at(&self, time: f64) -> Point3 {
        self.center_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.center_end - self.center_start)
    }
}

impl Hittable for MovingSphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        hit_record: &mut crate::hittable::HitRecord,
    ) -> bool {
        let oc = ray.origin - self.center_at(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center_at(ray.time)) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = Some(self.material.clone());

        true
    }
}
