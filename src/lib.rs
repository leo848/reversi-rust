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
    }
}
