pub mod player;

use reversi::reversi::*;

use clap::ArgMatches;

pub enum Opponent {
    Human,
    Bot,
}

pub fn run(opponent: Opponent, matches: &ArgMatches) {
    let board = Board::new();

    println!("{}", board);
} 
