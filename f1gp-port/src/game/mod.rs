//! Game module
//!
//! Manages game state, input handling, and game loop integration.

pub mod input;
pub mod state;

pub use input::{CarInput, InputManager};
pub use state::{GameMode, GameState};