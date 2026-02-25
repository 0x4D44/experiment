// Difficulty level management and progression

use crate::config::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl DifficultyLevel {
    /// Get the base speed multiplier for this difficulty
    pub fn to_speed_multiplier(self) -> f32 {
        match self {
            DifficultyLevel::Easy => BASE_SPEED_EASY,
            DifficultyLevel::Medium => BASE_SPEED_MEDIUM,
            DifficultyLevel::Hard => BASE_SPEED_HARD,
            DifficultyLevel::Expert => BASE_SPEED_EXPERT,
        }
    }

    /// Get base points for correctly typing a word at this difficulty
    pub fn base_points(self) -> usize {
        match self {
            DifficultyLevel::Easy => POINTS_EASY,
            DifficultyLevel::Medium => POINTS_MEDIUM,
            DifficultyLevel::Hard => POINTS_HARD,
            DifficultyLevel::Expert => POINTS_EXPERT,
        }
    }

    /// Get spawn rate (seconds between word spawns)
    pub fn spawn_rate(self) -> f32 {
        match self {
            DifficultyLevel::Easy => SPAWN_RATE_EASY,
            DifficultyLevel::Medium => SPAWN_RATE_MEDIUM,
            DifficultyLevel::Hard => SPAWN_RATE_HARD,
            DifficultyLevel::Expert => SPAWN_RATE_EXPERT,
        }
    }

    /// Check if a word length is valid for this difficulty
    pub fn is_valid_word_length(self, length: usize) -> bool {
        match self {
            DifficultyLevel::Easy => {
                length >= WORD_LENGTH_EASY_MIN && length <= WORD_LENGTH_EASY_MAX
            }
            DifficultyLevel::Medium => {
                length >= WORD_LENGTH_MEDIUM_MIN && length <= WORD_LENGTH_MEDIUM_MAX
            }
            DifficultyLevel::Hard => {
                length >= WORD_LENGTH_HARD_MIN && length <= WORD_LENGTH_HARD_MAX
            }
            DifficultyLevel::Expert => {
                length >= WORD_LENGTH_EXPERT_MIN && length <= WORD_LENGTH_EXPERT_MAX
            }
        }
    }

    /// Get all difficulty levels
    pub fn all() -> [DifficultyLevel; 4] {
        [
            DifficultyLevel::Easy,
            DifficultyLevel::Medium,
            DifficultyLevel::Hard,
            DifficultyLevel::Expert,
        ]
    }
}

impl fmt::Display for DifficultyLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DifficultyLevel::Easy => write!(f, "Easy"),
            DifficultyLevel::Medium => write!(f, "Medium"),
            DifficultyLevel::Hard => write!(f, "Hard"),
            DifficultyLevel::Expert => write!(f, "Expert"),
        }
    }
}

pub struct DifficultyProgression {
    /// Current base difficulty multiplier
    current_difficulty: f32,
    /// Time since last difficulty increase
    time_since_increase: f32,
}

impl DifficultyProgression {
    pub fn new() -> Self {
        Self {
            current_difficulty: 1.0,
            time_since_increase: 0.0,
        }
    }

    /// Update difficulty based on elapsed time
    /// Returns true if difficulty increased
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.time_since_increase += delta_time;

        if self.time_since_increase >= DIFFICULTY_INCREASE_INTERVAL {
            self.time_since_increase = 0.0;
            if self.current_difficulty < MAX_DIFFICULTY {
                self.current_difficulty =
                    (self.current_difficulty * (1.0 + DIFFICULTY_INCREASE_RATE)).min(MAX_DIFFICULTY);
                return true;
            }
        }
        false
    }

    /// Get current difficulty multiplier (1.0 = normal, >1.0 = harder)
    pub fn get_multiplier(&self) -> f32 {
        self.current_difficulty
    }

    /// Calculate speed multiplier based on difficulty and word count
    pub fn calculate_speed(&self, base_speed: f32, word_count: usize) -> f32 {
        let base = base_speed * self.current_difficulty;

        // Reduce speed slightly if many words on screen
        let word_count_factor = match word_count {
            0..=5 => 1.0,
            6..=10 => 0.95,
            11..=15 => 0.90,
            _ => 0.85,
        };

        base * word_count_factor
    }

    /// Calculate combo multiplier for scoring
    pub fn combo_multiplier(&self, combo_count: usize) -> f64 {
        match combo_count {
            0..=4 => 1.0,
            5..=9 => 1.1,
            10..=19 => 1.25,
            20..=49 => 1.5,
            50..=99 => 2.0,
            100.. => 3.0,
        }
    }
}

impl Default for DifficultyProgression {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_speed_progression() {
        let mut prog = DifficultyProgression::new();
        let initial = prog.get_multiplier();

        prog.update(DIFFICULTY_INCREASE_INTERVAL);
        let after_increase = prog.get_multiplier();

        assert!(after_increase > initial, "Difficulty should increase");
    }

    #[test]
    fn test_difficulty_max_cap() {
        let mut prog = DifficultyProgression::new();
        for _ in 0..100 {
            prog.update(DIFFICULTY_INCREASE_INTERVAL);
        }

        assert!(
            prog.get_multiplier() <= MAX_DIFFICULTY,
            "Difficulty should not exceed maximum"
        );
    }

    #[test]
    fn test_speed_calculation_with_many_words() {
        let prog = DifficultyProgression::new();
        let speed_few = prog.calculate_speed(1.0, 5);
        let speed_many = prog.calculate_speed(1.0, 20);

        assert!(speed_many < speed_few, "More words should reduce speed");
    }

    #[test]
    fn test_combo_multiplier_progression() {
        let prog = DifficultyProgression::new();

        let m_0 = prog.combo_multiplier(0);
        let m_5 = prog.combo_multiplier(5);
        let m_10 = prog.combo_multiplier(10);
        let m_20 = prog.combo_multiplier(20);
        let m_50 = prog.combo_multiplier(50);

        assert!(m_5 > m_0);
        assert!(m_10 > m_5);
        assert!(m_20 > m_10);
        assert!(m_50 > m_20);
    }
}
