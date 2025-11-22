use crossterm::event::KeyCode;
use std::io;

use crate::ai::{Difficulty, AI};
use crate::board::{Board, GameState, Player};
use crate::ui::UI;

/// Game mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsAI(Difficulty),
}

impl GameMode {
    pub fn description(&self) -> String {
        match self {
            GameMode::PlayerVsPlayer => "Player vs Player".to_string(),
            GameMode::PlayerVsAI(difficulty) => format!("Player vs AI ({})", difficulty.name()),
        }
    }
}

/// Game statistics
#[derive(Debug, Clone, Default)]
pub struct Statistics {
    pub red_wins: usize,
    pub yellow_wins: usize,
    pub draws: usize,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            red_wins: 0,
            yellow_wins: 0,
            draws: 0,
        }
    }

    pub fn record_result(&mut self, state: GameState) {
        match state {
            GameState::Won(Player::Red) => self.red_wins += 1,
            GameState::Won(Player::Yellow) => self.yellow_wins += 1,
            GameState::Draw => self.draws += 1,
            GameState::InProgress => {}
        }
    }
}

/// Main game controller
pub struct Game {
    board: Board,
    mode: GameMode,
    current_player: Player,
    ai: Option<AI>,
    selected_column: Option<usize>,
    stats: Statistics,
}

impl Game {
    pub fn new(mode: GameMode) -> Self {
        let ai = match mode {
            GameMode::PlayerVsAI(difficulty) => Some(AI::new(Player::Yellow, difficulty)),
            _ => None,
        };

        Game {
            board: Board::new(),
            mode,
            current_player: Player::Red,
            ai,
            selected_column: Some(3), // Start with center column selected
            stats: Statistics::new(),
        }
    }

    /// Run the main game loop
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.draw()?;

            // Check if AI should move
            if self.current_player == Player::Yellow && self.ai.is_some() {
                self.ai_move()?;
                continue;
            }

            // Get player input
            let key = UI::read_key()?;

            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return Ok(());
                }
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    self.undo_move()?;
                }
                KeyCode::Char(c) if ('1'..='7').contains(&c) => {
                    self.selected_column = UI::key_to_column(KeyCode::Char(c));
                }
                KeyCode::Left => {
                    if let Some(col) = self.selected_column {
                        if col > 0 {
                            self.selected_column = Some(col - 1);
                        }
                    }
                }
                KeyCode::Right => {
                    if let Some(col) = self.selected_column {
                        if col < Board::COLS - 1 {
                            self.selected_column = Some(col + 1);
                        }
                    }
                }
                KeyCode::Enter => {
                    if let Some(col) = self.selected_column {
                        self.make_move(col)?;
                    }
                }
                _ => {}
            }
        }
    }

    /// Make a move for the current player
    fn make_move(&mut self, col: usize) -> io::Result<()> {
        if !self.board.is_valid_move(col) {
            self.draw()?;
            UI::draw_error("Column is full! Choose another column.")?;
            std::thread::sleep(std::time::Duration::from_secs(1));
            return Ok(());
        }

        self.board.drop_piece(col, self.current_player);

        // Check game state
        match self.board.check_game_state() {
            GameState::Won(winner) => {
                self.stats.record_result(GameState::Won(winner));
                self.draw()?;
                UI::draw_win_message(winner)?;
                UI::draw_statistics(self.stats.red_wins, self.stats.yellow_wins, self.stats.draws)?;
                self.play_again()?;
            }
            GameState::Draw => {
                self.stats.record_result(GameState::Draw);
                self.draw()?;
                UI::draw_draw_message()?;
                UI::draw_statistics(self.stats.red_wins, self.stats.yellow_wins, self.stats.draws)?;
                self.play_again()?;
            }
            GameState::InProgress => {
                self.current_player = self.current_player.other();
            }
        }

        Ok(())
    }

    /// AI makes a move
    fn ai_move(&mut self) -> io::Result<()> {
        if let Some(ai) = &self.ai {
            self.draw()?;
            UI::draw_ai_thinking(ai.difficulty.name())?;

            // Small delay to show thinking message
            std::thread::sleep(std::time::Duration::from_millis(500));

            if let Some(col) = ai.get_best_move(&self.board) {
                self.board.drop_piece(col, self.current_player);

                // Check game state
                match self.board.check_game_state() {
                    GameState::Won(winner) => {
                        self.stats.record_result(GameState::Won(winner));
                        self.draw()?;
                        UI::draw_win_message(winner)?;
                        UI::draw_statistics(
                            self.stats.red_wins,
                            self.stats.yellow_wins,
                            self.stats.draws,
                        )?;
                        self.play_again()?;
                    }
                    GameState::Draw => {
                        self.stats.record_result(GameState::Draw);
                        self.draw()?;
                        UI::draw_draw_message()?;
                        UI::draw_statistics(
                            self.stats.red_wins,
                            self.stats.yellow_wins,
                            self.stats.draws,
                        )?;
                        self.play_again()?;
                    }
                    GameState::InProgress => {
                        self.current_player = self.current_player.other();
                    }
                }
            }
        }

        Ok(())
    }

    /// Undo the last move
    fn undo_move(&mut self) -> io::Result<()> {
        // In AI mode, undo both AI and player moves
        if self.ai.is_some() {
            self.board.undo_move();
            self.board.undo_move();
            self.current_player = Player::Red; // Player always goes first
        } else if self.board.undo_move().is_some() {
            self.current_player = self.current_player.other();
        }

        Ok(())
    }

    /// Draw the current game state
    fn draw(&self) -> io::Result<()> {
        UI::clear()?;
        UI::draw_board(&self.board, self.selected_column)?;
        UI::draw_status(
            self.current_player,
            self.board.move_count,
            &self.mode.description(),
        )?;
        UI::draw_controls()?;
        Ok(())
    }

    /// Ask if player wants to play again
    fn play_again(&mut self) -> io::Result<()> {
        UI::draw_play_again()?;

        loop {
            let key = UI::read_key()?;
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.reset_game();
                    return Ok(());
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }

    /// Reset the game for a new round
    fn reset_game(&mut self) {
        self.board = Board::new();
        self.current_player = Player::Red;
        self.selected_column = Some(3);
    }
}

