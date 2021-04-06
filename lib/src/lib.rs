mod math;
pub mod io;
mod color;

pub use math::*;
pub use color::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
