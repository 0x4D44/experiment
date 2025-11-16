//! Racing line following for AI drivers
//!
//! Implements path following algorithms for AI cars to follow the optimal racing line.

use crate::data::track::Track;
use glam::{Vec2, Vec3};

/// Racing line follower for AI cars
pub struct RacingLineFollower {
    /// Racing line points (2D positions)
    line_points: Vec<Vec2>,

    /// Target speeds at each point (m/s)
    target_speeds: Vec<f32>,

    /// Lookahead distance for path following (meters)
    lookahead_distance: f32,
}

impl RacingLineFollower {
    /// Create a new racing line follower from track data
    pub fn new(track: &Track, lookahead_distance: f32) -> Self {
        // Extract racing line points and speeds from track
        let mut line_points = Vec::new();
        let mut target_speeds = Vec::new();

        // TODO: Convert racing line segments to 3D points once coordinate calculation is implemented
        // For now, use track sections as racing line
        if !track.sections.is_empty() {
            for section in &track.sections {
                line_points.push(Vec2::new(section.position.x, section.position.z));
                target_speeds.push(50.0); // Default speed (will be calculated from curvature)
            }
        }

        // If still no racing line data, create a simple circular path for testing
        if line_points.is_empty() {
            let num_points = 32;
            let radius = 200.0;
            for i in 0..num_points {
                let angle = (i as f32 / num_points as f32) * std::f32::consts::TAU;
                line_points.push(Vec2::new(
                    radius * angle.cos(),
                    radius * angle.sin(),
                ));
                target_speeds.push(50.0);
            }
        }

        Self {
            line_points,
            target_speeds,
            lookahead_distance,
        }
    }

    /// Get the target point to steer towards (Pure Pursuit algorithm)
    pub fn get_target_point(&self, car_position: Vec3) -> Vec2 {
        let car_pos_2d = Vec2::new(car_position.x, car_position.z);

        // Find closest point on racing line
        let closest_idx = self.find_closest_point(car_pos_2d);

        // Look ahead along the racing line
        let lookahead_idx = self.find_lookahead_point(car_pos_2d, closest_idx);

        self.line_points[lookahead_idx]
    }

    /// Get the target speed at current position
    pub fn get_target_speed(&self, car_position: Vec3) -> f32 {
        let car_pos_2d = Vec2::new(car_position.x, car_position.z);
        let closest_idx = self.find_closest_point(car_pos_2d);
        self.target_speeds[closest_idx]
    }

    /// Calculate steering angle to reach target point
    /// Returns steering in range [-1.0, 1.0]
    pub fn calculate_steering(&self, car_position: Vec3, car_forward: Vec2) -> f32 {
        let target = self.get_target_point(car_position);
        let car_pos_2d = Vec2::new(car_position.x, car_position.z);

        // Vector from car to target
        let to_target = target - car_pos_2d;

        if to_target.length() < 0.1 {
            return 0.0; // Already at target
        }

        let to_target_normalized = to_target.normalize();

        // Calculate angle between car forward and target direction
        // Use cross product to determine left/right
        let cross = car_forward.x * to_target_normalized.y - car_forward.y * to_target_normalized.x;
        let dot = car_forward.dot(to_target_normalized);

        // Convert to angle
        let angle = cross.atan2(dot);

        // Convert angle to steering input [-1, 1]
        // Clamp to reasonable steering angle
        let max_angle = 45.0_f32.to_radians();
        (angle / max_angle).clamp(-1.0, 1.0)
    }

    /// Find the closest point on the racing line
    fn find_closest_point(&self, position: Vec2) -> usize {
        let mut closest_idx = 0;
        let mut min_dist_sq = f32::MAX;

        for (i, &point) in self.line_points.iter().enumerate() {
            let dist_sq = (point - position).length_squared();
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                closest_idx = i;
            }
        }

