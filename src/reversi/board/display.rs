use crate::reversi::*;

use std::time::Duration;

use colored::Colorize;

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

pub fn animate_between(board_before: &Board, board_after: &Board, animation_time: Duration, options: DisplayOptions) {
    let boards_between = animation_frames(board_before, board_after);

    let sleep_time = animation_time / boards_between.len() as u32;

    for board in boards_between {
        std::thread::sleep(sleep_time / 2);
        redraw_board(&board, &options);
        std::thread::sleep(sleep_time / 2);
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
