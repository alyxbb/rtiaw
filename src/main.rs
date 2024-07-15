use crate::{
    camera::Camera, hittable::Hittable, hittable_list::HittabbleList, sphere::Sphere, vec3::Point3,
};

mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 7680;

    let mut world = HittabbleList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let world_box: Box<dyn Hittable> = Box::new(world);

    let camera = Camera::new(image_width, aspect_ratio);
    camera.render(world_box)
}
