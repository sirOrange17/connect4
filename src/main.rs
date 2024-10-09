use std::io;

mod board_lib;
use crate::board_lib::Board;

mod node_lib;
use crate::node_lib::{MoveOps, uct_search};

impl MoveOps<u8> for Board {
    fn make_move(&mut self, move_: u8) {self.make_move(move_)}
    fn list_moves(&self) -> Vec<u8> {self.list_moves()}
    fn simulate_random_game(&mut self) -> bool {self.simulate_random_game()}
}

fn main() {
    let mut board = Board::new();

    loop {
        board.print_self();  // Print the board state.
		println!("{:?}", board);

        if board.is_win(0) || board.is_win(1) {break;} // Check if the game is won.

        let mut col_in = String::new();
        io::stdin()
            .read_line(&mut col_in)  // Read input from the user.
            .expect("Failed to read line.");
		println!();
		
		if col_in.trim() == "mc" { // check if to use MCTS over user input
			board.make_move(uct_search(&board, 5000,16)); // Make move based on a MCTS UCT search
		}
        else {
			board.make_move(col_in.trim().parse().expect("Please type a number."));  // Make the move on the board.
		}
		println!();
    }
}