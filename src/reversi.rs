pub mod board;

pub use board::*;

use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn other(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl From<Color> for char {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 'B',
            Color::White => 'W',
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Color::Black => write!(f, "⚪"),
            Color::White => write!(f, "⚫"),
        }
    }
}
