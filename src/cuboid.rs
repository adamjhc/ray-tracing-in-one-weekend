use std::sync::Arc;

use crate::{
    aabb::Aabb,
    aarect::{XYRect, XZRect, YZRect},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::Material,
    ray::Ray,
    vec3::Point3,
};

pub struct Cuboid {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.push(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            material.clone(),
        )));
        sides.push(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            material.clone(),
        )));
        sides.push(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            material.clone(),
        )));
        sides.push(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            material.clone(),
        )));
        sides.push(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            material.clone(),
        )));
        sides.push(Arc::new(YZRect::new(
            p0.y, p1.y, p0.z, p1.z, p0.x, material,
        )));

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}
