pub mod human_player;
pub mod minimax_bot;

pub use human_player::HumanPlayer;
pub use minimax_bot::MinimaxBot;

use reversi_game::reversi::*;

pub trait Player {
    fn turn(&self, board: &Board) -> Option<Field>;
    fn color(&self) -> Color;
    fn name(&self) -> String;
    fn redraw_options(&self) -> DisplayOptions;
}
