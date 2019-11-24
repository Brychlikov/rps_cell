extern crate ndarray;
extern crate rand;
extern crate num_traits;

use super::array::Array2D;

use rand::Rng;
use rand::prelude::*;
use std::ops::{Generator, GeneratorState, IndexMut};
use num_traits::identities;
use core::fmt::Alignment::Center;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    pub size: (usize, usize),
    rng: rand::rngs::ThreadRng
}

type Point = (usize, usize);
impl RpsAutomata {

    pub fn new(size_x: usize, size_y: usize) -> Self{

        Self {
            board: Array2D::empty(size_x, size_y),
            size: (size_x, size_y),
            rng: rand::thread_rng()
        }
    }

    fn random_neighbour(&mut self, coord: (usize, usize)) -> (Point, &Cell) {
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

        let index: usize = self.rng.gen_range(0, possible.len());
        let indices = possible[index];
        (possible[index], &self.board[indices])
    }


    pub fn update(&mut self) {
        let mut new_board: Array2D<Cell> = self.board.clone();

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let c = (x, y);

                let el = self.board[c];
                let (c_rival, rival) = self.random_neighbour(c);

                if let Color::White = el.color {
                    continue;
                }

                match rival.color {
                    Color::White => {
                        if el.strength > 0  {
                            new_board[c_rival] = Cell{color: el.color, strength: el.strength - 1};
                        }
                    },

                    Color::Red => {
                        if let Color::Green = el.color {
                            let mut rival_ref =new_board.index_mut(c_rival);
                            if rival_ref.strength > 1 {
                                rival_ref.strength -= 1;
                            } else{
                                *rival_ref = Cell::default();
                            }
                            let mut el_ref = new_board.index_mut(c);
                            if el_ref.strength < 10 {
                                el_ref.strength += 1;
                            }
                        }
                    },

                    Color::Green => {
                        if let Color::Blue = el.color {
                            let mut rival_ref =new_board.index_mut(c_rival);
                            if rival_ref.strength > 1 {
                                rival_ref.strength -= 1;
                            } else{
                                *rival_ref = Cell::default();
                            }
                            let mut  el_ref = new_board.index_mut(c);
                            if el_ref.strength < 10 {
                                el_ref.strength += 1;
                            }
                        }
                    },

                    Color::Blue => {
                        if let Color::Red = el.color {
                            let mut rival_ref = new_board.index_mut(c_rival);
                            if rival_ref.strength > 1 {
                                rival_ref.strength -= 1;
                            } else{
                                *rival_ref = Cell::default();
                            }
                            let mut el_ref = new_board.index_mut(c);
                            if el_ref.strength < 10 {
                                el_ref.strength += 1;
                            }
                        }
                    },
                }
            }
        }
        std::mem::swap(&mut self.board, &mut new_board);
    }



}
