use crate::{
    color,
    hittable::HitRecord,
    pdf::{CosinePdf, Pdf, SpherePdf},
    rtweekend::{random, Color, Point3, Ray, PI},
    texture::Texture,
    vec3::{random_in_unit_sphere, reflect, refract},
};
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub opt_pdf_ptr: Option<Arc<dyn Pdf>>,
}

pub trait Material: Sync + Send {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        color!(0, 0, 0)
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = false;
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.opt_pdf_ptr = Some(Arc::new(CosinePdf::new(&rec.normal)));
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.normal.dot(scattered.dir.unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let reflected = reflect(&r_in.dir.unit(), &rec.normal);
        srec.specular_ray = Ray::new_with_time(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );
        srec.attenuation = self.albedo;
        srec.is_specular = true;
        srec.opt_pdf_ptr = None;
        true
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = true;
        srec.opt_pdf_ptr = None;
        srec.attenuation = color!(1, 1, 1);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.dir.unit();
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &rec.normal);
            srec.specular_ray = Ray::new_with_time(rec.p, reflected, r_in.time);
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random() < reflect_prob {
            let reflected = reflect(&unit_direction, &rec.normal);
            srec.specular_ray = Ray::new_with_time(rec.p, reflected, r_in.time);
            return true;
        }
        let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
        srec.specular_ray = Ray::new_with_time(rec.p, refracted, r_in.time);
        true
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            color!(0, 0, 0)
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.opt_pdf_ptr = Some(Arc::new(SpherePdf));
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
