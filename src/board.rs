//src/board.rs

use crate::{piece::{Colour, Piece, PieceType}, position::Position};

pub struct Board {
    pub grid: [[Option<Piece>; Board::SIZE]; Board::SIZE]
}

impl Board {

    pub const SIZE: usize = 8;

    pub fn state(&self) -> Vec<Piece>{
        let mut pieces = Vec::new();

        for row in self.grid {
            for tile in row {
                if let Some(piece) = tile {
                    pieces.push(piece);
                }
            }
        }
        pieces
    }

    pub fn position_to_notation(pos: Position) -> String {
        let row = pos.get_row();
        let col = pos.get_col();
    
        if row < 0 || row >= Self::SIZE as i32 || col < 0 || col >= Self::SIZE as i32 {
            return "Invalid position".to_string();
        }
    
        let column_letter = (col + 65) as u8 as char; // A=65 in ASCII
        let row_number = (Self::SIZE - row as usize).to_string(); // Row is 1-8, so we need to adjust
    
        format!("{}{}", column_letter, row_number)
    }

    pub fn has_piece(&self, pos: Position) -> bool {
        self.get_piece(pos).is_some()
    }

    pub fn has_no_piece(&self, pos: Position) -> bool {
        self.get_piece(pos).is_none()
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece>{
        if pos.is_off_board() {
            return None
        }
        let row: usize = pos.get_row().try_into().unwrap();
        let col: usize = pos.get_col().try_into().unwrap();
        self.grid[row][col]
    }

    #[inline]
    pub fn has_enemy_piece(&self, pos: Position, ally_colour: Colour) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_colour() == !ally_colour
        } else {
            false
        }
    }

    pub fn has_friendly_piece(&self, pos: Position, ally_color: Colour) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_colour() == ally_color
        } else {
            false
        }
    }

    pub fn move_piece(mut self, mut piece: Piece, to: Position, ally_colour: Colour) -> Board {
        let row= to.get_row();
        let col = to.get_col();
        if piece.legal_moves(&self).contains(&to) {
            if self.has_enemy_piece(to, ally_colour) {
                self.get_piece( to).unwrap().set_captured();
            }
            self.grid[piece.get_pos().get_row() as usize][piece.get_pos().get_col() as usize] = None;

            piece.set_pos(Position::new(row,col));
            self.grid[row as usize][col as usize] = Some(piece);
        }
        else {
            println!("Move not valid!");
        }
        return self
    }

    pub fn new() -> Self {
        let mut board = Board {
            grid: [[None; 8]; 8]
        };

        // Rooks
        board.grid[7][0] = Some(Piece::new(PieceType::Rook, Colour::Black, Position::new(7, 0))); // A1
        board.grid[7][7] = Some(Piece::new(PieceType::Rook, Colour::Black, Position::new(7, 7))); // H1

        board.grid[0][0] = Some(Piece::new(PieceType::Rook, Colour::White, Position::new(0, 0))); // A8
        board.grid[0][7] = Some(Piece::new(PieceType::Rook, Colour::White, Position::new(0, 7))); // H8

        // Knights
        board.grid[7][1] = Some(Piece::new(PieceType::Knight, Colour::Black, Position::new(7, 1))); // B1
        board.grid[7][6] = Some(Piece::new(PieceType::Knight, Colour::Black, Position::new(7, 6))); // G1

        board.grid[0][1] = Some(Piece::new(PieceType::Knight, Colour::White, Position::new(0, 1))); // B8
        board.grid[0][6] = Some(Piece::new(PieceType::Knight, Colour::White, Position::new(0, 6))); // G8

        // Bishops
        board.grid[7][2] = Some(Piece::new(PieceType::Bishop, Colour::Black, Position::new(7, 2))); // C1
        board.grid[7][5] = Some(Piece::new(PieceType::Bishop, Colour::Black, Position::new(7, 5))); // F1

        board.grid[0][2] = Some(Piece::new(PieceType::Bishop, Colour::White, Position::new(0, 2))); // C8
        board.grid[0][5] = Some(Piece::new(PieceType::Bishop, Colour::White, Position::new(0, 5))); // F8

        // Royalty
        board.grid[7][3] = Some(Piece::new(PieceType::Queen, Colour::Black, Position::new(7, 3))); // D1
        board.grid[7][4] = Some(Piece::new(PieceType::King, Colour::Black, Position::new(7, 4)));  // E1

        board.grid[0][3] = Some(Piece::new(PieceType::Queen, Colour::White, Position::new(0, 3))); // D8
        board.grid[0][4] = Some(Piece::new(PieceType::King, Colour::White, Position::new(0, 4)));  // E8

        // Pawns
        for i in 0..8 {
            board.grid[6][i] = Some(Piece::new(PieceType::Pawn, Colour::Black, Position::new(6, i.try_into().unwrap()))); // 2nd row for Black
            board.grid[1][i] = Some(Piece::new(PieceType::Pawn, Colour::White, Position::new(1, i.try_into().unwrap()))); // 7th row for White
        }


        board

    }

    pub fn display(&self) {
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                match tile {
                    Some(piece) => print!("{}", piece),
                    None => print!("{}", 
                        if (row_idx + col_idx) % 2 != 0 {"■"}
                        else {"□"}
                    )
                }
            }
            println!("");
        }

        //Letter/Num Key Map:
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                print!("{}",Self::position_to_notation(Position::new(row_idx as i32,col_idx as i32)));
                print!(" ");
            }
            println!("");
        }
    }
}