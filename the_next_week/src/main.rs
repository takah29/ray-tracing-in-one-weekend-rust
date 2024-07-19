use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use the_next_week::{
    build_scene::cornell_box,
    bvh::BvhNode,
    camera::Camera,
    color,
    hittable::{HitRecord, Hittable},
    point3,
    ray::Ray,
    rtweekend::{random, Color, Point3, Vec3, INFINITY},
    utils::write_ppm,
    vec3,
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
    let mut attenuation = Color::default();
    let emitted = rec
        .opt_mat_ptr
        .as_ref()
        .expect("Material not set")
        .emitted(rec.u, rec.v, &rec.p);

    if !rec.opt_mat_ptr.as_ref().expect("Material not set").scatter(
        &r,
        &rec,
        &mut attenuation,
        &mut scattered,
    ) {
        return emitted;
    }

    emitted + attenuation * ray_color(scattered, background, world, depth - 1)
}

fn main() {
    let aspect_ratio = 1.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let background = color!(0, 0, 0);

    let world: Box<dyn Hittable> = Box::new(BvhNode::new_with_list(&mut cornell_box(), 0.0, 1.0));

    let lookfrom = point3!(278, 278, -800);
    let lookat = point3!(278, 278, 0);
    let vup = vec3!(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

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
