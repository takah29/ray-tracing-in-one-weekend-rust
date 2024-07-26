use crate::{
    aarect::{XyRect, XzRect, YzRect},
    bvh::BvhNode,
    camera::Camera,
    constant_medium::ConstantMedium,
    cuboid::Cuboid,
    hittable::{FlipFace, RotateY, Translate},
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

pub fn random_scene() -> (HittableList, Camera, Color, usize, usize) {
    let mut hittable_list = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Arc::new(SolidColor::new(color!(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(color!(0.9, 0.9, 0.9))),
    ));
    let ground_material = Arc::new(Lambertian::new(checker));
    hittable_list.add(Arc::new(Sphere::new(
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
                    hittable_list.add(Arc::new(MovingSphere::new(
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
                    hittable_list.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // grass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    hittable_list.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                };
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    hittable_list.add(Arc::new(Sphere::new(point3!(0, 1, 0), 1.0, material1)));
    let material2 = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.4, 0.2, 0.1
    )))));
    hittable_list.add(Arc::new(Sphere::new(point3!(-4, 1, 0), 1.0, material2)));
    let material3 = Arc::new(Metal::new(&color!(0.7, 0.6, 0.5), 0.0));
    hittable_list.add(Arc::new(Sphere::new(point3!(4, 1, 0), 1.0, material3)));

    // カメラの設定
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let background = color!(0.7, 0.8, 1);

    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let vup = vec3!(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;

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

    (hittable_list, cam, background, image_width, image_height)
}

pub fn two_spheres() -> (HittableList, Camera, Color, usize, usize) {
    let mut hittable_list = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Arc::new(SolidColor::new(color!(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(color!(0.9, 0.9, 0.9))),
    ));

    hittable_list.add(Arc::new(Sphere::new(
        point3!(0, -10, 0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    hittable_list.add(Arc::new(Sphere::new(
        point3!(0, 10, 0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));

    // カメラの設定
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let background = color!(0.7, 0.8, 1);

    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let vup = vec3!(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;

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

    (hittable_list, cam, background, image_width, image_height)
}

pub fn two_perlin_spheres() -> (HittableList, Camera, Color, usize, usize) {
    let mut hittable_list = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(5.0));

    hittable_list.add(Arc::new(Sphere::new(
        point3!(0, -1000, 0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    hittable_list.add(Arc::new(Sphere::new(
        point3!(0, 2, 0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    // カメラの設定
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let background = color!(0.7, 0.8, 1);

    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let vup = vec3!(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;

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

    (hittable_list, cam, background, image_width, image_height)
}

pub fn earth() -> (HittableList, Camera, Color, usize, usize) {
    let image_path = Path::new("./data/earthmap.jpg");
    let earth_texture = Arc::new(ImageTexture::new(image_path));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(point3!(0, 0, 0), 2.0, earth_surface));

    let hittable_list = HittableList::new_with_object(globe);

    // カメラの設定
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let background = color!(0.7, 0.8, 1);

    let lookfrom = point3!(13, 2, 3);
    let lookat = point3!(0, 0, 0);
    let vup = vec3!(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;

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

    (hittable_list, cam, background, image_width, image_height)
}

pub fn simple_light() -> (HittableList, Camera, Color, usize, usize) {
    let mut hittable_list = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    hittable_list.add(Arc::new(Sphere::new(
        point3!(0, -1000, 0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    hittable_list.add(Arc::new(Sphere::new(
        point3!(0, 2, 0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(color!(
        4, 4, 4
    )))));

    hittable_list.add(Arc::new(Sphere::new(
        point3!(0, 7, 0),
        2.0,
        difflight.clone(),
    )));
    hittable_list.add(Arc::new(XyRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));

    // カメラの設定
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let background = color!(0, 0, 0);

    let lookfrom = point3!(26, 3, 6);
    let lookat = point3!(0, 2, 0);
    let vup = vec3!(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 20.0;

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

    (hittable_list, cam, background, image_width, image_height)
}

pub fn cornell_box() -> (HittableList, Camera, Color, usize, usize) {
    // オブジェクトの設定
    let mut hittable_list = HittableList::new();

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

    hittable_list.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    hittable_list.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    hittable_list.add(Arc::new(FlipFace::new(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    hittable_list.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    hittable_list.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    hittable_list.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let aluminum = Arc::new(Metal::new(&color!(0.8, 0.85, 0.88), 0.0));
    let box1 = Arc::new(Cuboid::new(
        point3!(0, 0, 0),
        point3!(165, 330, 165),
        aluminum,
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, vec3!(265, 0, 295)));
    hittable_list.add(box1);

    let box2 = Arc::new(Cuboid::new(
        point3!(0, 0, 0),
        point3!(165, 165, 165),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, vec3!(130, 0, 65)));
    hittable_list.add(box2);

    // カメラの設定
    let aspect_ratio = 1.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let background = color!(0, 0, 0);

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

    (hittable_list, cam, background, image_width, image_height)
}

pub fn cornell_smoke() -> (HittableList, Camera, Color, usize, usize) {
    // オブジェクトの設定
    let mut hittable_list = HittableList::new();

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

    hittable_list.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    hittable_list.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    hittable_list.add(Arc::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    hittable_list.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    hittable_list.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    hittable_list.add(Arc::new(XyRect::new(
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
    hittable_list.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        Arc::new(SolidColor::new(color!(0, 0, 0))),
    )));

    let box2 = Arc::new(Cuboid::new(
        point3!(0, 0, 0),
        point3!(165, 165, 165),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, vec3!(130, 0, 65)));
    hittable_list.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        Arc::new(SolidColor::new(color!(1, 1, 1))),
    )));

    // カメラの設定
    let aspect_ratio = 1.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let background = color!(0, 0, 0);

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

    (hittable_list, cam, background, image_width, image_height)
}

pub fn final_scene() -> (HittableList, Camera, Color, usize, usize) {
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

            boxes1.add(Arc::new(Cuboid::new(
                point3!(x0, y0, z0),
                point3!(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut hittable_list = HittableList::new();

    hittable_list.add(Arc::new(BvhNode::new_with_list(&mut boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(color!(
        7, 7, 7
    )))));
    hittable_list.add(Arc::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center0 = point3!(400, 400, 200);
    let center1 = center0 + vec3!(30, 0, 0);
    let moving_sphere_material = Arc::new(Lambertian::new(Arc::new(SolidColor::new(color!(
        0.7, 0.3, 0.1
    )))));
    hittable_list.add(Arc::new(MovingSphere::new(
        center0,
        center1,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    hittable_list.add(Arc::new(Sphere::new(
        point3!(260, 150, 45),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    hittable_list.add(Arc::new(Sphere::new(
        point3!(0, 150, 145),
        50.0,
        Arc::new(Metal::new(&color!(0.8, 0.8, 0.9), 10.0)),
    )));

    let boundary1 = Arc::new(Sphere::new(
        point3!(360, 150, 145),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    hittable_list.add(boundary1.clone());
    hittable_list.add(Arc::new(ConstantMedium::new(
        boundary1.clone(),
        0.2,
        Arc::new(SolidColor::new(color!(0.2, 0.4, 0.9))),
    )));

    let boundary2 = Arc::new(Sphere::new(
        point3!(0, 0, 0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    hittable_list.add(Arc::new(ConstantMedium::new(
        boundary2,
        0.0001,
        Arc::new(SolidColor::new(color!(1, 1, 1))),
    )));

    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(Path::new(
        "./data/earthmap.jpg",
    )))));
    hittable_list.add(Arc::new(Sphere::new(point3!(400, 200, 400), 100.0, emat)));

    let pertext = Arc::new(NoiseTexture::new(0.1));
    hittable_list.add(Arc::new(Sphere::new(
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

    hittable_list.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_with_list(&mut boxes2, 0.0, 1.0)),
            15.0,
        )),
        vec3!(-100, 270, 395),
    )));

    // カメラの設定
    let aspect_ratio = 1.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let background = color!(0, 0, 0);

    let lookfrom = point3!(278 * 2, 278, -800);
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

    (hittable_list, cam, background, image_width, image_height)
}
