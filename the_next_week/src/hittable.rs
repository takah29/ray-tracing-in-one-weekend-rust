use crate::{
    aabb::AABB,
    material::Material,
    point3,
    rtweekend::{Point3, Ray, Vec3, INFINITY},
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

pub struct RotateY {
    obj_ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(obj_ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut bbox = AABB::new_with_inf();
        let hasbox = obj_ptr.bounding_box(0.0, 1.0, &mut bbox);

        let mut min = point3!(INFINITY, INFINITY, INFINITY);
        let mut max = point3!(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..=1 {
            for j in 0..=1 {
                for k in 0..=1 {
                    let zero_one_vec = point3!(i, j, k);
                    let p = zero_one_vec * bbox.max + (1.0 - zero_one_vec) * bbox.min;
                    let newx = cos_theta * p.e[0] + sin_theta * p.e[2];
                    let newz = -sin_theta * p.e[0] + cos_theta * p.e[2];

                    let tester = point3!(newx, p.e[1], newz);

                    for c in 0..3 {
                        min.e[c] = bbox.min.e[c].min(tester.e[c]);
                        max.e[c] = bbox.max.e[c].max(tester.e[c]);
                    }
                }
            }
        }

        Self {
            obj_ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox: AABB::new(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.orig.clone();
        let mut direction = r.dir.clone();

        origin.e[0] = self.cos_theta * r.orig.e[0] - self.sin_theta * r.orig.e[2];
        origin.e[2] = self.sin_theta * r.orig.e[0] + self.cos_theta * r.orig.e[2];
        direction.e[0] = self.cos_theta * r.dir.e[0] - self.sin_theta * r.dir.e[2];
        direction.e[2] = self.sin_theta * r.dir.e[0] + self.cos_theta * r.dir.e[2];

        let rotated_r = Ray::new_with_time(origin, direction, r.time);

        if !self.obj_ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.e[0] = self.cos_theta * rec.p.e[0] + self.sin_theta * rec.p.e[2];
        p.e[2] = -self.sin_theta * rec.p.e[0] + self.cos_theta * rec.p.e[2];

        normal.e[0] = self.cos_theta * rec.normal.e[0] + self.sin_theta * rec.normal.e[2];
        normal.e[2] = -self.sin_theta * rec.normal.e[0] + self.cos_theta * rec.normal.e[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        self.hasbox
    }
}
