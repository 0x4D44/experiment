/// High score persistence
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct HighScores {
    pub scores: Vec<u32>,
}

impl HighScores {
    pub fn new() -> Self {
        HighScores {
            scores: Vec::new(),
        }
    }

    pub fn add_score(&mut self, score: u32) {
        self.scores.push(score);
        self.scores.sort_by(|a, b| b.cmp(a));
        self.scores.truncate(10); // Keep top 10
    }

    pub fn get_high_score(&self) -> u32 {
        self.scores.first().copied().unwrap_or(0)
    }

    pub fn load() -> Self {
        let path = Self::get_save_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(scores) = serde_json::from_str(&contents) {
                return scores;
            }
        }
        Self::new()
    }

    pub fn save(&self) {
        let path = Self::get_save_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    fn get_save_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("tetris-rust");
        fs::create_dir_all(&path).ok();
        path.push("highscores.json");
        path
    }
}

// Fallback implementation if dirs crate is not available
mod dirs {
    use std::path::PathBuf;

    pub fn config_dir() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            std::env::var("APPDATA")
                .ok()
                .map(PathBuf::from)
        }

        #[cfg(not(target_os = "windows"))]
        {
            std::env::var("HOME")
                .ok()
                .map(|home| PathBuf::from(home).join(".config"))
        }
    }
}
