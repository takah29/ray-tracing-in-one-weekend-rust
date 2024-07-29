use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    rtweekend::{random_int, Ray, Vec3},
};
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::new_with_empty(),
        }
    }

    pub fn new_with_object(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = Aabb::from_boxes(self.bbox, object.bounding_box());
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = Aabb::new_with_empty();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &crate::vec3::Point3, v: &crate::vec3::Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let sum = self
            .objects
            .iter()
            .map(|x| weight * x.pdf_value(origin, v))
            .sum();

        sum
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let int_size = self.objects.len();
        self.objects[random_int(0, (int_size - 1) as i32) as usize].random(origin)
    }
}
