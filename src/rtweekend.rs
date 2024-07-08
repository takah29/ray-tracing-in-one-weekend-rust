pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
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
