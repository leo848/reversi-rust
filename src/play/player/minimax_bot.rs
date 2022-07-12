use super::Player;
use reversi_game::reversi::*;

use std::{
    io::{self, Write},
    ops::Sub,
};

use colored::Colorize;
use spinners::{Spinner, Spinners};

/// A strategy for the minimax bot.
#[derive(Debug, Clone, Copy)]
pub enum MinimaxStrategy {
    /// Minimize the board evaluation.
    Minimize,
    /// Maximize the board evaluation.
    Maximize,
}

impl MinimaxStrategy {
    /// Get the opposite of this strategy.
    ///
    /// # Examples
    /// ```
    /// use reversi::player::minimax_bot::MinimaxStrategy;
    /// let min = MinimaxStrategy::Minimize;
    /// let max = MinimaxStrategy::Maximize;
    ///
    /// assert_eq!(min.opposite(), max);
    /// assert_eq!(max.opposite(), min);
    /// ```
    fn other(&self) -> MinimaxStrategy {
        match self {
            MinimaxStrategy::Minimize => MinimaxStrategy::Maximize,
            MinimaxStrategy::Maximize => MinimaxStrategy::Minimize,
        }
    }

    /// Get the most suboptimal evaluation for this strategy.
    ///
    /// # Examples
    /// ```
    /// use reversi::player::minimax_bot::MinimaxStrategy;
    ///
    /// let min = MinimaxStrategy::Minimize;
    /// let max = MinimaxStrategy::Maximize;
    ///
    /// assert_eq!(min.worst_value()), i32::MAX);
    /// assert_eq!(max.worst_value()), i32::MIN);
    /// ```
    fn worst_value(&self) -> i32 {
        match self {
            MinimaxStrategy::Minimize => i32::MAX,
            MinimaxStrategy::Maximize => i32::MIN,
        }
    }
}

/// A color can be turned into a `MinimaxStrategy`.
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

/// A `MinimaxBot` is a player that plays using the minimax algorithm.
pub struct MinimaxBot {
    color: Color,
    depth: u8,
}

impl MinimaxBot {
    /// Create a new `MinimaxBot` with the given color and depth.
    pub fn new(color: Color, depth: u8) -> Self {
        MinimaxBot { color, depth }
    }

    /// Evaluate a given board.
    /// This is the evaluation function used by the minimax algorithm.
    ///
    /// # Examples
    /// ```
    /// # use reversi::player::minimax_bot::MinimaxBot;
    /// # use reversi::reversi::*;
    /// # use std::str::FromStr;
    ///
    /// let mut board = Board::new();
    /// let bot = MinimaxBot::new(Color::White, 1);
    ///
    /// assert_eq!(bot.eval(&board), 0);
    ///
    /// board.add_piece(Field::from_str("d3").unwrap(), Color::White);
    /// assert_eq!(bot.eval(&board), +3);
    ///
    /// board.add_piece(Field::from_str("e3").unwrap(), Color::Black);
    /// assert_eq!(bot.eval(&board), 0);
    ///
    /// board.add_piece(Field::from_str("f5").unwrap(), Color::White);
    /// assert_eq!(bot.eval(&board), +5);
    /// ```
    pub fn eval(&self, board: &Board) -> i32 {
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

    /// Find the best move using the minimax algorithm.
    /// This is the function used by the `MinimaxBot` to find the best move.
    ///
    /// # Arguments
    /// * `board` - The board to evaluate.
    /// * `depth` - The depth of the search. This is the number of moves to look ahead.
    /// * `strategy` - The strategy to use.
    ///
    /// # Examples
    /// ```
    /// # use reversi::player::minimax_bot::MinimaxBot;
    /// # use reversi::reversi::*;
    ///
    /// let mut board = Board::new();
    /// let bot = MinimaxBot::new(Color::White, 1);
    ///
    /// assert!(board.valid_moves().contains(bot.minimax(&board, 2, MinimaxStrategy::Maximize)));
    /// ```
    ///
    /// ```
    /// # use reversi::player::minimax_bot::MinimaxBot;
    /// # use reversi::reversi::*;
    ///
    /// let mut board = Board::new();
    /// let bot1 = MinimaxBot::new(Color::White, 1);
    /// let bot2 = MinimaxBot::new(Color::Black, 3);
    ///
    /// let mut counter = 0;
    /// while board.status() == GameStatus::InProgress {
    ///    counter += 1;
    ///    if (counter % 2) == 0 {
    ///    board.add_piece(bot1.minimax(&board, 1, MinimaxStrategy::Maximize), Color::White);
    ///    } else {
    ///    board.add_piece(bot2.minimax(&board, 3, MinimaxStrategy::Minimize), Color::Black);
    ///    }
    /// }
    ///
    /// assert_eq!(board.status(), GameStatus::Win(Color::Black));
    /// ```
    pub fn minimax(
        &self,
        board: &Board,
        depth: u8,
        strategy: MinimaxStrategy,
    ) -> (Option<Field>, i32) {
        if depth == 0 || board.status() != GameStatus::InProgress {
            return (None, self.eval(board));
        }

        let mut best_choice = (None, strategy.worst_value());

        for field in board.valid_moves(strategy.into()) {
            let mut board = board.clone();
            board.add_piece(field, strategy.into()).unwrap();

            let (_, evaluation) = self.minimax(&board, depth - 1, strategy.other());

            match strategy {
                MinimaxStrategy::Minimize => {
                    if evaluation <= best_choice.1 {
                        best_choice = (Some(field), evaluation);
                    }
                }
                MinimaxStrategy::Maximize => {
                    if evaluation >= best_choice.1 {
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

    /// Make a move using the minimax algorithm interactively.
    /// The interactive part of this includes displaying a spinner while the bot is thinking.
    fn turn(&self, board: &Board) -> Option<Field> {
        redraw_board(board, &Default::default());

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

    fn redraw_options(&self) -> DisplayOptions {
        Default::default()
    }
}
