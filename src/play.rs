pub mod player;

pub use player::*;

use reversi::reversi::*;

use clap::ArgMatches;
use colored::Colorize;

pub enum Opponent {
    Human,
    Bot,
}

pub fn run(opponent: &Opponent, matches: &ArgMatches) {
    let mut board = Board::new();

    println!("{}", board);

    let player_white: Box<dyn Player> =
        Box::new(HumanPlayer::new(Color::White, "Player 1".to_string()));
    let player_black: Box<dyn Player> = match opponent {
        Opponent::Human => Box::new(HumanPlayer::new(Color::Black, "Player 2".to_string())),
        Opponent::Bot => Box::new(MinimaxBot::new(
            Color::Black,
            *matches.get_one::<u8>("depth").unwrap(),
        )),
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
                .unwrap_or_else(|err| {
                    panic!("Failed to add piece `{}`: {}", field.to_string(), err);
                }),
            None => continue,
        };
    }

    clearscreen::clear().expect("Failed to clear screen");

    board.sort();
    println!("{}\n\n", board);

    println!("{}", "Final results".bold());

    println!(
        "\n{}: {} pieces",
        player_white.color(),
        board.count_pieces(Color::White)
    );
    println!(
        "{}: {} pieces",
        player_black.color(),
        board.count_pieces(Color::Black)
    );

    match board.status() {
        GameStatus::Win(Color::White) => {
            println!("\n{}, {}", player_white.name(), "you won!".bold().green());
        }
        GameStatus::Win(Color::Black) => {
            println!("\n{}, {}", player_black.name(), "you won!".bold().green());
        }
        GameStatus::Draw => println!("{}", "Draw!".yellow()),
        _ => unreachable!(),
    }
}
