use crate::{
    Color,
    HitRecord,
    Material,
    Ray,
    refract,
};

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(_color: Color, index_of_refraction: f64) -> Self {
        Self { ir: index_of_refraction }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut dyn rand::RngCore,
        ray_in: &Ray,
        rec: &HitRecord,
        ) -> Option<(Color, Ray)> {
        let attenuation = Color::white();
        let refraction_ratio = if rec.front_face {
            1.0/self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().to_unit_vector();
        let refracted = refract(&unit_direction, &rec.normal, refraction_ratio);
        Some((attenuation, Ray::new(rec.p, refracted)))

    }
}
