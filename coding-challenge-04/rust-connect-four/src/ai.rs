use crate::board::{Board, Cell, Player, GameState};
use rand::seq::SliceRandom;
use rand::thread_rng;

/// AI difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,    // Random moves
    Medium,  // Minimax depth 3-4
    Hard,    // Minimax depth 5-6
    Expert,  // Minimax depth 7+ with optimizations
}

impl Difficulty {
    pub fn depth(&self) -> usize {
        match self {
            Difficulty::Easy => 1,
            Difficulty::Medium => 4,
            Difficulty::Hard => 6,
            Difficulty::Expert => 8,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Expert => "Expert",
        }
    }
}

/// AI player using minimax algorithm with alpha-beta pruning
pub struct AI {
    pub player: Player,
    pub difficulty: Difficulty,
}

impl AI {
    pub fn new(player: Player, difficulty: Difficulty) -> Self {
        AI { player, difficulty }
    }

    /// Get the best move for the AI
    pub fn get_best_move(&self, board: &Board) -> Option<usize> {
        let valid_moves = board.valid_moves();
        if valid_moves.is_empty() {
            return None;
        }

        match self.difficulty {
            Difficulty::Easy => {
                // Random move
                let mut rng = thread_rng();
                valid_moves.choose(&mut rng).copied()
            }
            _ => {
                // Use minimax with alpha-beta pruning
                self.minimax_best_move(board)
            }
        }
    }

    /// Find the best move using minimax with alpha-beta pruning
    fn minimax_best_move(&self, board: &Board) -> Option<usize> {
        let valid_moves = board.valid_moves();
        if valid_moves.is_empty() {
            return None;
        }

        // Order moves: prioritize center columns (better positions)
        let mut ordered_moves = valid_moves.clone();
        ordered_moves.sort_by_key(|&col| {
            let center = Board::COLS / 2;
            (col as i32 - center as i32).abs()
        });

        let mut best_score = i32::MIN;
        let mut best_moves = Vec::new();

        for &col in &ordered_moves {
            let mut test_board = board.clone();
            test_board.drop_piece(col, self.player);

            let score = self.minimax(
                &test_board,
                self.difficulty.depth() - 1,
                i32::MIN,
                i32::MAX,
                false,
            );

            if score > best_score {
                best_score = score;
                best_moves.clear();
                best_moves.push(col);
            } else if score == best_score {
                best_moves.push(col);
            }
        }

        // If multiple moves have the same score, choose randomly
        let mut rng = thread_rng();
        best_moves.choose(&mut rng).copied()
    }

