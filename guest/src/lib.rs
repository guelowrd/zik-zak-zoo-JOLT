#![no_main]
use std::str::FromStr;

#[jolt::provable]
pub fn verify_player_win(input: &str) -> bool {
    let mut parts = input.split(',');
    
    // Parse the seed
    let seed = match parts.next().and_then(|s| u64::from_str(s).ok()) {
        Some(s) => s,
        None => return false, // Invalid seed
    };

    let mut rng = SimpleRNG::new(seed);
    let mut board = Board::new();
    let current_player = Cell::Z;

    // Process moves
    for move_str in parts {
        let player_move = match usize::from_str(move_str) {
            Ok(m) if m < 9 => m,
            _ => return false, // Invalid move
        };

        // Player's move
        if !board.make_move(player_move, current_player) {
            return false; // Invalid move
        }

        if let Some(winner) = board.check_winner() {
            return winner == Cell::Z; // Player wins
        }

        // Computer's move
        let empty_cells = board.get_empty_cells();
        if empty_cells.is_empty() {
            return false; // Draw
        }
        let computer_move = empty_cells[rng.rand_range(0, empty_cells.len() - 1)];
        board.make_move(computer_move, Cell::K);

        if board.check_winner() == Some(Cell::K) {
            return false; // Computer wins
        }
    }

    false // Game not finished or draw
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Empty,
    Z,
    K,
}

struct Board {
    cells: [Cell; 9],
}

struct SimpleRNG {
    state: u64,
}

impl SimpleRNG {
    fn new(seed: u64) -> Self {
        SimpleRNG { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn rand_range(&mut self, min: usize, max: usize) -> usize {
        (self.next() % (max - min + 1) as u64) as usize + min
    }
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    fn make_move(&mut self, position: usize, player: Cell) -> bool {
        if position < 9 && self.cells[position] == Cell::Empty {
            self.cells[position] = player;
            true
        } else {
            false
        }
    }

    fn get_empty_cells(&self) -> Vec<usize> {
        self.cells.iter().enumerate()
            .filter(|(_, &cell)| cell == Cell::Empty)
            .map(|(index, _)| index)
            .collect()
    }

    fn check_winner(&self) -> Option<Cell> {
        const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        for combo in WINNING_COMBINATIONS.iter() {
            if self.cells[combo[0]] != Cell::Empty
                && self.cells[combo[0]] == self.cells[combo[1]]
                && self.cells[combo[1]] == self.cells[combo[2]]
            {
                return Some(self.cells[combo[0]]);
            }
        }
        None
    }
}
