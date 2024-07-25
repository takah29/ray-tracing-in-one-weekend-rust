use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;
use the_rest_of_your_life::{
    aarect::XzRect,
    build_scene::cornell_box,
    bvh::BvhNode,
    color,
    hittable::{HitRecord, Hittable},
    material::DiffuseLight,
    pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf},
    ray::Ray,
    rtweekend::{random, Color, INFINITY},
    texture::SolidColor,
    utils::write_ppm,
};

fn ray_color(r: Ray, background: &Color, world: &Box<dyn Hittable>, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return color!(0, 0, 0);
    }

    if !world.hit(&r, 0.001, INFINITY, &mut rec) {
        return *background;
    }

    let mut scattered = Ray::default();
    let emitted = rec
        .opt_mat_ptr
        .as_ref()
        .expect("Material not set")
        .emitted(&rec, rec.u, rec.v, &rec.p);
    let mut pdf_val = 0.0;
    let mut albedo = color!(0, 0, 0);

    if !rec.opt_mat_ptr.as_ref().expect("Material not set").scatter(
        &r,
        &rec,
        &mut albedo,
        &mut scattered,
        &mut pdf_val,
    ) {
        return emitted;
    }
    let light_shape = Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(color!(
            1, 1, 1
        ))))),
    ));

    let p0 = Arc::new(HittablePdf::new(light_shape, rec.p));
    let p1 = Arc::new(CosinePdf::new(&rec.normal));
    let mixture_pdf = MixturePdf::new(p0, p1);

    let scattered = Ray::new_with_time(rec.p, mixture_pdf.generate(), r.time);
    pdf_val = mixture_pdf.value(&scattered.dir);

    emitted
        + albedo
            * rec
                .opt_mat_ptr
                .as_ref()
                .expect("Material not set")
                .scattering_pdf(&r, &rec, &scattered)
            * ray_color(scattered, background, world, depth - 1)
            / pdf_val
}

fn main() {
    let samples_per_pixel = 1000;
    let max_depth = 20;

    let (mut hittable_list, cam, background, image_width, image_height) = cornell_box();
    // let world: Box<dyn Hittable> = Box::new(hittable_list);
    let world: Box<dyn Hittable> = Box::new(BvhNode::new_with_list(&mut hittable_list, 0.0, 1.0));

    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let pixels = (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let mut row_data = vec![color!(0, 0, 0); image_width];
            for i in 0..image_width {
                let mut pixel_color = color!(0, 0, 0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + random()) / (image_width - 1) as f64;
                    let v = (j as f64 + random()) / (image_height - 1) as f64;
                    let r = cam.get_ray(u, v);

                    pixel_color += ray_color(r, &background, &world, max_depth);
                }
                row_data[i] = pixel_color / samples_per_pixel as f64;
            }
            pb.inc(1);
            row_data
        })
        .collect();
    pb.finish();

    eprint!("Write PPM ...");
    write_ppm(pixels, image_width, image_height);
    eprintln!(" Done.");
}
