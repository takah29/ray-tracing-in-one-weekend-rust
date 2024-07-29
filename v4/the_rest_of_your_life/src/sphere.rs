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
    pub center1: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
    pub is_moving: bool,
    pub center_vec: Vec3,
    pub bbox: Aabb,
}

impl Sphere {
    // Stationary Sphere
    pub fn new(center1: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        let rvec = vec3!(radius, radius, radius);

        Self {
            center1,
            radius,
            mat_ptr,
            is_moving: false,
            center_vec: center1,
            bbox: Aabb::new_with_points(center1 - rvec, center1 + rvec),
        }
    }
    // Moving Sphere
    pub fn new_with_moving_sphere(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        let rvec = vec3!(radius, radius, radius);
        let box1 = Aabb::new_with_points(center1 - rvec, center1 + rvec);
        let box2 = Aabb::new_with_points(center2 - rvec, center2 + rvec);

        Self {
            center1,
            radius: radius.max(0.0),
            mat_ptr,
            is_moving: true,
            center_vec: center2 - center1,
            bbox: Aabb::from_boxes(box1, box2),
        }
    }

    fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + time * self.center_vec
    }

    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.e[1]).acos();
        let phi = f64::atan2(-p.e[2], p.e[0]) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.sphere_center(r.time)
        } else {
            self.center1
        };
        let oc = center - r.orig;
        let a = r.dir.length_squared();
        let h = r.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.opt_mat_ptr = Some(self.mat_ptr.clone());

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
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
            (1.0 - self.radius.powi(2) / (self.center1 - *origin).length_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let direction = self.center1 - *origin;
        let distance_squared = direction.length_squared();
        let uvw = Onb::build_from_w(direction);

        uvw.transform_vec3(random_to_sphere(self.radius, distance_squared))
    }
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
            Sphere::get_sphere_uv(&p, &mut u_res, &mut v_res);
            assert_eq!(u_res, u_exp, "Failed u_res for input: '{:?}", p);
            assert_eq!(v_res, v_exp, "Failed v_res for input: '{:?}", p);
        }
    }
}
