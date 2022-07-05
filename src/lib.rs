#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]

pub mod reversi;

pub use crate::reversi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_board() {
        let board = Board::new();
        assert_eq!(board[Field(3, 3)], Some(Color::White));
        assert_eq!(board[Field(3, 4)], Some(Color::Black));
        assert_eq!(board[Field(4, 6)], None);
    }

    #[test]
    fn empty_board() {
        let board = Board::empty();
        for x in 0..8 {
            for y in 0..8 {
                assert_eq!(board[Field(x, y)], None);
            }
        }
    }

    #[test]
    fn display_board() {
        let board = Board::new();
        redraw_board(&board, &DisplayOptions::default());
    }

    #[test]
    fn move_validity() {
        let mut board = Board::new();
        board[Field(2, 4)] = Some(Color::White);

        redraw_board(&board, &DisplayOptions::default());

        let valid = board.move_validity(Field(3, 5), Color::White);
        assert!(valid.unwrap().contains(&Field(3, 4)));
    }

    #[test]
    fn valid_moves() {
        let board = Board::new();
        assert_eq!(
            board.valid_moves(Color::White),
            vec![Field(2, 4), Field(3, 5), Field(4, 2), Field(5, 3)]
        );
    }

    #[test]
    fn board_status() {
        use crate::reversi::Color::{Black, White};
        let mut board = Board::new();
        board[Field(2, 4)] = Some(White);
        board[Field(3, 5)] = Some(Black);

        assert_eq!(board.status(), GameStatus::InProgress);
    }
}
