use crate::{
    aabb::AABB,
    material::Material,
    rtweekend::{Point3, Ray, Vec3},
};
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub opt_mat_ptr: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool;
}

pub struct Translate {
    obj_ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(obj_ptr: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { obj_ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new_with_time(r.orig - self.offset, r.dir, r.time);
        if !self.obj_ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_r, &rec.normal.clone());

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if !self.obj_ptr.bounding_box(t0, t1, output_box) {
            return false;
        }

        *output_box = AABB::new(output_box.min + self.offset, output_box.max + self.offset);

        true
    }
}
