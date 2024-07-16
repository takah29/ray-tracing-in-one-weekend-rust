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
        let i = p.e[0].floor() as i32;
        let j = p.e[1].floor() as i32;
        let k = p.e[2].floor() as i32;
        let u = p.e[0] - i as f64;
        let v = p.e[1] - j as f64;
        let w = p.e[2] - k as f64;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        trilinear_interp(&c, u, v, w)
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

fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                    * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                    * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    accum
}
