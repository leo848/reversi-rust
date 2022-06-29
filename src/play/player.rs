use reversi::reversi::*;

use std::io::{self, Write};

use colored::Colorize;

pub trait Player {
    fn turn(&self, board: &Board) -> Option<Field>;
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
    fn turn(&self, board: &Board) -> Option<Field> {
        if board.valid_moves(self.color()).len() == 0 {
            None?;
        }

        match self.color {
            Color::White => println!("{:w>}", board),
            Color::Black => println!("{:b>}", board),
        }

        let mut input = String::new();
        println!("{} {}", self.color().to_string(), self.name.bold());
        print!("Enter a field: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let field = input.trim().parse().unwrap();

        Some(field)
    }

    fn color(&self) -> Color {
        self.color
    }
}
