use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }

        hit_record
    }

    // pub fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb> {
    //     if self.objects.is_empty() {
    //         return None;
    //     }

    //     let mut first_box = true;
    //     let mut bounding_box = None;
    //     for object in self.objects.iter() {
    //         if let Some(temp_box) = object.bounding_box(time_0, time_1) {
    //             bounding_box = if first_box {
    //                 Some(temp_box)
    //             } else {
    //                 Some(bounding_box?.surrounding_box(&temp_box))
    //             };
    //             first_box = false;
    //         } else {
    //             return None;
    //         }
    //     }

    //     bounding_box
    // }
}
