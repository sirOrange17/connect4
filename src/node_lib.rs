use std::thread;
use std::sync::{Arc, Mutex};

// Structure representing a node in the Monte Carlo Tree Search.
#[derive(Clone)]
pub struct Node<State,Move> {
    board: State,
    visits: u32,
    wins: u32,
    children: Vec<Node<State,Move>>,
    move_: Option<Move>,
}

pub trait MoveOps<Move> {
    fn make_move(&mut self, move_: Move);
    fn list_moves(&self) -> Vec<Move>;
    fn simulate_random_game(&mut self) -> bool;
}

impl<State: Clone + MoveOps<Move>,Move: Copy + std::fmt::Debug> Node<State,Move> {
    // Creates a new Node.
    pub fn new(board: State) -> Self {
        Node {
            board,
            visits: 0,
            wins: 0,
            children: Vec::new(),
            move_: None,
        }
    }
	
    // Selects the child node with the highest UCT value.
    pub fn select(&mut self) -> &mut Self {
        if self.children.is_empty() {
            return self;  // If there are no children, return the current node.
        }
    
        let mut best_child_index = None;
        let mut best_value = -f64::INFINITY;
        let parent_visits = self.visits;  // Pass the number of visits to the parent node
    
        for (index, child) in self.children.iter_mut().enumerate() {
            let uct_value = child.uct_value(parent_visits); // Compute the UCT value of the child
    
            if uct_value > best_value {
                best_value = uct_value;
                best_child_index = Some(index);
            }
        }
    
        &mut self.children[best_child_index.unwrap()]
    }
    
	
	// Computes UCT value of the node
	fn uct_value(&self, parent_visits: u32) -> f64 {
		if self.visits == 0 {
			return f64::INFINITY;  // Ensure unvisited nodes are prioritized for exploration
		}
		let win_rate = self.wins as f64 / self.visits as f64;
		let exploration = (2.0 * (parent_visits as f64).ln() / self.visits as f64).sqrt();
		
        win_rate + exploration
	}

    // Expands the node by generating its children.
	pub fn expand(&mut self) {
		let moves = self.board.list_moves();
		for &mv in &moves {
			let mut new_board = self.board.clone();
			new_board.make_move(mv);
			let mut new_node = Node::new(new_board);
			// Set the move for the new node
			new_node.move_ = Some(mv);
			self.children.push(new_node);
		}
	}

    // Simulates a game from the current node's board state.
    pub fn simulate(&mut self) -> bool {
        self.board.simulate_random_game()
    }

    // Backpropagates the result of a simulation up the tree.
    pub fn backpropagate(&mut self, result: bool) {
        self.visits += 1;
        if result {
            self.wins += 1;  // Increment win count if the result is a win.
        }
    }

    // Returns the best move based on the number of visits.
    fn best_move(&self) -> Move {
        let mut best_move = None;
        let mut best_value = -f64::INFINITY;
        for child in &self.children {
            let value = child.visits as f64;  // Use the number of visits as the value.
			println!("Child move: {:?}, visits: {}, wins: {}, w/v: {}", child.move_, child.visits, child.wins, (child.wins as f64)/(child.visits as f64));
            if value > best_value {
                best_value = value;
                best_move = Some(child.move_.unwrap());
            }
        }
		println!("Best move: {:?}", best_move);
        let sum: u32 = self.children.iter().map(|child| child.visits).sum();
        println!("{sum}");
        best_move.unwrap()  // Return the move associated with the best value.
    }
}

// Performs a UCT search for the best move.
pub fn uct_search<State: Clone + MoveOps<Move> + std::marker::Send + 'static,Move: Copy + std::cmp::PartialEq + std::fmt::Debug + std::marker::Send + 'static>(board: &State, iterations: u32, threads: u8) -> Move {
    let mut first_root = Node::new(board.clone());
    first_root.expand();
    let thread_out: Arc<Mutex<Node<State,Move>>> = Arc::new(Mutex::new(first_root));

    let mut handles = vec![];

    // Spawn multiple threads to run the search in parallel
    for _ in 0..threads {  // Adjust the number of threads as needed
        let board_clone = board.clone();
        let thread_out = Arc::clone(&thread_out);

        let handle = thread::spawn(move || {
            let mut root = Node::new(board_clone);
            root.expand();

            for _ in 0..iterations / (threads as u32) {  // Divide the iterations among threads
                let node = root.select();  // Select a node to simulate from.
                let result = node.simulate();  // Simulate a game from the node.
                node.backpropagate(result);  // Backpropagate the result of the simulation.
                root.backpropagate(result);
            }

            let mut thread_out = thread_out.lock().unwrap();
            for i in 0..root.children.len() {
                let entry = &mut thread_out.children[i];
                entry.visits += root.children[i].visits;
                entry.wins += root.children[i].wins;
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Calculate the best move based on the combined results from all threads
    let thread_out = thread_out.lock().unwrap();
    thread_out.best_move()
}