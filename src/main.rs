use ray_tracing_in_one_weekend::{
    camera::Camera,
    color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::Lambertian,
    point3,
    ray::Ray,
    rtweekend::{random, Color, Point3, INFINITY, PI},
    sphere::Sphere,
    utils::write_color,
};
use std::rc::Rc;

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return color!(0, 0, 0);
    }

    if world.hit(&r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.opt_mat_ptr.as_ref().expect("Material not set").scatter(
            &r,
            &rec,
            &mut attenuation,
            &mut scattered,
        ) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return color!(0, 0, 0);
    }

    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.e[1] + 1.0);
    (1.0 - t) * color!(1, 1, 1) + t * color!(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("P3\n{} {}\n255", image_width, image_height);

    let mut world = HittableList::new();

    let r = (PI / 4.0).cos();
    world.add(Box::new(Sphere::new(
        point3!(-r, 0, -1),
        r,
        Rc::new(Lambertian::new(&color!(0, 0, 1))),
    )));
    world.add(Box::new(Sphere::new(
        point3!(r, 0, -1),
        r,
        Rc::new(Lambertian::new(&color!(1, 0, 0))),
    )));

    let cam = Camera::new(90.0, image_width as f64 / image_height as f64);

    for j in (0..image_height).rev() {
        eprint!("\rScanline remaining: {:3}", j);
        for i in 0..image_width {
            let mut pixel_color = color!(0, 0, 0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / (image_width - 1) as f64;
                let v = (j as f64 + random()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);

                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone");
}
