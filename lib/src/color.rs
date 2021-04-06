pub struct Color {
    e: [f64; 3],
}

impl Color {
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { e: [r, g, b] }
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
