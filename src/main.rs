mod hittable;
mod ray;
mod utils;
mod vec3;

use ray::Ray;
use utils::write_color;
use vec3::{Color, Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir.length_squared();
    let half_b = oc * r.dir;
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(point3!(0, 0, -1), 0.5, r.clone());
    if t > 0.0 {
        let normal = (r.at(t) - vec3!(0, 0, -1)).unit();
        return 0.5 * color!(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
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

    for j in (0..image_height).rev() {
        eprint!("\rScanline remaining: {:3}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}
