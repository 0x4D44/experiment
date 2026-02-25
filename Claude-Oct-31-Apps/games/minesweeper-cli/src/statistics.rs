//! Game statistics tracking and reporting

use std::collections::HashMap;
use std::time::Duration;

/// Statistics for tracking game performance
#[derive(Clone, Debug)]
pub struct Statistics {
    games_played: usize,
    games_won: usize,
    games_lost: usize,
    total_time: Duration,
    best_times: HashMap<String, Duration>,
    mines_cleared: usize,
    current_streak: usize,
    longest_streak: usize,
}

impl Statistics {
    /// Create new statistics tracker
    pub fn new() -> Self {
        Statistics {
            games_played: 0,
            games_won: 0,
            games_lost: 0,
            total_time: Duration::ZERO,
            best_times: HashMap::new(),
            mines_cleared: 0,
            current_streak: 0,
            longest_streak: 0,
        }
    }

    /// Record a won game
    pub fn record_game_won(&mut self, difficulty: String, time_ms: u64) {
        self.games_played += 1;
        self.games_won += 1;
        self.current_streak += 1;
        if self.current_streak > self.longest_streak {
            self.longest_streak = self.current_streak;
        }

        let duration = Duration::from_millis(time_ms);
        self.total_time += duration;

        // Update best time
        self.best_times
            .entry(difficulty)
            .and_modify(|best| {
                if duration < *best {
                    *best = duration;
                }
            })
            .or_insert(duration);
    }

    /// Record a lost game
    pub fn record_game_lost(&mut self, time_ms: u64) {
        self.games_played += 1;
        self.games_lost += 1;
        self.current_streak = 0;

        let duration = Duration::from_millis(time_ms);
        self.total_time += duration;
    }

    /// Get total games played
    pub fn games_played(&self) -> usize {
        self.games_played
    }

    /// Get games won
    pub fn games_won(&self) -> usize {
        self.games_won
    }

    /// Get games lost
    pub fn games_lost(&self) -> usize {
        self.games_lost
    }

    /// Get win percentage (0-100)
    pub fn win_percentage(&self) -> f32 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.games_won as f32 / self.games_played as f32) * 100.0
        }
    }

    /// Get average game time
    pub fn average_time(&self) -> Duration {
        if self.games_played == 0 {
            Duration::ZERO
        } else {
            self.total_time / self.games_played as u32
        }
    }

    /// Get best time for a difficulty
    pub fn best_time(&self, difficulty: &str) -> Option<Duration> {
        self.best_times.get(difficulty).copied()
    }

    /// Get all best times
    pub fn best_times(&self) -> &HashMap<String, Duration> {
        &self.best_times
    }

    /// Get total time played
    pub fn total_time(&self) -> Duration {
        self.total_time
    }

    /// Get mines cleared
    pub fn mines_cleared(&self) -> usize {
        self.mines_cleared
    }

    /// Add mines cleared
    pub fn add_mines_cleared(&mut self, count: usize) {
        self.mines_cleared += count;
    }

    /// Get current win streak
    pub fn current_streak(&self) -> usize {
        self.current_streak
    }

    /// Get longest win streak
    pub fn longest_streak(&self) -> usize {
        self.longest_streak
    }

    /// Display statistics as formatted string
    pub fn display(&self) -> String {
        let mut output = String::new();
        output.push_str("=== MINESWEEPER STATISTICS ===\n");
        output.push_str(&format!("Games Played: {}\n", self.games_played));
        output.push_str(&format!("Games Won: {}\n", self.games_won));
        output.push_str(&format!("Games Lost: {}\n", self.games_lost));
        output.push_str(&format!("Win Rate: {:.1}%\n", self.win_percentage()));
        output.push_str(&format!("Total Time: {:.0}s\n", self.total_time.as_secs()));
        output.push_str(&format!(
            "Average Time: {:.1}s\n",
            self.average_time().as_secs_f32()
        ));
        output.push_str(&format!("Current Streak: {}\n", self.current_streak));
        output.push_str(&format!("Longest Streak: {}\n", self.longest_streak));
        output.push_str(&format!("Mines Cleared: {}\n", self.mines_cleared));

        if !self.best_times.is_empty() {
            output.push_str("\nBest Times:\n");
            for (difficulty, time) in &self.best_times {
                output.push_str(&format!("  {}: {:.1}s\n", difficulty, time.as_secs_f32()));
            }
        }

        output
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_creation() {
        let stats = Statistics::new();
        assert_eq!(stats.games_played(), 0);
        assert_eq!(stats.games_won(), 0);
        assert_eq!(stats.games_lost(), 0);
    }

    #[test]
    fn test_record_win() {
        let mut stats = Statistics::new();
        stats.record_game_won("Beginner".to_string(), 45000);

        assert_eq!(stats.games_played(), 1);
        assert_eq!(stats.games_won(), 1);
        assert_eq!(stats.games_lost(), 0);
        assert_eq!(stats.current_streak(), 1);
    }

    #[test]
    fn test_record_loss() {
        let mut stats = Statistics::new();
        stats.record_game_lost(30000);

        assert_eq!(stats.games_played(), 1);
        assert_eq!(stats.games_won(), 0);
        assert_eq!(stats.games_lost(), 1);
        assert_eq!(stats.current_streak(), 0);
    }

    #[test]
    fn test_win_percentage() {
        let mut stats = Statistics::new();
        stats.record_game_won("Beginner".to_string(), 45000);
        stats.record_game_won("Beginner".to_string(), 50000);
        stats.record_game_lost(30000);

        let expected = 200.0 / 3.0;
        assert!((stats.win_percentage() - expected).abs() < 0.01);
    }

    #[test]
    fn test_average_time() {
        let mut stats = Statistics::new();
        stats.record_game_won("Beginner".to_string(), 40000);
        stats.record_game_won("Beginner".to_string(), 60000);

        let avg = stats.average_time();
        assert_eq!(avg, Duration::from_millis(50000));
    }

    #[test]
    fn test_best_time_tracking() {
        let mut stats = Statistics::new();
        stats.record_game_won("Beginner".to_string(), 60000);
        stats.record_game_won("Beginner".to_string(), 40000);
        stats.record_game_won("Beginner".to_string(), 50000);

        let best = stats.best_time("Beginner").unwrap();
        assert_eq!(best, Duration::from_millis(40000));
    }

    #[test]
    fn test_streak_tracking() {
        let mut stats = Statistics::new();
        stats.record_game_won("Beginner".to_string(), 45000);
        stats.record_game_won("Beginner".to_string(), 50000);
        stats.record_game_won("Beginner".to_string(), 48000);
        stats.record_game_lost(30000);
        stats.record_game_won("Beginner".to_string(), 52000);

        assert_eq!(stats.current_streak(), 1);
        assert_eq!(stats.longest_streak(), 3);
    }

    #[test]
    fn test_display() {
        let mut stats = Statistics::new();
        stats.record_game_won("Beginner".to_string(), 45000);
        stats.record_game_lost(30000);

        let display = stats.display();
        assert!(display.contains("Games Played: 2"));
        assert!(display.contains("Games Won: 1"));
        assert!(display.contains("Games Lost: 1"));
    }
}
