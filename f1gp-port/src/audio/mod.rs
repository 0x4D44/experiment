//! Audio module - Stage 6.6
//! Audio system using rodio for sound playback
//!
//! Note: Audio features require the "audio" feature to be enabled

#[cfg(feature = "audio")]
pub mod engine;
pub mod sound;

#[cfg(feature = "audio")]
pub use engine::AudioEngine;
pub use sound::{SoundType, SoundSource};