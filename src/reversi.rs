pub mod board;

pub use board::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