        closest_idx
    }

    /// Find the lookahead point along the racing line
    fn find_lookahead_point(&self, _position: Vec2, start_idx: usize) -> usize {
        let num_points = self.line_points.len();
        let mut accumulated_distance = 0.0;
        let mut current_idx = start_idx;

        // Walk along the racing line until we've covered lookahead_distance
        for _ in 0..num_points {
            let next_idx = (current_idx + 1) % num_points;
            let segment_length = (self.line_points[next_idx] - self.line_points[current_idx]).length();

            accumulated_distance += segment_length;

            if accumulated_distance >= self.lookahead_distance {
                return next_idx;
            }

            current_idx = next_idx;
        }

        // If we looped through everything, return start point
        start_idx
    }

    /// Get number of racing line points
    pub fn num_points(&self) -> usize {
        self.line_points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::track::{TrackSection, SurfaceType, RacingLinePoint};

    fn create_test_track() -> Track {
        // Create a simple circular track for testing
        let mut sections = Vec::new();
        let mut racing_line_points = Vec::new();
        let num_sections = 8;
        let radius = 200.0;

        for i in 0..num_sections {
            let angle = (i as f32 / num_sections as f32) * std::f32::consts::TAU;
            let x = radius * angle.cos();
            let z = radius * angle.sin();

            sections.push(TrackSection {
                position: Vec3::new(x, 0.0, z),
                width: 15.0,
                length: 50.0,
                banking: 0.0,
                elevation: 0.0,
                surface: SurfaceType::Track,
            });

            racing_line_points.push(RacingLinePoint {
                position: Vec3::new(x, 0.0, z),
                speed: 50.0,
                is_braking_zone: false,
            });
        }

        Track {
            name: "Test Track".to_string(),
            length: radius * std::f32::consts::TAU,
            object_shapes: Vec::new(),
            sections,
            racing_line: crate::data::track::RacingLine {
                points: racing_line_points,
            },
            ai_behavior: crate::data::track::AIBehavior {
                aggression: 0.5,
                consistency: 0.7,
                car_setup: crate::data::track::CarSetup {
                    front_wing: 10,
                    rear_wing: 10,
                    gear_ratios: [1, 2, 3, 4, 5, 6],
                    brake_balance: 50,
                },
            },
            pit_lane: Vec::new(),
            cameras: Vec::new(),
            checksum: 0,
        }
    }

    #[test]
    fn test_racing_line_follower_creation() {
        let track = create_test_track();
        let follower = RacingLineFollower::new(&track, 20.0);
        assert_eq!(follower.num_points(), 8);
        assert_eq!(follower.lookahead_distance, 20.0);
    }

    #[test]
    fn test_get_target_point() {
        let track = create_test_track();
        let follower = RacingLineFollower::new(&track, 20.0);

        let car_pos = Vec3::new(200.0, 0.0, 0.0);
        let target = follower.get_target_point(car_pos);

        // Should return a point ahead on the racing line
        assert!(target.length() > 0.0);
    }

    #[test]
    fn test_get_target_speed() {
        let track = create_test_track();
        let follower = RacingLineFollower::new(&track, 20.0);

        let car_pos = Vec3::new(200.0, 0.0, 0.0);
        let speed = follower.get_target_speed(car_pos);

        // Should return a valid speed
        assert!(speed > 0.0);
        assert!(speed < 100.0); // Reasonable range
    }

    #[test]
    fn test_calculate_steering() {
        let track = create_test_track();
        let follower = RacingLineFollower::new(&track, 20.0);

        let car_pos = Vec3::new(200.0, 0.0, 0.0);
        let car_forward = Vec2::new(1.0, 0.0);

        let steering = follower.calculate_steering(car_pos, car_forward);

        // Should return steering in valid range
        assert!(steering >= -1.0 && steering <= 1.0);
    }

    #[test]
    fn test_find_closest_point() {
        let track = create_test_track();
        let follower = RacingLineFollower::new(&track, 20.0);

        let pos = Vec2::new(200.0, 0.0);
        let closest = follower.find_closest_point(pos);

        // Should find a valid index
        assert!(closest < follower.num_points());
    }
}
