use crate::{
    Color,
    HitRecord,
    Material,
    Ray,
    reflect,
};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        _rng: &mut dyn rand::RngCore,
        ray_in: &Ray,
        rec: &HitRecord
    ) -> Option<(Color, Ray)> {
        let reflected = reflect(&ray_in.direction().to_unit_vector(), &rec.normal);
        if reflected.dot(&rec.normal) > 0.0 {
            Some((self.albedo, Ray::new(rec.p, reflected)))
        } else {
            None
        }
    }
}
