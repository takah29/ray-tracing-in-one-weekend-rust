use crate::{
    aarect::{XyRect, XzRect, YzRect},
    cuboid::Cuboid,
    hittable::{RotateY, Translate},
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    moving_sphere::MovingSphere,
    rtweekend::{random, random_range, Color, Point3, Vec3},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor},
    {color, point3, vec3},
};
use std::path::Path;
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Arc::new(SolidColor::new(color!(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(color!(0.9, 0.9, 0.9))),
    ));
    let ground_material = Arc::new(Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
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
                        Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo))));
                    let center2 = center + vec3!(0, random_range(0.0, 0.5), 0);
                    world.add(Arc::new(MovingSphere::new(
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
                    let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // grass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                };
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(point3!(0, 1, 0), 1.0, material1)));
    let material2 = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.4, 0.2, 0.1
    )))));
    world.add(Arc::new(Sphere::new(point3!(-4, 1, 0), 1.0, material2)));
    let material3 = Arc::new(Metal::new(&color!(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(point3!(4, 1, 0), 1.0, material3)));

    world
}

pub fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Arc::new(SolidColor::new(color!(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(color!(0.9, 0.9, 0.9))),
    ));

    world.add(Arc::new(Sphere::new(
        point3!(0, -10, 0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        point3!(0, 10, 0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));

    world
}

pub fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(5.0));

    world.add(Arc::new(Sphere::new(
        point3!(0, -1000, 0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        point3!(0, 2, 0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    world
}

pub fn earth() -> HittableList {
    let image_path = Path::new("./data/earthmap.jpg");
    let earth_texture = Arc::new(ImageTexture::new(image_path));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(point3!(0, 0, 0), 2.0, earth_surface));
    return HittableList::new_with_object(globe);
}

pub fn simple_light() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        point3!(0, -1000, 0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        point3!(0, 2, 0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(color!(
        4, 4, 4
    )))));

    world.add(Arc::new(Sphere::new(
        point3!(0, 7, 0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(XyRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));

    world
}

pub fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.65, 0.05, 0.05
    )))));
    let white = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.73, 0.73, 0.73
    )))));
    let green = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.12, 0.45, 0.15
    )))));
    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(color!(
        15, 15, 15
    )))));

    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(Cuboid::new(
        point3!(0, 0, 0),
        point3!(165, 330, 165),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, vec3!(265, 0, 295)));
    world.add(box1);

    let box2 = Arc::new(Cuboid::new(
        point3!(0, 0, 0),
        point3!(165, 165, 165),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, vec3!(130, 0, 65)));
    world.add(box2);

    world
}
