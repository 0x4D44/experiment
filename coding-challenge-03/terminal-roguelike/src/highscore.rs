//! High score tracking and persistence

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighScore {
    pub score: i32,
    pub level: i32,
    pub name: String,
}

impl HighScore {
    pub fn new(score: i32, level: i32) -> Self {
        Self {
            score,
            level,
            name: "Hero".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighScores {
    scores: Vec<HighScore>,
}

impl HighScores {
    pub fn new() -> Self {
        Self { scores: Vec::new() }
    }

    pub fn load() -> Self {
        if let Some(path) = Self::get_save_path() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(scores) = serde_json::from_str(&data) {
                    return scores;
                }
            }
        }
        Self::new()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::get_save_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let data = serde_json::to_string_pretty(self)?;
            fs::write(&path, data)?;
        }
        Ok(())
    }

    pub fn add_score(&mut self, score: HighScore) {
        self.scores.push(score);
        self.scores.sort_by(|a, b| b.score.cmp(&a.score));
        self.scores.truncate(10); // Keep top 10
    }

    pub fn get_scores(&self) -> &[HighScore] {
        &self.scores
    }

    pub fn is_high_score(&self, score: i32) -> bool {
        self.scores.len() < 10 || score > self.scores.last().map_or(0, |s| s.score)
    }

    fn get_save_path() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            dirs::data_local_dir().map(|d| d.join("terminal_roguelike").join("highscores.json"))
        }
        #[cfg(not(target_os = "windows"))]
        {
            dirs::home_dir().map(|d| d.join(".terminal_roguelike").join("highscores.json"))
        }
    }
}

impl Default for HighScores {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_score_creation() {
        let score = HighScore::new(1000, 5);
        assert_eq!(score.score, 1000);
        assert_eq!(score.level, 5);
        assert!(!score.name.is_empty());
    }

    #[test]
    fn test_high_scores_add() {
        let mut scores = HighScores::new();
        scores.add_score(HighScore::new(100, 1));
        scores.add_score(HighScore::new(200, 2));

        assert_eq!(scores.get_scores().len(), 2);
        assert_eq!(scores.get_scores()[0].score, 200);
        assert_eq!(scores.get_scores()[1].score, 100);
    }

    #[test]
    fn test_high_scores_limit() {
        let mut scores = HighScores::new();

        for i in 1..=15 {
            scores.add_score(HighScore::new(i * 100, i));
        }

        assert_eq!(scores.get_scores().len(), 10);
        assert_eq!(scores.get_scores()[0].score, 1500);
    }

    #[test]
    fn test_is_high_score() {
        let mut scores = HighScores::new();

        for i in 1..=10 {
            scores.add_score(HighScore::new(i * 100, i));
        }

        assert!(scores.is_high_score(1100));
        assert!(!scores.is_high_score(50));
    }

    #[test]
    fn test_serialization() {
        let mut scores = HighScores::new();
        scores.add_score(HighScore::new(100, 1));

        let json = serde_json::to_string(&scores).unwrap();
        let deserialized: HighScores = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.get_scores().len(), 1);
        assert_eq!(deserialized.get_scores()[0].score, 100);
    }
}

// Add dirs crate for cross-platform directories
mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
    }

    #[allow(dead_code)]
    pub fn data_local_dir() -> Option<PathBuf> {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|h| h.join(".local").join("share")))
    }
}
