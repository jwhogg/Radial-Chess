use board::Board;
use position::Position;

mod piece;
mod board;
mod position;
fn main() {
    let board = Board::new();
    board.display();
    // println!("{:?}", board.state()[0]);
    // println!("{:?}", Position::is_on_board(&Position::new(8,7)));
    let my_pawn = board.get_piece(Position::new(1,1)).unwrap();
    println!("{:?}", my_pawn.get_piece_type());
    println!("{:?}", my_pawn.legal_moves(&board));
}
