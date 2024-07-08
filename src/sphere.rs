use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc * r.dir;
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = r.clone().at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }

            let temp = (-half_b + root) / a;
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = r.clone().at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}
