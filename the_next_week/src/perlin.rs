use crate::rtweekend::{random, random_int, Point3};

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let ranfloat = (0..Self::POINT_COUNT).map(|_| random()).collect();
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.e[0]) as i32 & 255;
        let j = (4.0 * p.e[1]) as i32 & 255;
        let k = (4.0 * p.e[2]) as i32 & 255;

        return self.ranfloat[(self.perm_x[i as usize]
            ^ self.perm_y[j as usize]
            ^ self.perm_z[k as usize]) as usize];
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = (0..Self::POINT_COUNT as i32).collect();
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut Vec<i32>) {
        for i in (1..p.len()).rev() {
            let target = random_int(0, i as i32) as usize;
            (p[i], p[target]) = (p[target], p[i]);
        }
    }
}
