pub mod player;

pub use player::*;

use reversi::reversi::*;

use clap::ArgMatches;

pub enum Opponent {
    Human,
    Bot,
}

pub fn run(opponent: Opponent, _matches: &ArgMatches) {
    let board = Board::new();

    println!("{}", board);

    let player_white = HumanPlayer::new(Color::White, "Human".to_string());
    let player_black = match opponent {
        Opponent::Human => HumanPlayer::new(Color::Black, "Human".to_string()),
        Opponent::Bot => todo!(),
    };

    while board.status() == board::GameStatus::InProgress {
        let player = if board.turn() == Color::White {
            &player_white
        } else {
            &player_black
        };
        let field = player.turn(&board);
        println!("{}", board);
    }
}
