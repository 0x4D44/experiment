//! Track file format structures and types
//!
//! This module contains data structures representing the F1GP track file format.
//! Based on reverse engineering and ArgDocs documentation.

use serde::{Deserialize, Serialize};
use glam::Vec3;
use super::objects::ObjectShape;

/// Offsets into the track file
/// These offsets are stored at 0x1000 and point to various data sections
#[derive(Debug, Clone)]
pub struct TrackOffsets {
    pub base_offset: i16,
    pub unknown2: i16,
    pub unknown3: i16,
    pub unknown4: i16,
    /// Position of file checksum (adjusted by +0x1010)
    pub checksum_position: i16,
    /// Position of object data (adjusted by +0x1010)
    pub object_data: i16,
    /// Position of track section header and data (adjusted by +0x1010)
    pub track_data: i16,
}

/// Track section header that precedes the section data
/// Source: ArgData TrackSectionHeader.cs
#[derive(Debug, Clone)]
pub struct TrackSectionHeader {
    pub first_section_angle: u16,
    pub first_section_height: i16,
    pub track_center_x: i16,
    pub track_center_z: i16,
    pub track_center_y: i16,
    pub start_width: i16,
    pub pole_side: i16,
    pub pits_side: u8,
    pub surrounding_area: u8,
    pub right_verge_start_width: u8,
    pub left_verge_start_width: u8,
    pub kerb_type: u8,
    // Additional bytes vary by kerb type
    // Minimum header size: 19 bytes
}

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
    /// Position in 3D space (calculated from curvature/length)
    pub position: Vec3,

    /// Length of this section in meters (converted from raw byte)
    pub length: f32,

    /// Curvature/turn radius (raw i16 from file, + = right, - = left, 0 = straight)
    pub curvature: i16,

    /// Height/elevation change (raw i16 from file)
    pub height: i16,

    /// Flags bitfield (raw u16 from file)
    pub flags: u16,

    /// Right shoulder width (raw byte)
    pub right_verge_width: u8,

    /// Left shoulder width (raw byte)
    pub left_verge_width: u8,

    /// Commands associated with this section
    pub commands: Vec<TrackSectionCommand>,

    // Parsed flag fields
    /// Has left kerb
    pub has_left_kerb: bool,

    /// Has right kerb
    pub has_right_kerb: bool,

    /// Kerb height type
    pub kerb_height: KerbHeight,

    /// Pit lane entrance
    pub pit_lane_entrance: bool,

    /// Pit lane exit
    pub pit_lane_exit: bool,

    /// Road signs (300/200/100m markers)
    pub road_signs: bool,

    /// Road sign arrow
    pub road_sign_arrow: bool,

    /// Surface type (derived from flags or default)
    pub surface: SurfaceType,

    /// Track width at this section (meters, from track header)
    pub width: f32,

    /// Banking angle (calculated from flags, radians)
    pub banking: f32,

    /// Elevation in world space (calculated)
    pub elevation: f32,
}

/// Track section command (embedded in track data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackSectionCommand {
    /// Command ID
    pub command_id: u8,

    /// Arguments
    pub args: Vec<i16>,
}

/// Kerb height type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KerbHeight {
    /// Low kerb, can be driven over
    Low,

    /// High kerb that upsets vehicle
    High,
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
    /// Initial displacement/offset
    pub displacement: i16,

    /// Racing line segments
    pub segments: Vec<RacingLineSegment>,
}

/// A segment of the racing line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacingLineSegment {
    /// Segment length (raw byte)
    pub length: u8,

    /// Steering correction value
    pub correction: i16,

    /// Segment type (normal or wide radius)
    pub segment_type: SegmentType,
}

/// Type of racing line segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SegmentType {
    /// Normal segment with single radius
    Normal { radius: i16 },

    /// Wide radius segment with dual radii for complex curves
    WideRadius { high_radius: i16, low_radius: i16 },
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
            racing_line: RacingLine {
                displacement: 0,
                segments: Vec::new(),
            },
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

impl Default for TrackSection {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            length: 0.0,
            curvature: 0,
            height: 0,
            flags: 0,
            right_verge_width: 0,
            left_verge_width: 0,
            commands: Vec::new(),
            has_left_kerb: false,
            has_right_kerb: false,
            kerb_height: KerbHeight::Low,
            pit_lane_entrance: false,
            pit_lane_exit: false,
            road_signs: false,
            road_sign_arrow: false,
            surface: SurfaceType::Track,
            width: 10.0,
            banking: 0.0,
            elevation: 0.0,
        }
    }
}

impl TrackSection {
    /// Create a simple straight section for testing
    pub fn straight(length_meters: f32) -> Self {
        Self {
            length: length_meters,
            curvature: 0,  // Straight
            ..Default::default()
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
        track.sections.push(TrackSection::straight(100.0));
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
