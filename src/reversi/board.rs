#![allow(clippy::module_name_repetitions)]

pub mod display;

pub use display::{animate_between, animate_by, animate_results, redraw_board, DisplayOptions};

use crate::reversi::Color;

use std::{
    cmp::Ordering::{Equal, Greater, Less},
    error::Error,
    fmt,
    ops::{Deref, DerefMut, Index, IndexMut, Not},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Field(pub usize, pub usize);

impl Field {
    /// Check if the field is in bounds.
    ///
    /// # Examples
    /// ```
    /// # use reversi::Field;
    /// assert!(Field(0, 3).in_bounds());
    /// assert!(Field(7, 5).in_bounds());
    /// assert!(!Field(3, 8).in_bounds());
    /// ```
    pub fn in_bounds(&self) -> bool {
        self.0 < 8 && self.1 < 8
    }

    /// Return all possible fields that are in bounds.
    ///
    /// # Examples
    /// ```
    /// # use reversi::Field;
    /// let possible_fields = Field::all();
    /// assert_eq!(possible_fields.count(), 64);
    /// ```
    pub fn all() -> impl DoubleEndedIterator<Item = Field> {
        (0..8).flat_map(move |x| (0..8).map(move |y| Self(x, y)))
    }

    pub fn from_board_move(input: &str, board: &Board) -> Result<Self, PlaceError> {
        let index = input.parse::<usize>().or(Err(PlaceError::InvalidNumber))?;
        board
            .valid_moves(Color::White)
            .get(index)
            .ok_or(PlaceError::OutOfBounds)
            .map(|&field| field)
    }

    pub fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();

        for delta_x in [-1_i8, 0, 1] {
            for delta_y in [-1_i8, 0, 1] {
                let (x, y) = (self.0 as i8 + delta_x, self.1 as i8 + delta_y);
                let (x, y) = (x.try_into(), y.try_into());

                let (x, y) = match (x, y) {
                    (Ok(x), Ok(y)) => (x, y),
                    _ => continue,
                };

                let neighbor = Field(x, y);
                if neighbor.in_bounds() {
                    neighbors.push(neighbor);
                }
            }
        }

        neighbors
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        assert!(self.in_bounds());
        ('a'..='h').nth(self.0).unwrap().to_string() + &(8 - self.1).to_string()
    }
}

impl FromStr for Field {
    type Err = PlaceError;

    /// Parse a field from a string.
    /// The string must be in the format `a8` or `h1`.
    ///
    /// # Examples
    /// ```
    /// # use reversi::Field;
    /// # use std::str::FromStr;
    /// let field = Field::from_str("a8").unwrap();
    /// assert_eq!(field, Field(0, 0));
    ///
    /// let field2 = Field::from_str("h1").unwrap();
    /// assert_eq!(field2, Field(7, 7));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let x = chars.next().ok_or(PlaceError::InvalidLength)?;
        let y = chars
            .next()
            .ok_or(PlaceError::InvalidLength)
            .map(|c| c.to_digit(10).ok_or(PlaceError::InvalidNumber))?;
        let y: usize = y?.try_into().map_err(|_| PlaceError::InvalidNumber)?;
        if chars.next().is_some() {
            Err(PlaceError::InvalidLength)
        } else {
            Ok(Self(
                ('a'..='h')
                    .position(|c| c == x)
                    .ok_or(PlaceError::OutOfBounds)?,
                usize::checked_sub(8, y).ok_or(PlaceError::OutOfBounds)?,
            ))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PlaceError {
    InvalidLength,
    InvalidNumber,
    Occupied,
    OutOfBounds,
    CapturesNone,
}

impl fmt::Display for PlaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlaceError::InvalidLength => write!(f, "Invalid length"),
            PlaceError::InvalidNumber => write!(f, "Invalid number"),
            PlaceError::Occupied => write!(f, "Field is already occupied"),
            PlaceError::OutOfBounds => write!(f, "Field is out of bounds"),
            PlaceError::CapturesNone => write!(f, "Field captures no pieces"),
        }
    }
}

