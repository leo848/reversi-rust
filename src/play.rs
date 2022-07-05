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
    let animation_speed: Duration = match matches.value_of("animation-speed") {
        Some("slow") => Duration::from_millis(800),
        Some("medium") => {
            if matches.is_present("no-animation") {
                Duration::ZERO
            } else {
                Duration::from_millis(300)
            }
        }
        Some("fast") => Duration::from_millis(100),
        _ => unreachable!(),
    };

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

            let mut captures = board
                .add_piece(field, player.color())
                .unwrap_or_else(|err| {
                    panic!("Failed to add piece `{}`: {}", field.to_string(), err);
                });

            captures.sort_by_key(|capture| {
                usize::wrapping_sub(field.0, capture.0).wrapping_pow(2)
                    + usize::wrapping_sub(field.1, capture.1).wrapping_pow(2)
            });

            animate_by(&anim_board, &captures, animation_speed, Default::default());
        } else {
            continue;
        }
    }

    // board.sort();

    // redraw_board(
    //     &board,
    //     &DisplayOptions {
    //         empty_lines: 2,
    //         title: Some("Final results".into()),
    //         ..Default::default()
    //     },
    // );

    animate_results(board.clone(), animation_speed, &Default::default());

    println!(
        "{}: {} pieces",
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
