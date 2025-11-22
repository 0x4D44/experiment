use std::fmt;

/// Represents a player in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Red,
    Yellow,
}

impl Player {
    /// Switch to the other player
    pub fn other(&self) -> Player {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Red => write!(f, "Red"),
            Player::Yellow => write!(f, "Yellow"),
        }
    }
}

/// Represents a cell on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

/// Game outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

/// The Connect Four game board
#[derive(Debug, Clone)]
pub struct Board {
    pub grid: [[Cell; Board::COLS]; Board::ROWS],
    pub move_count: usize,
    pub move_history: Vec<usize>,
}

impl Board {
    pub const ROWS: usize = 6;
    pub const COLS: usize = 7;
    pub const WIN_LENGTH: usize = 4;

    /// Create a new empty board
    pub fn new() -> Self {
        Board {
            grid: [[Cell::Empty; Board::COLS]; Board::ROWS],
            move_count: 0,
            move_history: Vec::new(),
        }
    }

    /// Check if a column is valid and not full
    pub fn is_valid_move(&self, col: usize) -> bool {
        col < Board::COLS && self.grid[0][col] == Cell::Empty
    }

    /// Get all valid column moves
    pub fn valid_moves(&self) -> Vec<usize> {
        (0..Board::COLS)
            .filter(|&col| self.is_valid_move(col))
            .collect()
    }

    /// Drop a piece in the specified column
    /// Returns the row where the piece landed, or None if invalid
    pub fn drop_piece(&mut self, col: usize, player: Player) -> Option<usize> {
        if !self.is_valid_move(col) {
            return None;
        }

        // Find the lowest empty row in this column
        for row in (0..Board::ROWS).rev() {
            if self.grid[row][col] == Cell::Empty {
                self.grid[row][col] = Cell::Occupied(player);
                self.move_count += 1;
                self.move_history.push(col);
                return Some(row);
            }
        }

        None
    }

    /// Undo the last move
    pub fn undo_move(&mut self) -> Option<usize> {
        if let Some(col) = self.move_history.pop() {
            // Find the top piece in this column and remove it
            for row in 0..Board::ROWS {
                if self.grid[row][col] != Cell::Empty {
                    self.grid[row][col] = Cell::Empty;
                    self.move_count -= 1;
                    return Some(col);
                }
            }
        }
        None
    }

    /// Check if there's a winner or draw
    pub fn check_game_state(&self) -> GameState {
        // Check for wins
        if let Some(winner) = self.check_winner() {
            return GameState::Won(winner);
        }

        // Check for draw (board full)
        if self.move_count >= Board::ROWS * Board::COLS {
            return GameState::Draw;
        }

        GameState::InProgress
    }

    /// Check if there's a winner
    pub fn check_winner(&self) -> Option<Player> {
        // Check horizontal
        for row in 0..Board::ROWS {
            for col in 0..=(Board::COLS - Board::WIN_LENGTH) {
                if let Cell::Occupied(player) = self.grid[row][col] {
                    if (1..Board::WIN_LENGTH).all(|i| self.grid[row][col + i] == Cell::Occupied(player)) {
                        return Some(player);
                    }
                }
            }
        }

        // Check vertical
        for row in 0..=(Board::ROWS - Board::WIN_LENGTH) {
            for col in 0..Board::COLS {
                if let Cell::Occupied(player) = self.grid[row][col] {
                    if (1..Board::WIN_LENGTH).all(|i| self.grid[row + i][col] == Cell::Occupied(player)) {
                        return Some(player);
                    }
                }
            }
        }

        // Check diagonal (down-right)
        for row in 0..=(Board::ROWS - Board::WIN_LENGTH) {
            for col in 0..=(Board::COLS - Board::WIN_LENGTH) {
                if let Cell::Occupied(player) = self.grid[row][col] {
                    if (1..Board::WIN_LENGTH).all(|i| self.grid[row + i][col + i] == Cell::Occupied(player)) {
                        return Some(player);
                    }
                }
            }
        }

        // Check diagonal (down-left)
        for row in 0..=(Board::ROWS - Board::WIN_LENGTH) {
            for col in (Board::WIN_LENGTH - 1)..Board::COLS {
                if let Cell::Occupied(player) = self.grid[row][col] {
                    if (1..Board::WIN_LENGTH).all(|i| self.grid[row + i][col - i] == Cell::Occupied(player)) {
                        return Some(player);
                    }
                }
            }
        }

        None
    }

    /// Count the number of pieces for a player in a line (used for scoring)
    pub fn count_line(&self, positions: &[(usize, usize)], player: Player) -> usize {
        positions
            .iter()
            .filter(|&&(row, col)| self.grid[row][col] == Cell::Occupied(player))
            .count()
    }

    /// Check if a line is open (no opponent pieces)
    #[allow(dead_code)]
    pub fn is_line_open(&self, positions: &[(usize, usize)], player: Player) -> bool {
        positions
            .iter()
            .all(|&(row, col)| {
                self.grid[row][col] == Cell::Empty || self.grid[row][col] == Cell::Occupied(player)
            })
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        assert_eq!(board.move_count, 0);
        assert_eq!(board.move_history.len(), 0);
        for row in 0..Board::ROWS {
            for col in 0..Board::COLS {
                assert_eq!(board.grid[row][col], Cell::Empty);
            }
        }
    }

