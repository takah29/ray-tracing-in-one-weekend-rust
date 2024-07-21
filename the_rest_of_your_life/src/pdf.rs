use crate::{
    onb::Onb,
    rtweekend::{Vec3, PI},
    vec3::random_cosine_direction,
};

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
