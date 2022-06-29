use reversi::reversi::*;

use std::{
    io
};

pub trait Player {
    fn turn(&self, board: &Board) -> Field;
    fn color(&self) -> Color;
}

pub struct HumanPlayer {
    color: Color,
    name: String,
}

impl HumanPlayer {
    pub fn new(color: Color, name: String) -> Self {
        HumanPlayer { color, name }
    }
}

impl Player for HumanPlayer {
    fn turn(&self, board: &Board) -> Field {
        let mut input = String::new();
        println!("{}'s turn:", self.name);
        println!("{}", board);
        println!("Enter a field:");
        io::stdin().read_line(&mut input).unwrap();
        let field = Field::from_str(&input.trim()).unwrap();
        field
    }

    fn color(&self) -> Color {
        self.color
    }
}
