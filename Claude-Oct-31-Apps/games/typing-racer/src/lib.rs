// Typing Speed Racer - Core Library
// Contains all reusable game logic and tests

pub mod config;
pub mod dictionary;
pub mod difficulty;
pub mod game;
pub mod input;
pub mod physics;
pub mod render;
pub mod scoring;
pub mod word;

// Re-export commonly used types
pub use config::*;
pub use dictionary::Dictionary;
pub use difficulty::DifficultyLevel;
pub use game::{Game, GameState};
pub use input::InputBuffer;
pub use render::Renderer;
pub use scoring::ScoreEngine;
pub use word::Word;

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== WORD RANDOMIZATION TESTS ====================
    #[test]
    fn test_word_selection_randomization() {
        let dict = Dictionary::new();
        let mut words_selected = std::collections::HashSet::new();

        // Select 100 random words
        for _ in 0..100 {
            let word = dict.select_random(DifficultyLevel::Easy);
            words_selected.insert(word);
        }

        // Should have variety (at least 50 unique words from 100 selections)
        assert!(
            words_selected.len() >= 50,
            "Word randomization should produce variety. Got {} unique words from 100 selections",
            words_selected.len()
        );
    }

    #[test]
    fn test_word_selection_by_difficulty() {
        let dict = Dictionary::new();

        // Easy words should be mostly 3-5 chars (with some variation allowed)
        let mut valid_count = 0;
        for _ in 0..20 {
            let word = dict.select_random(DifficultyLevel::Easy);
            if word.len() >= 3 && word.len() <= 5 {
                valid_count += 1;
            }
        }
        assert!(valid_count >= 15, "Most easy words should be 3-5 chars");

        // Medium words should be mostly 6-8 chars
        valid_count = 0;
        for _ in 0..20 {
            let word = dict.select_random(DifficultyLevel::Medium);
            if word.len() >= 6 && word.len() <= 8 {
                valid_count += 1;
            }
        }
        assert!(valid_count >= 15, "Most medium words should be 6-8 chars");

        // Hard words should be mostly 9-12 chars
        valid_count = 0;
        for _ in 0..20 {
            let word = dict.select_random(DifficultyLevel::Hard);
            if word.len() >= 9 && word.len() <= 12 {
                valid_count += 1;
            }
        }
        assert!(valid_count >= 15, "Most hard words should be 9-12 chars");

        // Expert words should have many longer words
        valid_count = 0;
        for _ in 0..30 {
            let word = dict.select_random(DifficultyLevel::Expert);
            if word.len() >= 9 {
                valid_count += 1;
            }
        }
        assert!(valid_count >= 15, "Most expert words should be longer");
    }

    #[test]
    fn test_word_no_empty_strings() {
        let dict = Dictionary::new();

        for difficulty in &[
            DifficultyLevel::Easy,
            DifficultyLevel::Medium,
            DifficultyLevel::Hard,
            DifficultyLevel::Expert,
        ] {
            for _ in 0..50 {
                let word = dict.select_random(*difficulty);
                assert!(!word.is_empty(), "Word should not be empty");
            }
        }
    }

    // ==================== WPM CALCULATION TESTS ====================
    #[test]
    fn test_wpm_calculation_basic() {
        // 50 characters in 1 minute = (50 / 5) / 1 = 10 WPM
        let wpm = calculate_wpm(50, 60.0);
        assert_eq!(wpm, 10.0, "WPM calculation should be correct");
    }

    #[test]
    fn test_wpm_calculation_half_minute() {
        // 50 characters in 30 seconds = (50 / 5) / 0.5 = 20 WPM
        let wpm = calculate_wpm(50, 30.0);
        assert_eq!(wpm, 20.0, "WPM at 30 seconds should be 20");
    }

    #[test]
    fn test_wpm_calculation_five_minutes() {
        // 300 characters in 5 minutes = (300 / 5) / 5 = 12 WPM
        let wpm = calculate_wpm(300, 300.0);
        assert_eq!(wpm, 12.0, "WPM at 5 minutes should be 12");
    }

    #[test]
    fn test_wpm_zero_time() {
        // Should handle edge case gracefully
        let wpm = calculate_wpm(50, 0.0);
        assert_eq!(wpm, 0.0, "WPM with 0 time should be 0");
    }

    #[test]
    fn test_wpm_very_small_time() {
        // 10 characters in 1 second = (10 / 5) / (1/60) = 120 WPM
        let wpm = calculate_wpm(10, 1.0);
        assert_eq!(wpm, 120.0, "Fast typing should show high WPM");
    }

    // ==================== ACCURACY TESTS ====================
    #[test]
    fn test_accuracy_perfect() {
        let accuracy = calculate_accuracy(10, 0);
        assert_eq!(accuracy, 100.0, "Perfect accuracy should be 100%");
    }

    #[test]
    fn test_accuracy_50_percent() {
        let accuracy = calculate_accuracy(10, 10);
        assert_eq!(
            accuracy, 50.0,
            "50% correct/incorrect should give 50% accuracy"
        );
    }

    #[test]
    fn test_accuracy_zero_words() {
        let accuracy = calculate_accuracy(0, 0);
        assert_eq!(accuracy, 100.0, "No words should default to 100%");
    }

    #[test]
    fn test_accuracy_all_wrong() {
        let accuracy = calculate_accuracy(0, 20);
        assert_eq!(accuracy, 0.0, "All wrong should be 0%");
    }

    #[test]
    fn test_accuracy_calculation_formula() {
        // (correct / (correct + incorrect)) * 100
        let accuracy = calculate_accuracy(75, 25);
        assert_eq!(accuracy, 75.0, "Formula: (75 / (75+25)) * 100 = 75%");
    }

    // ==================== WORD MATCHING TESTS ====================
    #[test]
    fn test_word_matching_exact_prefix() {
        let word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        let input = "HE";
        assert!(
            word.matches_prefix(input),
            "Input 'HE' should match word 'HELLO'"
        );
    }

    #[test]
    fn test_word_matching_case_insensitive() {
        let word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        let input = "he";
        assert!(
            word.matches_prefix(input),
            "Input 'he' should match word 'HELLO' (case insensitive)"
        );
    }

    #[test]
    fn test_word_matching_no_match() {
        let word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        let input = "WORLD";
        assert!(
            !word.matches_prefix(input),
            "Input 'WORLD' should not match word 'HELLO'"
        );
    }

    #[test]
    fn test_word_matching_complete() {
        let word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        let input = "HELLO";
        assert!(
            word.matches_prefix(input),
            "Complete input should match word"
        );
    }

    #[test]
    fn test_word_matching_longer_than_word() {
        let word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        let input = "HELLOWORLD";
        assert!(
            !word.matches_prefix(input),
            "Input longer than word should not match"
        );
    }

    #[test]
    fn test_word_exact_match() {
        let word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        assert!(
            word.is_exact_match("HELLO"),
            "Should match exact word"
        );
    }

    #[test]
    fn test_word_exact_match_case_insensitive() {
        let word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        assert!(
            word.is_exact_match("hello"),
            "Should match exact word (case insensitive)"
        );
    }

    // ==================== SCORE CALCULATION TESTS ====================
    #[test]
    fn test_score_easy_word() {
        let score = calculate_word_score(DifficultyLevel::Easy, 5, 1.0, 1.0);
        // Easy: 10 points
        assert_eq!(score, 50, "Easy word (5 chars) should score 50");
    }

    #[test]
    fn test_score_medium_word() {
        let score = calculate_word_score(DifficultyLevel::Medium, 7, 1.0, 1.0);
        // Medium: 25 points
        assert_eq!(score, 175, "Medium word (7 chars) should score 175");
    }

    #[test]
    fn test_score_hard_word() {
        let score = calculate_word_score(DifficultyLevel::Hard, 10, 1.0, 1.0);
        // Hard: 50 points
        assert_eq!(score, 500, "Hard word (10 chars) should score 500");
    }

    #[test]
    fn test_score_with_accuracy_multiplier() {
        let score = calculate_word_score(DifficultyLevel::Easy, 5, 1.5, 1.0);
        // Easy: 10 points, with 1.5x accuracy multiplier
        assert_eq!(score, 75, "Accuracy multiplier should increase score");
    }

    #[test]
    fn test_score_with_combo_multiplier() {
        let score = calculate_word_score(DifficultyLevel::Easy, 5, 1.0, 1.5);
        // Easy: 10 points, with 1.5x combo multiplier
        assert_eq!(score, 75, "Combo multiplier should increase score");
    }

    #[test]
    fn test_score_with_both_multipliers() {
        let score = calculate_word_score(DifficultyLevel::Medium, 7, 1.2, 1.3);
        // Medium: 25 * 7 * 1.2 * 1.3 = 273
        assert_eq!(score, 273, "Both multipliers should apply");
    }

    #[test]
    fn test_combo_bonus_calculation() {
        let bonus_5 = combo_bonus(5);
        assert_eq!(bonus_5, 0, "5-word combo gives no milestone bonus");

        let bonus_10 = combo_bonus(10);
        assert_eq!(bonus_10, 50, "10-word combo gives 50 bonus");

        let bonus_20 = combo_bonus(20);
        assert_eq!(bonus_20, 100, "20-word combo gives 100 bonus");
    }

    // ==================== COLLISION DETECTION TESTS ====================
    #[test]
    fn test_collision_at_bottom() {
        let mut word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        word.y = 38.0;
        assert!(word.collided_with_bottom(40), "Word at y=38 should collide");
    }

    #[test]
    fn test_no_collision_above_bottom() {
        let word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        assert!(
            !word.collided_with_bottom(40),
            "Word at y=10 should not collide"
        );
    }

    #[test]
    fn test_collision_exactly_at_bottom() {
        let mut word = Word::new("HELLO".to_string(), 10, 10.0, DifficultyLevel::Easy);
        word.y = 39.0;
        assert!(word.collided_with_bottom(40), "Word at y=39 should collide");
    }

    // ==================== DIFFICULTY SCALING TESTS ====================
    #[test]
    fn test_difficulty_speed_progression() {
        let mut game = Game::new();
        game.start_new_game(DifficultyLevel::Easy);

        let speed_easy = game.current_speed(0.0);
        game.difficulty_progression.update(15.0); // Update difficulty
        let speed_after_10s = game.current_speed(0.0);

        assert!(
            speed_after_10s >= speed_easy,
            "Speed should be at least as high after difficulty update"
        );
    }

    #[test]
    fn test_difficulty_word_count_effect() {
        let game = Game::new();

        let speed_few = game.speed_with_word_count(5, 1.0);
        let speed_many = game.speed_with_word_count(20, 1.0);

        assert!(
            speed_many < speed_few,
            "Speed should be capped when many words on screen"
        );
    }

    #[test]
    fn test_difficulty_combo_increase() {
        let game = Game::new();

        let base = 1.0;
        let after_10_combo = game.combo_multiplier(10);
        let after_20_combo = game.combo_multiplier(20);

        assert!(
            after_10_combo >= base,
            "Combo multiplier should increase"
        );
        assert!(
            after_20_combo >= after_10_combo,
            "Higher combo should have higher multiplier"
        );
    }

    // ==================== INPUT BUFFER TESTS ====================
    #[test]
    fn test_input_buffer_add_character() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('A');
        assert_eq!(buffer.get_text(), "A", "Buffer should contain added character");
    }

    #[test]
    fn test_input_buffer_multiple_characters() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('H');
        buffer.add_char('E');
        buffer.add_char('L');
        buffer.add_char('L');
        buffer.add_char('O');
        assert_eq!(buffer.get_text(), "HELLO", "Buffer should contain all characters");
    }

    #[test]
    fn test_input_buffer_backspace() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('H');
        buffer.add_char('I');
        buffer.backspace();
        assert_eq!(buffer.get_text(), "H", "Backspace should remove last character");
    }

    #[test]
    fn test_input_buffer_backspace_empty() {
        let mut buffer = InputBuffer::new();
        buffer.backspace();
        assert_eq!(
            buffer.get_text(),
            "",
            "Backspace on empty buffer should remain empty"
        );
    }

    #[test]
    fn test_input_buffer_clear() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('H');
        buffer.add_char('E');
        buffer.add_char('L');
        buffer.clear();
        assert_eq!(buffer.get_text(), "", "Clear should empty buffer");
    }

    #[test]
    fn test_input_buffer_case_conversion() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('h');
        buffer.add_char('e');
        buffer.add_char('l');
        buffer.add_char('l');
        buffer.add_char('o');
        // Should be stored as uppercase internally for matching
        assert_eq!(
            buffer.get_text(),
            "HELLO",
            "Buffer should convert to uppercase"
        );
    }

    // ==================== WORD STRUCTURE TESTS ====================
    #[test]
    fn test_word_creation() {
        let word = Word::new("HELLO".to_string(), 10, 5.0, DifficultyLevel::Easy);
        assert_eq!(word.text, "HELLO");
        assert_eq!(word.x, 10);
        assert_eq!(word.y, 0.0); // Words start at y=0
    }

    #[test]
    fn test_word_falling_update() {
        let mut word = Word::new("HELLO".to_string(), 10, 5.0, DifficultyLevel::Easy);
        let initial_y = word.y;
        word.update(1.0); // Update with 1 frame at default speed
        assert!(word.y > initial_y, "Word should fall downward");
    }

    #[test]
    fn test_word_color_coding() {
        let easy = Word::new("HI".to_string(), 10, 5.0, DifficultyLevel::Easy);
        let medium = Word::new("MEDIUM".to_string(), 10, 5.0, DifficultyLevel::Medium);
        let hard = Word::new("DIFFICULT".to_string(), 10, 5.0, DifficultyLevel::Hard);
        let expert = Word::new("EXTRAORDINARY".to_string(), 10, 5.0, DifficultyLevel::Expert);

        // Colors should differ based on difficulty
        assert_ne!(easy.get_color(), medium.get_color());
        assert_ne!(medium.get_color(), hard.get_color());
        assert_ne!(hard.get_color(), expert.get_color());
    }

    // ==================== GAME STATE TESTS ====================
    #[test]
    fn test_game_initialization() {
        let game = Game::new();
        assert_eq!(game.lives, 3, "Game should start with 3 lives");
        assert_eq!(game.score, 0, "Game should start with 0 score");
        assert_eq!(game.words.len(), 0, "Game should start with no words");
        assert_eq!(game.elapsed_time, 0.0, "Game should start at time 0");
    }

    #[test]
    fn test_game_life_loss() {
        let mut game = Game::new();
        let initial_lives = game.lives;
        game.lose_life();
        assert_eq!(game.lives, initial_lives - 1, "Losing life should decrease count");
    }

    #[test]
    fn test_game_game_over_at_zero_lives() {
        let mut game = Game::new();
        game.start_new_game(DifficultyLevel::Easy);
        game.lives = 0;
        game.update(0.1); // Trigger the update that checks lives
        assert!(game.is_game_over(), "Game should be over at 0 lives");
    }

    #[test]
    fn test_game_add_word() {
        let mut game = Game::new();
        game.add_word("HELLO".to_string(), DifficultyLevel::Easy);
        assert_eq!(game.words.len(), 1, "Game should have 1 word");
    }

    #[test]
    fn test_game_word_limit() {
        let mut game = Game::new();
        // Add more than max words (20)
        for i in 0..30 {
            game.add_word(format!("WORD{}", i), DifficultyLevel::Easy);
        }
        assert!(
            game.words.len() <= 20,
            "Game should not exceed maximum word count"
        );
    }

    // ==================== DIFFICULTY LEVEL TESTS ====================
    #[test]
    fn test_difficulty_level_ordering() {
        let easy = DifficultyLevel::Easy.to_speed_multiplier();
        let medium = DifficultyLevel::Medium.to_speed_multiplier();
        let hard = DifficultyLevel::Hard.to_speed_multiplier();
        let expert = DifficultyLevel::Expert.to_speed_multiplier();

        assert!(easy < medium, "Easy should be slower than Medium");
        assert!(medium < hard, "Medium should be slower than Hard");
        assert!(hard < expert, "Hard should be slower than Expert");
    }

    #[test]
    fn test_difficulty_level_to_points() {
        let easy_points = DifficultyLevel::Easy.base_points();
        let medium_points = DifficultyLevel::Medium.base_points();
        let hard_points = DifficultyLevel::Hard.base_points();
        let expert_points = DifficultyLevel::Expert.base_points();

        assert_eq!(easy_points, 10);
        assert_eq!(medium_points, 25);
        assert_eq!(hard_points, 50);
        assert_eq!(expert_points, 100);
    }

    // ==================== DICTIONARY LOADING TESTS ====================
    #[test]
    fn test_dictionary_initialization() {
        let dict = Dictionary::new();
        assert!(
            dict.count_words(DifficultyLevel::Easy) > 0,
            "Dictionary should have Easy words"
        );
        assert!(
            dict.count_words(DifficultyLevel::Medium) > 0,
            "Dictionary should have Medium words"
        );
        assert!(
            dict.count_words(DifficultyLevel::Hard) > 0,
            "Dictionary should have Hard words"
        );
        assert!(
            dict.count_words(DifficultyLevel::Expert) > 0,
            "Dictionary should have Expert words"
        );
    }

    #[test]
    fn test_dictionary_total_words() {
        let dict = Dictionary::new();
        let total = dict.count_words(DifficultyLevel::Easy)
            + dict.count_words(DifficultyLevel::Medium)
            + dict.count_words(DifficultyLevel::Hard)
            + dict.count_words(DifficultyLevel::Expert);

        assert!(
            total >= 1000,
            "Dictionary should have at least 1000 words, got {}",
            total
        );
    }

    // ==================== HELPER FUNCTIONS ====================

    fn calculate_wpm(characters_typed: usize, elapsed_seconds: f64) -> f64 {
        if elapsed_seconds == 0.0 {
            return 0.0;
        }
        let words = characters_typed as f64 / 5.0;
        let minutes = elapsed_seconds / 60.0;
        if minutes == 0.0 {
            0.0
        } else {
            words / minutes
        }
    }

    fn calculate_accuracy(correct: usize, incorrect: usize) -> f64 {
        if correct + incorrect == 0 {
            100.0
        } else {
            (correct as f64 / (correct + incorrect) as f64) * 100.0
        }
    }

    fn calculate_word_score(
        difficulty: DifficultyLevel,
        word_length: usize,
        accuracy_multiplier: f64,
        combo_multiplier: f64,
    ) -> usize {
        let base_points = difficulty.base_points();
        (base_points as f64 * word_length as f64 * accuracy_multiplier * combo_multiplier)
            as usize
    }

    fn combo_bonus(combo_count: usize) -> usize {
        match combo_count {
            10..=19 => 50,
            20..=49 => 100,
            50..=99 => 200,
            100..=199 => 500,
            200.. => 1000,
            _ => 0,
        }
    }
}