    /// Minimax algorithm with alpha-beta pruning
    fn minimax(
        &self,
        board: &Board,
        depth: usize,
        mut alpha: i32,
        mut beta: i32,
        is_maximizing: bool,
    ) -> i32 {
        // Check terminal states
        match board.check_game_state() {
            GameState::Won(player) => {
                if player == self.player {
                    return 10000 + depth as i32; // Prefer faster wins
                } else {
                    return -10000 - depth as i32; // Prefer slower losses
                }
            }
            GameState::Draw => return 0,
            GameState::InProgress => {}
        }

        // Depth limit reached, evaluate position
        if depth == 0 {
            return self.evaluate_position(board);
        }

        let valid_moves = board.valid_moves();
        if valid_moves.is_empty() {
            return 0; // Draw
        }

        // Order moves: prioritize center columns
        let mut ordered_moves = valid_moves.clone();
        ordered_moves.sort_by_key(|&col| {
            let center = Board::COLS / 2;
            (col as i32 - center as i32).abs()
        });

        if is_maximizing {
            let mut max_eval = i32::MIN;
            for &col in &ordered_moves {
                let mut test_board = board.clone();
                test_board.drop_piece(col, self.player);

                let eval = self.minimax(&test_board, depth - 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);

                if beta <= alpha {
                    break; // Beta cutoff
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            let opponent = self.player.other();
            for &col in &ordered_moves {
                let mut test_board = board.clone();
                test_board.drop_piece(col, opponent);

                let eval = self.minimax(&test_board, depth - 1, alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);

                if beta <= alpha {
                    break; // Alpha cutoff
                }
            }
            min_eval
        }
    }

    /// Evaluate the board position heuristically
    fn evaluate_position(&self, board: &Board) -> i32 {
        let mut score = 0;

        // Evaluate all possible 4-cell windows
        // Horizontal windows
        for row in 0..Board::ROWS {
            for col in 0..=(Board::COLS - 4) {
                let window: Vec<(usize, usize)> = (0..4).map(|i| (row, col + i)).collect();
                score += self.evaluate_window(board, &window);
            }
        }

        // Vertical windows
        for row in 0..=(Board::ROWS - 4) {
            for col in 0..Board::COLS {
                let window: Vec<(usize, usize)> = (0..4).map(|i| (row + i, col)).collect();
                score += self.evaluate_window(board, &window);
            }
        }

        // Diagonal windows (down-right)
        for row in 0..=(Board::ROWS - 4) {
            for col in 0..=(Board::COLS - 4) {
                let window: Vec<(usize, usize)> = (0..4).map(|i| (row + i, col + i)).collect();
                score += self.evaluate_window(board, &window);
            }
        }

        // Diagonal windows (down-left)
        for row in 0..=(Board::ROWS - 4) {
            for col in 3..Board::COLS {
                let window: Vec<(usize, usize)> = (0..4).map(|i| (row + i, col - i)).collect();
                score += self.evaluate_window(board, &window);
            }
        }

        // Bonus for center control
        let center_col = Board::COLS / 2;
        for row in 0..Board::ROWS {
            if board.grid[row][center_col] == Cell::Occupied(self.player) {
                score += 3;
            }
        }

        score
    }

    /// Evaluate a 4-cell window
    fn evaluate_window(&self, board: &Board, window: &[(usize, usize)]) -> i32 {
        let ai_count = board.count_line(window, self.player);
        let opponent_count = board.count_line(window, self.player.other());
        let empty_count = window
            .iter()
            .filter(|&&(row, col)| board.grid[row][col] == Cell::Empty)
            .count();

        // If window has both players' pieces, it's not useful
        if ai_count > 0 && opponent_count > 0 {
            return 0;
        }

        // Score based on number of AI pieces
        let mut score = 0;
        if ai_count == 3 && empty_count == 1 {
            score += 100; // One move away from winning
        } else if ai_count == 2 && empty_count == 2 {
            score += 10; // Two moves away from winning
        } else if ai_count == 1 && empty_count == 3 {
            score += 1; // Three moves away
        }

        // Block opponent threats (slightly less valuable than creating our own)
        if opponent_count == 3 && empty_count == 1 {
            score += 90; // Must block immediate threat
        } else if opponent_count == 2 && empty_count == 2 {
            score += 8;
        } else if opponent_count == 1 && empty_count == 3 {
            score += 1;
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_easy_returns_valid_move() {
        let board = Board::new();
        let ai = AI::new(Player::Yellow, Difficulty::Easy);

        let move_col = ai.get_best_move(&board);
        assert!(move_col.is_some());
        assert!(board.is_valid_move(move_col.unwrap()));
    }

    #[test]
    fn test_ai_blocks_winning_move() {
        let mut board = Board::new();
        let ai = AI::new(Player::Yellow, Difficulty::Medium);

        // Red has three in a row horizontally (0, 1, 2)
        board.drop_piece(0, Player::Red);
        board.drop_piece(1, Player::Red);
        board.drop_piece(2, Player::Red);

        // AI should block by playing in column 3
        let move_col = ai.get_best_move(&board);
        assert_eq!(move_col, Some(3));
    }

    #[test]
    fn test_ai_takes_winning_move() {
        let mut board = Board::new();
        let ai = AI::new(Player::Yellow, Difficulty::Medium);

        // Yellow has three in a row (AI's pieces)
        board.drop_piece(0, Player::Yellow);
        board.drop_piece(1, Player::Yellow);
        board.drop_piece(2, Player::Yellow);

        // AI should win by playing in column 3
        let move_col = ai.get_best_move(&board);
        assert_eq!(move_col, Some(3));
    }

    #[test]
    fn test_ai_prefers_center() {
        let board = Board::new();
        let ai = AI::new(Player::Yellow, Difficulty::Hard);

        // On an empty board, AI should prefer center columns
        let move_col = ai.get_best_move(&board);
        assert!(move_col.is_some());

        // Center column is 3, so move should be near center
        let col = move_col.unwrap();
        assert!(col >= 2 && col <= 4);
    }

    #[test]
    fn test_evaluate_position_winning() {
        let mut board = Board::new();
        let ai = AI::new(Player::Yellow, Difficulty::Medium);

        // Create a winning position for Yellow
        board.drop_piece(0, Player::Yellow);
        board.drop_piece(1, Player::Yellow);
        board.drop_piece(2, Player::Yellow);

        let score = ai.evaluate_position(&board);
        assert!(score > 100); // Should have high score for 3-in-a-row
    }

    #[test]
    fn test_difficulty_depths() {
        assert_eq!(Difficulty::Easy.depth(), 1);
        assert_eq!(Difficulty::Medium.depth(), 4);
        assert_eq!(Difficulty::Hard.depth(), 6);
        assert_eq!(Difficulty::Expert.depth(), 8);
    }

    #[test]
    fn test_ai_no_valid_moves() {
        let mut board = Board::new();
        let ai = AI::new(Player::Yellow, Difficulty::Medium);

        // Fill the entire board
        for col in 0..Board::COLS {
            for _ in 0..Board::ROWS {
                board.drop_piece(col, Player::Red);
            }
        }

        let move_col = ai.get_best_move(&board);
        assert_eq!(move_col, None);
    }

    #[test]
    fn test_ai_blocks_vertical_threat() {
        let mut board = Board::new();
        let ai = AI::new(Player::Yellow, Difficulty::Medium);

        // Red has three vertically in column 3
        board.drop_piece(3, Player::Red);
        board.drop_piece(3, Player::Red);
        board.drop_piece(3, Player::Red);

        // AI should block
        let move_col = ai.get_best_move(&board);
        assert_eq!(move_col, Some(3));
    }
}
