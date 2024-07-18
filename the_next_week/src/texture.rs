use crate::{
    color,
    perlin::Perlin,
    rtweekend::{Color, Point3},
};
use image::Rgb32FImage;
use std::path::Path;
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.e[0]).sin() * (10.0 * p.e[1]).sin() * (10.0 * p.e[2]).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        let noise = Perlin::new();
        Self { noise, scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        color!(1, 1, 1) * 0.5 * (1.0 + (self.scale * p.e[2] + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Rgb32FImage,
}

impl ImageTexture {
    pub fn new(filename: &Path) -> Self {
        let data = image::open(filename)
            .expect("Failed to load image")
            .into_rgb32f();

        Self { data }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * self.data.width() as f64) as u32;
        let mut j = (v * self.data.height() as f64) as u32;

        if i >= self.data.width() {
            i = self.data.width() - 1;
        }
        if j >= self.data.height() {
            j = self.data.height() - 1;
        }

        let pixel = *self.data.get_pixel(i, j);

        color!(pixel[0], pixel[1], pixel[2])
    }
}
