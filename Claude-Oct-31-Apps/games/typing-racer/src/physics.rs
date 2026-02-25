// Physics engine for falling words

use crate::word::Word;
use rand::Rng;

pub struct PhysicsEngine {
    pub words: Vec<Word>,
    spawn_timer: f32,
    spawn_rate: f32,
}

impl PhysicsEngine {
    pub fn new(spawn_rate: f32) -> Self {
        Self {
            words: Vec::new(),
            spawn_timer: 0.0,
            spawn_rate,
        }
    }

    /// Update all words and check for spawning new ones
    pub fn update(&mut self, delta_time: f32) {
        // Update existing words
        for word in &mut self.words {
            word.update(delta_time);
        }

        // Update spawn timer
        self.spawn_timer += delta_time;
    }

    /// Check if a new word should spawn
    pub fn should_spawn(&mut self) -> bool {
        if self.spawn_timer >= self.spawn_rate {
            self.spawn_timer = 0.0;
            true
        } else {
            false
        }
    }

    /// Add a new word to the play area
    pub fn add_word(&mut self, word: Word, max_words: usize) {
        if self.words.len() < max_words {
            self.words.push(word);
        } else if !self.words.is_empty() {
            // Remove the oldest (highest y) word if at capacity
            self.words.remove(0);
            self.words.push(word);
        }
    }

    /// Remove all words
    pub fn clear(&mut self) {
        self.words.clear();
    }

    /// Get random x position for spawning
    pub fn get_random_spawn_x(width: u16) -> u16 {
        let mut rng = rand::thread_rng();
        rng.gen_range(5..=width.saturating_sub(20))
    }

    /// Check for collisions with bottom
    pub fn check_bottom_collisions(&mut self, height: u16) -> Vec<String> {
        let mut collided = Vec::new();

        self.words.retain(|word| {
            if word.collided_with_bottom(height) {
                collided.push(word.text.clone());
                false
            } else {
                true
            }
        });

        collided
    }

    /// Find word that matches the input
    pub fn find_matching_word(&self, input: &str) -> Option<usize> {
        // Find exact match first
        for (i, word) in self.words.iter().enumerate() {
            if word.is_exact_match(input) {
                return Some(i);
            }
        }

        // Return first word that matches prefix
        for (i, word) in self.words.iter().enumerate() {
            if word.matches_prefix(input) {
                return Some(i);
            }
        }

        None
    }

    /// Find all words that match a prefix
    pub fn find_matching_words(&self, input: &str) -> Vec<usize> {
        self.words
            .iter()
            .enumerate()
            .filter(|(_, word)| word.matches_prefix(input))
            .map(|(i, _)| i)
            .collect()
    }

    /// Remove word at index
    pub fn remove_word(&mut self, index: usize) -> Option<Word> {
        if index < self.words.len() {
            Some(self.words.remove(index))
        } else {
            None
        }
    }

    /// Get word at index
    pub fn get_word(&self, index: usize) -> Option<&Word> {
        self.words.get(index)
    }

    /// Get total word count
    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    /// Set spawn rate
    pub fn set_spawn_rate(&mut self, rate: f32) {
        self.spawn_rate = rate;
    }
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new(2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difficulty::DifficultyLevel;

    #[test]
    fn test_physics_engine_creation() {
        let engine = PhysicsEngine::new(2.0);
        assert_eq!(engine.word_count(), 0);
    }

    #[test]
    fn test_add_word() {
        let mut engine = PhysicsEngine::new(2.0);
        let word = Word::new("HELLO".to_string(), 10, 0.5, DifficultyLevel::Easy);
        engine.add_word(word, 20);
        assert_eq!(engine.word_count(), 1);
    }

    #[test]
    fn test_max_words_limit() {
        let mut engine = PhysicsEngine::new(2.0);
        let max_words = 5;

        for i in 0..10 {
            let word = Word::new(
                format!("WORD{}", i),
                10,
                0.5,
                DifficultyLevel::Easy,
            );
            engine.add_word(word, max_words);
        }

        assert!(engine.word_count() <= max_words);
    }

    #[test]
    fn test_find_matching_word() {
        let mut engine = PhysicsEngine::new(2.0);
        let word = Word::new("HELLO".to_string(), 10, 0.5, DifficultyLevel::Easy);
        engine.add_word(word, 20);

        let index = engine.find_matching_word("HELLO");
        assert!(index.is_some());
    }

    #[test]
    fn test_find_matching_word_prefix() {
        let mut engine = PhysicsEngine::new(2.0);
        let word = Word::new("HELLO".to_string(), 10, 0.5, DifficultyLevel::Easy);
        engine.add_word(word, 20);

        let index = engine.find_matching_word("HEL");
        assert!(index.is_some());
    }

    #[test]
    fn test_collision_detection() {
        let mut engine = PhysicsEngine::new(2.0);
        let mut word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        word.y = 28.0;
        engine.add_word(word, 20);

        let collisions = engine.check_bottom_collisions(30);
        assert_eq!(collisions.len(), 1);
        assert_eq!(engine.word_count(), 0);
    }

    #[test]
    fn test_remove_word() {
        let mut engine = PhysicsEngine::new(2.0);
        let word = Word::new("HELLO".to_string(), 10, 0.5, DifficultyLevel::Easy);
        engine.add_word(word, 20);

        engine.remove_word(0);
        assert_eq!(engine.word_count(), 0);
    }

    #[test]
    fn test_spawn_rate_check() {
        let mut engine = PhysicsEngine::new(2.0);

        // Initially should not spawn
        assert!(!engine.should_spawn());

        // Update with less time than spawn rate
        engine.update(1.0);
        assert!(!engine.should_spawn());

        // Update enough to trigger spawn
        engine.update(1.5);
        assert!(engine.should_spawn());
    }

    #[test]
    fn test_clear_words() {
        let mut engine = PhysicsEngine::new(2.0);
        for i in 0..5 {
            let word = Word::new(
                format!("WORD{}", i),
                10,
                0.5,
                DifficultyLevel::Easy,
            );
            engine.add_word(word, 20);
        }

        engine.clear();
        assert_eq!(engine.word_count(), 0);
    }
}
