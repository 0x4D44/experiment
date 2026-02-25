// Scoring system and WPM calculation

use crate::difficulty::DifficultyLevel;
use crate::config::*;

pub struct ScoreEngine {
    pub total_score: usize,
    pub correct_words: usize,
    pub incorrect_words: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub combo_count: usize,
    pub best_combo: usize,
    pub elapsed_seconds: f64,
}

impl ScoreEngine {
    pub fn new() -> Self {
        Self {
            total_score: 0,
            correct_words: 0,
            incorrect_words: 0,
            correct_chars: 0,
            incorrect_chars: 0,
            combo_count: 0,
            best_combo: 0,
            elapsed_seconds: 0.0,
        }
    }

    /// Add points for a correctly typed word
    pub fn add_word(
        &mut self,
        word: &str,
        difficulty: DifficultyLevel,
        accuracy_multiplier: f64,
        combo_multiplier: f64,
    ) {
        let base_points = difficulty.base_points() as f64;
        let word_length = word.len() as f64;
        let score = (base_points * word_length * accuracy_multiplier * combo_multiplier) as usize;

        self.total_score += score;
        self.correct_words += 1;
        self.correct_chars += word.len();
        self.combo_count += 1;

        if self.combo_count > self.best_combo {
            self.best_combo = self.combo_count;
        }

        // Add combo bonuses
        self.total_score += self.get_combo_bonus();
    }

    /// Mark a word as missed/incorrect
    pub fn add_missed_word(&mut self, word: &str) {
        self.incorrect_words += 1;
        self.incorrect_chars += word.len();
        self.combo_count = 0;
    }

    /// Get combo bonus points
    fn get_combo_bonus(&self) -> usize {
        match self.combo_count {
            10 => COMBO_BONUS_10,
            20 => COMBO_BONUS_20,
            _ => 0,
        }
    }

    /// Calculate WPM (Words Per Minute)
    pub fn calculate_wpm(&self) -> f64 {
        if self.elapsed_seconds == 0.0 {
            return 0.0;
        }

        let total_chars = self.correct_chars as f64;
        let words = total_chars / 5.0; // 5 is the standard word length
        let minutes = self.elapsed_seconds / 60.0;

        if minutes == 0.0 {
            0.0
        } else {
            words / minutes
        }
    }

    /// Calculate accuracy percentage
    pub fn calculate_accuracy(&self) -> f64 {
        let total = self.correct_words + self.incorrect_words;
        if total == 0 {
            100.0
        } else {
            (self.correct_words as f64 / total as f64) * 100.0
        }
    }

    /// Get accuracy multiplier for scoring
    pub fn get_accuracy_multiplier(&self) -> f64 {
        let accuracy = self.calculate_accuracy();
        if accuracy >= 100.0 {
            ACCURACY_MULTIPLIER_PERFECT
        } else if accuracy >= 90.0 {
            ACCURACY_MULTIPLIER_HIGH
        } else if accuracy >= 80.0 {
            ACCURACY_MULTIPLIER_NORMAL
        } else {
            ACCURACY_MULTIPLIER_LOW
        }
    }

    /// Update elapsed time
    pub fn update_time(&mut self, delta_seconds: f64) {
        self.elapsed_seconds += delta_seconds;
    }

    /// Get statistics summary
    pub fn get_stats(&self) -> Stats {
        Stats {
            wpm: self.calculate_wpm(),
            accuracy: self.calculate_accuracy(),
            total_score: self.total_score,
            correct_words: self.correct_words,
            incorrect_words: self.incorrect_words,
            combo_count: self.combo_count,
            best_combo: self.best_combo,
            elapsed_seconds: self.elapsed_seconds,
        }
    }
}

impl Default for ScoreEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub wpm: f64,
    pub accuracy: f64,
    pub total_score: usize,
    pub correct_words: usize,
    pub incorrect_words: usize,
    pub combo_count: usize,
    pub best_combo: usize,
    pub elapsed_seconds: f64,
}

impl Stats {
    pub fn format_wpm(&self) -> String {
        format!("{:.1}", self.wpm)
    }

    pub fn format_accuracy(&self) -> String {
        format!("{:.1}%", self.accuracy)
    }

