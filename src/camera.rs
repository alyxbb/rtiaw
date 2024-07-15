use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

pub struct Camera {
    pub image_width: u64,
    pub aspect_ratio: f64,
    image_height: u64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: Box<dyn Hittable>) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines Remaining {:>5}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_colour = Self::ray_colour(&r, &world);
                println!("{}", pixel_colour.format_colour())
            }
        }
        eprintln!("\rDone                        ")
    }

    fn ray_colour(r: &Ray, hittable: &Box<dyn Hittable>) -> Color {
        let t = hittable.hit(r, Interval::new(0.0, f64::INFINITY));
        match t {
            Some(c) => 0.5 * (c.normal + Color::new(1.0, 1.0, 1.0)),
            _ => {
                let unit_direction = r.direction().unit_vector();
                let a = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    pub fn new(image_width: u64, aspect_ratio: f64) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u64;
        if image_height < 1 {
            image_height = 1
        };

        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            aspect_ratio,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_v,
            pixel_delta_u,
        }
    }
}
