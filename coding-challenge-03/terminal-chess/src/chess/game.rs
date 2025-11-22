use super::{Board, Color, Move, Position};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsAI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    Playing,
    Check,
    Checkmate(Color), // Winner
    Stalemate,
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HistoryEntry {
    mov: Move,
    en_passant: Option<Position>,
    castling: (bool, bool, bool, bool),
    halfmove: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    pub current_player: Color,
    pub state: GameState,
    pub mode: GameMode,
    pub ai_difficulty: u8,
    history: Vec<HistoryEntry>,
}

impl Game {
    pub fn new(mode: GameMode, ai_difficulty: u8) -> Self {
        let board = Board::new();
        let state = Self::determine_state(&board, Color::White);

        Game {
            board,
            current_player: Color::White,
            state,
            mode,
            ai_difficulty,
            history: Vec::new(),
        }
    }

    /// Get all legal moves for the current player
    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.board.generate_legal_moves(self.current_player)
    }

    /// Make a move if it's legal
    pub fn make_move(&mut self, mov: Move) -> Result<(), String> {
        // Verify it's a legal move
        let legal_moves = self.get_legal_moves();
        if !legal_moves.contains(&mov) {
            return Err("Illegal move".to_string());
        }

        // Save state for undo
        let entry = HistoryEntry {
            mov,
            en_passant: self.board.en_passant_target,
            castling: (
                self.board.white_can_castle_kingside,
                self.board.white_can_castle_queenside,
                self.board.black_can_castle_kingside,
                self.board.black_can_castle_queenside,
            ),
            halfmove: self.board.halfmove_clock,
        };
        self.history.push(entry);

        // Make the move
        self.board.make_move(&mov);

        // Update fullmove number
        if self.current_player == Color::Black {
            self.board.fullmove_number += 1;
        }

        // Switch player
        self.current_player = self.current_player.opposite();

        // Update game state
        self.state = Self::determine_state(&self.board, self.current_player);

        Ok(())
    }

    /// Undo the last move
    pub fn undo_move(&mut self) -> Result<(), String> {
        if let Some(entry) = self.history.pop() {
            self.board.unmake_move(&entry.mov, entry.en_passant, entry.castling, entry.halfmove);

            // Switch player back
            self.current_player = self.current_player.opposite();

            // Update fullmove number
            if self.current_player == Color::Black {
                self.board.fullmove_number -= 1;
            }

            // Update game state
            self.state = Self::determine_state(&self.board, self.current_player);

            Ok(())
        } else {
            Err("No moves to undo".to_string())
        }
    }

    /// Determine the current game state
    fn determine_state(board: &Board, current_player: Color) -> GameState {
        let legal_moves = board.generate_legal_moves(current_player);
        let in_check = board.is_in_check(current_player);

        if legal_moves.is_empty() {
            if in_check {
                GameState::Checkmate(current_player.opposite())
            } else {
                GameState::Stalemate
            }
        } else if in_check {
            GameState::Check
        } else if board.halfmove_clock >= 100 {
            // 50-move rule (halfmove clock counts half-moves)
            GameState::Draw
        } else {
            GameState::Playing
        }
    }

    /// Check if the game is over
    pub fn is_game_over(&self) -> bool {
        matches!(self.state, GameState::Checkmate(_) | GameState::Stalemate | GameState::Draw)
    }

    /// Get the move history
    pub fn get_history(&self) -> Vec<Move> {
        self.history.iter().map(|entry| entry.mov).collect()
    }

    /// Get captured pieces
    pub fn get_captured_pieces(&self) -> (Vec<super::Piece>, Vec<super::Piece>) {
        use super::{Piece, PieceType};

        let mut white_captured = Vec::new();
        let mut black_captured = Vec::new();

        // Count pieces on board
        let mut white_pieces = [0; 6];
        let mut black_pieces = [0; 6];

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board.get_piece(Position::new(row, col).unwrap()) {
                    let idx = match piece.piece_type {
                        PieceType::Pawn => 0,
                        PieceType::Knight => 1,
                        PieceType::Bishop => 2,
                        PieceType::Rook => 3,
                        PieceType::Queen => 4,
                        PieceType::King => 5,
                    };

                    if piece.color == Color::White {
                        white_pieces[idx] += 1;
                    } else {
                        black_pieces[idx] += 1;
                    }
                }
            }
        }

        // Starting piece counts
        let starting_counts = [8, 2, 2, 2, 1, 1]; // pawns, knights, bishops, rooks, queens, kings

        // Calculate captured pieces
        let piece_types = [
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ];

        for (idx, &piece_type) in piece_types.iter().enumerate() {
            let white_missing = starting_counts[idx] - white_pieces[idx];
            for _ in 0..white_missing {
                white_captured.push(Piece::new(piece_type, Color::White));
            }

            let black_missing = starting_counts[idx] - black_pieces[idx];
            for _ in 0..black_missing {
                black_captured.push(Piece::new(piece_type, Color::Black));
            }
        }

        (white_captured, black_captured)
    }

    /// Save game to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize game: {}", e))?;

        fs::write(path, json)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }

    /// Load game from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let json = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let game = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize game: {}", e))?;

        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new(GameMode::PlayerVsPlayer, 3);
        assert_eq!(game.current_player, Color::White);
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn test_legal_move() {
        let mut game = Game::new(GameMode::PlayerVsPlayer, 3);
        let from = Position::from_algebraic("e2").unwrap();
        let to = Position::from_algebraic("e4").unwrap();
        let piece = game.board.get_piece(from).unwrap();
        let mov = Move::new(from, to, piece);

        assert!(game.make_move(mov).is_ok());
        assert_eq!(game.current_player, Color::Black);
    }

    #[test]
    fn test_undo_move() {
        let mut game = Game::new(GameMode::PlayerVsPlayer, 3);
        let from = Position::from_algebraic("e2").unwrap();
        let to = Position::from_algebraic("e4").unwrap();
        let piece = game.board.get_piece(from).unwrap();
        let mov = Move::new(from, to, piece);

        game.make_move(mov).unwrap();
        assert_eq!(game.current_player, Color::Black);

        game.undo_move().unwrap();
        assert_eq!(game.current_player, Color::White);
        assert!(game.board.get_piece(from).is_some());
        assert!(game.board.get_piece(to).is_none());
    }

    #[test]
    fn test_checkmate_detection() {
        let mut game = Game::new(GameMode::PlayerVsPlayer, 3);

        // Set up fool's mate position
        // 1. f3 e6 2. g4 Qh4#
        let moves = [
            ("f2", "f3"),
            ("e7", "e6"),
            ("g2", "g4"),
            ("d8", "h4"),
        ];

        for (from_str, to_str) in moves.iter() {
            let from = Position::from_algebraic(from_str).unwrap();
            let to = Position::from_algebraic(to_str).unwrap();
            let piece = game.board.get_piece(from).unwrap();
            let mov = Move::new(from, to, piece);
            game.make_move(mov).unwrap();
        }

        assert!(matches!(game.state, GameState::Checkmate(_)));
    }
}
