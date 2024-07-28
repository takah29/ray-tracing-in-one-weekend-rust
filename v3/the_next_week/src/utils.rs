use crate::vec3::Color;

pub fn write_ppm(pixels: Vec<Color>, image_width: usize, image_height: usize) {
    println!("P3\n{} {}\n255", image_width, image_height);
    for pixel_color in pixels {
        // ガンマ補正のために平方根を取っている
        let [ir, ig, ib] = pixel_color
            .e
            .map(|x| (256.0 * x.sqrt().clamp(0.0, 0.999)) as u8);
        println!("{} {} {}", ir, ig, ib);
    }
}
