// Word representation and manipulation

use crate::difficulty::DifficultyLevel;
use crossterm::style::Color;

#[derive(Debug, Clone)]
pub struct Word {
    pub text: String,
    pub x: u16,
    pub y: f32,
    pub speed: f32,
    pub difficulty: DifficultyLevel,
}

impl Word {
    pub fn new(text: String, x: u16, speed: f32, difficulty: DifficultyLevel) -> Self {
        Self {
            text: text.to_uppercase(),
            x,
            y: 0.0,
            speed,
            difficulty,
        }
    }

    /// Update word position (falling)
    pub fn update(&mut self, delta_time: f32) {
        self.y += self.speed * delta_time;
    }

    /// Check if input buffer matches word prefix (case-insensitive)
    pub fn matches_prefix(&self, input: &str) -> bool {
        let input_upper = input.to_uppercase();
        self.text.starts_with(&input_upper) && input_upper.len() <= self.text.len()
    }

    /// Check if input exactly matches word (case-insensitive)
    pub fn is_exact_match(&self, input: &str) -> bool {
        self.text.to_lowercase() == input.to_lowercase()
    }

    /// Get length of word
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Check if word is empty
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Check if word has collided with bottom of screen
    pub fn collided_with_bottom(&self, play_area_height: u16) -> bool {
        self.y >= (play_area_height - 2) as f32
    }

    /// Check if word is off-screen to the right
    pub fn off_screen_right(&self, play_area_width: u16) -> bool {
        self.x as u32 > play_area_width as u32 + 10
    }

    /// Check if word is off-screen to the left
    pub fn off_screen_left(&self) -> bool {
        (self.x as i32) < -20
    }

    /// Get the color for this word based on difficulty
    pub fn get_color(&self) -> Color {
        match self.difficulty {
            DifficultyLevel::Easy => Color::Green,
            DifficultyLevel::Medium => Color::Yellow,
            DifficultyLevel::Hard => Color::Red,
            DifficultyLevel::Expert => Color::Magenta,
        }
    }

    /// Get score for completing this word
    pub fn get_base_score(&self) -> usize {
        self.difficulty.base_points() * self.len()
    }

    /// Get how much of the word has been typed
    pub fn get_typed_portion(&self, input: &str) -> usize {
        let input_upper = input.to_uppercase();
        if self.matches_prefix(&input_upper) {
            input_upper.len()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_creation() {
        let word = Word::new("HELLO".to_string(), 10, 0.5, DifficultyLevel::Easy);
        assert_eq!(word.text, "HELLO");
        assert_eq!(word.x, 10);
        assert_eq!(word.difficulty, DifficultyLevel::Easy);
    }

    #[test]
    fn test_word_case_normalization() {
        let word = Word::new("hello".to_string(), 10, 0.5, DifficultyLevel::Easy);
        assert_eq!(word.text, "HELLO");
    }

    #[test]
    fn test_word_falling() {
        let mut word = Word::new("HELLO".to_string(), 10, 1.0, DifficultyLevel::Easy);
        let initial_y = word.y;
        word.update(1.0);
        assert!(word.y > initial_y);
    }

    #[test]
    fn test_word_match_prefix() {
        let word = Word::new("HELLO".to_string(), 10, 0.5, DifficultyLevel::Easy);
        assert!(word.matches_prefix("HE"));
        assert!(word.matches_prefix("HEL"));
        assert!(!word.matches_prefix("HI"));
    }

    #[test]
    fn test_word_exact_match() {
        let word = Word::new("HELLO".to_string(), 10, 0.5, DifficultyLevel::Easy);
        assert!(word.is_exact_match("HELLO"));
        assert!(word.is_exact_match("hello"));
        assert!(!word.is_exact_match("HELP"));
    }

    #[test]
    fn test_word_collision() {
        let mut word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        word.y = 28.0;
        assert!(word.collided_with_bottom(30));
    }

    #[test]
    fn test_word_color_coding() {
        let easy = Word::new("HI".to_string(), 10, 0.5, DifficultyLevel::Easy);
        let hard = Word::new("DIFFICULT".to_string(), 10, 0.5, DifficultyLevel::Hard);

        assert_eq!(easy.get_color(), Color::Green);
        assert_eq!(hard.get_color(), Color::Red);
    }
}
