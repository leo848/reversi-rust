use super::Player;

pub struct MinimaxBot {
    color: Color,
    depth: u8,
}

impl MinimaxBot {
    pub fn new(color: Color, depth: u8) -> Self {
        MinimaxBot { color, depth }
    }

    fn eval(&self, board: &Board) -> i32 {
        match board.status() {
            GameStatus::Win(color) => {
                if color == self.color {
                    i32::MAX
                } else {
                    i32::MIN
                }
            }
            GameStatus::Draw => 0,
            GameStatus::InProgress => i32::sub(
                board.count_pieces(Color::White) as i32,
                board.count_pieces(Color::Black) as i32,
            ),
        }
    }

    fn minimax(&self, board: &Board, depth: u8, maximize: bool) -> (Option<Field>, i32) {
        if depth == 0 || board.status() != GameStatus::InProgress {
            return (None, self.eval(board));
        }

        let mut best_choice = (None, i32::MIN);

    }
}

impl Player for MinimaxBot {
    fn name(&self) -> String {
        format!("Minimax Bot (depth {})", self.depth)
    }

    fn color(&self) -> Color {
        self.color
    }

    fn turn(&self, board: &Board) -> Option<Field> {
        None
    }
}
