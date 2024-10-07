use std::cmp::max;

use crate::{board::Board, piece::Colour};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    #[inline]
    pub fn is_on_board(&self) -> bool {
        !self.is_off_board()
    }

    #[inline]
    pub fn is_off_board(&self) -> bool {
        self.row < 0 || self.row > 7 || self.col < 0 || self.col > 7
    }

    #[inline]
    pub fn get_row(&self) -> i32 {
        self.row
    }

    #[inline]
    pub fn get_col(&self) -> i32 {
        self.col
    }

    #[inline]
    pub fn is_diagonal_to(&self, other: Self) -> bool {
        // Algorithm for determining whether or not two squares are diagonal
        // https://math.stackexchange.com/questions/1194565/how-to-know-if-two-points-are-diagonally-aligned
        (self.col - other.col).abs() == (self.row - other.row).abs()
    }

    #[inline]
    fn diagonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs()
    }

    #[inline]
    pub fn is_orthogonal_to(&self, other: Self) -> bool {
        (self.col == other.col) || (self.row == other.row)
    }

    #[inline]
    fn orthogonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs() + (self.row - other.row).abs()
    }

    #[inline]
    pub fn is_adjacent_to(&self, other: Self) -> bool {
        if self.is_orthogonal_to(other) {
            self.orthogonal_distance(other) == 1
        } else if self.is_diagonal_to(other) {
            self.diagonal_distance(other) == 1
        } else {
            false
        }
    }

    #[inline]
    pub fn is_below(&self, other: Self) -> bool {
        self.row < other.row
    }

    #[inline]
    pub fn is_above(&self, other: Self) -> bool {
        self.row > other.row
    }

    #[inline]
    pub fn is_left_of(&self, other: Self) -> bool {
        self.col < other.col
    }

    #[inline]
    pub fn is_right_of(&self, other: Self) -> bool {
        self.col > other.col
    }

    
    /// Get the position directly below this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_below(&self) -> Self {
        Self::new(self.row - 1, self.col)
    }

    /// Get the position directly above this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_above(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    /// Get the next square upwards from a respective player's
    /// pawn.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn pawn_up(&self, ally_color: Colour) -> Self {
        match ally_color {
            Colour::White => self.next_below(),
            Colour::Black => self.next_above(),
        }
    }

    #[inline]
    pub fn pawn_back(&self, ally_color: Colour) -> Self {
        self.pawn_up(!ally_color)
    }

    /// Get the position directly left of this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_left(&self) -> Self {
        Self::new(self.row, self.col - 1)
    }

    /// Get the position directly right of this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    #[inline]
    pub fn is_starting_pawn(&self, color: Colour) -> bool {
        match color {
            Colour::White => self.row == 1,
            Colour::Black => self.row == 6,
        }
    }

    /// Is this the starting position of the kingside rook?
    #[inline]
    pub fn is_kingside_rook(&self) -> bool {
        (self.row == 0 || self.row == 7) && self.col == 7
    }

    /// Is this the starting position of the queenside rook?
    #[inline]
    pub fn is_queenside_rook(&self) -> bool {
        (self.row == 0 || self.row == 7) && self.col == 0
    }

    #[inline]
    fn add_row(&self, drow: i32) -> Self {
        let mut result = *self;
        result.row += drow;
        result
    }

    #[inline]
    fn add_col(&self, dcol: i32) -> Self {
        let mut result = *self;
        result.col += dcol;
        result
    }

    /// Returns a vector of positions along the diagonal from `self` to `to`.
    /// If the positions are not on the same diagonal, returns an empty vector.
    /// Moves along the diagonal by determining the row and column steps 
    /// (positive or negative) and accumulates the positions in between.
    pub fn diagonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_diagonal_to(to) {
            return Vec::new();
        }

        let row_step;
        let col_step;
        if self.is_left_of(to) {
            col_step = 1;
        } else {
            col_step = -1;
        }

        if self.is_below(to) {
            row_step = 1;
        } else {
            row_step = -1;
        }

        let mut acc = *self;
        let mut result = Vec::new();
        for _ in 0..self.diagonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    pub fn orthogonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_orthogonal_to(to) {
            return Vec::new();
        }
        let mut row_step = 0;
        let mut col_step = 0;
        if self.is_left_of(to) {
            col_step = 1;
        } else if self.is_right_of(to) {
            col_step = -1;
        } else if self.is_above(to) {
            row_step = -1;
        } else if self.is_below(to) {
            row_step = 1;
        }

        let mut acc = *self;
        let mut result = Vec::new();

        for _ in 0..self.orthogonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    #[inline]
    pub fn is_knight_move(&self, other: Self) -> bool {
        (self.row - other.row).abs() == 2 && (self.col - other.col).abs() == 1
            || (self.row - other.row).abs() == 1 && (self.col - other.col).abs() == 2
    }

    //The max distance you can travel in a given direction up to and including a colision with an enemy piece
    pub fn max_travel(board: &Board, pos: Position, ally_colour: Colour, direction: &str) -> Position{
        let to: Position = match direction {
            "UP" => Position::new(Board::SIZE as i32, pos.get_col()),
            "DOWN" => Position::new(0, pos.get_col()),
            "LEFT" => Position::new(pos.get_row(), 0),
            "RIGHT" => Position::new(pos.get_row(), Board::SIZE as i32),
            "NE" => Position::new(pos.get_row(), Board::SIZE as i32),
            "NW" => Position::new(Board::SIZE as i32, pos.get_col()),
            "SE" => Position::new(Board::SIZE as i32, pos.get_col()),
            "SW" => Position::new(Board::SIZE as i32, pos.get_col()),
            _ => pos,
        };

        let mut max_travel = 0;
        if pos.row == to.row || pos.col == to.col {
            //orthogonal:
            for p in Position::orthogonals_to(&pos, to) {
                if board.has_no_piece(p) {
                    max_travel += 1
                }
                //allow move onto enemy piece but then break loop
                if board.has_enemy_piece(p, ally_colour) {
                    max_travel += 1;
                    break
                }
                if board.has_piece(p) {
                    break
                }
            }
        }
        else if (pos.row - to.row).abs() == (pos.col - to.col).abs() {
            //diagonal:
            for p in Position::diagonals_to(&pos, to) {
                if board.has_no_piece(p) {
                    max_travel += 1
                }
                //allow move onto enemy piece but then break loop
                if board.has_enemy_piece(p, ally_colour) {
                    max_travel += 1;
                    break
                }
                if board.has_piece(p) {
                    break
                }
            }
        }
        let max_travel_to: Position = match direction {
            "UP" => Position::new(pos.get_row()+max_travel, pos.get_col()),
            "DOWN" => Position::new(pos.get_row()-max_travel, pos.get_col()),
            "LEFT" => Position::new(pos.get_row(), pos.get_col()-max_travel),
            "RIGHT" => Position::new(pos.get_row(), pos.get_col()+max_travel),
            "NE" => Position::new(pos.get_row()+max_travel, pos.get_col()+max_travel),
            "NW" => Position::new(pos.get_row()+max_travel, pos.get_col()-max_travel),
            "SE" => Position::new(pos.get_row()-max_travel, pos.get_col()+max_travel),
            "SW" => Position::new(pos.get_row()-max_travel, pos.get_col()-max_travel),
            _ => pos,
        };
        max_travel_to

    }
    
}