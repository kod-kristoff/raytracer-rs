use crate::{Color, HitRecord, Ray};

pub trait Material {
    fn scatter(
        &self,
        rng: &mut dyn rand::RngCore,
        ray_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Color, Ray)>;
}