impl Error for PlaceError {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GameStatus {
    InProgress,
    Win(Color),
    Draw,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Board(pub [[Option<Color>; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        let mut new_board = Board::empty();

        for x in 3..=4 {
            for y in 3..=4 {
                new_board[Field(x, y)] = match (x + y) % 2 {
                    0 => Some(Color::White),
                    1 => Some(Color::Black),
                    _ => unreachable!(),
                }
            }
        }

        new_board
    }

    /// Returns a new empty board.
    pub fn empty() -> Self {
        Board([[None; 8]; 8])
    }

    /// Flip a piece on the board.
    fn flip(&mut self, field: Field) {
        self[field] = self[field].map(Color::other);
    }

    /// Count the amount of pieces of a given color.
    ///
    /// # Examples
    /// ```
    /// # use reversi::{Board, Field, Color};
    /// let mut board = Board::new();
    ///
    /// assert_eq!(board.count_pieces(Color::Black), 2);
    /// board.add_piece(Field(2, 4), Color::White);
    /// assert_eq!(board.count_pieces(Color::Black), 1);
    /// ```
    pub fn count_pieces(&self, color: Color) -> usize {
        Field::all()
            .filter(|&field| self[field] == Some(color))
            .count()
    }

    /// Check whose turn it is.
    ///
    /// # Examples
    /// ```
    /// # use reversi::{Board, Field, Color};
    /// let mut board = Board::new();
    /// assert_eq!(board.turn(), Color::White);
    /// board.add_piece(Field(2, 4), Color::White);
    /// assert_eq!(board.turn(), Color::Black);
    /// ```
    pub fn turn(&self) -> Color {
        match Field::all().filter(|&field| self[field].is_some()).count() % 2 {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!(),
        }
    }

    /// Return the game status, assuming the game is done.
    fn final_status(&self) -> GameStatus {
        match self
            .count_pieces(Color::White)
            .cmp(&self.count_pieces(Color::Black))
        {
            Less => GameStatus::Win(Color::Black),
            Greater => GameStatus::Win(Color::White),
            Equal => GameStatus::Draw,
        }
    }

    /// Check for the game status.
    ///
    /// # Examples
    /// ```
    /// # use reversi::{Board, Field, Color, GameStatus};
    /// let mut board = Board::new();
    /// assert_eq!(board.status(), GameStatus::InProgress);
    /// ```
    pub fn status(&self) -> GameStatus {
        if Field::all().all(|field| self[field].is_some()).not() {
            match (
                self.count_pieces(Color::White),
                self.count_pieces(Color::Black),
            ) {
                (0, _) => GameStatus::Win(Color::Black),
                (_, 0) => GameStatus::Win(Color::White),
                _ => {
                    if self.valid_moves(Color::White).is_empty()
                        && self.valid_moves(Color::Black).is_empty()
                    {
                        self.final_status()
                    } else {
                        GameStatus::InProgress
                    }
                }
            }
        } else {
            self.final_status()
        }
    }

    /// Check if a given move is valid.
    ///
    /// # Returns
    /// - A vector of fields that are captured by the move, if the move is valid.
    /// - A `PlaceError` if the move is invalid.
    pub fn move_validity(&self, field: Field, color: Color) -> Result<Vec<Field>, PlaceError> {
        if !field.in_bounds() {
            Err(PlaceError::OutOfBounds)?;
        }

        if self[field].is_some() {
            Err(PlaceError::Occupied)?;
        }

        if field.neighbors().iter().all(|&field| self[field].is_none()) {
            Err(PlaceError::CapturesNone)?;
        }

        let captured_pieces: Vec<Field> = Field::all()
            .filter(|&other| self[other] == Some(color)) // needs to be the same color
            .filter_map(|other| Board::line_between((field, other))) // a line between the two
            // fields has to exist
            .filter(|line| line.iter().all(|&field| self[field] == Some(color.other())))
            .flatten()
            .collect();

        if captured_pieces.is_empty() {
            Err(PlaceError::CapturesNone)?;
        }

        for piece in &captured_pieces {
            let mut counter = 0;
            for other_piece in &captured_pieces {
                if other_piece == piece {
                    counter += 1;
                }
            }
            assert!(counter == 1, "Captured pieces are not unique");
        }

        Ok(captured_pieces)
    }

    /// Check if a given move is valid.
    pub fn is_valid(&self, field: Field, color: Color) -> bool {
        self.move_validity(field, color).is_ok()
    }

    /// Return all valid moves a given color can make.
    pub fn valid_moves(&self, color: Color) -> Vec<Field> {
        Field::all()
            .filter(|&field| self.move_validity(field, color).is_ok())
            .collect()
    }

    /// Add a piece to the board and execute all captures.
    ///
    /// # Returns
    /// see `move_validity`
    pub fn add_piece(&mut self, field: Field, color: Color) -> Result<Vec<Field>, PlaceError> {
        let captured_pieces = self.move_validity(field, color)?;

        self.add_piece_unchecked(field, color);

        for &captured_piece in &captured_pieces {
            self.flip(captured_piece);
        }

        Ok(captured_pieces)
    }

    /// Set a field to a color.
    fn add_piece_unchecked(&mut self, field: Field, color: Color) {
        self[field] = Some(color);
    }

    /// Calculate a line (horizontal, vertical or diagonal) between two fields.
    ///
    /// # Returns
    /// A vector of fields that are part of the line, or None if no valid line exists.
    fn line_between(fields: (Field, Field)) -> Option<Vec<Field>> {
        let (Field(x1, y1), Field(x2, y2)) = fields;

        let range_x = || x1.min(x2)..=x2.max(x1);
        let range_y = || y1.min(y2)..=y2.max(y1);

        if x1 == x2 {
            // Vertical line
            Some(range_y().map(|y| Field(x1, y)).collect())
        } else if y1 == y2 {
            // Horizontal line
            Some(range_x().map(|x| Field(x, y1)).collect())
        } else if usize::abs_diff(x1, x2) == usize::abs_diff(y1, y2) {
            if (x1 > x2 && y1 > y2) || (x1 < x2 && y1 < y2) {
                // Diagonal line: \
                Some(
                    (range_x())
                        .zip(range_y())
                        .map(|(x, y)| Field(x, y))
                        .collect(),
                )
            } else {
                // Diagonal line: /
                Some(
                    (range_x())
                        .zip(range_y().rev())
                        .map(|(x, y)| Field(x, y))
                        .collect(),
                )
            }
        } else {
            // No line
            None
        }
        .and_then(|line: Vec<Field>| if line.len() < 3 { None } else { Some(line) })
        .map(|line| line[1..line.len() - 1].to_vec())
    }

    pub fn fmt_by_color(&self, f: &mut fmt::Formatter, color: Option<Color>) -> fmt::Result {
        let valid_moves = color.map(|color| self.valid_moves(color));
        writeln!(f, "╭──{}──╮", "──┬──".repeat(self.len() - 1))?;
        for y in 0..self.len() {
            if y != 0 {
                writeln!(f, "├──{}──┤", "──┼──".repeat(self.len() - 1))?;
            }
            for x in 0..self.len() {
                write!(f, "│")?;
                match self[Field(x, y)] {
                    Some(color) => write!(f, " {} ", color)?,
                    None => match valid_moves {
                        Some(ref moves) if moves.contains(&Field(x, y)) => {
                            write!(f, " {:2} ", Field(x, y).to_string())?;
                        }
                        _ => write!(f, "    ")?,
                    },
                }
                if x == self.len() - 1 {
                    write!(f, "│")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "╰──{}──╯", "──┴──".repeat(self.len() - 1))?;

        Ok(())
    }

    /// Sorts the board for displaying purposes.
    pub fn sort(&mut self) {
        let (white_count, black_count) = (
            self.count_pieces(Color::White),
            self.count_pieces(Color::Black),
        );
        let none_count = self.len() * self.len() - white_count - black_count;

        for (index, field) in Field::all()
            .map(|field| Field(field.1, field.0))
            .rev()
            .enumerate()
        {
            if index < white_count {
                self[field] = Some(Color::White);
            } else if index < white_count + none_count {
                self[field] = None;
            } else {
                self[field] = Some(Color::Black);
            }
        }
    }
}

impl Index<Field> for Board {
    type Output = Option<Color>;

    fn index(&self, field: Field) -> &Self::Output {
        &self.0[field.1][field.0]
    }
}

impl IndexMut<Field> for Board {
    fn index_mut(&mut self, field: Field) -> &mut Self::Output {
        &mut self.0[field.1][field.0]
    }
}

impl fmt::Display for Board {
    /// Display the board in a human-readable format.
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match f.fill() {
            'w' => self.fmt_by_color(f, Some(Color::White))?,
            'b' => self.fmt_by_color(f, Some(Color::Black))?,
            _ => self.fmt_by_color(f, None)?,
        }

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Deref for Board {
    type Target = [[Option<Color>; 8]; 8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
