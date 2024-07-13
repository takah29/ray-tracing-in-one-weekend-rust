use crate::vec3::{Point3, Vec3};

#[derive(Default, Debug, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point3, vec3};

    #[test]
    fn test_ray() {
        let ray = Ray::new(point3!(1, 1, 1), vec3!(1, 0, 0));
        let point = ray.at(2.0);
        assert_eq!(point, point3!(3, 1, 1));
    }
}
