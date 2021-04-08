use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::utils;

#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point = Vec3;

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 { e: [f64::default(), f64::default(), f64::default()] }
    }
}

impl Vec3 {
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn random_in_0_1(rng: &mut dyn rand::RngCore) -> Self {
        Self::from_xyz(
            utils::random_in_0_1(rng),
            utils::random_in_0_1(rng),
            utils::random_in_0_1(rng),
        )
    }

    pub fn random_in_interval(
        rng: &mut dyn rand::RngCore,
        min: f64,
        max: f64,
    ) -> Self {
        Self::from_xyz(
            utils::random_in_interval(rng,min,max),
            utils::random_in_interval(rng,min,max),
            utils::random_in_interval(rng,min,max),
        )
    }

    pub fn random_in_unit_sphere(
        rng: &mut dyn rand::RngCore
    ) -> Self {
        loop {
            let vec = Self::random_in_interval(rng, -1., 1.);
            if vec.length_squared() < 1. {
                return vec;
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn to_unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self { e: [self.e[0]/rhs, self.e[1]/rhs, self.e[2]/rhs] }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [self*rhs.e[0], self*rhs.e[1], self*rhs.e[2]] }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]] }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod instantiation {
        use super::*;

        #[test]
        fn create_default() {
            let v = Vec3::default();

            assert_eq!(v.x(), f64::default());
        }
    }
}
