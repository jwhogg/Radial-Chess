use std::cmp::max;

use crate::{board::Board, piece::Colour};


pub const A1: Position = Position::new(0, 0);
pub const A2: Position = Position::new(1, 0);
pub const A3: Position = Position::new(2, 0);
pub const A4: Position = Position::new(3, 0);
pub const A5: Position = Position::new(4, 0);
pub const A6: Position = Position::new(5, 0);
pub const A7: Position = Position::new(6, 0);
pub const A8: Position = Position::new(7, 0);

pub const B1: Position = Position::new(0, 1);
pub const B2: Position = Position::new(1, 1);
pub const B3: Position = Position::new(2, 1);
pub const B4: Position = Position::new(3, 1);
pub const B5: Position = Position::new(4, 1);
pub const B6: Position = Position::new(5, 1);
pub const B7: Position = Position::new(6, 1);
pub const B8: Position = Position::new(7, 1);

pub const C1: Position = Position::new(0, 2);
pub const C2: Position = Position::new(1, 2);
pub const C3: Position = Position::new(2, 2);
pub const C4: Position = Position::new(3, 2);
pub const C5: Position = Position::new(4, 2);
pub const C6: Position = Position::new(5, 2);
pub const C7: Position = Position::new(6, 2);
pub const C8: Position = Position::new(7, 2);

pub const D1: Position = Position::new(0, 3);
pub const D2: Position = Position::new(1, 3);
pub const D3: Position = Position::new(2, 3);
pub const D4: Position = Position::new(3, 3);
pub const D5: Position = Position::new(4, 3);
pub const D6: Position = Position::new(5, 3);
pub const D7: Position = Position::new(6, 3);
pub const D8: Position = Position::new(7, 3);

pub const E1: Position = Position::new(0, 4);
pub const E2: Position = Position::new(1, 4);
pub const E3: Position = Position::new(2, 4);
pub const E4: Position = Position::new(3, 4);
pub const E5: Position = Position::new(4, 4);
pub const E6: Position = Position::new(5, 4);
pub const E7: Position = Position::new(6, 4);
pub const E8: Position = Position::new(7, 4);

pub const F1: Position = Position::new(0, 5);
pub const F2: Position = Position::new(1, 5);
pub const F3: Position = Position::new(2, 5);
pub const F4: Position = Position::new(3, 5);
pub const F5: Position = Position::new(4, 5);
pub const F6: Position = Position::new(5, 5);
pub const F7: Position = Position::new(6, 5);
pub const F8: Position = Position::new(7, 5);

pub const G1: Position = Position::new(0, 6);
pub const G2: Position = Position::new(1, 6);
pub const G3: Position = Position::new(2, 6);
pub const G4: Position = Position::new(3, 6);
pub const G5: Position = Position::new(4, 6);
pub const G6: Position = Position::new(5, 6);
pub const G7: Position = Position::new(6, 6);
pub const G8: Position = Position::new(7, 6);

pub const H1: Position = Position::new(0, 7);
pub const H2: Position = Position::new(1, 7);
pub const H3: Position = Position::new(2, 7);
pub const H4: Position = Position::new(3, 7);
pub const H5: Position = Position::new(4, 7);
pub const H6: Position = Position::new(5, 7);
pub const H7: Position = Position::new(6, 7);
pub const H8: Position = Position::new(7, 7);

pub fn interpret_position(input: &str) -> Option<Position> {
    match input {
        "A1" => Some(A1),
        "A2" => Some(A2),
        "A3" => Some(A3),
        "A4" => Some(A4),
        "A5" => Some(A5),
        "A6" => Some(A6),
        "A7" => Some(A7),
        "A8" => Some(A8),
        "B1" => Some(B1),
        "B2" => Some(B2),
        "B3" => Some(B3),
        "B4" => Some(B4),
        "B5" => Some(B5),
        "B6" => Some(B6),
        "B7" => Some(B7),
        "B8" => Some(B8),
        "C1" => Some(C1),
        "C2" => Some(C2),
        "C3" => Some(C3),
        "C4" => Some(C4),
        "C5" => Some(C5),
        "C6" => Some(C6),
        "C7" => Some(C7),
        "C8" => Some(C8),
        "D1" => Some(D1),
        "D2" => Some(D2),
        "D3" => Some(D3),
        "D4" => Some(D4),
        "D5" => Some(D5),
        "D6" => Some(D6),
        "D7" => Some(D7),
        "D8" => Some(D8),
        "E1" => Some(E1),
        "E2" => Some(E2),
        "E3" => Some(E3),
        "E4" => Some(E4),
        "E5" => Some(E5),
        "E6" => Some(E6),
        "E7" => Some(E7),
        "E8" => Some(E8),
        "F1" => Some(F1),
        "F2" => Some(F2),
        "F3" => Some(F3),
        "F4" => Some(F4),
        "F5" => Some(F5),
        "F6" => Some(F6),
        "F7" => Some(F7),
        "F8" => Some(F8),
        "G1" => Some(G1),
        "G2" => Some(G2),
        "G3" => Some(G3),
        "G4" => Some(G4),
        "G5" => Some(G5),
        "G6" => Some(G6),
        "G7" => Some(G7),
        "G8" => Some(G8),
        "H1" => Some(H1),
        "H2" => Some(H2),
        "H3" => Some(H3),
        "H4" => Some(H4),
        "H5" => Some(H5),
        "H6" => Some(H6),
        "H7" => Some(H7),
        "H8" => Some(H8),
        _ => None, // Return None for invalid inputs
    }
}


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