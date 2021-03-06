use super::Player;
use reversi_game::reversi::*;

use std::io::{self, Write};

use colored::Colorize;

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
        redraw_board(
            board,
            &DisplayOptions {
                color: Some(self.color),
                ..Default::default()
            },
        );

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

    fn redraw_options(&self) -> DisplayOptions {
        DisplayOptions {
            color: Some(self.color),
            ..Default::default()
        }
    }
}
