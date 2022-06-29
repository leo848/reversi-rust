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

    let mut player = match opponent {
        Opponent::Human => HumanPlayer::new(),
        Opponent::Bot => todo!(),
    };

    while board.status() == board::GameStatus::InProgress {}
}