/// Show main menu and get game mode selection
pub fn show_menu() -> io::Result<GameMode> {
    loop {
        UI::draw_menu()?;

        let key = UI::read_key()?;
        match key.code {
            KeyCode::Char('1') => return Ok(GameMode::PlayerVsPlayer),
            KeyCode::Char('2') => return Ok(GameMode::PlayerVsAI(Difficulty::Easy)),
            KeyCode::Char('3') => return Ok(GameMode::PlayerVsAI(Difficulty::Medium)),
            KeyCode::Char('4') => return Ok(GameMode::PlayerVsAI(Difficulty::Hard)),
            KeyCode::Char('5') => return Ok(GameMode::PlayerVsAI(Difficulty::Expert)),
            KeyCode::Char('q') | KeyCode::Char('Q') => std::process::exit(0),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new(GameMode::PlayerVsPlayer);
        assert_eq!(game.current_player, Player::Red);
        assert_eq!(game.board.move_count, 0);
        assert!(game.ai.is_none());
    }

    #[test]
    fn test_new_game_with_ai() {
        let game = Game::new(GameMode::PlayerVsAI(Difficulty::Medium));
        assert_eq!(game.current_player, Player::Red);
        assert!(game.ai.is_some());
    }

    #[test]
    fn test_statistics_record_win() {
        let mut stats = Statistics::new();
        stats.record_result(GameState::Won(Player::Red));
        assert_eq!(stats.red_wins, 1);
        assert_eq!(stats.yellow_wins, 0);
        assert_eq!(stats.draws, 0);
    }

    #[test]
    fn test_statistics_record_draw() {
        let mut stats = Statistics::new();
        stats.record_result(GameState::Draw);
        assert_eq!(stats.draws, 1);
    }

    #[test]
    fn test_game_mode_description() {
        assert_eq!(
            GameMode::PlayerVsPlayer.description(),
            "Player vs Player"
        );
        assert_eq!(
            GameMode::PlayerVsAI(Difficulty::Hard).description(),
            "Player vs AI (Hard)"
        );
    }

    #[test]
    fn test_reset_game() {
        let mut game = Game::new(GameMode::PlayerVsPlayer);

        // Make some moves
        game.board.drop_piece(0, Player::Red);
        game.board.drop_piece(1, Player::Yellow);
        game.current_player = Player::Yellow;

        // Reset
        game.reset_game();

        assert_eq!(game.board.move_count, 0);
        assert_eq!(game.current_player, Player::Red);
        assert_eq!(game.selected_column, Some(3));
    }
}
