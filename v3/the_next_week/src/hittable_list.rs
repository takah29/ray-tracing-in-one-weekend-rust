use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::rtweekend::Ray;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn new_with_object(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut crate::aabb::AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::new_with_empty();
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(t0, t1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                surrounding_box(output_box.clone(), temp_box.clone())
            };
            first_box = false;
        }

        true
    }
}
