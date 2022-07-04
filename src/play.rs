pub mod player;

pub use player::*;

use reversi::reversi::*;

use std::time::Duration;

use clap::ArgMatches;
use colored::Colorize;

pub enum Opponent {
    Human,
    Bot,
}

pub fn run(opponent: &Opponent, matches: &ArgMatches) {
    let mut board = Board::new();

    redraw_board(&board, &Default::default());

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

        let player = match counter % 2 {
            0 => &player_black,
            1 => &player_white,
            _ => unreachable!(),
        };

        redraw_board(&board, &player.redraw_options());

        let field = player.turn(&board);

        

        if let Some(field) = field {
            let mut anim_board = board.clone();
            anim_board[field] = Some(player.color());

            board
                .add_piece(field, player.color())
                .unwrap_or_else(|err| {
                    panic!("Failed to add piece `{}`: {}", field.to_string(), err);
                });

            animate_between(&anim_board, &board, Duration::from_secs(1), Default::default());
        } else {
            continue;
        }
    }

    clearscreen::clear().expect("Failed to clear screen");

    board.sort();

    redraw_board(&board, &DisplayOptions {
        empty_lines: 2,
        title: Some("Final results".into()),
        ..Default::default()
    });

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
