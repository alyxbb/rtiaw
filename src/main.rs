use core::f64;


use hittable::Hittable;
use sphere::Sphere;

use crate::{sphere::Sphere, ray::Ray, vec3::{Color, Point3, Vec3}};

mod vec3;
mod ray;
mod hittable;
mod sphere;


pub fn ray_colour(r: Ray) -> Color {
    let s = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let t = s.hit(r, 0.0, 1000.0);
    match t{
        Some(c) => {
            let n = r.at(c) - Vec3::new(0.0, 0.0, -1.0);
            return 0.5*Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0);
            
        },
        _ => (),
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5*(unit_direction.y() + 1.0);
    (1.0-a) * Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0/9.0;
    let image_width = 2160;


    let mut image_height = (image_width as f64/aspect_ratio) as u64;
    if image_height <1 {image_height = 1};

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5*(pixel_delta_u + pixel_delta_v);


    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines Remaining {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_colour = ray_colour(r);
            println!("{}", pixel_colour.format_colour())
        }
    }
    eprintln!("\rDone                        ")
}
