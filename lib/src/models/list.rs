use crate::{Hittable, HitRecord, Ray};
use std::sync::Arc;

pub struct List {
    objects: Vec<Arc<dyn Hittable>>,
}


impl List {
    pub fn new() -> Self {
        Self { objects: Vec::<Arc<dyn Hittable>>::new() }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }

}

impl Hittable for List {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<HitRecord> {
        let mut rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }

        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }
}