    pub fn format_time(&self) -> String {
        let minutes = self.elapsed_seconds as u32 / 60;
        let seconds = (self.elapsed_seconds as u32) % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wpm_calculation_basic() {
        let mut engine = ScoreEngine::new();
        engine.correct_chars = 50;
        engine.elapsed_seconds = 60.0; // 1 minute

        // (50 / 5) / 1 = 10 WPM
        assert_eq!(engine.calculate_wpm(), 10.0);
    }

    #[test]
    fn test_wpm_calculation_half_minute() {
        let mut engine = ScoreEngine::new();
        engine.correct_chars = 50;
        engine.elapsed_seconds = 30.0; // 30 seconds

        // (50 / 5) / 0.5 = 20 WPM
        assert_eq!(engine.calculate_wpm(), 20.0);
    }

    #[test]
    fn test_accuracy_perfect() {
        let mut engine = ScoreEngine::new();
        engine.correct_words = 10;
        engine.incorrect_words = 0;

        assert_eq!(engine.calculate_accuracy(), 100.0);
    }

    #[test]
    fn test_accuracy_50_percent() {
        let mut engine = ScoreEngine::new();
        engine.correct_words = 10;
        engine.incorrect_words = 10;

        assert_eq!(engine.calculate_accuracy(), 50.0);
    }

    #[test]
    fn test_accuracy_multiplier_perfect() {
        let mut engine = ScoreEngine::new();
        engine.correct_words = 10;
        engine.incorrect_words = 0;

        assert_eq!(
            engine.get_accuracy_multiplier(),
            ACCURACY_MULTIPLIER_PERFECT
        );
    }

    #[test]
    fn test_accuracy_multiplier_high() {
        let mut engine = ScoreEngine::new();
        engine.correct_words = 9;
        engine.incorrect_words = 1;

        assert_eq!(engine.get_accuracy_multiplier(), ACCURACY_MULTIPLIER_HIGH);
    }

    #[test]
    fn test_accuracy_multiplier_low() {
        let mut engine = ScoreEngine::new();
        engine.correct_words = 5;
        engine.incorrect_words = 10;

        assert_eq!(engine.get_accuracy_multiplier(), ACCURACY_MULTIPLIER_LOW);
    }

    #[test]
    fn test_word_score_calculation() {
        let mut engine = ScoreEngine::new();
        engine.add_word("HELLO", DifficultyLevel::Easy, 1.0, 1.0);

        // Easy: 10 points * 5 chars = 50 points
        assert!(engine.total_score >= 50);
    }

    #[test]
    fn test_combo_increases() {
        let mut engine = ScoreEngine::new();
        assert_eq!(engine.combo_count, 0);

        engine.add_word("HELLO", DifficultyLevel::Easy, 1.0, 1.0);
        assert_eq!(engine.combo_count, 1);

        engine.add_word("WORLD", DifficultyLevel::Easy, 1.0, 1.0);
        assert_eq!(engine.combo_count, 2);
    }

    #[test]
    fn test_combo_resets() {
        let mut engine = ScoreEngine::new();
        engine.add_word("HELLO", DifficultyLevel::Easy, 1.0, 1.0);
        assert_eq!(engine.combo_count, 1);

        engine.add_missed_word("WORLD");
        assert_eq!(engine.combo_count, 0);
    }

    #[test]
    fn test_best_combo_tracking() {
        let mut engine = ScoreEngine::new();

        // Build combo to 5
        for _ in 0..5 {
            engine.add_word("TEST", DifficultyLevel::Easy, 1.0, 1.0);
        }
        assert_eq!(engine.best_combo, 5);

        // Reset and build to 10
        engine.add_missed_word("TEST");
        for _ in 0..10 {
            engine.add_word("TEST", DifficultyLevel::Easy, 1.0, 1.0);
        }
        assert_eq!(engine.best_combo, 10);
    }

    #[test]
    fn test_stats_formatting() {
        let mut engine = ScoreEngine::new();
        engine.correct_chars = 100;
        engine.elapsed_seconds = 120.0;
        engine.correct_words = 8;
        engine.incorrect_words = 2;

        let stats = engine.get_stats();
        assert!(!stats.format_wpm().is_empty());
        assert!(!stats.format_accuracy().is_empty());
        assert!(!stats.format_time().is_empty());
    }
}
