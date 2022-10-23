use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

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
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center_at(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center_at(ray.time)) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        Some(HitRecord::new(
            p,
            root,
            u,
            v,
            ray,
            outward_normal,
            Some(self.material.clone()),
        ))
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb> {
        let box_0 = Aabb::new(
            self.center_at(time_0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at(time_0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box_1 = Aabb::new(
            self.center_at(time_1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at(time_1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(box_0.surrounding_box(&box_1))
    }
}
