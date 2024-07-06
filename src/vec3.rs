use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new($x as f64, $y as f64, $z as f64)
    };
}

type Point3 = Vec3;
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
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
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

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

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

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < f64::EPSILON
            && (self.z - other.z).abs() < f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec3() {
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(5, 7, 9)),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3), vec3!(0, 0, 0)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs + rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs),);
        }
    }

    #[test]
    fn test_add_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 1.0, vec3!(2, 3, 4)),
            (vec3!(-1, -2, -3), 1.0, vec3!(0, -1, -2)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs + rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs),);
        }
    }

    #[test]
    fn test_sub_vec3() {
        // (lhs, rhs, expected)
        let test_cases = vec![
            (vec3!(1, 2, 3), vec3!(4, 5, 6), vec3!(-3, -3, -3)),
            (vec3!(-1, -2, -3), vec3!(1, 2, 3), vec3!(-2, -4, -6)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs - rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs),);
        }
    }

    #[test]
    fn test_sub_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 1.0, vec3!(0, 1, 2)),
            (vec3!(-1, -2, -3), 1.0, vec3!(-2, -3, -4)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs - rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs),);
        }
    }

    #[test]
    fn test_mul_vec3_f64() {
        let test_cases = vec![
            (vec3!(1, 2, 3), 2.0, vec3!(2, 4, 6)),
            (vec3!(-1, -2, -3), 2.0, vec3!(-2, -4, -6)),
        ];

        for (lhs, rhs, expected) in test_cases {
            let result = lhs * rhs;
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs),);
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
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs),);
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
            assert_eq!(result, expected, "Failed for input: '{:?}", (lhs, rhs),);
        }
    }
}
