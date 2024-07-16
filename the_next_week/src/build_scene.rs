use crate::{
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    moving_sphere::MovingSphere,
    rtweekend::{random, random_range, Color, Point3, Vec3},
    sphere::Sphere,
    texture::{CheckerTexture, NoiseTexture, SolidColor},
    {color, point3, vec3},
};
use std::rc::Rc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new(
        Rc::new(SolidColor::new(color!(0.2, 0.3, 0.1))),
        Rc::new(SolidColor::new(color!(0.9, 0.9, 0.9))),
    ));
    let ground_material = Rc::new(Lambertian::new(checker));
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
                    let sphere_material =
                        Rc::new(Lambertian::new(Rc::new(SolidColor::new(albedo))));
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
    let material2 = Rc::new(Lambertian::new(Rc::new(SolidColor::new(color!(
        0.4, 0.2, 0.1
    )))));
    world.add(Rc::new(Sphere::new(point3!(-4, 1, 0), 1.0, material2)));
    let material3 = Rc::new(Metal::new(&color!(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(point3!(4, 1, 0), 1.0, material3)));

    world
}

pub fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new(
        Rc::new(SolidColor::new(color!(0.2, 0.3, 0.1))),
        Rc::new(SolidColor::new(color!(0.9, 0.9, 0.9))),
    ));

    world.add(Rc::new(Sphere::new(
        point3!(0, -10, 0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        point3!(0, 10, 0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    )));

    world
}

pub fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new(5.0));

    world.add(Rc::new(Sphere::new(
        point3!(0, -1000, 0),
        1000.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        point3!(0, 2, 0),
        2.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));

    world
}
