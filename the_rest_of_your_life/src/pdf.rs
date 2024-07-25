use crate::{
    hittable::Hittable,
    onb::Onb,
    rtweekend::{Point3, Vec3, PI},
    vec3::random_cosine_direction,
};
use std::sync::Arc;

pub trait Pdf: Sync + Send {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
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
        self.uvw.local_vec3(random_cosine_direction())
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
