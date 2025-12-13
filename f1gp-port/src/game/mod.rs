//! Game module
//!
//! Manages game state, input handling, and game loop integration.

pub mod championship;
pub mod damage;
pub mod input;
pub mod pitstop;
pub mod qualifying;
pub mod session;
pub mod state;
pub mod weather;
pub mod weekend;

pub use championship::{create_1991_season, Championship, DriverStanding, RaceResult};
pub use damage::{CarComponent, CollisionType, DamageLevel, DamageState, FailureType};
pub use input::{CarInput, InputManager};
pub use pitstop::{PitStopManager, PitStopRequest, RaceStrategy, TireCompound, TireSet};
pub use qualifying::{
    create_1991_qualifying, QualifyingResult, QualifyingSession, QualifyingState,
};
pub use session::{DriverResult, RaceFlag, RaceSession, RaceState};
pub use state::{GameMode, GameState};
pub use weather::{WeatherCondition, WeatherSystem};
pub use weekend::{create_weekend, RaceWeekend, WeekendEntry, WeekendSession, WeekendState};
