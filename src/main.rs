use ray_tracing_in_one_weekend::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    rtweekend::INFINITY,
    sphere::Sphere,
    utils::write_color,
    vec3::{Color, Point3, Vec3},
    {color, point3, vec3},
};

fn ray_color(r: Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + color!(1, 1, 1));
    }

    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * color!(1, 1, 1) + t * color!(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    println!("P3\n{} {}\n255", image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3!(0, 0, 0);
    let horizontal = vec3!(viewport_width, 0, 0);
    let vertical = vec3!(0, viewport_height, 0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3!(0, 0, focal_length);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(point3!(0, 0, -1), 0.5)));
    world.add(Box::new(Sphere::new(point3!(0, -100.5, -1), 100.0)));

    for j in (0..image_height).rev() {
        eprint!("\rScanline remaining: {:3}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r, &world);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}
