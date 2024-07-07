mod utils;
mod vec3;

use utils::write_color;
use vec3::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height - 1).rev() {
        eprint!("\rScanline remaining: {:3}", j);
        for i in 0..image_width {
            let pixel_color = color!(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25
            );
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}
