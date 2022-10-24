use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use rand::{thread_rng, Rng};
use std::{cmp::Ordering, sync::Arc};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: Aabb,
}

impl BVHNode {
    pub fn new(
        src_objects: &Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time_0: f64,
        time_1: f64,
    ) -> BVHNode {
        let mut objects = src_objects.clone();

        let axis = thread_rng().gen_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Random integer out of range"),
        };

        let object_span = end - start;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]).is_lt() {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            (
                Arc::new(BVHNode::new(src_objects, start, mid, time_0, time_1)),
                Arc::new(BVHNode::new(src_objects, mid, end, time_0, time_1)),
            )
        };

        if let (Some(box_left), Some(box_right)) = (
            left.bounding_box(time_0, time_1),
            right.bounding_box(time_0, time_1),
        ) {
            Self {
                left,
                right,
                bounding_box: box_left.surrounding_box(&box_right),
            }
        } else {
            panic!("No bounding box in BVHNode constructor");
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        if let Some(hit_record) = self.left.hit(ray, t_min, t_max) {
            Some(hit_record)
        } else {
            self.right.hit(ray, t_min, t_max)
        }
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    if let (Some(box_a), Some(box_b)) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        box_a.minimum[axis].total_cmp(&box_b.minimum[axis])
    } else {
        panic!("No bounding box in BVHNode contructor")
    }
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
