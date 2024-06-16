use crate::vec3::Color;

mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines Remaining {}", image_height - j);
        for i in 0..image_width {
            let pixel_colour = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            println!("{}", pixel_colour.format_colour())
        }
    }
    eprintln!("\rDone                        ")
}
