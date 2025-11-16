//! Game module
//!
//! Manages game state, input handling, and game loop integration.

pub mod input;
pub mod session;
pub mod state;

pub use input::{CarInput, InputManager};
pub use session::{DriverResult, RaceFlag, RaceSession, RaceState};
pub use state::{GameMode, GameState};