//! Track collision and surface detection
//!
//! Determines which surface the car is on and modifies physics accordingly.

use crate::data::track::{SurfaceType, Track, TrackSection};
use glam::{Vec2, Vec3};

/// Result of collision detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollisionResult {
    /// Surface type at car position
    pub surface: SurfaceType,

    /// Distance from track center line (negative = left, positive = right)
    pub distance_from_center: f32,

    /// Is car fully on track?
    pub on_track: bool,

    /// Grip multiplier based on surface (1.0 = full grip)
    pub grip_multiplier: f32,

    /// Closest track section index
    pub nearest_section: usize,
}

/// Track collision detector
pub struct TrackCollision {
    /// Track data
    track: Track,

    /// Cached track positions for faster lookup
    track_positions: Vec<Vec2>,
}

impl TrackCollision {
    /// Create a new track collision detector
    pub fn new(track: Track) -> Self {
        // Cache track positions as 2D points
        let track_positions: Vec<Vec2> = track
            .sections
            .iter()
            .map(|s| Vec2::new(s.position.x, s.position.z))
            .collect();

        Self {
            track,
            track_positions,
        }
    }

    /// Check collision and surface for a car at given position
    pub fn check_collision(&self, car_position: Vec3) -> CollisionResult {
        let car_pos_2d = Vec2::new(car_position.x, car_position.z);

        // Find nearest track section
        let nearest_section = self.find_nearest_section(car_pos_2d);

        // Get section data
        let section = &self.track.sections[nearest_section];
        let section_pos = Vec2::new(section.position.x, section.position.z);

        // Calculate distance from track center
        let distance_from_center = (car_pos_2d - section_pos).length();

        // Determine if on track based on track width
        let half_width = section.width / 2.0;
        let on_track = distance_from_center <= half_width;

        // Determine surface type
        let (surface, grip_multiplier) = if on_track {
            // On track surface
            match section.surface {
                SurfaceType::Track => (SurfaceType::Track, 1.0),
                SurfaceType::PitLane => (SurfaceType::PitLane, 0.95),
                SurfaceType::Kerb => (SurfaceType::Kerb, 0.85),
                _ => (SurfaceType::Track, 1.0),
            }
        } else {
            // Off track - determine by how far off
            let off_track_distance = distance_from_center - half_width;

            if off_track_distance < 2.0 {
                // Just off track - might be grass or kerb
                (SurfaceType::Grass, 0.4)
            } else if off_track_distance < 5.0 {
                // Further off - grass
                (SurfaceType::Grass, 0.3)
            } else {
                // Way off - gravel trap
                (SurfaceType::Gravel, 0.2)
            }
        };

        CollisionResult {
            surface,
            distance_from_center,
            on_track,
            grip_multiplier,
            nearest_section,
        }
    }

    /// Find nearest track section to a position
    fn find_nearest_section(&self, position: Vec2) -> usize {
        let mut nearest_idx = 0;
        let mut nearest_dist = f32::MAX;

        for (i, &track_pos) in self.track_positions.iter().enumerate() {
            let dist = (position - track_pos).length_squared();
            if dist < nearest_dist {
                nearest_dist = dist;
                nearest_idx = i;
            }
        }

        nearest_idx
    }

    /// Get track section count
    pub fn section_count(&self) -> usize {
        self.track.sections.len()
    }

    /// Get track section
    pub fn get_section(&self, index: usize) -> Option<&TrackSection> {
        self.track.sections.get(index)
    }

    /// Check if car has crossed start/finish line
    pub fn check_lap_crossing(&self, prev_section: usize, current_section: usize) -> bool {
        // Lap crossing happens when going from last section to first section
        let section_count = self.section_count();
        if section_count == 0 {
            return false;
        }

        // Check if we crossed from last to first section
        (prev_section == section_count - 1 && current_section == 0)
            || (prev_section == section_count - 1 && current_section == 1)
    }
}

/// Surface physics properties
pub struct SurfacePhysics;

impl SurfacePhysics {
    /// Get grip multiplier for surface type
    pub fn grip_multiplier(surface: SurfaceType) -> f32 {
        match surface {
            SurfaceType::Track => 1.0,
            SurfaceType::Grass => 0.3,
            SurfaceType::Gravel => 0.2,
            SurfaceType::Kerb => 0.85,
            SurfaceType::PitLane => 0.95,
            SurfaceType::Wall => 0.0,
        }
    }

    /// Get rolling resistance for surface
    pub fn rolling_resistance(surface: SurfaceType) -> f32 {
        match surface {
            SurfaceType::Track => 0.015,
            SurfaceType::Grass => 0.08,
            SurfaceType::Gravel => 0.15,
            SurfaceType::Kerb => 0.02,
            SurfaceType::PitLane => 0.015,
            SurfaceType::Wall => 0.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::track::{AIBehavior, RacingLine};

    fn create_test_track() -> Track {
        Track {
            name: "Test".to_string(),
            length: 1000.0,
            object_shapes: vec![],
            sections: vec![
                TrackSection {
                    position: Vec3::new(0.0, 0.0, 0.0),
                    width: 15.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
                TrackSection {
                    position: Vec3::new(100.0, 0.0, 0.0),
                    width: 15.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
            ],
            racing_line: RacingLine {
                displacement: 0,
                segments: Vec::new(),
            },
            ai_behavior: AIBehavior::default(),
            pit_lane: vec![],
            cameras: vec![],
            checksum: 0,
        }
    }

    #[test]
    fn test_collision_on_track() {
        let track = create_test_track();
        let collision = TrackCollision::new(track);

        // Car on track
        let result = collision.check_collision(Vec3::new(0.0, 0.0, 0.0));
        assert!(result.on_track);
        assert_eq!(result.surface, SurfaceType::Track);
        assert_eq!(result.grip_multiplier, 1.0);
    }

    #[test]
    fn test_collision_off_track() {
        let track = create_test_track();
        let collision = TrackCollision::new(track);

        // Car far off track (12.5 units off track = gravel)
        let result = collision.check_collision(Vec3::new(0.0, 0.0, 20.0));
        assert!(!result.on_track);
        assert_eq!(result.surface, SurfaceType::Gravel);
        assert!(result.grip_multiplier < 1.0);
    }

    #[test]
    fn test_surface_physics() {
        assert_eq!(SurfacePhysics::grip_multiplier(SurfaceType::Track), 1.0);
        assert_eq!(SurfacePhysics::grip_multiplier(SurfaceType::Grass), 0.3);
        assert_eq!(SurfacePhysics::grip_multiplier(SurfaceType::Gravel), 0.2);
    }

    #[test]
    fn test_lap_crossing() {
        let track = create_test_track();
        let collision = TrackCollision::new(track);

        // Crossing from last to first section
        assert!(collision.check_lap_crossing(1, 0));

        // Not crossing
        assert!(!collision.check_lap_crossing(0, 1));
    }

    #[test]
    fn test_nearest_section() {
        let track = create_test_track();
        let collision = TrackCollision::new(track);

        // Position near first section
        let result = collision.check_collision(Vec3::new(5.0, 0.0, 0.0));
        assert_eq!(result.nearest_section, 0);

        // Position near second section
        let result = collision.check_collision(Vec3::new(95.0, 0.0, 0.0));
        assert_eq!(result.nearest_section, 1);
    }
}
