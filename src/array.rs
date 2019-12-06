use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Array2D<T> {
    data: Vec<T>,
    pub size_x: usize,
    pub size_y:usize
}

impl <T> Array2D<T> {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            data: Vec::with_capacity(size_y * size_x),
            size_x,
            size_y
        }
    }
}

impl <T: Default> Array2D<T> {
    pub fn empty(size_x: usize, size_y: usize) -> Self {
        Self {
            data: (0..size_x*size_y).map(|_| T::default()).collect(),
            size_x,
            size_y
        }
    }       

    pub fn dim(&self) -> (usize, usize) {
        (self.size_x, self.size_y)
    }
}

impl <T> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        let index = self.size_x * coord.1 + coord.0;
        &self.data[index]
    }
}
impl <T> IndexMut<(usize, usize)> for Array2D<T> {

    fn index_mut(&mut self, coord: (usize, usize)) -> &mut Self::Output {
        let index = self.size_x * coord.1 + coord.0;
        &mut self.data[index]
    }
}
