use super::Player;
use reversi::reversi::*;

use std::{
    io::{self, Write},
    ops::Sub,
};

use colored::Colorize;
use spinners::{Spinner, Spinners};

#[derive(Debug, Clone, Copy)]
enum MinimaxStrategy {
    Minimize,
    Maximize,
}

impl MinimaxStrategy {
    fn other(&self) -> MinimaxStrategy {
        match self {
            MinimaxStrategy::Minimize => MinimaxStrategy::Maximize,
            MinimaxStrategy::Maximize => MinimaxStrategy::Minimize,
        }
    }

    fn value(&self) -> i32 {
        match self {
            MinimaxStrategy::Minimize => i32::MAX,
            MinimaxStrategy::Maximize => i32::MIN,
        }
    }
}

impl From<Color> for MinimaxStrategy {
    fn from(color: Color) -> Self {
        match color {
            Color::White => MinimaxStrategy::Maximize,
            Color::Black => MinimaxStrategy::Minimize,
        }
    }
}

impl From<MinimaxStrategy> for Color {
    fn from(strategy: MinimaxStrategy) -> Self {
        match strategy {
            MinimaxStrategy::Minimize => Color::Black,
            MinimaxStrategy::Maximize => Color::White,
        }
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
            GameStatus::Win(color) => match color {
                Color::White => i32::MAX,
                Color::Black => i32::MIN,
            },
            GameStatus::Draw => 0,
            GameStatus::InProgress => i32::sub(
                board.count_pieces(Color::White) as i32,
                board.count_pieces(Color::Black) as i32,
            ),
        }
    }

    fn minimax(&self, board: &Board, depth: u8, strategy: MinimaxStrategy) -> (Option<Field>, i32) {
        if depth == 0 || board.status() != GameStatus::InProgress {
            return (None, self.eval(board));
        }

        let mut best_choice = (None, strategy.value());

        for field in board.valid_moves(strategy.into()) {
            let mut board = board.clone();
            board.add_piece(field, strategy.into()).unwrap();

            let (_, evaluation) = self.minimax(&board, depth - 1, strategy.other());

            match strategy {
                MinimaxStrategy::Minimize => {
                    if evaluation < best_choice.1 {
                        best_choice = (Some(field), evaluation);
                    }
                }
                MinimaxStrategy::Maximize => {
                    if evaluation > best_choice.1 {
                        best_choice = (Some(field), evaluation);
                    }
                }
            }
        }

        best_choice
    }
}

impl Player for MinimaxBot {
    fn name(&self) -> String {
        format!("Minimax Bot (depth {})", self.depth,)
    }

    fn color(&self) -> Color {
        self.color
    }

    fn turn(&self, board: &Board) -> Option<Field> {
        println!("{}", board);

        println!("{} {}\n", self.color(), self.name().bold());

        let mut sp = Spinner::new(Spinners::Dots8Bit, "Thinking".into());
        let best_move = self.minimax(board, self.depth, self.color.into());
        sp.stop();

        if let Some(field) = best_move.0 {
            println!(
                "\x1b[2K\rThe bot plays {} ({:+})",
                field.to_string(),
                best_move.1
            );
        } else {
            println!("\x1b[2K\rThe bot has no valid moves. It passes.");
        }

        print!("Press <Enter> to continue ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();

        best_move.0
    }
}
