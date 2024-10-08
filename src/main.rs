use std::io::{self, Write};
use board::Board;
use piece::Piece;
use position::Position;

mod piece;
mod board;
mod position;
fn main() {
    let mut board = Board::new();
    // board.display();

    // let my_pawn = board.get_piece(Position::new(1,1)).unwrap();
    // println!("{:?}", my_pawn.get_piece_type());
    // println!("{:?}", my_pawn.legal_moves(&board));
    // let legal_moves = my_pawn.legal_moves(&board);
    // board = board.move_piece(my_pawn, legal_moves[0], my_pawn.get_colour());
    // board.display();

    //Game Loop

    loop {
        board.display();
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim(); 

        if input == "exit" {
            println!("Exiting...");
            break;
        }

        //expect something like D2 -> A3
        let keywords: Vec<&str> = input.split(" ").collect();
        assert!(keywords.len() == 3);
        assert!(keywords[1] == "->");
        let origin: Vec<char> = keywords[0].chars().collect();
        let destination: Vec<char> = keywords[2].chars().collect();
        assert!(origin[0] >= 'A' && origin[0] <= 'H');
        assert!(origin[1] >= '1' && origin[1] <= '8');
        assert!(destination[0] >= 'A' && destination[0] <= 'H');
        assert!(destination[1] >= '1' && destination[1] <= '8');
        
        let origin_str: &str = &origin.iter().collect::<String>();
        let destination_str: &str = &destination.iter().collect::<String>();

        let origin_pos: Position = position::interpret_position(origin_str).unwrap();
        let chosen_piece: Piece = board.get_piece(origin_pos).unwrap();

        let destination_pos: Position = position::interpret_position(destination_str).unwrap();
        
        board = board.move_piece(chosen_piece, destination_pos, chosen_piece.get_colour());

        println!("\n");
    }
}
