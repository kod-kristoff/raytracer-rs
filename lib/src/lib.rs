mod camera;
pub mod canvas;
mod color;
mod hittable;
pub mod io;
mod material;
pub mod materials;
mod math;
pub mod models;
mod point;
mod ray;
mod renderer;
pub mod utils;

pub use camera::*;
pub use canvas::Canvas;
pub use color::*;
pub use hittable::*;
pub use material::*;
pub use math::*;
pub use point::*;
pub use ray::*;
pub use renderer::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
