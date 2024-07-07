use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    let val = 255.999 * pixel_color;
    let ir = val.x as i32;
    let ig = val.y as i32;
    let ib = val.z as i32;

    println!("{} {} {}", ir, ig, ib);
}
