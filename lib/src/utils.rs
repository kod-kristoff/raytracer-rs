use rand::{Rng, RngCore};


pub fn random_in_0_1(rng: &mut dyn RngCore) -> f64 {
    rng.gen()
}

pub fn random_in_interval(rng: &mut dyn RngCore, min: f64, max: f64) -> f64 {
    min + (max-min)*random_in_0_1(rng)
}
