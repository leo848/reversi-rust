use crate::reversi::*;

use std::time::Duration;

use colored::Colorize;
use itertools::Itertools;
use split_iter::Splittable;

#[derive(Debug)]
pub struct DisplayOptions {
    pub clear_screen: bool,
    pub color: Option<Color>,

    pub bold_title: bool,
    pub title: Option<String>,

    pub empty_lines: u8,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            clear_screen: true,
            color: None,
            title: None,
            bold_title: true,
            empty_lines: 1,
        }
    }
}

pub fn redraw_board(board: &Board, options: &DisplayOptions) {
    if options.clear_screen {
        clearscreen::clear().unwrap();
    }

    if let Some(title) = &options.title {
        println!(
            "{}\n",
            if options.bold_title {
                title.bold()
            } else {
                title.normal()
            }
        );
    }

    match options.color {
        None => println!("{}", board),
        Some(Color::White) => println!("{:w>}", board),
        Some(Color::Black) => println!("{:b>}", board),
    }

    print!("{}", "\n".repeat(options.empty_lines as usize));
}

pub fn animate_between(
    board_before: &Board,
    board_after: &Board,
    animation_time: Duration,
    options: DisplayOptions,
) {
    let boards_between = animation_frames(board_before, board_after);

    let sleep_time = animation_time / boards_between.len() as u32;

    for board in boards_between {
        std::thread::sleep(sleep_time / 2);
        redraw_board(&board, &options);
        std::thread::sleep(sleep_time / 2);
    }
}

pub fn animate_by(
    initial_board: &Board,
    captures: &[Field],
    time_per_flip: Duration,
    options: DisplayOptions,
) {
    use std::thread::sleep;

    let mut anim_board = initial_board.clone();

    sleep(time_per_flip);
    redraw_board(&anim_board, &options);
    sleep(time_per_flip / 2);

    for capture in captures {
        sleep(time_per_flip / 2);

        anim_board.flip(*capture);
        redraw_board(&anim_board, &options);

        sleep(time_per_flip / 2);
    }
}

fn animation_frames(board_before: &Board, board_after: &Board) -> Vec<Board> {
    let mut boards_between = vec![board_before.clone()];

    let mut board_slice = board_before.clone();

    for x in 0..8 {
        for y in 0..8 {
            if board_before[Field(x, y)] != board_after[Field(x, y)] {
                board_slice[Field(x, y)] = board_after[Field(x, y)];
                boards_between.push(board_slice.clone());
            }
        }
    }

    boards_between
}

pub fn animate_results(mut board: Board, time_per_flip: Duration, options: &DisplayOptions) {
    use std::thread::sleep;

    board.sort();

    let mut fields = Field::all().map(|field| board[field]).collect::<Vec<_>>();
    fields.sort_by_key(|piece| match piece {
        Some(Color::White) => 0,
        None => 1,
        Some(Color::Black) => 2,
    });
    let (white_fields, black_fields) = fields
        .into_iter()
        .enumerate()
        .map(|(i, piece)| (Field(i % 8, i / 8), piece))
        .filter(|(_, c)| c.is_some())
        .split(|(_, c)| c == &Some(Color::Black));

    let display_fields =
        white_fields.interleave(black_fields.collect::<Vec<_>>().into_iter().rev());

    let mut anim_board = Board::empty();

    for (index, color) in display_fields {
        sleep(time_per_flip / 2);
        anim_board[index] = color;
        redraw_board(&anim_board, options);
        sleep(time_per_flip / 2);
    }
}
