use std::ops::Not;
use std::{fmt};

use crate::board::Board;
use crate::position::{self, Position};

// src/piece.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colour {
    Black,
    White
}

impl Not for Colour {
    type Output = Colour;

    fn not(self) -> Self::Output {
        match self {
            Colour::Black => Colour::White,
            Colour::White => Colour::Black
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum Piece {
    King(Colour, Position),
    Queen(Colour, Position),
    Rook(Colour, Position),
    Bishop(Colour, Position),
    Knight(Colour, Position),
    Pawn(Colour, Position),
    Captured(Colour), //todo: need to update the logic so captured peices keep the type they were but have no pos
}

impl Piece {
    pub fn new(piece_type: PieceType, colour: Colour, position: Position) -> Self {
        match piece_type {
            PieceType::King => Piece::King(colour, position),
            PieceType::Queen => Piece::Queen(colour, position),
            PieceType::Rook => Piece::Rook(colour, position),
            PieceType::Bishop => Piece::Bishop(colour, position),
            PieceType::Knight => Piece::Knight(colour, position),
            PieceType::Pawn => Piece::Pawn(colour, position),
        }
    }

    pub fn get_pos(&self) -> Position {
        match self {
            Piece::King(_, pos) => *pos,
            Piece::Queen(_, pos) => *pos,
            Piece::Rook(_, pos) => *pos,
            Piece::Bishop(_, pos) => *pos,
            Piece::Knight(_, pos) => *pos,
            Piece::Pawn(_, pos) => *pos,
            Piece::Captured(_) => panic!("Trying to get Position of captured piece!"),
        }
    }

    pub fn get_piece_type(&self) -> PieceType {
        match self {
            Piece::King(_,_) => PieceType::King,
            Piece::Queen(_,_) => PieceType::Queen,
            Piece::Rook(_,_) => PieceType::Rook,
            Piece::Bishop(_,_) => PieceType::Bishop,
            Piece::Knight(_,_) => PieceType::Knight,
            Piece::Pawn(_,_) => PieceType::Pawn,
            Piece::Captured(_) => panic!("Trying to get Piece type of captured piece!"),
        }
    }

    pub fn get_colour(&self) -> Colour {
        match self {
            Piece::King(colour, _) => *colour,
            Piece::Queen(colour, _) => *colour,
            Piece::Rook(colour, _) => *colour,
            Piece::Bishop(colour, _) => *colour,
            Piece::Knight(colour, _) => *colour,
            Piece::Pawn(colour, _) => *colour,
            Piece::Captured(_) => panic!("Trying to get Colour of captured piece!"),
        }
    }

    pub fn set_captured(&mut self) {
        let colour = match self {
            Piece::King(col, _) |
            Piece::Queen(col, _) |
            Piece::Rook(col, _) |
            Piece::Bishop(col, _) |
            Piece::Knight(col, _) |
            Piece::Pawn(col, _) => *col,

            Piece::Captured(col) => {
                return;
            }
        };
        *self = Piece::Captured(colour);
    }

    pub fn set_pos(&mut self, new_pos: Position) {
        match self {
            Piece::King(_, pos) => *pos = new_pos,
            Piece::Queen(_, pos) => *pos = new_pos,
            Piece::Rook(_, pos) => *pos = new_pos,
            Piece::Bishop(_, pos) => *pos = new_pos,
            Piece::Knight(_, pos) => *pos = new_pos,
            Piece::Pawn(_, pos) => *pos = new_pos,
            Piece::Captured(_) => panic!("Captured pieces cannot have their position set"),
        }
    }

    pub fn legal_moves(&self, board: &Board) -> Vec<Position> { //TODO return an array of Move types instead: Move::Piece(pos, to_pos)
        let mut result = Vec::new();

        match *self {
            Self::Pawn(ally_colour, pos) => {
                let up: Position = pos.pawn_up(ally_colour);
                let double_up: Position = up.pawn_up(ally_colour);
                let up_left = up.next_left();
                let up_right = up.next_right();

                if double_up.is_on_board()
                    && Position::is_starting_pawn(&self.get_pos(), ally_colour)
                    && board.has_no_piece(up)
                    && board.has_no_piece(double_up)
                {
                    result.push(double_up)
                }

                if up.is_on_board() && board.has_no_piece(up) {
                    result.push(up)
                }

                if up_left.is_on_board() && board.has_enemy_piece(pos, ally_colour) {
                    result.push(up_left)
                }

                if up_right.is_on_board() && board.has_enemy_piece(pos, ally_colour) {
                    result.push(up_right)
                }

                //TODO implement en passant
            }

            Self::King(ally_colour, pos) => {
                for p in &[
                    pos.next_above(),
                    pos.next_below(),
                    pos.next_left(),
                    pos.next_right(),
                    pos.next_above().next_left(),
                    pos.next_above().next_right(),
                    pos.next_below().next_left(),
                    pos.next_below().next_right(),
                ] {
                    if p.is_on_board() && !board.has_friendly_piece(*p, ally_colour) {
                        result.push(*p)
                    }
                }
                //TODO: implement kingside and queenside rook
            },

            Self::Queen(ally_colour, pos) => {
                for p in &[
                    Position::max_travel(board, pos, ally_colour, "UP"),
                    Position::max_travel(board, pos, ally_colour, "DOWN"),
                    Position::max_travel(board, pos, ally_colour, "LEFT"),
                    Position::max_travel(board, pos, ally_colour, "RIGHT"),
                ] {
                    for tile in Position::orthogonals_to(&pos, *p) {
                        if p.is_on_board() && !board.has_friendly_piece(*p, ally_colour) {
                            result.push(*p)
                        }
                    }
                }

                for p in &[
                    Position::max_travel(board, pos, ally_colour, "NE"),
                    Position::max_travel(board, pos, ally_colour, "NW"),
                    Position::max_travel(board, pos, ally_colour, "SE"),
                    Position::max_travel(board, pos, ally_colour, "SW"),
                ] {
                    for tile in Position::diagonals_to(&pos, *p) {
                        if p.is_on_board() && !board.has_friendly_piece(*p, ally_colour) {
                            result.push(*p)
                        }
                    }
                }

                //TODO: Implement queen movement for horse
            },

            Self::Rook(ally_colour, pos) => {
                for p in &[
                    Position::max_travel(board, pos, ally_colour, "UP"),
                    Position::max_travel(board, pos, ally_colour, "DOWN"),
                    Position::max_travel(board, pos, ally_colour, "LEFT"),
                    Position::max_travel(board, pos, ally_colour, "RIGHT"),
                ] {
                    for tile in Position::orthogonals_to(&pos, *p) {
                        if p.is_on_board() && !board.has_friendly_piece(*p, ally_colour) {
                            result.push(*p)
                        }
                    }
                }
            },

            Self::Bishop(ally_colour, pos) => {
                for p in &[
                    Position::max_travel(board, pos, ally_colour, "NE"),
                    Position::max_travel(board, pos, ally_colour, "NW"),
                    Position::max_travel(board, pos, ally_colour, "SE"),
                    Position::max_travel(board, pos, ally_colour, "SW"),
                ] {
                    for tile in Position::diagonals_to(&pos, *p) {
                        if p.is_on_board() && !board.has_friendly_piece(*p, ally_colour) {
                            result.push(*p)
                        }
                    }
                }   
            },
            
            Self::Knight(ally_colour, pos) => {
                let potential_knight_moves = vec![
                    Position::new(pos.get_row() + 2, pos.get_col() + 1), // Move 2 up, 1 right
                    Position::new(pos.get_row() + 2, pos.get_col() - 1), // Move 2 up, 1 left
                    Position::new(pos.get_row() - 2, pos.get_col() + 1), // Move 2 down, 1 right
                    Position::new(pos.get_row() - 2, pos.get_col() - 1), // Move 2 down, 1 left
                    Position::new(pos.get_row() + 1, pos.get_col() + 2), // Move 1 up, 2 right
                    Position::new(pos.get_row() + 1, pos.get_col() - 2), // Move 1 up, 2 left
                    Position::new(pos.get_row() - 1, pos.get_col() + 2), // Move 1 down, 2 right
                    Position::new(pos.get_row() - 1, pos.get_col() - 2), // Move 1 down, 2 left
                ];

                for p in potential_knight_moves {
                    if p.is_on_board() && !board.has_enemy_piece(p, ally_colour) {
                        result.push(p);
                    }
                }
            },

            Self::Captured(_) => panic!("Caputed piece has no legal moves")
        };

        result
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::King => write!(f, "k"),
            PieceType::Queen => write!(f, "q"),
            PieceType::Rook => write!(f, "r"),
            PieceType::Bishop => write!(f, "b"),
            PieceType::Knight => write!(f, "n"),
            PieceType::Pawn => write!(f, "p"),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let symbol = match (self.get_piece_type(), self.get_colour()) {
            (PieceType::Pawn, Colour::White) => "♙",
            (PieceType::Pawn, Colour::Black) => "♟",

            (PieceType::Rook, Colour::White) => "♖",
            (PieceType::Rook, Colour::Black) => "♜",

            (PieceType::Knight, Colour::White) => "♘",
            (PieceType::Knight, Colour::Black) => "♞",

            (PieceType::Bishop, Colour::White) => "♗",
            (PieceType::Bishop, Colour::Black) => "♝",

            (PieceType::King, Colour::White) => "♔",
            (PieceType::King, Colour::Black) => "♚",

            (PieceType::Queen, Colour::White) => "♕",
            (PieceType::Queen, Colour::Black) => "♛",
        };
        write!(f, "{}", symbol)
    }
}