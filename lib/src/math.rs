pub struct Vec3 {
    e: [f64; 3],
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 { e: [f64::default(), f64::default(), f64::default()] }
    }
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
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
