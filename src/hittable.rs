use crate::{
    aabb::Aabb,
    material::Material,
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3, Vec3},
};
use std::{f64::INFINITY, sync::Arc};

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
        let (front_face, normal) = Self::calculate_face_normal(ray, outward_normal);
        Self {
            p,
            t,
            u,
            v,
            normal,
            front_face,
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        (self.front_face, self.normal) = Self::calculate_face_normal(ray, outward_normal);
    }

    fn calculate_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }
}

pub struct Translate {
    pointer: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(pointer: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { pointer, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        self.pointer
            .hit(&moved_ray, t_min, t_max)
            .map(|mut hit_record| {
                hit_record.p += self.offset;
                hit_record.set_face_normal(&moved_ray, hit_record.normal);
                hit_record
            })
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb> {
        self.pointer.bounding_box(time_0, time_1).map(|output_box| {
            Aabb::new(
                output_box.minimum + self.offset,
                output_box.maximum + self.offset,
            )
        })
    }
}

pub struct RotateY {
    pointer: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<Aabb>,
}

impl RotateY {
    pub fn new(pointer: Arc<dyn Hittable>, angle_degrees: f64) -> Self {
        let radians = degrees_to_radians(angle_degrees);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bounding_box = pointer.bounding_box(0.0, 1.0).map(|bounding_box| {
            let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
            let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

            (0..2).for_each(|i| {
                (0..2).for_each(|j| {
                    (0..2).for_each(|k| {
                        let x = i as f64 * bounding_box.maximum.x
                            + (1 - i) as f64 * bounding_box.minimum.x;
                        let y = j as f64 * bounding_box.maximum.y
                            + (1 - j) as f64 * bounding_box.minimum.y;
                        let z = k as f64 * bounding_box.maximum.z
                            + (1 - k) as f64 * bounding_box.minimum.z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(new_x, y, new_z);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    });
                });
            });

            Aabb::new(min, max)
        });

        Self {
            pointer,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction, ray.time);

        self.pointer
            .hit(&rotated_ray, t_min, t_max)
            .map(|mut hit_record| {
                let mut p = hit_record.p;
                let mut normal = hit_record.normal;

                p[0] = self.cos_theta * hit_record.p[0] + self.sin_theta * hit_record.p[2];
                p[2] = -self.sin_theta * hit_record.p[0] + self.cos_theta * hit_record.p[2];

                normal[0] =
                    self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
                normal[2] =
                    -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

                hit_record.p = p;
                hit_record.set_face_normal(&rotated_ray, normal);
                hit_record
            })
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        self.bounding_box
    }
}
