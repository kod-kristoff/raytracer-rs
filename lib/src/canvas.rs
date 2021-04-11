mod grid;

pub use grid::*;

use crate::{Color, Point2};

pub trait Canvas {
    fn height(&self) -> usize;    
    fn width(&self) -> usize;
    fn get(&self, p: Point2) -> &Color;
    fn set(&mut self, p: Point2, color: Color);
}
