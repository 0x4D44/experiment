use serde::{Deserialize, Serialize};
use std::fs;

/// Score tracking system
#[derive(Debug, Clone)]
pub struct ScoreSystem {
    pub score: u32,
    pub combo: u32,
    pub combo_timer: f32,
    pub combo_timeout: f32,
    pub multiplier: f32,
    pub multiplier_timer: f32,
    pub high_score: u32,
}

impl ScoreSystem {
    pub fn new() -> Self {
        let high_score = Self::load_high_score();
        Self {
            score: 0,
            combo: 0,
            combo_timer: 0.0,
            combo_timeout: 3.0,
            multiplier: 1.0,
            multiplier_timer: 0.0,
            high_score,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.combo > 0 {
            self.combo_timer += dt;
            if self.combo_timer >= self.combo_timeout {
                self.combo = 0;
                self.combo_timer = 0.0;
            }
        }

        if self.multiplier_timer > 0.0 {
            self.multiplier_timer -= dt;
            if self.multiplier_timer <= 0.0 {
                self.multiplier = 1.0;
                self.multiplier_timer = 0.0;
            }
        }
    }

    pub fn add_kill(&mut self, base_score: u32) {
        self.combo += 1;
        self.combo_timer = 0.0;

        let combo_multiplier = 1.0 + (self.combo as f32 * 0.1).min(3.0);
        let final_score = (base_score as f32 * combo_multiplier * self.multiplier) as u32;

        self.score += final_score;

        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += (points as f32 * self.multiplier) as u32;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn set_multiplier(&mut self, multiplier: f32, duration: f32) {
        self.multiplier = multiplier;
        self.multiplier_timer = duration;
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.combo = 0;
        self.combo_timer = 0.0;
        self.multiplier = 1.0;
        self.multiplier_timer = 0.0;
    }

    pub fn save_high_score(&self) {
        let data = HighScoreData {
            high_score: self.high_score,
        };

        if let Ok(json) = serde_json::to_string(&data) {
            let _ = fs::write("highscore.json", json);
        }
    }

    fn load_high_score() -> u32 {
        if let Ok(contents) = fs::read_to_string("highscore.json") {
            if let Ok(data) = serde_json::from_str::<HighScoreData>(&contents) {
                return data.high_score;
            }
        }
        0
    }
}

impl Default for ScoreSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
struct HighScoreData {
    high_score: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_addition() {
        let mut score = ScoreSystem::new();
        score.add_kill(100);
        // First kill has combo multiplier of 1.1x (1 + 0.1)
        assert_eq!(score.score, 110);
        assert_eq!(score.combo, 1);
    }

    #[test]
    fn test_combo_multiplier() {
        let mut score = ScoreSystem::new();
        score.add_kill(100); // combo 1: 1.1x = 110
        assert_eq!(score.score, 110);

        score.add_kill(100); // combo 2: 1.2x = 120
        assert_eq!(score.score, 230); // 110 + 120
    }

    #[test]
    fn test_combo_timeout() {
        let mut score = ScoreSystem::new();
        score.add_kill(100);
        assert_eq!(score.combo, 1);

        score.update(3.5);
        assert_eq!(score.combo, 0);
    }
}
