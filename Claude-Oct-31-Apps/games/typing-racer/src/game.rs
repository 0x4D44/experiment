// Main game state and logic

use crate::config::*;
use crate::dictionary::Dictionary;
use crate::difficulty::{DifficultyLevel, DifficultyProgression};
use crate::input::InputBuffer;
use crate::physics::PhysicsEngine;
use crate::scoring::ScoreEngine;
use crate::word::Word;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct Game {
    pub state: GameState,
    pub lives: usize,
    pub score: usize,
    pub words: Vec<Word>,
    pub input: InputBuffer,
    pub elapsed_time: f32,
    pub difficulty: DifficultyLevel,
    pub physics: PhysicsEngine,
    pub score_engine: ScoreEngine,
    pub difficulty_progression: DifficultyProgression,
    pub dictionary: Dictionary,
    last_spawn_time: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Menu,
            lives: DEFAULT_LIVES,
            score: 0,
            words: Vec::new(),
            input: InputBuffer::new(),
            elapsed_time: 0.0,
            difficulty: DifficultyLevel::Easy,
            physics: PhysicsEngine::new(DifficultyLevel::Easy.spawn_rate()),
            score_engine: ScoreEngine::new(),
            difficulty_progression: DifficultyProgression::new(),
            dictionary: Dictionary::new(),
            last_spawn_time: 0.0,
        }
    }

    pub fn start_new_game(&mut self, difficulty: DifficultyLevel) {
        self.state = GameState::Playing;
        self.lives = DEFAULT_LIVES;
        self.score = 0;
        self.words.clear();
        self.input.clear();
        self.elapsed_time = 0.0;
        self.difficulty = difficulty;
        self.physics = PhysicsEngine::new(difficulty.spawn_rate());
        self.score_engine = ScoreEngine::new();
        self.difficulty_progression = DifficultyProgression::new();
        self.last_spawn_time = 0.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.state != GameState::Playing {
            return;
        }

        self.elapsed_time += delta_time;
        self.score_engine.update_time(delta_time as f64);

        // Update difficulty
        self.difficulty_progression.update(delta_time);

        // Update physics
        self.physics.update(delta_time);

        // Check for spawning new word
        if self.physics.should_spawn() && self.words.len() < MAX_WORDS_ON_SCREEN {
            self.spawn_word();
        }

        // Check for collisions
        let collisions = self.physics.check_bottom_collisions(PLAY_AREA_HEIGHT);
        for word in collisions {
            self.lose_life();
            self.score_engine.add_missed_word(&word);
        }

        // Update words reference
        self.words = self.physics.words.clone();

        // Check game over
        if self.lives == 0 {
            self.state = GameState::GameOver;
        }
    }

    pub fn spawn_word(&mut self) {
        let word_text = self.dictionary.select_random(self.difficulty);
        let x = PhysicsEngine::get_random_spawn_x(PLAY_AREA_WIDTH);
        let speed = self.difficulty.to_speed_multiplier();

        let word = Word::new(word_text, x, speed, self.difficulty);
        self.physics.add_word(word, MAX_WORDS_ON_SCREEN);
    }

    pub fn handle_input(&mut self, c: char) {
        if self.state != GameState::Playing {
            return;
        }

        match c {
            ' ' => {
                // Space: try to match word
                if !self.input.is_empty() {
                    self.try_match_word();
                }
            }
            '\u{0008}' | '\u{007F}' => {
                // Backspace
                self.input.backspace();
            }
            c if c.is_alphabetic() => {
                self.input.add_char(c);
            }
            _ => {}
        }
    }

    fn try_match_word(&mut self) {
        let input_text = self.input.get_text();

        // Find matching word
        if let Some(index) = self.physics.find_matching_word(&input_text) {
            if let Some(word) = self.physics.remove_word(index) {
                // Calculate score
                let accuracy_mult = self.score_engine.get_accuracy_multiplier();
                let combo_mult = self.difficulty_progression.combo_multiplier(
                    self.score_engine.combo_count,
                );

                self.score_engine.add_word(
                    &word.text,
                    self.difficulty,
                    accuracy_mult,
                    combo_mult,
                );

                self.score = self.score_engine.total_score;
                self.words = self.physics.words.clone();
            }
        } else {
            // Word not found - penalize
            self.score_engine.add_missed_word(&input_text);
        }

        self.input.clear();
    }

    pub fn lose_life(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
        }
    }

    pub fn add_word(&mut self, word_text: String, difficulty: DifficultyLevel) {
        let x = PhysicsEngine::get_random_spawn_x(PLAY_AREA_WIDTH);
        let speed = difficulty.to_speed_multiplier();
        let word = Word::new(word_text, x, speed, difficulty);
        self.physics.add_word(word, MAX_WORDS_ON_SCREEN);
        self.words = self.physics.words.clone();
    }

    pub fn is_game_over(&self) -> bool {
        self.state == GameState::GameOver
    }

    pub fn get_wpm(&self) -> f64 {
        self.score_engine.calculate_wpm()
    }

    pub fn get_accuracy(&self) -> f64 {
        self.score_engine.calculate_accuracy()
    }

    pub fn current_speed(&self, word_count: f32) -> f32 {
        let base_speed = self.difficulty.to_speed_multiplier();
        self.difficulty_progression.calculate_speed(
            base_speed,
            word_count as usize,
        )
    }

    pub fn speed_with_word_count(&self, word_count: usize, _base: f32) -> f32 {
        let base_speed = self.difficulty.to_speed_multiplier();
        self.difficulty_progression.calculate_speed(base_speed, word_count)
    }

    pub fn combo_multiplier(&self, combo: usize) -> f64 {
        self.difficulty_progression.combo_multiplier(combo)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_initialization() {
        let game = Game::new();
        assert_eq!(game.state, GameState::Menu);
        assert_eq!(game.lives, DEFAULT_LIVES);
        assert_eq!(game.score, 0);
    }

    #[test]
    fn test_game_start() {
        let mut game = Game::new();
        game.start_new_game(DifficultyLevel::Easy);
        assert_eq!(game.state, GameState::Playing);
        assert_eq!(game.difficulty, DifficultyLevel::Easy);
    }

    #[test]
    fn test_lose_life() {
        let mut game = Game::new();
        game.start_new_game(DifficultyLevel::Easy);
        let initial_lives = game.lives;
        game.lose_life();
        assert_eq!(game.lives, initial_lives - 1);
    }

    #[test]
    fn test_game_over_condition() {
        let mut game = Game::new();
        game.start_new_game(DifficultyLevel::Easy);
        game.lives = 0;
        game.update(0.1); // Trigger update to set game over state
        assert!(game.is_game_over());
    }

    #[test]
    fn test_add_word() {
        let mut game = Game::new();
        game.start_new_game(DifficultyLevel::Easy);
        game.add_word("HELLO".to_string(), DifficultyLevel::Easy);
        assert_eq!(game.words.len(), 1);
    }

    #[test]
    fn test_spawn_word() {
        let mut game = Game::new();
        game.start_new_game(DifficultyLevel::Easy);
        game.spawn_word();
        game.words = game.physics.words.clone(); // Sync words from physics
        assert_eq!(game.words.len(), 1);
    }

    #[test]
    fn test_handle_input_alphabetic() {
        let mut game = Game::new();
        game.start_new_game(DifficultyLevel::Easy);
        game.handle_input('h');
        assert_eq!(game.input.get_text(), "H");
    }
}
