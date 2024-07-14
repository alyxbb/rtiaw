use crate::{hittable::{HitRecord, Hittable}, ray::Ray, vec3::Point3};


pub struct Sphere {
    center: Point3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Point3, mut radius: f64) -> Sphere {
        if radius < 0.0 { radius = 0.0}
        Sphere{center, radius}

    }
}

impl Hittable for Sphere{

    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (h-sqrtd)/a;
            if root<= ray_tmin || ray_tmax <= root {
                root = (h+sqrtd)/a;
                if root<= ray_tmin || ray_tmax <= root {
                    return None
                }
            }
            let point = r.at(root);
            let normal = (point - self.center) / self.radius;
            Some(HitRecord::new(point, normal, root))
        }
    }
}
