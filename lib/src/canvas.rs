mod grid;

pub use grid::*;

use crate::{Point2};

pub trait Canvas {
    fn height(&self) -> usize;    
    fn width(&self) -> usize;
    fn get(&self, p: &Point2) -> &[u8; 3];
    fn set(&mut self, p: &Point2, color: [u8; 3]);
}
