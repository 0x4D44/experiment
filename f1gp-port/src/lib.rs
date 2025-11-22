//! F1GP Modern Port
//!
//! A modern reimplementation of the classic Formula 1 Grand Prix racing simulator
//! by Geoff Crammond (MicroProse, 1991).
//!
//! This project aims to port the game to modern platforms using Rust, while
//! maintaining the authentic feel and behavior of the original game.

// Module declarations
pub mod ai;
pub mod audio;
pub mod data;
pub mod game;
pub mod physics;
pub mod platform;
pub mod render;
pub mod render3d; // Phase 6: 3D rendering
pub mod telemetry;
pub mod ui;
pub mod utils;

// Re-exports for convenience
pub use data::*;
pub use telemetry::*;
// pub use physics::*;  // Commented out until physics module is implemented
