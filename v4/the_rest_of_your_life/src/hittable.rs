use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    point3,
    rtweekend::{Point3, Ray, Vec3, INFINITY},
    vec3,
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
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
    fn pdf_value(&self, _origin: &Point3, _direction: &Vec3) -> f64 {
        return 0.0;
    }

    fn random(&self, _origin: &Vec3) -> Vec3 {
        return vec3!(1, 0, 0);
    }
}

pub struct Translate {
    obj_ptr: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(obj_ptr: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            obj_ptr: obj_ptr.clone(),
            offset,
            bbox: obj_ptr.bounding_box() + offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_with_time(r.orig - self.offset, r.dir, r.time);
        if !self.obj_ptr.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY {
    obj_ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(obj_ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = obj_ptr.bounding_box();

        let mut min = point3!(INFINITY, INFINITY, INFINITY);
        let mut max = point3!(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..=1 {
            for j in 0..=1 {
                for k in 0..=1 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = point3!(newx, y, newz);

                    for c in 0..3 {
                        min.e[c] = min.e[c].min(tester.e[c]);
                        max.e[c] = max.e[c].max(tester.e[c]);
                    }
                }
            }
        }

        Self {
            obj_ptr,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_with_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut origin = r.orig.clone();
        let mut direction = r.dir.clone();

        origin.e[0] = self.cos_theta * r.orig.e[0] - self.sin_theta * r.orig.e[2];
        origin.e[2] = self.sin_theta * r.orig.e[0] + self.cos_theta * r.orig.e[2];

        direction.e[0] = self.cos_theta * r.dir.e[0] - self.sin_theta * r.dir.e[2];
        direction.e[2] = self.sin_theta * r.dir.e[0] + self.cos_theta * r.dir.e[2];

        let rotated_r = Ray::new_with_time(origin, direction, r.time);

        if !self.obj_ptr.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        let mut p = rec.p;
        p.e[0] = self.cos_theta * rec.p.e[0] + self.sin_theta * rec.p.e[2];
        p.e[2] = -self.sin_theta * rec.p.e[0] + self.cos_theta * rec.p.e[2];

        let mut normal = rec.normal;
        normal.e[0] = self.cos_theta * rec.normal.e[0] + self.sin_theta * rec.normal.e[2];
        normal.e[2] = -self.sin_theta * rec.normal.e[0] + self.cos_theta * rec.normal.e[2];

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
