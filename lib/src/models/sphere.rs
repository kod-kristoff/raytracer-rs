use crate::{
    Hittable,
    HitRecord,
    Material,
    Point,
    Ray,
};
use std::sync::Arc;

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        };
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd)/a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd)/a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = Some(self.material.clone());

        Some(rec)
    }

}
