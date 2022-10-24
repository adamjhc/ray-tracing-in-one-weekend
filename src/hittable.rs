use crate::{
    aabb::Aabb,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        t: f64,
        u: f64,
        v: f64,
        ray: &Ray,
        outward_normal: Vec3,
        material: Option<Arc<dyn Material>>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        Self {
            p,
            t,
            u,
            v,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            front_face,
            material,
        }
    }
}
