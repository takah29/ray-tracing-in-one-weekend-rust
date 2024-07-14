use std::rc::Rc;
use the_next_week::{
    bvh::BvhNode,
    camera::Camera,
    color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    moving_sphere::MovingSphere,
    point3,
    ray::Ray,
    rtweekend::{random, random_range, Color, Point3, Vec3, INFINITY},
    sphere::Sphere,
    utils::write_color,
    vec3,
};

fn ray_color(r: Ray, world: &Box<dyn Hittable>, depth: i32) -> Color {
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(&color!(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        point3!(0, -1000, 0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = point3!(a as f64 + 0.9 * random(), 0.2, b as f64 + random());

            if (center - vec3!(4, 0.2, 0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(&albedo));
                    let center2 = center + vec3!(0, random_range(0.0, 0.5), 0);
                    world.add(Rc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random();
                    let sphere_material = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // grass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                };
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(point3!(0, 1, 0), 1.0, material1)));
    let material2 = Rc::new(Lambertian::new(&color!(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(point3!(-4, 1, 0), 1.0, material2)));
    let material3 = Rc::new(Metal::new(&color!(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(point3!(4, 1, 0), 1.0, material3)));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("P3\n{} {}\n255", image_width, image_height);

    let world: Box<dyn Hittable> = Box::new(BvhNode::new_with_list(&mut random_scene(), 0.0, 1.0));
    // let world: Box<dyn Hittable> = Box::new(random_scene());

    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let vup = vec3!(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

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
