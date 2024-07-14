pub use crate::ray::Ray;
pub use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;
use std::cell::RefCell;
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

thread_local! {
    static RNG: RefCell<rand::rngs::ThreadRng> = RefCell::new(rand::thread_rng());
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random() -> f64 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

pub fn random_range(min: f64, max: f64) -> f64 {
    RNG.with(|rng| rng.borrow_mut().gen_range(min..max))
}

pub fn random_int(min: i32, max: i32) -> i32 {
    RNG.with(|rng| rng.borrow_mut().gen_range(min..=max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degrees_to_radians() {
        let test_cases = vec![(0.0, 0.0), (90.0, PI / 2.0), (180.0, PI)];

        for (input, expected) in test_cases {
            let result = degrees_to_radians(input);
            assert_eq!(result, expected, "Failed for input: '{}", input);
        }
    }
}
