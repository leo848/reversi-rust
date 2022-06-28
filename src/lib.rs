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
        println!("{}", board);
    }

    #[test]
    fn flip_board() {
        let mut board = Board::new();

        board.flip(Field(3, 3));
        board.flip(Field(4, 4));
        board.flip(Field(5, 5));

        assert_eq!(board[Field(3, 3)], Some(Color::Black));
        assert_eq!(board[Field(3, 4)], Some(Color::Black));
        assert_eq!(board[Field(5, 5)], None);
    }

    #[test]
    fn move_validity() {
        let mut board = Board::new();
        board[Field(2, 4)] = Some(Color::White);
        
        println!("{}", board);

        let valid = board.move_validity(Field(3, 5), Color::White);

        assert!(valid.unwrap().contains(&Field(3,4)));
    }
}
