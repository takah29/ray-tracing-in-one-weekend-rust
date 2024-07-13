use crate::vec3::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    // ガンマ補正のために平方根を取っている
    let [ir, ig, ib] = (pixel_color / samples_per_pixel as f64)
        .e
        .map(|x| (256.0 * x.sqrt().clamp(0.0, 0.999)) as u8);

    println!("{} {} {}", ir, ig, ib);
}
