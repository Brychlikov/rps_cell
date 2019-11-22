extern crate ndarray;
extern crate rand;
extern crate num_traits;

use super::array::Array2D;

use rand::Rng;
use std::ops::{Generator, GeneratorState};
use num_traits::identities;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    White
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub strength: i32,
    pub color: Color
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            strength: 0,
            color: Color::White
        }
    }
}

impl Cell {
    pub const max_strength: i32 = 4;
}

pub struct RpsAutomata {
    pub board: Array2D<Cell>,
    pub size: (usize, usize)
}

type Point = (usize, usize);
impl RpsAutomata {

    pub fn new(size_x: usize, size_y: usize) -> Self{

        Self {
            board: Array2D::empty(size_x, size_y),
            size: (size_x, size_y)
        }
    }

    fn random_neighbour(&self, coord: (usize, usize)) -> &Cell {
        let mut possible = Vec::with_capacity(8);
        
        let left_safe = coord.0 >= 1;
        let top_safe = coord.1 >= 1;
        let right_safe = coord.0 + 1 < self.size.0;
        let bot_safe = coord.1 + 1 < self.size.1;

        if left_safe {
            possible.push((coord.0 - 1, coord.1));
        }

        if left_safe && top_safe {
            possible.push((coord.0 - 1, coord.1));
        }

        if top_safe {
            possible.push((coord.0, coord.1 - 1));
        }

        if top_safe && right_safe {
            possible.push((coord.0 + 1, coord.1 - 1));
        }

        if right_safe {
            possible.push((coord.0 + 1, coord.1));
        }

        if right_safe && bot_safe {
            possible.push((coord.0 + 1, coord.1 + 1));
        }

        if bot_safe {
            possible.push((coord.0, coord.1 + 1));
        }

        if bot_safe && left_safe {
            possible.push((coord.0 - 1, coord.1 + 1));
        }

        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0, possible.len());
        let indices = possible[index];
        &self.board[indices]
    }


    pub fn update(&mut self) {
        let mut new_board: Array2D<Cell> = Array2D::empty(self.board.dim().0, self.board.dim().1);

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let c = (x, y);

                let el = self.board[c];
                let rival = self.random_neighbour(c);

                match el.color {
                    Color::White => {
                        if rival.strength > 0 {
                            new_board[c] = Cell{strength: rival.strength - 1, color: rival.color};
                            continue;
                        }
                    },
                    Color::Red => {
                        if let Color::Green = rival.color {
                            // new_board[c] = Cell{strength: Cell::max_strength, color: Color::Green};
                            new_board[c] = Cell{strength: 0, color: Color::Green};
                            continue;
                        }
                    },
                    Color::Green => {
                        if let Color::Blue = rival.color {
                            new_board[c] = Cell{strength: 0, color: Color::Blue};
                            continue;
                        }
                    },
                    Color::Blue => {
                        if let Color::Red = rival.color {
                            new_board[c] = Cell{strength: 0, color: Color::Red};
                            continue;
                        }
                    }
                }
                new_board[c] = el;
            }
        }
        std::mem::swap(&mut self.board, &mut new_board);
    }



}
