use crate::reversi::*;

use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board(pub [[Option<Color>; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        let mut new_board = Board::empty();

        for x in 3..=4 {
            for y in 3..=4 {
                new_board[x][y] = match (x + y) % 2 {
                    0 => Some(Color::White),
                    1 => Some(Color::Black),
                    _ => unreachable!(),
                }
            }
        }

        new_board
    }

    pub fn empty() -> Self {
        Board([[None; 8]; 8])
    }
}

impl Deref for Board {
    type Target = [[Option<Color>; 8]; 8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
