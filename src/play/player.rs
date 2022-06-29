use reversi::reversi::*;

trait Player {
    fn turn(&self, board: &Board) -> Field;
}
