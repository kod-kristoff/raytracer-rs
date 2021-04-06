mod math;
pub mod io;
mod color;
mod ray;

pub use math::*;
pub use color::*;
pub use ray::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
