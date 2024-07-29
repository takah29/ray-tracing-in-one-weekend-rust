use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    onb::Onb,
    rtweekend::{Point3, Ray, Vec3, PI},
    vec3,
    vec3::random_to_sphere,
};
use std::f64::INFINITY;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            // 値が小さい方のt
            if ray_t.min < temp && temp < ray_t.max {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal.clone());
                get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
                rec.opt_mat_ptr = Some(self.mat_ptr.clone());
                return true;
            }

            let temp = (-half_b + root) / a;

            // 値が大きい方のt
            if ray_t.min < temp && temp < ray_t.max {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal.clone());
                get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
                rec.opt_mat_ptr = Some(self.mat_ptr.clone());
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new_with_points(
            self.center - vec3!(self.radius, self.radius, self.radius),
            self.center + vec3!(self.radius, self.radius, self.radius),
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

        let cos_theta_max =
            (1.0 - self.radius.powi(2) / (self.center - *origin).length_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let direction = self.center - *origin;
        let distance_squared = direction.length_squared();
        let uvw = Onb::build_from_w(direction);

        uvw.transform_vec3(random_to_sphere(self.radius, distance_squared))
    }
}

fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
    let theta = (-p.e[1]).acos();
    let phi = f64::atan2(-p.e[2], p.e[0]) + PI;

    *u = phi / (2.0 * PI);
    *v = theta / PI;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sphere_uv() {
        let test_cases = vec![
            (vec3!(1, 0, 0), 0.5, 0.5),
            (vec3!(0, 1, 0), 0.5, 1.0),
            (vec3!(0, 0, 1), 0.25, 0.5),
        ];

        for (p, u_exp, v_exp) in test_cases {
            let mut u_res = 0.0;
            let mut v_res = 0.0;
            get_sphere_uv(&p, &mut u_res, &mut v_res);
            assert_eq!(u_res, u_exp, "Failed u_res for input: '{:?}", p);
            assert_eq!(v_res, v_exp, "Failed v_res for input: '{:?}", p);
        }
    }
}
