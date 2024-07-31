use crate::{
    hittable::Hittable,
    onb::Onb,
    rtweekend::{random, Point3, Vec3, PI},
    vec3::{random_cosine_direction, random_unit_vector},
};
use std::sync::Arc;

pub trait Pdf: Sync + Send {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(*w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.unit().dot(self.uvw.w());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform_vec3(random_cosine_direction())
    }
}

pub struct HittablePdf {
    obj_ptr: Arc<dyn Hittable>,
    origin: Point3,
}

impl HittablePdf {
    pub fn new(obj_ptr: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self { obj_ptr, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.obj_ptr.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.obj_ptr.random(&self.origin)
    }
}

pub struct MixturePdf {
    p: [Arc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
