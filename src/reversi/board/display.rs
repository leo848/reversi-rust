use crate::reversi::*;

use colored::Colorize;

#[derive(Debug)]
#[non_exhaustive]
pub struct Options {
    pub clear_screen: bool,
    pub color: Option<Color>,

    pub bold_title: bool,
    pub title: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            clear_screen: true,
            color: None,
            title: None,
            bold_title: false,
        }
    }
}

pub fn redraw_board(board: &Board, options: Options) {
    if options.clear_screen {
        clearscreen::clear().unwrap();
    }

    if let Some(title) = options.title {
        println!(
            "{}",
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
}
