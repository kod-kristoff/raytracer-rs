use crate::{
    Canvas,
    Color,
    Point2,
};

pub struct Grid {
    width: usize,
    height: usize,
    array: Vec<Color>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height,
            array: [Color::default()].repeat(width * height),
        }
    }
}

impl Canvas for Grid {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, p: Point2) -> &Color {
        &self.array[p.x + self.width*p.y]
    }

    fn set(&mut self, p: Point2, color: Color) {
        self.array[p.x + self.width*p.y] = color;
    }
}
