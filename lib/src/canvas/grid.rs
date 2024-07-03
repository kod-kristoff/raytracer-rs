use crate::{
    Canvas,
    Point2,
};

pub struct Grid {
    width: usize,
    height: usize,
    array: Vec<[u8; 3]>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height,
            array: [<[u8; 3]>::default()].repeat(width * height),
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

    fn get(&self, p: &Point2) -> &[u8; 3] {
        &self.array[p.x + self.width*p.y]
    }

    fn set(&mut self, p: &Point2, color: [u8; 3]) {
        self.array[p.x + self.width*p.y] = color;
    }
}
