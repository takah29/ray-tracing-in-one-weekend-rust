use crate::color;
use crate::hittable::HitRecord;
use crate::rtweekend::{random, Color, Ray};
use crate::texture::Texture;
use crate::vec3::{random_in_unit_sphere, random_unit_vector, reflect, refract};
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + random_unit_vector();
        *scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time);
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
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
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.dir.unit(), &rec.normal);
        *scattered = Ray::new_with_time(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );
        *attenuation = self.albedo;
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
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = color!(1, 1, 1);
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

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
