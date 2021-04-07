use std::ops::{Add, AddAssign, Mul};

pub struct Color {
    e: [f64; 3],
}

impl Color {
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { e: [r, g, b] }
    }

    pub fn black() -> Self {
        Self { e: [0., 0., 0.] }
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }
}

impl Add for Color {
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

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color { e: [self*rhs.e[0], self*rhs.e[1], self*rhs.e[2]] }
    }
}

