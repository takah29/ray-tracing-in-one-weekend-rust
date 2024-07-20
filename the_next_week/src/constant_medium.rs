use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Isotropic,
    material::Material,
    rtweekend::{random, Ray, Vec3, INFINITY},
    texture::Texture,
    vec3,
};
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(a)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && random() < 0.00001;

        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }

        if debugging {
            eprintln!("\nt0={}, t1={}", rec1.t, rec2.t)
        };

        if rec1.t < t_min {
            rec1.t = t_min
        };
        if rec2.t > t_max {
            rec2.t = t_max
        };

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.clone().at(rec.t);

        if debugging {
            eprintln!(
                "hit_distance = {}\nrec.t = {}\nrec.p = {:?}",
                hit_distance, rec.t, rec.p
            );
        }

        rec.normal = vec3!(1, 0, 0); // どんな値でもよい
        rec.front_face = true; // 同じくどんな値でもよい
        rec.opt_mat_ptr = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(t0, t1, output_box)
    }
}
