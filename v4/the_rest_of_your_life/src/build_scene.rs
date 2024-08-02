use crate::{
    bvh::BvhNode,
    camera::Camera,
    constant_medium::ConstantMedium,
    hittable::{RotateY, Translate},
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, EmptyMaterial, Lambertian, Metal},
    quad::{create_box, Quad},
    rtweekend::{random, random_range, Color, Point3, Vec3},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor},
    {color, point3, vec3},
};
use std::path::Path;
use std::sync::Arc;

pub fn minimal_scene() -> (HittableList, HittableList, Camera, bool) {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        0.32,
        Arc::new(SolidColor::new(color!(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(color!(0.9, 0.9, 0.9))),
    ));
    let ground_material = Arc::new(Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
        point3!(0, -1000, 0),
        1000.0,
        ground_material,
    )));

    let material = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.4, 0.2, 0.1
    )))));
    world.add(Arc::new(Sphere::new(point3!(-4, 1, 0), 1.0, material)));

    // ライトの設定
    let lights = HittableList::new();
    let direct_light_sampling = lights.objects.len() != 0; // 指定した光源の直接サンプリングを有効にする

    // カメラの設定
    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0.7, 0.8, 1);
    let vfov = 20.0;
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}

pub fn random_scene() -> (HittableList, HittableList, Camera, bool) {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        0.32,
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
                    world.add(Arc::new(Sphere::new_with_moving_sphere(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random();
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
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
    let material3 = Arc::new(Metal::new(color!(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(point3!(4, 1, 0), 1.0, material3)));

    // ライトの設定
    let lights = HittableList::new();
    let direct_light_sampling = lights.objects.len() != 0;

    // カメラの設定
    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0.7, 0.8, 1);
    let vfov = 20.0;
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}

pub fn two_spheres() -> (HittableList, HittableList, Camera, bool) {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        0.32,
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

    // ライトの設定
    let lights = HittableList::new();
    let direct_light_sampling = lights.objects.len() != 0;

    // カメラの設定
    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0.7, 0.8, 1);
    let vfov = 20.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}

pub fn two_perlin_spheres() -> (HittableList, HittableList, Camera, bool) {
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

    // ライトの設定
    let lights = HittableList::new();
    let direct_light_sampling = lights.objects.len() != 0;

    // カメラの設定
    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0.7, 0.8, 1);
    let vfov = 20.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}

pub fn earth() -> (HittableList, HittableList, Camera, bool) {
    let image_path = Path::new("./data/earthmap.jpg");
    let earth_texture = Arc::new(ImageTexture::new(image_path));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(point3!(0, 0, 0), 2.0, earth_surface));

    let world = HittableList::new_with_object(globe);

    // ライトの設定
    let lights = HittableList::new();
    let direct_light_sampling = lights.objects.len() != 0;

    // カメラの設定
    let lookfrom = point3!(0, 0, 12);
    let lookat = point3!(0, 0, 0);
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0.7, 0.8, 1);
    let vfov = 20.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}

pub fn simple_light() -> (HittableList, HittableList, Camera, bool) {
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
    world.add(Arc::new(Quad::new(
        point3!(3, 1, -2),
        vec3!(2, 0, 0),
        vec3!(0, 2, 0),
        difflight,
    )));

    // ライトの設定
    let lights = HittableList::new();
    let direct_light_sampling = lights.objects.len() != 0;

    // カメラの設定
    let lookfrom = point3!(26, 3, 6);
    let lookat = point3!(0, 2, 0);
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0, 0, 0);
    let vfov = 20.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}

pub fn cornell_box() -> (HittableList, HittableList, Camera, bool) {
    // オブジェクトの設定
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

    // Cornell box sides
    world.add(Arc::new(Quad::new(
        point3!(555, 0, 0),
        vec3!(0, 0, 555),
        vec3!(0, 555, 0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        point3!(0, 0, 555),
        vec3!(0, 0, -555),
        vec3!(0, 555, 0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        point3!(0, 555, 0),
        vec3!(555, 0, 0),
        vec3!(0, 0, 555),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        point3!(0, 0, 555),
        vec3!(555, 0, 0),
        vec3!(0, 0, -555),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        point3!(555, 0, 555),
        vec3!(-555, 0, 0),
        vec3!(0, 555, 0),
        white.clone(),
    )));

    // Light
    world.add(Arc::new(Quad::new(
        point3!(213, 554, 227),
        vec3!(130, 0, 0),
        vec3!(0, 0, 105),
        light,
    )));

    // Box
    let box1 = Arc::new(create_box(
        point3!(0, 0, 0),
        point3!(165, 330, 165),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, vec3!(265, 0, 295)));
    world.add(box1);

    // Grass Sphere
    let grass = Arc::new(Dielectric::new(1.5));
    let sphere = Arc::new(Sphere::new(point3!(190, 90, 190), 90.0, grass));
    world.add(sphere);

    // Light source
    let empty_material = Arc::new(EmptyMaterial);
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        point3!(213, 554, 227),
        vec3!(130, 0, 0),
        vec3!(0, 0, 105),
        empty_material.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        point3!(190, 90, 190),
        90.0,
        empty_material.clone(),
    )));
    let direct_light_sampling = lights.objects.len() != 0;

    // カメラの設定
    let lookfrom = point3!(278, 278, -800);
    let lookat = point3!(278, 278, 0);
    let image_width = 600;
    let aspect_ratio = 1.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0, 0, 0);
    let vfov = 40.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}

