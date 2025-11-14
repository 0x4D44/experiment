//! Track file format structures and types
//!
//! This module contains data structures representing the F1GP track file format.
//! Based on reverse engineering and ArgDocs documentation.

use serde::{Deserialize, Serialize};
use glam::Vec3;
use super::objects::ObjectShape;

/// A complete F1GP track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    /// Track name (derived from filename, e.g., "Monaco")
    pub name: String,

    /// Track length in meters
    pub length: f32,

    /// Object shapes in the track
    pub object_shapes: Vec<ObjectShape>,

    /// Track sections (geometry)
    pub sections: Vec<TrackSection>,

    /// Racing line for AI
    pub racing_line: RacingLine,

    /// AI setup and behavior
    pub ai_behavior: AIBehavior,

    /// Pit lane sections
    pub pit_lane: Vec<TrackSection>,

    /// Camera definitions
    pub cameras: Vec<Camera>,

    /// File checksum (last 4 bytes)
    pub checksum: u32,
}

/// Track section representing a segment of the circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackSection {
    /// Position in 3D space
    pub position: Vec3,

    /// Track width at this section (meters)
    pub width: f32,

    /// Banking angle (radians, positive = banking right)
    pub banking: f32,

    /// Elevation change from previous section
    pub elevation: f32,

    /// Surface type (asphalt, grass, gravel, etc.)
    pub surface: SurfaceType,

    /// Length of this section (meters)
    pub length: f32,
}

/// Surface type for track sections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceType {
    /// Track surface (asphalt)
    Track,

    /// Grass runoff area
    Grass,

    /// Gravel trap
    Gravel,

    /// Kerb/curb
    Kerb,

    /// Pit lane surface
    PitLane,

    /// Wall or barrier
    Wall,
}

/// Racing line for AI drivers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacingLine {
    /// Points defining the optimal line
    pub points: Vec<RacingLinePoint>,
}

/// A point on the racing line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacingLinePoint {
    /// Position on track
    pub position: Vec3,

    /// Target speed at this point (m/s)
    pub speed: f32,

    /// Braking zone flag
    pub is_braking_zone: bool,
}

/// AI behavior and setup for this track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBehavior {
    /// Aggression level (0.0 - 1.0)
    pub aggression: f32,

    /// Consistency (0.0 - 1.0, higher = fewer mistakes)
    pub consistency: f32,

    /// Track-specific car setup
    pub car_setup: CarSetup,
}

/// Car setup parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarSetup {
    /// Front wing setting (1-20)
    pub front_wing: u8,

    /// Rear wing setting (1-20)
    pub rear_wing: u8,

    /// Gear ratios (6 gears)
    pub gear_ratios: [u8; 6],

    /// Brake balance (front/rear, 0-100)
    pub brake_balance: u8,
}

/// Camera definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    /// Camera position
    pub position: Vec3,

    /// Look-at target
    pub target: Vec3,

    /// Field of view (degrees)
    pub fov: f32,
}

impl Track {
    /// Create a new empty track
    pub fn new(name: String) -> Self {
        Self {
            name,
            length: 0.0,
            object_shapes: Vec::new(),
            sections: Vec::new(),
            racing_line: RacingLine { points: Vec::new() },
            ai_behavior: AIBehavior::default(),
            pit_lane: Vec::new(),
            cameras: Vec::new(),
            checksum: 0,
        }
    }

    /// Validate track data
    pub fn validate(&self) -> Result<(), String> {
        if self.sections.is_empty() {
            return Err("Track has no sections".to_string());
        }

        if self.length <= 0.0 {
            return Err("Track length must be positive".to_string());
        }

        Ok(())
    }
}

impl Default for AIBehavior {
    fn default() -> Self {
        Self {
            aggression: 0.5,
            consistency: 0.5,
            car_setup: CarSetup::default(),
        }
    }
}

impl Default for CarSetup {
    fn default() -> Self {
        Self {
            front_wing: 10,
            rear_wing: 10,
            gear_ratios: [4, 8, 12, 16, 20, 24],
            brake_balance: 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_creation() {
        let track = Track::new("Test Track".to_string());
        assert_eq!(track.name, "Test Track");
        assert_eq!(track.sections.len(), 0);
    }

    #[test]
    fn test_track_validation() {
        let mut track = Track::new("Test".to_string());

        // Empty track should fail validation
        assert!(track.validate().is_err());

        // Add a section
        track.sections.push(TrackSection {
            position: Vec3::ZERO,
            width: 10.0,
            banking: 0.0,
            elevation: 0.0,
            surface: SurfaceType::Track,
            length: 100.0,
        });
        track.length = 100.0;

        // Should now pass
        assert!(track.validate().is_ok());
    }

    #[test]
    fn test_surface_types() {
        let track_surface = SurfaceType::Track;
        let grass = SurfaceType::Grass;

        assert_ne!(track_surface, grass);
        assert_eq!(track_surface, SurfaceType::Track);
    }
}
