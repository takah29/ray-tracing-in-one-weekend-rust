use crate::point3;
use crate::rtweekend::{Point3, Ray, INFINITY};

#[derive(Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn new_with_inf() -> Self {
        Self::new(
            point3!(-INFINITY, -INFINITY, -INFINITY),
            point3!(INFINITY, INFINITY, INFINITY),
        )
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for axis_num in 0..3 {
            let inv_d = 1.0 / r.dir.e[axis_num];
            let mut t0 = (self.min.e[axis_num] - r.orig.e[axis_num]) * inv_d;
            let mut t1 = (self.max.e[axis_num] - r.orig.e[axis_num]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = point3!(
        box0.min.e[0].min(box1.min.e[0]),
        box0.min.e[1].min(box1.min.e[1]),
        box0.min.e[2].min(box1.min.e[2])
    );
    let big = point3!(
        box0.max.e[0].max(box1.max.e[0]),
        box0.max.e[1].max(box1.max.e[1]),
        box0.max.e[2].max(box1.max.e[2])
    );
    AABB::new(small, big)
}
