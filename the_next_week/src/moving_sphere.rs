use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::rtweekend::{Point3, Ray, Vec3};
use crate::vec3;
use std::sync::Arc;

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center(r.time);
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            // 値が小さい方のt
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = r.clone().at(rec.t);
                let outward_normal = (rec.p - self.center(r.time)) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.opt_mat_ptr = Some(self.mat_ptr.clone());
                return true;
            }

            let temp = (-half_b + root) / a;

            // 値が大きい方のt
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = r.clone().at(rec.t);
                let outward_normal = (rec.p - self.center(r.time)) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.opt_mat_ptr = Some(self.mat_ptr.clone());
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            self.center(t0) - vec3!(self.radius, self.radius, self.radius),
            self.center(t0) + vec3!(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(t1) - vec3!(self.radius, self.radius, self.radius),
            self.center(t1) + vec3!(self.radius, self.radius, self.radius),
        );

        *output_box = surrounding_box(box0, box1);

        true
    }
}
