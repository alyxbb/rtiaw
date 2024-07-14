use crate::{ray::Ray, vec3::{Point3, Vec3}};


pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) ->  HitRecord {
        HitRecord { p, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>; 


}
