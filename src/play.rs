pub mod player;

pub use player::*;

use reversi::reversi::*;

use clap::ArgMatches;

pub enum Opponent {
    Human,
    Bot,
}

pub fn run(opponent: Opponent, _matches: &ArgMatches) {
    let mut board = Board::new();

    println!("{}", board);

    let player_white = HumanPlayer::new(Color::White, "Player 1".to_string());
    let player_black = match opponent {
        Opponent::Human => HumanPlayer::new(Color::Black, "Player 2".to_string()),
        Opponent::Bot => todo!(),
    };

    let mut counter = 0;
    while board.status() == board::GameStatus::InProgress {
        counter += 1;

        clearscreen::clear().expect("Failed to clear screen");
        let player = match counter % 2 {
            0 => &player_black,
            1 => &player_white,
            _ => unreachable!(),
        };

        let field = player.turn(&board);

        match field {
            Some(field) => board
                .add_piece(field, player.color())
                .expect("Failed to add piece"),
            None => continue,
        };
    }

    println!("Checking for the winner...");
}
