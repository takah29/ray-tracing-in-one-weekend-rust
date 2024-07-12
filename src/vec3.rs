use crate::rtweekend::{random, random_range, PI};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new($x as f64, $y as f64, $z as f64)
    };
}

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr, $z:expr) => {
        Color::new($x as f64, $y as f64, $z as f64)
    };
}

#[macro_export]
macro_rules! point3 {
    ($x:expr, $y:expr, $z:expr) => {
        Point3::new($x as f64, $y as f64, $z as f64)
    };
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_range(0.0, 2.0 * PI);
    let z = random_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    return vec3!(r * a.cos(), r * a.sin(), z);
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn origin() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn random() -> Self {
        Self {
            e: [random(), random(), random()],
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            e: [
                random_range(min, max),
                random_range(min, max),
                random_range(min, max),
            ],
        }
    }

    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }
}

// ========== Neg ==========

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

// ========== Add ==========

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] + rhs, self.e[1] + rhs, self.e[2] + rhs],
        }
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self + rhs.e[0], self + rhs.e[1], self + rhs.e[2]],
        }
    }
}

// ========== Sub ==========

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] - rhs, self.e[1] - rhs, self.e[2] - rhs],
        }
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self - rhs.e[0], self - rhs.e[1], self - rhs.e[2]],
        }
    }
}

// ========== Mul ==========

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

// ========== Div ==========

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

// ========== AddAssign ==========

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}

// ========== SubAssign ==========

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.e[0] -= rhs;
        self.e[1] -= rhs;
        self.e[2] -= rhs;
    }
}

// ========== MulAssign ==========

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

// ========== DivAssign ==========

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

// ========== PartialEq ==========

// 数値計算の誤差で完全一致しない場合があるのでEPSILONの統合を許す
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.e[0] - other.e[0]).abs() < f64::EPSILON
            && (self.e[1] - other.e[1]).abs() < f64::EPSILON
            && (self.e[2] - other.e[2]).abs() < f64::EPSILON
    }
}