pub fn cornell_smoke() -> (HittableList, HittableList, Camera, bool) {
    // オブジェクトの設定
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
        7, 7, 7
    )))));

    // Cornell box sides
    world.add(Arc::new(Quad::new(
        point3!(555, 0, 0),
        vec3!(0, 0, 555),
        vec3!(0, 555, 0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        point3!(0, 0, 555),
        vec3!(0, 0, -555),
        vec3!(0, 555, 0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        point3!(0, 555, 0),
        vec3!(555, 0, 0),
        vec3!(0, 0, 555),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        point3!(0, 0, 555),
        vec3!(555, 0, 0),
        vec3!(0, 0, -555),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        point3!(555, 0, 555),
        vec3!(-555, 0, 0),
        vec3!(0, 555, 0),
        white.clone(),
    )));

    // Light
    world.add(Arc::new(Quad::new(
        point3!(113, 554, 127),
        vec3!(330, 0, 0),
        vec3!(0, 0, 305),
        light,
    )));

    // Box
    let box1 = Arc::new(create_box(
        point3!(0, 0, 0),
        point3!(165, 330, 165),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, vec3!(265, 0, 295)));
    world.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        Arc::new(SolidColor::new(color!(0, 0, 0))),
    )));

    let box2 = Arc::new(create_box(
        point3!(0, 0, 0),
        point3!(165, 165, 165),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, vec3!(130, 0, 65)));
    world.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        Arc::new(SolidColor::new(color!(1, 1, 1))),
    )));

    // ライトの設定
    let empty_material = Arc::new(EmptyMaterial);
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        point3!(113, 554, 127),
        vec3!(330, 0, 0),
        vec3!(0, 0, 305),
        empty_material.clone(),
    )));
    let direct_light_sampling = lights.objects.len() != 0;

    // カメラの設定
    let lookfrom = point3!(278, 278, -800);
    let lookat = point3!(278, 278, 0);
    let image_width = 600;
    let aspect_ratio = 1.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0, 0, 0);
    let vfov = 40.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}

pub fn final_scene() -> (HittableList, HittableList, Camera, bool) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.48, 0.83, 0.53
    )))));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(create_box(
                point3!(x0, y0, z0),
                point3!(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BvhNode::new_with_list(&mut boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(color!(
        7, 7, 7
    )))));
    world.add(Arc::new(Quad::new(
        point3!(123, 554, 147),
        vec3!(300, 0, 0),
        vec3!(0, 0, 265),
        light,
    )));

    // Moving Sphere
    let center1 = point3!(400, 400, 200);
    let center2 = center1 + vec3!(30, 0, 0);
    let sphere_material = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.7, 0.3, 0.1
    )))));
    world.add(Arc::new(Sphere::new_with_moving_sphere(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        point3!(260, 150, 45),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        point3!(0, 150, 145),
        50.0,
        Arc::new(Metal::new(color!(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary1 = Arc::new(Sphere::new(
        point3!(360, 150, 145),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary1.clone());
    world.add(Arc::new(ConstantMedium::new(
        boundary1.clone(),
        0.2,
        Arc::new(SolidColor::new(color!(0.2, 0.4, 0.9))),
    )));

    let boundary2 = Arc::new(Sphere::new(
        point3!(0, 0, 0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_with_color(
        boundary2,
        0.0001,
        color!(1, 1, 1),
    )));

    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(Path::new(
        "./data/earthmap.jpg",
    )))));
    world.add(Arc::new(Sphere::new(point3!(400, 200, 400), 100.0, emat)));

    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        point3!(220, 280, 300),
        80.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.73, 0.73, 0.73
    )))));

    for _ in 0..1000 {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_with_list(&mut boxes2, 0.0, 1.0)),
            15.0,
        )),
        vec3!(-100, 270, 395),
    )));

    // ライトの設定
    let empty_material = Arc::new(EmptyMaterial);
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        point3!(123, 554, 147),
        vec3!(300, 0, 0),
        vec3!(0, 0, 265),
        empty_material.clone(),
    )));
    let direct_light_sampling = lights.objects.len() != 0;

    // カメラの設定
    let lookfrom = point3!(478, 278, -600);
    let lookat = point3!(278, 278, 0);
    let image_width = 600;
    let aspect_ratio = 1.0;
    let samples_per_pixel = 100;
    let max_depth = 20;
    let background = color!(0, 0, 0);
    let vfov = 40.0;
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        defocus_angle,
        focus_dist,
    );

    (world, lights, cam, direct_light_sampling)
}
