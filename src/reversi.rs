pub mod board;

pub use board::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}
