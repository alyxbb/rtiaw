use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct HittabbleList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittabbleList {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest = interval.max;

        for object in &self.objects {
            match object.hit(r, Interval::new(interval.min, closest)) {
                Some(hit_record) => {
                    closest = hit_record.t;
                    hit = Some(hit_record);
                }
                _ => (),
            };
        }
        hit
    }
}

impl HittabbleList {
    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.objects.push(item);
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}
