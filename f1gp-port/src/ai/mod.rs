//! AI module
//!
//! Provides AI drivers for opponent cars in races.

pub mod driver;
pub mod racing_line;

pub use driver::{AIDriver, AIState, DriverPersonality};
pub use racing_line::RacingLineFollower;
