use crate::{
    interval::Interval,
    rtweekend::{Point3, Ray, Vec3},
};
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn new_with_points(a: Point3, b: Point3) -> Self {
        let x = if a.e[0] <= b.e[0] {
            Interval::new(a.e[0], b.e[0])
        } else {
            Interval::new(b.e[0], a.e[0])
        };
        let y = if a.e[1] <= b.e[1] {
            Interval::new(a.e[1], b.e[1])
        } else {
            Interval::new(b.e[1], a.e[1])
        };
        let z = if a.e[2] <= b.e[2] {
            Interval::new(a.e[2], b.e[2])
        } else {
            Interval::new(b.e[2], a.e[2])
        };

        let mut bbox = Self { x, y, z };
        bbox.pad_to_minimums();

        bbox
    }

    pub fn new_with_empty() -> Self {
        let mut bbox = Self::new(
            Interval::new_with_empty(),
            Interval::new_with_empty(),
            Interval::new_with_empty(),
        );
        bbox.pad_to_minimums();

        bbox
    }

    pub fn from_boxes(box0: Aabb, box1: Aabb) -> Aabb {
        let x = Interval::from_intervals(box0.x, box1.x);
        let y = Interval::from_intervals(box0.y, box1.y);
        let z = Interval::from_intervals(box0.z, box1.z);

        Aabb::new(x, y, z)
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        for axis_num in 0..3 {
            let ax = self.axis_interval(axis_num);
            let adinv = 1.0 / r.dir.e[axis_num];

            let t0 = (ax.min - r.orig.e[axis_num]) * adinv;
            let t1 = (ax.max - r.orig.e[axis_num]) * adinv;

            let mut ray_t = ray_t;
            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                };
                if t1 < ray_t.max {
                    ray_t.max = t1;
                };
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                };
                if t0 < ray_t.max {
                    ray_t.max = t0;
                };
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            return if self.x.size() > self.z.size() { 0 } else { 2 };
        } else {
            return if self.y.size() > self.z.size() { 1 } else { 2 };
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Unexpected value n: {}", n),
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, offset: Vec3) -> Self::Output {
        Self {
            x: self.x + offset.e[0],
            y: self.y + offset.e[1],
            z: self.z + offset.e[2],
        }
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, aabb: Aabb) -> Self::Output {
        aabb + self
    }
}
