use reversi::reversi::*;

pub trait Player {
    fn turn(&self, board: &Board) -> Field;
    fn color(&self) -> Color;
}

pub struct HumanPlayer {}

impl HumanPlayer {
    pub fn new() -> Self {
        HumanPlayer {}
    }
}
