use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new($x as f64, $y as f64, $z as f64)
    };
}

#[allow(dead_code)]
type Point3 = Vec3;
#[allow(dead_code)]
type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn origin() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        self * other
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
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
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// ========== Add ==========

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

// ========== Sub ==========

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

// ========== Mul ==========

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

// Inner Product
impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// ========== Div ==========

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// ========== AddAssign ==========

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

// ========== SubAssign ==========

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

// ========== MulAssign ==========

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
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
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < f64::EPSILON
            && (self.z - other.z).abs() < f64::EPSILON
    }
}

// ========== PartialOrd ==========

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp_x = self.x.partial_cmp(&other.x);
        let cmp_y = self.y.partial_cmp(&other.y);
        let cmp_z = self.z.partial_cmp(&other.z);
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
            (vec3!(1, 2, 3), vec3!(4, 5, 6), 32.0),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3), -14.0),
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
