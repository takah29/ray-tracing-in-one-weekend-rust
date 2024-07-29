use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    rtweekend::{random_range, Point3, Ray, Vec3, INFINITY},
    {point3, vec3},
};
use std::sync::Arc;

pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mat_ptr: Arc<dyn Material>,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mat_ptr,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.e[2]) / r.dir.e[2];
        if t < ray_t.min || ray_t.max < t {
            return false;
        }
        let x = r.orig.e[0] + t * r.dir.e[0];
        let y = r.orig.e[1] + t * r.dir.e[1];
        if x < self.x0 || self.x1 < x || y < self.y0 || self.y1 < y {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = vec3!(0, 0, 1);
        rec.set_face_normal(r, &outward_normal);
        rec.opt_mat_ptr = Some(self.mat_ptr.clone());
        rec.p = r.at(t);

        true
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            point3!(self.x0, self.y0, self.k - 0.0001),
            point3!(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat_ptr: Arc<dyn Material>,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mat_ptr,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.e[1]) / r.dir.e[1];
        if t < ray_t.min || ray_t.max < t {
            return false;
        }
        let x = r.orig.e[0] + t * r.dir.e[0];
        let z = r.orig.e[2] + t * r.dir.e[2];
        if x < self.x0 || self.x1 < x || z < self.z0 || self.z1 < z {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = vec3!(0, 1, 0);
        rec.set_face_normal(r, &outward_normal);
        rec.opt_mat_ptr = Some(self.mat_ptr.clone());
        rec.p = r.at(t);

        true
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            point3!(self.x0, self.k - 0.0001, self.z0),
            point3!(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(*origin, *v),
            Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = v.dot(rec.normal).abs() / v.length();

        distance_squared / (cosine * area)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let random_point = point3!(
            random_range(self.x0, self.x1),
            self.k,
            random_range(self.z0, self.z1)
        );
        return random_point - *origin;
    }
}

pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat_ptr: Arc<dyn Material>,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mat_ptr,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.e[0]) / r.dir.e[0];
        if t < ray_t.min || ray_t.max < t {
            return false;
        }
        let y = r.orig.e[1] + t * r.dir.e[1];
        let z = r.orig.e[2] + t * r.dir.e[2];
        if y < self.y0 || self.y1 < y || z < self.z0 || self.z1 < z {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = vec3!(1, 0, 0);
        rec.set_face_normal(r, &outward_normal);
        rec.opt_mat_ptr = Some(self.mat_ptr.clone());
        rec.p = r.at(t);

        true
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            point3!(self.k - 0.0001, self.y0, self.z0),
            point3!(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}
