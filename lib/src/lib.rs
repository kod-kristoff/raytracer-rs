mod camera;
mod math;
pub mod io;
mod color;
mod hittable;
pub mod models;
mod ray;
pub mod utils;

pub use camera::*;
pub use hittable::*;
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