// ========== PartialOrd ==========

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp_x = self.e[0].partial_cmp(&other.e[0]);
        let cmp_y = self.e[1].partial_cmp(&other.e[1]);
        let cmp_z = self.e[2].partial_cmp(&other.e[2]);
        if cmp_x == Some(Ordering::Less)
            && cmp_y == Some(Ordering::Less)
            && cmp_z == Some(Ordering::Less)
        {
            Some(Ordering::Less)
        } else if cmp_x == Some(Ordering::Greater)
            && cmp_y == Some(Ordering::Greater)
            && cmp_z == Some(Ordering::Greater)
        {
            Some(Ordering::Greater)
        } else if cmp_x == Some(Ordering::Equal)
            && cmp_y == Some(Ordering::Equal)
            && cmp_z == Some(Ordering::Equal)
        {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_squared() {
        let test_cases = vec![(vec3!(1, 2, 3), 14.0), (vec3!(-1, -2, -3), 14.0)];

        for (val, expected) in test_cases {
            let result = val.length_squared();
            assert_eq!(result, expected, "Failed for input: '{:?}", val);
        }
    }

    #[test]
    fn test_length() {
        let test_cases = vec![(vec3!(2, 0, 0), 2.0), (vec3!(0, -2, 0), 2.0)];

        for (val, expected) in test_cases {
            let result = val.length();
            assert_eq!(result, expected, "Failed for input: '{:?}", val);
        }
    }

    #[test]
    fn test_cross() {
        let test_cases = vec![
            (vec3!(1, 0, 0), vec3!(0, 1, 0), vec3!(0, 0, 1)),
            (vec3!(1, 0, 0), vec3!(0, 0, 1), vec3!(0, -1, 0)),
            (vec3!(0, 1, 0), vec3!(0, 0, 1), vec3!(1, 0, 0)),
        ];

        for (val, other, expected) in test_cases {
            let result = val.cross(other);
            assert_eq!(result, expected, "Failed for input: '{:?}", (val, other));
        }
    }

    #[test]
    fn test_unit() {
        let test_cases = vec![
            (vec3!(1, 0, 0), 1.0),
            (vec3!(0, 1, 0), 1.0),
            (vec3!(0, 0, 1), 1.0),
            (vec3!(1, 1, 1), 1.0),
            (vec3!(100, 100, 100), 1.0),
        ];

        for (val, expected) in test_cases {
            let result = val.unit().length();
            assert_eq!(result, expected, "Failed for input: '{:?}", val);
        }
    }

    // ========== Neg ==========

    #[test]
    fn test_neg_vec3() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(-1, -2, -3)),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3)),
        ];

        for (val, expected) in test_cases {
            let result = -val;
            assert_eq!(result, expected, "Failed for input: '{:?}", val);
        }
    }

    // ========== Add ==========

    #[test]
    fn test_add_vec3_vec3() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(5, 7, 9)),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3), vec3!(0, 0, 0)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs + rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_add_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 1.0, vec3!(2, 3, 4)),
            (vec3!(-1, -2, -3), 1.0, vec3!(0, -1, -2)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs + rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_add_f64_vec3() {
        let test_cases = vec![
            (1.0, vec3!(1, 2, 3), vec3!(2, 3, 4)),
            (1.0, vec3!(-1, -2, -3), vec3!(0, -1, -2)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs + rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    // ========== Sub ==========

    #[test]
    fn test_sub_vec3_vec3() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(-3, -3, -3)),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3), vec3!(-2, -4, -6)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs - rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_sub_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 1.0, vec3!(0, 1, 2)),
            (vec3!(-1, -2, -3), 1.0, vec3!(-2, -3, -4)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs - rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_sub_f64_vec3() {
        let test_cases = vec![
            (1.0, vec3!(1, 2, 3), vec3!(0, -1, -2)),
            (1.0, vec3!(-1, -2, -3), vec3!(2, 3, 4)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs - rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    // ========== Mul ==========

    #[test]
    fn test_mul_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 2.0, vec3!(2, 4, 6)),
            (vec3!(-1, -2, -3), 2.0, vec3!(-2, -4, -6)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs * rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_mul_f64_vec3() {
        let test_cases = vec![
            (2.0, vec3!(1, 2, 3), vec3!(2, 4, 6)),
            (2.0, vec3!(-1, -2, -3), vec3!(-2, -4, -6)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs * rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_mul_vec3_vec3() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(4, 10, 18)),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3), vec3!(-1, -4, -9)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs * rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    // ========== Div ==========

    #[test]
    fn test_div_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 2.0, vec3!(0.5, 1.0, 1.5)),
            (vec3!(-1, -2, -3), 2.0, vec3!(-0.5, -1.0, -1.5)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs / rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    // ========== AddAssign ==========

    #[test]
    fn test_add_assign_vec3_vec3() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(5, 7, 9)),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3), vec3!(0, 0, 0)),
        ];

        for (mut lhs, rhs, expected) in test_cases {
            let original_lhs = lhs.clone();
            lhs += rhs;
            assert_eq!(
                lhs,
                expected,
                "Failed for input: '{:?}",
                (original_lhs, rhs)
            );
        }
    }

    #[test]
    fn test_add_assign_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 1.0, vec3!(2, 3, 4)),
            (vec3!(-1, -2, -3), 1.0, vec3!(0, -1, -2)),
        ];

        for (mut lhs, rhs, expected) in test_cases {
            let original_lhs = lhs.clone();
            lhs += rhs;
            assert_eq!(
                lhs,
                expected,
                "Failed for input: '{:?}",
                (original_lhs, rhs)
            );
        }
    }

    // ========== SubAssign ==========

    #[test]
    fn test_sub_assign_vec3_vec3() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(-3, -3, -3)),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3), vec3!(-2, -4, -6)),
        ];

        for (mut lhs, rhs, expected) in test_cases {
            let original_lhs = lhs.clone();
            lhs -= rhs;
            assert_eq!(
                lhs,
                expected,
                "Failed for input: '{:?}",
                (original_lhs, rhs)
            );
        }
    }

    #[test]
    fn test_sub_assign_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 1.0, vec3!(0, 1, 2)),
            (vec3!(-1, -2, -3), 1.0, vec3!(-2, -3, -4)),
        ];

        for (mut lhs, rhs, expected) in test_cases {
            let original_lhs = lhs.clone();
            lhs -= rhs;
            assert_eq!(
                lhs,
                expected,
                "Failed for input: '{:?}",
                (original_lhs, rhs)
            );
        }
    }

    // ========== MulAssign ==========

    #[test]
    fn test_mul_assign_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 2.0, vec3!(2, 4, 6)),
            (vec3!(-1, -2, -3), 2.0, vec3!(-2, -4, -6)),
        ];

        for (mut lhs, rhs, expected) in test_cases {
            let original_lhs = lhs.clone();
            lhs *= rhs;
            assert_eq!(
                lhs,
                expected,
                "Failed for input: '{:?}",
                (original_lhs, rhs)
            );
        }
    }

    // ========== DivAssign ==========

    #[test]
    fn test_div_assing_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 2.0, vec3!(0.5, 1.0, 1.5)),
            (vec3!(-1, -2, -3), 2.0, vec3!(-0.5, -1.0, -1.5)),
        ];

        for (mut lhs, rhs, expected) in test_cases {
            let original_lhs = lhs.clone();
            lhs /= rhs;
            assert_eq!(
                lhs,
                expected,
                "Failed for input: '{:?}",
                (original_lhs, rhs)
            );
        }
    }

    // ========== PartialEq ==========

    #[test]
    fn test_partial_eq() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(1, 2, 3), true),
            (vec3!(1, 2, 3), vec3!(1, 2, 4), false),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs == rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    // ========== PartialOrd ==========

    #[test]
    fn test_partial_cmp() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(2, 3, 4), Some(Ordering::Less)),
            (vec3!(1, 2, 3), vec3!(0, 1, 2), Some(Ordering::Greater)),
            (vec3!(1, 2, 3), vec3!(1, 2, 3), Some(Ordering::Equal)),
            (vec3!(1, 2, 3), vec3!(1, 3, 2), None),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs.partial_cmp(&rhs);
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_less_than() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(2, 3, 4), true),
            (vec3!(1, 2, 3), vec3!(0, 1, 2), false),
            (vec3!(1, 2, 3), vec3!(1, 2, 3), false),
            (vec3!(1, 2, 3), vec3!(1, 3, 2), false),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs < rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_less_eq() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(2, 3, 4), true),
            (vec3!(1, 2, 3), vec3!(0, 1, 2), false),
            (vec3!(1, 2, 3), vec3!(1, 2, 3), true),
            (vec3!(1, 2, 3), vec3!(1, 3, 2), false),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs <= rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_greater_than() {
        let test_cases = vec![
            (vec3!(2, 3, 4), vec3!(1, 2, 3), true),
            (vec3!(0, 1, 2), vec3!(1, 2, 3), false),
            (vec3!(1, 2, 3), vec3!(1, 2, 3), false),
            (vec3!(1, 3, 2), vec3!(1, 2, 3), false),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs > rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }

    #[test]
    fn test_greater_eq() {
        let test_cases = vec![
            (vec3!(2, 3, 4), vec3!(1, 2, 3), true),
            (vec3!(0, 1, 2), vec3!(1, 2, 3), false),
            (vec3!(1, 2, 3), vec3!(1, 2, 3), true),
            (vec3!(1, 3, 2), vec3!(1, 2, 3), false),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs >= rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs));
        }
    }
}
