use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;
use the_rest_of_your_life::{
    aarect::XzRect,
    build_scene::cornell_box,
    bvh::BvhNode,
    color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::{DiffuseLight, ScatterRecord},
    pdf::{HittablePdf, MixturePdf, Pdf},
    point3,
    ray::Ray,
    rtweekend::{random, Color, Point3, INFINITY},
    sphere::Sphere,
    texture::SolidColor,
    utils::write_ppm,
};

fn ray_color(
    r: Ray,
    background: &Color,
    world: &Box<dyn Hittable>,
    lights: &Arc<dyn Hittable>,
    depth: i32,
) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return color!(0, 0, 0);
    }

    if !world.hit(&r, 0.001, INFINITY, &mut rec) {
        return *background;
    }

    let mut srec = ScatterRecord::default();
    let emitted = rec
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
        return emitted;
    }

    if srec.is_specular {
        return srec.attenuation
            * ray_color(srec.specular_ray, background, world, lights, depth - 1);
    }

    let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.p));
    let p = MixturePdf::new(light_ptr, srec.opt_pdf_ptr.expect("PDF not set"));

    let scattered = Ray::new_with_time(rec.p, p.generate(), r.time);
    let pdf_val = p.value(&scattered.dir);

    emitted
        + srec.attenuation
            * rec
                .opt_mat_ptr
                .as_ref()
                .expect("Material not set")
                .scattering_pdf(&r, &rec, &scattered)
            * ray_color(scattered, background, world, lights, depth - 1)
            / pdf_val
}

fn main() {
    let samples_per_pixel = 1000;
    let max_depth = 20;

    let (mut hittable_list, cam, background, image_width, image_height) = cornell_box();
    // let world: Box<dyn Hittable> = Box::new(hittable_list);
    let world: Box<dyn Hittable> = Box::new(BvhNode::new_with_list(&mut hittable_list, 0.0, 1.0));

    let mut lights = HittableList::new();
    lights.add(Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(color!(
            0, 0, 0
        ))))),
    )));
    lights.add(Arc::new(Sphere::new(
        point3!(190, 90, 190),
        90.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(color!(
            0, 0, 0
        ))))),
    )));
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

                    pixel_color += ray_color(r, &background, &world, &lights, max_depth);
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
