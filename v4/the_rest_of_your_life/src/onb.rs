use crate::{rtweekend::Vec3, vec3};
use std::ops::{Index, IndexMut};

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new() -> Self {
        Self {
            axis: [vec3!(1, 0, 0), vec3!(0, 1, 0), vec3!(0, 0, 1)],
        }
    }

    pub fn build_from_w(n: Vec3) -> Self {
        let w = n.unit();
        let a = if w.e[0].abs() > 0.9 {
            vec3!(0, 1, 0)
        } else {
            vec3!(1, 0, 0)
        };
        let v = w.cross(a).unit();
        let u = w.cross(v);

        Self { axis: [u, v, w] }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn transform(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self[0] + b * self[1] + c * self[2]
    }

    pub fn transform_vec3(&self, v: Vec3) -> Vec3 {
        v.e[0] * self[0] + v.e[1] * self[1] + v.e[2] * self[2]
    }
}

impl Index<usize> for Onb {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}

impl IndexMut<usize> for Onb {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.axis[index]
    }
}
