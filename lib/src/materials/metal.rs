use crate::{
    Color,
    HitRecord,
    Material,
    Ray,
    reflect,
    Vec3,
};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {

        Self { 
            albedo,
            fuzz: if fuzz < 1.0 {
                fuzz
            } else {
                1.0
            }
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut dyn rand::RngCore,
        ray_in: &Ray,
        rec: &HitRecord
    ) -> Option<(Color, Ray)> {
        let reflected = reflect(&ray_in.direction().to_unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere(rng));
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
