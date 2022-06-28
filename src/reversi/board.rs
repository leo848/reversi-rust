use crate::reversi::*;

use std::{
    fmt,
    ops::{Deref, DerefMut, Index, IndexMut},
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Field(pub usize, pub usize);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board(pub [[Option<Color>; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        let mut new_board = Board::empty();

        for x in 3..=4 {
            for y in 3..=4 {
                new_board[Field(x, y)] = match (x + y) % 2 {
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

    pub fn flip(&mut self, field: Field) {
        self[field] = self[field].map(Color::other);
    }

    pub fn add_piece(&mut self, field: Field, color: Color) {
        self[field] = Some(color);
    }

    fn line_between(fields: (Field, Field)) -> Option<Vec<Field>> {
        let (Field(x1, y1), Field(x2, y2)) = fields;
        if x1 == x2 {
            // Horizontal line
            Some((y1..y2).map(|y| Field(x1, y)).collect())
        } else if y1 == y2 {
            // Vertical line
            Some((x1..x2).map(|x| Field(x, y1)).collect())
        } else if usize::abs_diff(x1, x2) == usize::abs_diff(y1, y2) {
            // Diagonal line
            Some((x1..x2).zip(y1..y2).map(|(x, y)| Field(x, y)).collect())
        } else {
            // No line
            None
        }
    }
}

impl Index<Field> for Board {
    type Output = Option<Color>;

    fn index(&self, field: Field) -> &Self::Output {
        &self.0[field.0][field.1]
    }
}

impl IndexMut<Field> for Board {
    fn index_mut(&mut self, field: Field) -> &mut Self::Output {
        &mut self.0[field.0][field.1]
    }
}

impl fmt::Display for Board {
    /// Display the board in a human-readable format.
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "╭──{}──╮", "──┬──".repeat(self.len() - 1))?;
        for x in 0..self.len() {
            if x != 0 {
                writeln!(f, "├──{}──┤", "──┼──".repeat(self.len() - 1))?;
            }
            for y in 0..self.len() {
                write!(f, "│")?;
                match self[Field(x, y)] {
                    Some(Color::White) => write!(f, " ⚪ ")?,
                    Some(Color::Black) => write!(f, " ⚫ ")?,
                    None => write!(f, "    ")?,
                }
                if y == self.len() - 1 {
                    write!(f, "│")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "╰──{}──╯", "──┴──".repeat(self.len() - 1))?;

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
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
