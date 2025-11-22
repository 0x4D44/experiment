//! Audio module
//!
//! Provides audio playback for F1GP sound effects and engine sounds.
//!
//! Features:
//! - Real-time engine sound synthesis (RPM-based)
//! - Gear shift sounds
//! - Menu sound effects
//! - Volume control and muting

pub mod sound_engine;

pub use sound_engine::SoundEngine;
