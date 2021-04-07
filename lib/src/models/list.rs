use crate::{Hittable, HitRecord, Point, Ray, Vec3};
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
        rec: &mut HitRecord
    ) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

