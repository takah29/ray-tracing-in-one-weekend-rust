use crate::{
    color,
    hittable::HitRecord,
    onb::Onb,
    rtweekend::{random, Color, Point3, Ray, PI},
    texture::Texture,
    vec3::{random_cosine_direction, random_in_unit_sphere, reflect, refract},
};
use std::sync::Arc;

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _albedo: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        albedo: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let uvw = Onb::build_from_w(rec.normal);
        let direction = uvw.local_vec3(random_cosine_direction());
        *scattered = Ray::new_with_time(rec.p, direction.unit(), r_in.time);
        *albedo = self.albedo.value(rec.u, rec.v, &rec.p);
        *pdf = uvw.w().dot(scattered.dir) / PI;
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        albedo: &mut Color,
        scattered: &mut Ray,
        _: &mut f64,
    ) -> bool {
        let reflected = reflect(&r_in.dir.unit(), &rec.normal);
        *scattered = Ray::new_with_time(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );
        *albedo = self.albedo;
        scattered.dir.dot(rec.normal) > 0.0
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        albedo: &mut Color,
        scattered: &mut Ray,
        _: &mut f64,
    ) -> bool {
        *albedo = color!(1, 1, 1);
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
            *scattered = Ray::new_with_time(rec.p, reflected, r_in.time);
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random() < reflect_prob {
            let reflected = reflect(&unit_direction, &rec.normal);
            *scattered = Ray::new_with_time(rec.p, reflected, r_in.time);
            return true;
        }
        let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
        *scattered = Ray::new_with_time(rec.p, refracted, r_in.time);
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
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _albedo: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        albedo: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *scattered = Ray::new_with_time(rec.p, random_in_unit_sphere(), r_in.time);
        *albedo = self.albedo.value(rec.u, rec.v, &rec.p);

        true
    }
}
