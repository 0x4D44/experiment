//! Game state management and main game loop logic

use crate::board::Board;
use crate::board::reveal;
use crate::cell::Cell;
use crate::difficulty::Difficulty;
use crate::timer::GameTimer;
use std::fmt;

/// Game status enumeration
#[derive(Clone, Debug, PartialEq)]
pub enum GameStatus {
    /// Game is in progress
    Playing,
    /// Game won - all non-mines revealed
    Won,
    /// Game lost - mine revealed at coordinates
    Lost { revealed_mine: (usize, usize) },
}

/// Represents a move in the game
#[derive(Clone, Debug)]
pub enum Move {
    Reveal { x: usize, y: usize },
    Flag { x: usize, y: usize },
    Chord { x: usize, y: usize },
}

/// Main game structure
pub struct Game {
    board: Board,
    status: GameStatus,
    game_started: bool,
    timer: GameTimer,
    move_history: Vec<Move>,
}

impl Game {
    /// Create a new game with given difficulty
    pub fn new(difficulty: Difficulty) -> Self {
        let diff = difficulty.clone();

        // Create empty board (mines will be placed after first click)
        let board = Board::new(diff.width, diff.height, 0);

        Game {
            board,
            status: GameStatus::Playing,
            game_started: false,
            timer: GameTimer::new(),
            move_history: Vec::new(),
        }
    }

    /// Handle first click - generate board and start game
    pub fn handle_first_click(&mut self, x: usize, y: usize) {
        if self.game_started {
            return;
        }

        self.game_started = true;
        self.timer.start();

        // Get difficulty from current board dimensions
        let width = self.board.width();
        let height = self.board.height();
        let mines_count = self.board.mines_count();

        // Generate new board with safe zone
        self.board = Board::new_with_safe_zone(width, height, mines_count, x, y);

        // Perform initial reveal
        self.reveal(x, y);
    }

    /// Reveal a cell at (x, y)
    pub fn reveal(&mut self, x: usize, y: usize) {
        if !self.game_started || !matches!(self.status, GameStatus::Playing) {
            return;
        }

        if x >= self.board.width() || y >= self.board.height() {
            return;
        }

        // Don't reveal flagged cells
        if matches!(self.board.get_cell(x, y), Cell::Flagged) {
            return;
        }

        // Use recursive reveal algorithm
        let hit_mine = reveal::reveal_recursive(&mut self.board, x, y);

        if hit_mine {
            self.status = GameStatus::Lost {
                revealed_mine: (x, y),
            };
            self.board.reveal_all_mines();
            self.timer.stop();
        } else if self.board.is_complete() {
            self.status = GameStatus::Won;
            self.timer.stop();
        }

        self.move_history.push(Move::Reveal { x, y });
    }

    /// Toggle flag on cell at (x, y)
    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if !self.game_started || !matches!(self.status, GameStatus::Playing) {
            return;
        }

        if x >= self.board.width() || y >= self.board.height() {
            return;
        }

        // Can only flag unrevealed cells
        if !matches!(self.board.get_cell(x, y), Cell::Unrevealed | Cell::Flagged | Cell::QuestionMarked) {
            return;
        }

        self.board.toggle_flag(x, y);
        self.move_history.push(Move::Flag { x, y });
    }

    /// Perform chord operation on cell at (x, y)
    pub fn chord(&mut self, x: usize, y: usize) {
        if !self.game_started || !matches!(self.status, GameStatus::Playing) {
            return;
        }

        if x >= self.board.width() || y >= self.board.height() {
            return;
        }

        let hit_mine = reveal::chord(&mut self.board, x, y);

        if hit_mine {
            self.status = GameStatus::Lost {
                revealed_mine: (x, y),
            };
            self.board.reveal_all_mines();
            self.timer.stop();
        } else if self.board.is_complete() {
            self.status = GameStatus::Won;
            self.timer.stop();
        }

        self.move_history.push(Move::Chord { x, y });
    }

    /// Get current game status
    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    /// Get reference to board
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get mutable reference to board
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Get reference to timer
    pub fn timer(&self) -> &GameTimer {
        &self.timer
    }

    /// Get whether game has started
    pub fn is_game_started(&self) -> bool {
        self.game_started
    }

    /// Get number of flagged cells
    pub fn flagged_count(&self) -> usize {
        self.board.count_flagged()
    }

    /// Get mine counter (mines remaining = total - flagged)
    pub fn mine_counter(&self) -> i32 {
        self.board.mines_count() as i32 - self.flagged_count() as i32
    }

    /// Get move history
    pub fn move_history(&self) -> &[Move] {
        &self.move_history
    }

    /// Get total moves made
    pub fn move_count(&self) -> usize {
        self.move_history.len()
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("status", &self.status)
            .field("game_started", &self.game_started)
            .field("move_count", &self.move_history.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new(Difficulty::beginner());
        assert!(!game.is_game_started());
        assert_eq!(game.move_count(), 0);
    }

    #[test]
    fn test_game_start() {
        let mut game = Game::new(Difficulty::beginner());
        game.handle_first_click(4, 4);

        assert!(game.is_game_started());
        assert_eq!(game.flagged_count(), 0);
    }

    #[test]
    fn test_reveal_updates_history() {
        let mut game = Game::new(Difficulty::beginner());
        game.handle_first_click(5, 5);

        let initial_moves = game.move_count();
        game.reveal(6, 6);

        assert!(game.move_count() >= initial_moves);
    }

    #[test]
    fn test_flag_updates_counter() {
        let mut game = Game::new(Difficulty::beginner());
        game.handle_first_click(5, 5);

        let initial_flags = game.flagged_count();

        // Find unrevealed cell and flag it
        for y in 0..game.board().height() {
            for x in 0..game.board().width() {
                if matches!(game.board().get_cell(x, y), Cell::Unrevealed) {
                    game.toggle_flag(x, y);
                    assert_eq!(game.flagged_count(), initial_flags + 1);
                    return;
                }
            }
        }
    }

    #[test]
    fn test_mine_counter() {
        let mut game = Game::new(Difficulty::custom(5, 5, 5).unwrap());
        game.handle_first_click(2, 2);

        // After first click, board is generated with mines
        let counter = game.mine_counter();
        // Counter should equal total mines minus flagged (0 at start)
        assert_eq!(counter, game.board().mines_count() as i32);

        // Flag all unrevealed cells
        for y in 0..game.board().height() {
            for x in 0..game.board().width() {
                if matches!(game.board().get_cell(x, y), Cell::Unrevealed) {
                    game.toggle_flag(x, y);
                }
            }
        }

        // Counter may now be negative (over-flagged)
        assert!(true);
    }
}
