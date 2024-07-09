use crate::vec3::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let [ir, ig, ib] = (pixel_color / samples_per_pixel as f64)
        .e
        .map(|x| (256.0 * x.clamp(0.0, 0.999)) as u8);

    println!("{} {} {}", ir, ig, ib);
}
