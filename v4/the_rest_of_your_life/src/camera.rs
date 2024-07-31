use crate::{
    hittable::HitRecord,
    hittable::Hittable,
    interval::Interval,
    material::ScatterRecord,
    pdf::{HittablePdf, MixturePdf, Pdf},
    rtweekend::{random, Color, Point3, Ray, Vec3, INFINITY},
    utils::write_ppm,
    vec3::random_in_unit_disk,
    {color, vec3},
};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;

pub struct Camera {
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub image_width: usize,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub background: Color,
    pub vfov: f64,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub vup: Vec3,

    image_height: usize,
    pixel_samples_scale: f64,
    sqrt_spp: u32,
    recip_sqrt_spp: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        image_width: usize,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_depth: u32,
        background: Color,
        vfov: f64,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = image_width as f64 / aspect_ratio;
        let image_height = if image_height < 1.0 {
            1
        } else {
            image_height as usize
        };

        let sqrt_spp = (samples_per_pixel as f64).sqrt() as u32;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let recip_sqrt_spp = 1.0 / sqrt_spp as f64;

        let center = lookfrom;

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let vup = vec3!(0, 1, 0);
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            lookfrom,
            lookat,
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            background,
            vfov,
            defocus_angle,
            focus_dist,
            vup,
            image_height,
            pixel_samples_scale,
            sqrt_spp,
            recip_sqrt_spp,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(
        &self,
        world: &Box<dyn Hittable>,
        lights: &Arc<dyn Hittable>,
        direct_light_sampling: bool,
    ) {
        let pb = ProgressBar::new(self.image_height as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        let pixels = (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                let mut row_data = vec![color!(0, 0, 0); self.image_width];
                for i in 0..self.image_width {
                    let mut pixel_color = color!(0, 0, 0);
                    for s_j in 0..self.sqrt_spp {
                        for s_i in 0..self.sqrt_spp {
                            let r = self.get_ray(i, j, s_i, s_j);

                            pixel_color += self.ray_color(
                                r,
                                &world,
                                &lights,
                                direct_light_sampling,
                                self.max_depth,
                            );
                        }
                    }
                    row_data[i] = self.pixel_samples_scale * pixel_color;
                }
                pb.inc(1);
                row_data
            })
            .collect();
        pb.finish();

        eprint!("Write PPM ...");
        write_ppm(pixels, self.image_width, self.image_height);
        eprintln!(" Done.");
    }

    pub fn get_ray(&self, i: usize, j: usize, s_i: u32, s_j: u32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.e[0]) * self.pixel_delta_u)
            + ((j as f64 + offset.e[1]) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vec3 {
        let px = ((s_i as f64 + random()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random()) * self.recip_sqrt_spp) - 0.5;

        vec3!(px, py, 0)
    }

    #[allow(dead_code)]
    fn sample_square() -> Vec3 {
        vec3!(random() - 0.5, random() - 0.5, 0.0)
    }

    #[allow(dead_code)]
    fn sample_disk(radius: f64) -> Vec3 {
        radius * random_in_unit_disk()
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.e[0] * self.defocus_disk_u) + (p.e[1] * self.defocus_disk_v)
    }

    fn ray_color(
        &self,
        r: Ray,
        world: &Box<dyn Hittable>,
        lights: &Arc<dyn Hittable>,
        direct_light_sampling: bool,
        depth: u32,
    ) -> Color {
        if depth <= 0 {
            return color!(0, 0, 0);
        }

        let mut rec = HitRecord::default();
        if !world.hit(&r, Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }

        let mut srec = ScatterRecord::default();
        let color_from_emission = rec
            .opt_mat_ptr
            .as_ref()
            .expect("Material not set")
            .emitted(&rec, rec.u, rec.v, &rec.p);

        if !rec
            .opt_mat_ptr
            .as_ref()
            .expect("Material not set")
            .scatter(&r, &rec, &mut srec)
        {
            return color_from_emission;
        }

        if srec.skip_pdf {
            return srec.attenuation
                * self.ray_color(
                    srec.skip_pdf_ray,
                    world,
                    lights,
                    direct_light_sampling,
                    depth - 1,
                );
        }

        let p: Arc<dyn Pdf> = if direct_light_sampling {
            let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.p));
            Arc::new(MixturePdf::new(
                light_ptr,
                srec.opt_pdf_ptr.expect("PDF not set"),
            ))
        } else {
            srec.opt_pdf_ptr.expect("PDF not set")
        };

        let scattered = Ray::new_with_time(rec.p, p.generate(), r.time);
        let pdf_value = p.value(&scattered.dir);

        let scattering_pdf = rec
            .opt_mat_ptr
            .as_ref()
            .expect("Material not set")
            .scattering_pdf(&r, &rec, &scattered);

        let sample_color =
            self.ray_color(scattered, world, lights, direct_light_sampling, depth - 1);
        let color_from_scatter = (srec.attenuation * scattering_pdf * sample_color) / pdf_value;

        color_from_emission + color_from_scatter
    }
}
