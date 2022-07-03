use reversi::reversi::*;

use std::{
    io::{self, Write},
    ops::Sub,
};

use colored::Colorize;

pub trait Player {
    fn turn(&self, board: &Board) -> Option<Field>;
    fn color(&self) -> Color;
    fn name(&self) -> String;
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
    fn name(&self) -> String {
        self.name.clone()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn turn(&self, board: &Board) -> Option<Field> {
        match self.color {
            Color::White => println!("{:w>}", board),
            Color::Black => println!("{:b>}", board),
        }

        println!("{} {}", self.color(), self.name.bold());

        if board.valid_moves(self.color()).is_empty() {
            println!("You have no valid moves. Press <Enter> to pass.");
            io::stdin().read_line(&mut String::new()).unwrap();
            None?;
        }

        let field = loop {
            let mut input = String::new();
            print!("Enter a field: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse() {
                Ok(field) => match board.move_validity(field, self.color()) {
                    Ok(_) => break field,
                    Err(error) => {
                        println!("Invalid move: {:?} {}", field, error);
                        continue;
                    }
                },
                Err(error) => {
                    println!("Invalid input: {}", error);
                    continue;
                }
            };
        };

        Some(field)
    }
}

pub struct MinimaxBot {
    color: Color,
    depth: u8,
}

impl MinimaxBot {
    pub fn new(color: Color, depth: u8) -> Self {
        MinimaxBot { color, depth }
    }

    fn eval(&self, board: &Board) -> i32 {
        match board.status() {
            GameStatus::Win(color) => {
                if color == self.color {
                    i32::MAX
                } else {
                    i32::MIN
                }
            }
            GameStatus::Draw => 0,
            GameStatus::InProgress => i32::sub(
                board.count_pieces(Color::White) as i32,
                board.count_pieces(Color::Black) as i32,
            ),
        }
    }
}

impl Player for MinimaxBot {
    fn name(&self) -> String {
        format!("Minimax Bot (depth {})", self.depth)
    }

    fn color(&self) -> Color {
        self.color
    }

    fn turn(&self, board: &Board) -> Option<Field> {
        None
    }
}
