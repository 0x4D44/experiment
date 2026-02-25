// Terminal rendering using Crossterm

use crossterm::{
    style::Color,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

use crate::config::*;
use crate::scoring::Stats;
use crate::word::Word;

pub struct Renderer {
    buffer: String,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(8192),
        }
    }

    pub fn init() -> io::Result<()> {
        enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    pub fn cleanup() -> io::Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Render a frame with all game elements
    pub fn render_frame(
        &mut self,
        words: &[Word],
        input: &str,
        stats: &Stats,
        lives: usize,
        difficulty: &str,
    ) -> io::Result<()> {
        self.clear();

        // Top border and header
        self.buffer.push_str(&format!(
            "╔{}╗\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));
        self.buffer.push_str(&format!(
            "║ TYPING SPEED RACER {:40}Lives: {} {} ║\n",
            "", self.render_lives(lives), difficulty
        ));
        self.buffer.push_str(&format!(
            "╠{}╣\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));

        // Play area with words
        for row in 0..PLAY_AREA_HEIGHT {
            self.buffer.push('║');
            let row_str = self.render_play_area_row(words, row as u16);
            self.buffer.push_str(&row_str);
            // Pad to width
            let padding = PLAY_AREA_WIDTH as usize - row_str.len();
            self.buffer.push_str(&" ".repeat(padding));
            self.buffer.push('║');
            self.buffer.push('\n');
        }

        // Separator
        self.buffer.push_str(&format!(
            "╠{}╣\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));

        // Input area
        self.buffer.push_str(&format!(
            "║ Input: [{}{}] │ WPM: {} │ Acc: {} │ Score: {} │ Combo: {} ║\n",
            input,
            " ".repeat((PLAY_AREA_WIDTH as usize).saturating_sub(input.len() + 15)),
            stats.format_wpm(),
            stats.format_accuracy(),
            stats.total_score,
            stats.combo_count
        ));

        // Bottom border
        self.buffer.push_str(&format!(
            "╚{}╝",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));

        // Write to stdout
        print!("{}", self.buffer);
        io::stdout().flush()?;
        Ok(())
    }

    /// Render a single row of the play area
    fn render_play_area_row(&self, words: &[Word], row: u16) -> String {
        let mut row_str = String::new();

        // Find all words at this row
        for word in words {
            let word_row = word.y as u16;
            if word_row == row {
                // Pad to x position
                let padding = word.x as usize;
                row_str.push_str(&" ".repeat(padding));
                row_str.push_str(&word.text);
            }
        }

        // Limit to play area width
        if row_str.len() > PLAY_AREA_WIDTH as usize {
            row_str.truncate(PLAY_AREA_WIDTH as usize);
        }

        row_str
    }

    /// Render lives indicator
    fn render_lives(&self, lives: usize) -> String {
        let heart = "❤";
        let empty = "🤍";
        let max_lives = 3;
        let filled = (0..lives).map(|_| heart).collect::<String>();
        let empty_hearts = (0..(max_lives - lives))
            .map(|_| empty)
            .collect::<String>();
        format!("{}{}", filled, empty_hearts)
    }

    /// Render game over screen
    pub fn render_game_over(&mut self, stats: &Stats) -> io::Result<()> {
        self.clear();

        self.buffer.push_str(&format!(
            "╔{}╗\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));
        self.buffer.push_str("║                          GAME OVER                             ║\n");
        self.buffer.push_str(&format!(
            "╠{}╣\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));

        self.buffer.push_str(&format!("║ Final WPM:         {:<50} ║\n", stats.format_wpm()));
        self.buffer
            .push_str(&format!("║ Accuracy:         {:<50} ║\n", stats.format_accuracy()));
        self.buffer
            .push_str(&format!("║ Total Score:      {:<50} ║\n", stats.total_score));
        self.buffer.push_str(&format!(
            "║ Words Typed:      {:<50} ║\n",
            stats.correct_words + stats.incorrect_words
        ));
        self.buffer.push_str(&format!(
            "║ Best Combo:       {:<50} ║\n",
            stats.best_combo
        ));
        self.buffer
            .push_str(&format!("║ Game Duration:    {:<50} ║\n", stats.format_time()));

        self.buffer.push_str(&format!(
            "╠{}╣\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));
        self.buffer.push_str("║ Press Q to quit or any other key to play again               ║\n");
        self.buffer.push_str(&format!(
            "╚{}╝",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));

        print!("{}", self.buffer);
        io::stdout().flush()?;
        Ok(())
    }

    /// Render start screen
    pub fn render_start_screen(&mut self) -> io::Result<()> {
        self.clear();

        self.buffer.push_str(&format!(
            "╔{}╗\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));
        self.buffer.push_str("║                    TYPING SPEED RACER                         ║\n");
        self.buffer.push_str("║                  Type words to destroy them!                   ║\n");
        self.buffer.push_str(&format!(
            "╠{}╣\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));

        self.buffer.push_str("║ GAME INSTRUCTIONS:                                            ║\n");
        self.buffer.push_str("║                                                               ║\n");
        self.buffer.push_str("║ 1. Words fall from the top of the screen                      ║\n");
        self.buffer.push_str("║ 2. Type the words to destroy them before they hit the bottom  ║\n");
        self.buffer.push_str("║ 3. Lose a life when a word reaches the bottom                 ║\n");
        self.buffer.push_str("║ 4. Game ends when you run out of lives                        ║\n");
        self.buffer.push_str("║ 5. Build combos to earn bonus points                          ║\n");
        self.buffer.push_str("║                                                               ║\n");
        self.buffer.push_str("║ DIFFICULTY LEVELS:                                            ║\n");
        self.buffer.push_str("║ - Easy:   Slower words, shorter words (3-5 chars)             ║\n");
        self.buffer.push_str("║ - Medium: Medium speed, medium words (6-8 chars)              ║\n");
        self.buffer.push_str("║ - Hard:   Fast words, longer words (9-12 chars)               ║\n");
        self.buffer.push_str("║ - Expert: Very fast, expert words (13-15 chars)               ║\n");
        self.buffer.push_str("║                                                               ║\n");

        self.buffer.push_str(&format!(
            "╠{}╣\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));
        self.buffer.push_str("║ Press any key to start...                                     ║\n");
        self.buffer.push_str(&format!(
            "╚{}╝",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));

        print!("{}", self.buffer);
        io::stdout().flush()?;
        Ok(())
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Color helper for words
pub fn get_word_color(word: &Word) -> Color {
    word.get_color()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new();
        assert!(renderer.buffer.is_empty());
    }

    #[test]
    fn test_renderer_clear() {
        let mut renderer = Renderer::new();
        renderer.buffer.push_str("test");
        assert!(!renderer.buffer.is_empty());
        renderer.clear();
        assert!(renderer.buffer.is_empty());
    }

    #[test]
    fn test_lives_rendering() {
        let renderer = Renderer::new();
        let lives_str = renderer.render_lives(3);
        assert!(!lives_str.is_empty());
    }
}
