use crate::{Material, Point, Ray, Vec3};
use std::sync::Arc;

pub trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point::default(),
            normal: Vec3::default(),
            t: f64::default(),
            front_face: bool::default(),
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}
