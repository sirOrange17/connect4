use fastrand::usize as rand_usize;

// Structure representing the Connect Four board.
#[derive(Clone, Debug)]
pub struct Board {
    bitboards: [u64; 2],  // Bitboards for each player's moves.
    heights: [u8; 7],     // Heights of the columns (where the next piece will be placed).
    counter: u8,          // Counter to track the number of moves.
    moves: Vec<u8>,       // List of moves made so far.
}

impl Board {
    pub fn new() -> Self {
        Board {
            bitboards: [0, 0],
            heights: [0, 7, 14, 21, 28, 35, 42],
            counter: 0,
            moves: Vec::new(),
        }
    }

    // Makes a move in the specified column.
    pub fn make_move(&mut self, col: u8) {
        let cmove: u64 = 1_u64 << self.heights[col as usize];  // Calculate the bit position for the move.
        self.heights[col as usize] += 1;  // Update the height of the column.
        self.bitboards[self.counter as usize & 1] ^= cmove;  // Update the bitboard for the current player.
        self.moves.push(col);  // Record the move.
        self.counter += 1;  // Increment the move counter.
    }

    // Undoes the last move.
    fn undo_move(&mut self) {
        self.counter -= 1;  // Decrement the move counter.
        let col: u8 = self.moves[self.counter as usize];  // Get the column of the last move.
        self.heights[col as usize] -= 1;  // Update the height of the column.
        let cmove: u64 = 1_u64 << self.heights[col as usize];  // Calculate the bit position for the move.
        self.bitboards[self.counter as usize & 1] ^= cmove;  // Undo the move in the bitboard.
        self.moves.pop();  // Remove the move from the list.
    }

    // Checks if there is a win for the specified player.
    pub fn is_win(&self, side: usize) -> bool {
        if self.bitboards[side] & (self.bitboards[side] >> 6) & (self.bitboards[side] >> 12) & (self.bitboards[side] >> 18) != 0 { return true; } // diagonal \
        if self.bitboards[side] & (self.bitboards[side] >> 8) & (self.bitboards[side] >> 16) & (self.bitboards[side] >> 24) != 0 { return true; } // diagonal /
        if self.bitboards[side] & (self.bitboards[side] >> 7) & (self.bitboards[side] >> 14) & (self.bitboards[side] >> 21) != 0 { return true; } // horizontal
        if self.bitboards[side] & (self.bitboards[side] >> 1) & (self.bitboards[side] >> 2) & (self.bitboards[side] >> 3) != 0 { return true; } // vertical
        false
    }
	
    // Lists all possible moves (columns) that are not yet full.
    pub fn list_moves(&self) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();
        let top: u64 = 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000;
        for col in 0..7_u8 {
            // Check if the column is not full (if the top bit is 0).
            if (top & (1_u64 << self.heights[col as usize])) == 0 {
                moves.push(col);
            }
        }
        moves
    }

    // Prints the current state of the board to the console.
    pub fn print_self(&self) {
        for i in (0..6).rev() {  // Iterate from the top row to the bottom row.
            let mut out = String::from("");
            for j in 0..7 {  // Iterate through each column.
                if ((self.bitboards[0] >> (i + 7 * j)) & 1) == 1 {
                    out.push('X');  // Player 0's piece.
                } else if ((self.bitboards[1] >> (i + 7 * j)) & 1) == 1 {
                    out.push('O');  // Player 1's piece.
                } else {
                    out.push('.');  // Empty cell.
                }
                out.push(' ');
            }
            println!("{}", out);  // Print the row.
        }
    }

    // Simulates a random game from the current board state.
    pub fn simulate_random_game(&mut self) -> bool {
        let mut moves_made = Vec::new();
        while !self.list_moves().is_empty() {
            let moves = self.list_moves();
            let random_move = moves[rand_usize(0..moves.len())];
            self.make_move(random_move);
            moves_made.push(random_move);
            if self.is_win((self.counter - 1) as usize & 1) {  // Check if the last move resulted in a win.
                let winner_move = self.counter;
				while let Some(_m) = moves_made.pop() {
                    self.undo_move();  // Undo the move if it resulted in a win.
                }
                return (winner_move - 1) as usize & 1 == (self.counter - 1) as usize & 1;  // Return if Player 0 won.
            }
        }
        while let Some(_m) = moves_made.pop() {
            self.undo_move();  // Undo all moves if no win is detected.
        }
        false
    }
}