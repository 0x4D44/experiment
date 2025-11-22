use crate::chess::{Board, Color, Move};

pub struct ChessAI {
    depth: u8,
}

impl ChessAI {
    pub fn new(depth: u8) -> Self {
        ChessAI { depth }
    }

    /// Find the best move using minimax with alpha-beta pruning
    pub fn find_best_move(&self, board: &Board, color: Color) -> Option<Move> {
        let legal_moves = board.generate_legal_moves(color);

        if legal_moves.is_empty() {
            return None;
        }

        let mut best_move = legal_moves[0];
        let mut best_score = i32::MIN;
        let mut alpha = i32::MIN;
        let beta = i32::MAX;

        for mov in legal_moves {
            let mut new_board = board.clone();
            new_board.make_move(&mov);

            let score = -self.minimax(&new_board, self.depth - 1, -beta, -alpha, color.opposite());

            if score > best_score {
                best_score = score;
                best_move = mov;
            }

            alpha = alpha.max(score);
        }

        Some(best_move)
    }

    /// Minimax algorithm with alpha-beta pruning
    fn minimax(&self, board: &Board, depth: u8, mut alpha: i32, beta: i32, color: Color) -> i32 {
        // Base case: depth 0 or game over
        if depth == 0 {
            return self.evaluate_position(board, color);
        }

        let legal_moves = board.generate_legal_moves(color);

        // Checkmate or stalemate
        if legal_moves.is_empty() {
            if board.is_in_check(color) {
                // Checkmate - heavily penalize
                return -100000 + (self.depth - depth) as i32; // Prefer quicker mates
            } else {
                // Stalemate
                return 0;
            }
        }

        let mut max_score = i32::MIN;

        for mov in legal_moves {
            let mut new_board = board.clone();
            new_board.make_move(&mov);

            let score = -self.minimax(&new_board, depth - 1, -beta, -alpha, color.opposite());

            max_score = max_score.max(score);
            alpha = alpha.max(score);

            // Alpha-beta pruning
            if alpha >= beta {
                break;
            }
        }

        max_score
    }

    /// Evaluate the board position from the perspective of the given color
    fn evaluate_position(&self, board: &Board, color: Color) -> i32 {
        let base_eval = board.evaluate();

        // Adjust based on whose turn it is
        let eval = match color {
            Color::White => base_eval,
            Color::Black => -base_eval,
        };

        // Add mobility bonus (number of legal moves)
        let mobility = board.generate_legal_moves(color).len() as i32;
        let opponent_mobility = board.generate_legal_moves(color.opposite()).len() as i32;
        let mobility_score = (mobility - opponent_mobility) * 10;

        eval + mobility_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_creation() {
        let ai = ChessAI::new(3);
        assert_eq!(ai.depth, 3);
    }
}
