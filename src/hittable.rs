use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, outwards_normal: Vec3, t: f64, r: &Ray) -> HitRecord {
        let front_face = r.direction().dot(outwards_normal) < 0.0;
        let normal = if front_face {
            outwards_normal
        } else {
            -outwards_normal
        };

        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord>;
}
