use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;
use the_rest_of_your_life::{
    build_scene::two_perlin_spheres,
    bvh::BvhNode,
    color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::ScatterRecord,
    pdf::{HittablePdf, MixturePdf, Pdf},
    ray::Ray,
    rtweekend::{random, Color, INFINITY},
    utils::write_ppm,
};

fn ray_color(
    r: Ray,
    background: &Color,
    world: &Box<dyn Hittable>,
    lights: &Arc<dyn Hittable>,
    direct_light_sampling: bool,
    depth: i32,
) -> Color {
    if depth <= 0 {
        return color!(0, 0, 0);
    }

    let mut rec = HitRecord::default();
    if !world.hit(&r, Interval::new(0.001, INFINITY), &mut rec) {
        return *background;
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
            * ray_color(
                srec.skip_pdf_ray,
                background,
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

    let sample_color = ray_color(
        scattered,
        background,
        world,
        lights,
        direct_light_sampling,
        depth - 1,
    );
    let color_from_scatter = (srec.attenuation * scattering_pdf * sample_color) / pdf_value;

    color_from_emission + color_from_scatter
}

fn main() {
    let samples_per_pixel = 100;
    let max_depth = 20;

    let (
        mut hittable_list,
        lights,
        direct_light_sampling,
        cam,
        background,
        image_width,
        image_height,
    ) = two_perlin_spheres();
    // let world: Box<dyn Hittable> = Box::new(hittable_list);
    let world: Box<dyn Hittable> = Box::new(BvhNode::new_with_list(&mut hittable_list, 0.0, 1.0));
    let lights: Arc<dyn Hittable> = Arc::new(lights);

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

                    pixel_color += ray_color(
                        r,
                        &background,
                        &world,
                        &lights,
                        direct_light_sampling,
                        max_depth,
                    );
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
