// Game configuration constants

pub const GAME_WIDTH: u16 = 120;
pub const GAME_HEIGHT: u16 = 30;
pub const PLAY_AREA_WIDTH: u16 = 115;
pub const PLAY_AREA_HEIGHT: u16 = 22;

pub const DEFAULT_LIVES: usize = 3;
pub const MAX_WORDS_ON_SCREEN: usize = 20;

// Speed constants (pixels per frame at 60 FPS)
pub const BASE_SPEED_EASY: f32 = 0.3;
pub const BASE_SPEED_MEDIUM: f32 = 0.5;
pub const BASE_SPEED_HARD: f32 = 0.8;
pub const BASE_SPEED_EXPERT: f32 = 1.2;

// Spawn rate (seconds between word spawns)
pub const SPAWN_RATE_EASY: f32 = 2.5;
pub const SPAWN_RATE_MEDIUM: f32 = 2.0;
pub const SPAWN_RATE_HARD: f32 = 1.5;
pub const SPAWN_RATE_EXPERT: f32 = 1.0;

// Difficulty progression
pub const DIFFICULTY_INCREASE_INTERVAL: f32 = 10.0; // seconds
pub const DIFFICULTY_INCREASE_RATE: f32 = 0.05; // 5% per interval
pub const MAX_DIFFICULTY: f32 = 3.0;

// Scoring constants
pub const POINTS_EASY: usize = 10;
pub const POINTS_MEDIUM: usize = 25;
pub const POINTS_HARD: usize = 50;
pub const POINTS_EXPERT: usize = 100;

pub const ACCURACY_MULTIPLIER_PERFECT: f64 = 1.5; // 100% accuracy
pub const ACCURACY_MULTIPLIER_HIGH: f64 = 1.2; // 90-99% accuracy
pub const ACCURACY_MULTIPLIER_NORMAL: f64 = 1.0; // 80-89% accuracy
pub const ACCURACY_MULTIPLIER_LOW: f64 = 0.8; // <80% accuracy

// Combo bonuses (points)
pub const COMBO_BONUS_5: usize = 0;
pub const COMBO_BONUS_10: usize = 50;
pub const COMBO_BONUS_20: usize = 100;

// Frame rate
pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME_MS: u64 = 1000 / TARGET_FPS as u64;

// Input buffer size
pub const MAX_INPUT_LENGTH: usize = 100;

// Word length ranges
pub const WORD_LENGTH_EASY_MIN: usize = 3;
pub const WORD_LENGTH_EASY_MAX: usize = 5;
pub const WORD_LENGTH_MEDIUM_MIN: usize = 6;
pub const WORD_LENGTH_MEDIUM_MAX: usize = 8;
pub const WORD_LENGTH_HARD_MIN: usize = 9;
pub const WORD_LENGTH_HARD_MAX: usize = 12;
pub const WORD_LENGTH_EXPERT_MIN: usize = 13;
pub const WORD_LENGTH_EXPERT_MAX: usize = 15;
