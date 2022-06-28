pub mod reversi;

pub use crate::reversi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_board() {
        let board = Board::new();
        assert_eq!(board[3][3], Some(Color::White));
        assert_eq!(board[3][4], Some(Color::Black));
        assert_eq!(board[4][6], None);
    }

    #[test]
    fn empty_board() {
        let board = Board::empty();
        for x in 0..8 {
            for y in 0..8 {
                assert_eq!(board[x][y], None);
            }
        }
    }

    #[test]
    fn display_board() {
        let board = Board::new();
        println!("{}", board);
    }
}