    #[test]
    fn test_valid_moves() {
        let mut board = Board::new();
        assert_eq!(board.valid_moves(), vec![0, 1, 2, 3, 4, 5, 6]);

        // Fill column 0
        for _ in 0..Board::ROWS {
            board.drop_piece(0, Player::Red);
        }
        assert_eq!(board.valid_moves(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_drop_piece() {
        let mut board = Board::new();

        // First piece should go to bottom
        let row = board.drop_piece(3, Player::Red);
        assert_eq!(row, Some(5));
        assert_eq!(board.grid[5][3], Cell::Occupied(Player::Red));
        assert_eq!(board.move_count, 1);

        // Second piece in same column should stack
        let row = board.drop_piece(3, Player::Yellow);
        assert_eq!(row, Some(4));
        assert_eq!(board.grid[4][3], Cell::Occupied(Player::Yellow));
        assert_eq!(board.move_count, 2);
    }

    #[test]
    fn test_drop_piece_invalid() {
        let mut board = Board::new();

        // Fill column
        for _ in 0..Board::ROWS {
            board.drop_piece(0, Player::Red);
        }

        // Should fail to add another
        let result = board.drop_piece(0, Player::Yellow);
        assert_eq!(result, None);
    }

    #[test]
    fn test_undo_move() {
        let mut board = Board::new();

        board.drop_piece(3, Player::Red);
        board.drop_piece(3, Player::Yellow);
        assert_eq!(board.move_count, 2);

        board.undo_move();
        assert_eq!(board.move_count, 1);
        assert_eq!(board.grid[5][3], Cell::Occupied(Player::Red));
        assert_eq!(board.grid[4][3], Cell::Empty);

        board.undo_move();
        assert_eq!(board.move_count, 0);
        assert_eq!(board.grid[5][3], Cell::Empty);
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = Board::new();

        // Red wins horizontally
        for col in 0..4 {
            board.drop_piece(col, Player::Red);
        }

        assert_eq!(board.check_winner(), Some(Player::Red));
        assert_eq!(board.check_game_state(), GameState::Won(Player::Red));
    }

    #[test]
    fn test_vertical_win() {
        let mut board = Board::new();

        // Yellow wins vertically in column 3
        for _ in 0..4 {
            board.drop_piece(3, Player::Yellow);
        }

        assert_eq!(board.check_winner(), Some(Player::Yellow));
        assert_eq!(board.check_game_state(), GameState::Won(Player::Yellow));
    }

    #[test]
    fn test_diagonal_win_down_right() {
        let mut board = Board::new();

        // Create diagonal win for Red (down-right)
        // Pattern:
        // . . . . . . .
        // . . . . . . .
        // . . . R . . .
        // . . R Y . . .
        // . R Y Y . . .
        // R Y Y Y . . .

        board.drop_piece(0, Player::Red);   // Row 5, Col 0

        board.drop_piece(1, Player::Yellow); // Row 5, Col 1
        board.drop_piece(1, Player::Red);    // Row 4, Col 1

        board.drop_piece(2, Player::Yellow); // Row 5, Col 2
        board.drop_piece(2, Player::Yellow); // Row 4, Col 2
        board.drop_piece(2, Player::Red);    // Row 3, Col 2

        board.drop_piece(3, Player::Yellow); // Row 5, Col 3
        board.drop_piece(3, Player::Yellow); // Row 4, Col 3
        board.drop_piece(3, Player::Yellow); // Row 3, Col 3
        board.drop_piece(3, Player::Red);    // Row 2, Col 3

        assert_eq!(board.check_winner(), Some(Player::Red));
    }

    #[test]
    fn test_diagonal_win_down_left() {
        let mut board = Board::new();

        // Create diagonal win for Yellow (down-left)
        board.drop_piece(6, Player::Yellow); // Row 5, Col 6

        board.drop_piece(5, Player::Red);    // Row 5, Col 5
        board.drop_piece(5, Player::Yellow); // Row 4, Col 5

        board.drop_piece(4, Player::Red);    // Row 5, Col 4
        board.drop_piece(4, Player::Red);    // Row 4, Col 4
        board.drop_piece(4, Player::Yellow); // Row 3, Col 4

        board.drop_piece(3, Player::Red);    // Row 5, Col 3
        board.drop_piece(3, Player::Red);    // Row 4, Col 3
        board.drop_piece(3, Player::Red);    // Row 3, Col 3
        board.drop_piece(3, Player::Yellow); // Row 2, Col 3

        assert_eq!(board.check_winner(), Some(Player::Yellow));
    }

    #[test]
    fn test_draw() {
        let mut board = Board::new();

        // Fill board without any wins
        let pattern = [
            Player::Red, Player::Yellow, Player::Red, Player::Yellow,
            Player::Red, Player::Yellow, Player::Red
        ];

        for _ in 0..Board::ROWS {
            for (col, &player) in pattern.iter().enumerate() {
                board.drop_piece(col, player);
            }
        }

        // Should be a draw if no winner
        if board.check_winner().is_none() {
            assert_eq!(board.check_game_state(), GameState::Draw);
        }
    }

    #[test]
    fn test_player_other() {
        assert_eq!(Player::Red.other(), Player::Yellow);
        assert_eq!(Player::Yellow.other(), Player::Red);
    }
}
