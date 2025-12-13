//! Game module
//!
//! Manages game state, input handling, and game loop integration.

pub mod championship;
pub mod input;
pub mod session;
pub mod state;
pub mod weather;

pub use championship::{Championship, DriverStanding, RaceResult, create_1991_season};
pub use input::{CarInput, InputManager};
pub use session::{DriverResult, RaceFlag, RaceSession, RaceState};
pub use state::{GameMode, GameState};
pub use weather::{WeatherCondition, WeatherSystem};
