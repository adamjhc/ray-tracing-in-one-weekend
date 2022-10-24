use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct XYRect {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t_max < t {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || self.x1 < x || y < self.y0 || self.y1 < y {
            return None;
        }

        Some(HitRecord::new(
            ray.at(t),
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            ray,
            Vec3::new(0.0, 0.0, 1.0),
            Some(self.material.clone()),
        ))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XZRect {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t_max < t {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || self.x1 < x || z < self.z0 || self.z1 < z {
            return None;
        }

        Some(HitRecord::new(
            ray.at(t),
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
            ray,
            Vec3::new(0.0, 1.0, 0.0),
            Some(self.material.clone()),
        ))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount
        Some(Aabb::new(
            Point3::new(self.x0, self.z0, self.k - 0.0001),
            Point3::new(self.x1, self.z1, self.k + 0.0001),
        ))
    }
}

pub struct YZRect {
    material: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t_max < t {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || self.y1 < y || z < self.z0 || self.z1 < z {
            return None;
        }

        Some(HitRecord::new(
            ray.at(t),
            t,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
            ray,
            Vec3::new(1.0, 0.0, 0.0),
            Some(self.material.clone()),
        ))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount
        Some(Aabb::new(
            Point3::new(self.y0, self.z0, self.k - 0.0001),
            Point3::new(self.y1, self.z1, self.k + 0.0001),
        ))
    }
}
