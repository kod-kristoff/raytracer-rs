use crate::{
    Color,
    HitRecord,
    Material,
    Ray,
    reflect,
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
        use rand::Rng;

        let attenuation = Color::white();
        let refraction_ratio = if rec.front_face {
            1.0/self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().to_unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        Some((attenuation, Ray::new(rec.p, direction)))

    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1.0-ref_idx)/(1.0+ref_idx);
    r0 = r0*r0;
    r0 + (1.0-r0)*(1.0-cosine).powi(5)
}
