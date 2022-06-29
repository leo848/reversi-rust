use crate::reversi::*;

use std::{
    error::Error,
    fmt,
    ops::{Deref, DerefMut, Index, IndexMut},
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Field(pub usize, pub usize);

impl Field {
    pub fn in_bounds(&self) -> bool {
        self.0 < 8 && self.1 < 8
    }

    pub fn all() -> impl Iterator<Item = Field> {
        (0..8).flat_map(move |x| (0..8).map(move |y| Self(x, y)))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PlaceError {
    Occupied,
    OutOfBounds,
    CapturesNone,
}

impl fmt::Display for PlaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlaceError::Occupied => write!(f, "Field is already occupied"),
            PlaceError::OutOfBounds => write!(f, "Field is out of bounds"),
            PlaceError::CapturesNone => write!(f, "Field captures no pieces"),
        }
    }
}

impl Error for PlaceError {}

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

    /// Check if a given move is valid.
    ///
    /// # Returns
    /// - A vector of fields that are captured by the move, if the move is valid.
    /// - A `PlaceError` if the move is invalid.
    pub fn move_validity(&self, field: Field, color: Color) -> Result<Vec<Field>, PlaceError> {
        if !field.in_bounds() {
            Err(PlaceError::OutOfBounds)?;
        }

        if self[field].is_some() {
            Err(PlaceError::Occupied)?;
        }

        let captured_pieces: Vec<Field> = Field::all()
            .filter(|&other| self[other] == Some(color)) // needs to be the same color
            .filter_map(|other| Board::line_between((field, other))) // a line between the two
            // fields has to exist
            .filter(|line| line.iter().all(|&field| self[field] == Some(color.other())))
            .flatten()
            .collect();

        if captured_pieces.is_empty() {
            Err(PlaceError::CapturesNone)?;
        }

        Ok(captured_pieces)
    }

    pub fn valid_moves(&self, color: Color) -> Vec<Field> {
        Field::all()
            .filter(|&field| self.move_validity(field, color).is_ok())
            .collect()
    }

    pub fn add_piece(&mut self, field: Field, color: Color) -> Result<usize, PlaceError> {
        let captured_pieces = self.move_validity(field, color)?;

        self.add_piece_unchecked(field, color);

        for &captured_piece in &captured_pieces {
            self.flip(captured_piece);
        }

        Ok(captured_pieces.len())
    }

    fn add_piece_unchecked(&mut self, field: Field, color: Color) {
        self[field] = Some(color);
    }

    fn line_between(fields: (Field, Field)) -> Option<Vec<Field>> {
        let (Field(x1, y1), Field(x2, y2)) = fields;
        if x1 == x2 {
            // Vertical line
            Some((y1.min(y2)..=y2.max(y1)).map(|y| Field(x1, y)).collect())
        } else if y1 == y2 {
            // Horizontal line
            Some((x1.min(x2)..=x2.max(x1)).map(|x| Field(x, y1)).collect())
        } else if usize::abs_diff(x1, x2) == usize::abs_diff(y1, y2) {
            // Diagonal line
            Some(
                (x1.min(x2)..=x2.max(x1))
                    .zip(y1.min(y2)..=y2.max(y1))
                    .map(|(x, y)| Field(x, y))
                    .collect(),
            )
        } else {
            // No line
            None
        }
        .and_then(|line: Vec<Field>| if line.len() < 3 { None } else { Some(line) })
        .and_then(|line| Some(line[1..line.len() - 1].to_vec()))
    }
}

impl Index<Field> for Board {
    type Output = Option<Color>;

    fn index(&self, field: Field) -> &Self::Output {
        &self.0[field.1][field.0]
    }
}

impl IndexMut<Field> for Board {
    fn index_mut(&mut self, field: Field) -> &mut Self::Output {
        &mut self.0[field.1][field.0]
    }
}

impl fmt::Display for Board {
    /// Display the board in a human-readable format.
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "╭──{}──╮", "──┬──".repeat(self.len() - 1))?;
        for y in 0..self.len() {
            if y != 0 {
                writeln!(f, "├──{}──┤", "──┼──".repeat(self.len() - 1))?;
            }
            for x in 0..self.len() {
                write!(f, "│")?;
                match self[Field(x, y)] {
                    Some(Color::White) => write!(f, " ⚪ ")?,
                    Some(Color::Black) => write!(f, " ⚫ ")?,
                    None => write!(f, " {x} {y}")?,
                }
                if x == self.len() - 1 {
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
