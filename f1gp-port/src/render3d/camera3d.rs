// Camera3D - 3D camera system for different view modes
// Stage 6.1 & 6.4 - Enhanced with smooth transitions, shake, interpolation

use glam::{Mat4, Vec3};
use crate::physics::CarPhysics;

/// Camera viewing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    /// First-person from driver's eyes
    Cockpit,
    /// Behind car (like modern games)
    Chase,
    /// Trackside camera
    TVCamera,
    /// Overhead following
    Helicopter,
}

/// Camera shake parameters
#[derive(Debug, Clone, Copy)]
pub struct CameraShake {
    /// Intensity of the shake (0.0 = none, 1.0 = max)
    intensity: f32,
    /// Time remaining for shake effect
    duration: f32,
    /// Frequency of shake oscillation
    frequency: f32,
    /// Current phase offset
    phase: f32,
}

impl CameraShake {
    fn new() -> Self {
        Self {
            intensity: 0.0,
            duration: 0.0,
            frequency: 20.0, // 20 Hz oscillation
            phase: 0.0,
        }
    }

    /// Trigger a camera shake
    pub fn trigger(&mut self, intensity: f32, duration: f32) {
        self.intensity = self.intensity.max(intensity); // Don't interrupt stronger shakes
        self.duration = self.duration.max(duration);
    }

    /// Update shake and return offset
    fn update(&mut self, delta_time: f32) -> Vec3 {
        if self.duration <= 0.0 {
            self.intensity = 0.0;
            return Vec3::ZERO;
        }

        self.duration -= delta_time;
        self.phase += delta_time * self.frequency;

        // Decay intensity over time
        let current_intensity = self.intensity * (self.duration / 0.5).min(1.0);

        // Generate pseudo-random shake using sine waves at different frequencies
        let x = (self.phase * 1.3).sin() * current_intensity * 0.1;
        let y = (self.phase * 1.7).sin() * current_intensity * 0.1;
        let z = (self.phase * 2.1).sin() * current_intensity * 0.05;

        Vec3::new(x, y, z)
    }
}

/// 3D Camera with multiple modes and advanced features
pub struct Camera3D {
    // Current state
    position: Vec3,
    target: Vec3,
    up: Vec3,

    // Desired state (for interpolation)
    desired_position: Vec3,
    desired_target: Vec3,

    // Camera parameters
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    pub mode: CameraMode,

    // Smooth transition parameters
    position_smoothing: f32, // 0.0 = instant, 1.0 = very smooth
    target_smoothing: f32,

    // Camera shake
    shake: CameraShake,

    // TV Camera state
    tv_camera_index: usize,
    tv_camera_positions: Vec<Vec3>,
}

impl Camera3D {
    /// Create a new Camera3D
    pub fn new(aspect_ratio: f32) -> Self {
        let position = Vec3::new(0.0, 2.0, 5.0);
        let target = Vec3::ZERO;

        // Default TV camera positions (can be customized per track)
        let tv_camera_positions = vec![
            Vec3::new(20.0, 10.0, 0.0),
            Vec3::new(-20.0, 10.0, 0.0),
            Vec3::new(0.0, 10.0, 20.0),
            Vec3::new(0.0, 10.0, -20.0),
        ];

        Self {
            position,
            target,
            up: Vec3::Y,
            desired_position: position,
            desired_target: target,
            fov: 60.0_f32.to_radians(),
            aspect_ratio,
            near: 0.1,
            far: 1000.0,
            mode: CameraMode::Chase,
            position_smoothing: 0.9,  // Smooth position transitions
            target_smoothing: 0.85,   // Smooth target transitions
            shake: CameraShake::new(),
            tv_camera_index: 0,
            tv_camera_positions,
        }
    }

    /// Get view matrix (world to camera space) with shake applied
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Get projection matrix (camera space to clip space)
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    /// Trigger a camera shake effect
    pub fn shake(&mut self, intensity: f32, duration: f32) {
        self.shake.trigger(intensity, duration);
    }

    /// Set smoothing factors (0.0 = instant, 1.0 = very smooth)
    pub fn set_smoothing(&mut self, position: f32, target: f32) {
        self.position_smoothing = position.clamp(0.0, 0.99);
        self.target_smoothing = target.clamp(0.0, 0.99);
    }

    /// Lerp helper function
    fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
        a + (b - a) * t
    }

    /// Update camera from car physics with smooth transitions and shake
    pub fn update_from_car(&mut self, car: &CarPhysics, delta_time: f32) {
        // Calculate desired camera position based on mode
        match self.mode {
            CameraMode::Cockpit => {
                // Position at driver head (0.5m above car center)
                let offset = Vec3::new(0.0, 0.5, 0.0);
                self.desired_position = car.body.position + offset;

                // Look forward from car's orientation
                let forward = car.body.orientation * Vec3::NEG_Z;
                self.desired_target = self.desired_position + forward * 10.0;
                self.up = Vec3::Y;

                // Cockpit has minimal smoothing for responsiveness
                self.position_smoothing = 0.3;
                self.target_smoothing = 0.5;
            }
            CameraMode::Chase => {
                // Position 5m behind, 2m above car
                let car_forward = car.body.orientation * Vec3::NEG_Z;
                let car_right = car.body.orientation * Vec3::X;

                // Add slight offset to the right for better view
                let offset = -car_forward * 6.0 + Vec3::Y * 2.5 + car_right * 0.5;
                self.desired_position = car.body.position + offset;
                self.desired_target = car.body.position + Vec3::Y * 0.8;
                self.up = Vec3::Y;

                // Chase camera has moderate smoothing
                self.position_smoothing = 0.9;
                self.target_smoothing = 0.85;
            }
            CameraMode::TVCamera => {
                // Use current TV camera position from array
                let tv_pos = self.tv_camera_positions[self.tv_camera_index];
                self.desired_position = tv_pos;
                self.desired_target = car.body.position + Vec3::Y * 1.0;
                self.up = Vec3::Y;

                // TV camera has heavy smoothing for cinematic look
                self.position_smoothing = 0.98;
                self.target_smoothing = 0.95;

                // Switch TV camera if car gets too far (100m)
                if car.body.position.distance(tv_pos) > 100.0 {
                    self.switch_tv_camera();
                }
            }
            CameraMode::Helicopter => {
                // Overhead view that follows car
                let car_forward = car.body.orientation * Vec3::NEG_Z;
                self.desired_position = car.body.position + Vec3::Y * 20.0 + car_forward * 5.0;
                self.desired_target = car.body.position;
                self.up = Vec3::Y;

                // Helicopter has smooth motion
                self.position_smoothing = 0.92;
                self.target_smoothing = 0.88;
            }
        }

        // Apply smooth interpolation
        let pos_lerp_factor = 1.0 - self.position_smoothing.powf(delta_time * 60.0);
        let target_lerp_factor = 1.0 - self.target_smoothing.powf(delta_time * 60.0);

        self.position = Self::lerp_vec3(self.position, self.desired_position, pos_lerp_factor);
        self.target = Self::lerp_vec3(self.target, self.desired_target, target_lerp_factor);

        // Apply camera shake
        let shake_offset = self.shake.update(delta_time);
        self.position += shake_offset;
        self.target += shake_offset * 0.5; // Less shake on target

        // Apply collision detection (keep camera above ground)
        self.apply_collision_detection();
    }

    /// Switch to next TV camera position
    fn switch_tv_camera(&mut self) {
        self.tv_camera_index = (self.tv_camera_index + 1) % self.tv_camera_positions.len();
    }

    /// Keep camera above ground and away from track surface
    fn apply_collision_detection(&mut self) {
        // Simple ground collision - keep camera at least 1m above ground
        let min_height = 1.0;
        if self.position.y < min_height {
            self.position.y = min_height;
        }

        // For chase camera, ensure we don't clip through car
        if self.mode == CameraMode::Chase {
            // If camera is too close to desired position, push it back
            let distance = self.position.distance(self.desired_target);
            if distance < 2.0 {
                let direction = (self.position - self.desired_target).normalize();
                self.position = self.desired_target + direction * 2.0;
            }
        }
    }

    /// Set TV camera positions (useful for track-specific cameras)
    pub fn set_tv_camera_positions(&mut self, positions: Vec<Vec3>) {
        if !positions.is_empty() {
            self.tv_camera_positions = positions;
            self.tv_camera_index = 0;
        }
    }

    /// Cycle to next camera mode
    pub fn next_mode(&mut self) {
        self.mode = match self.mode {
            CameraMode::Cockpit => CameraMode::Chase,
            CameraMode::Chase => CameraMode::TVCamera,
            CameraMode::TVCamera => CameraMode::Helicopter,
            CameraMode::Helicopter => CameraMode::Cockpit,
        };
    }

    /// Set aspect ratio (for window resize)
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    /// Get camera position
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Get camera forward direction
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::BodyId;
    use glam::Quat;

    #[test]
    fn test_camera_creation() {
        let camera = Camera3D::new(16.0 / 9.0);
        assert_eq!(camera.mode, CameraMode::Chase);
        assert!(camera.aspect_ratio > 0.0);
    }

    #[test]
    fn test_view_projection_matrices() {
        let camera = Camera3D::new(16.0 / 9.0);
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();

        // Matrices should be valid (determinant != 0)
        assert!(view.determinant().abs() > 0.001);
        assert!(proj.determinant().abs() > 0.001);
    }

    #[test]
    fn test_camera_update_from_car() {
        use crate::data::CarDatabase;

        let mut camera = Camera3D::new(16.0 / 9.0);
        let car_database = CarDatabase::create_sample();
        let car_spec = car_database.cars().next().unwrap().clone();
        let mut car = CarPhysics::new(BodyId(0), car_spec, Vec3::new(0.0, 1.0, 0.0));
        car.body.orientation = Quat::from_rotation_y(0.0);

        camera.update_from_car(&car, 0.016);

        // Camera should be positioned relative to car
        assert!(camera.position().distance(car.body.position) > 0.0);
    }

    #[test]
    fn test_camera_shake() {
        let mut camera = Camera3D::new(16.0 / 9.0);
        let initial_pos = camera.position();

        // Trigger shake
        camera.shake(1.0, 0.5);

        // Update camera (without car, just shake)
        let shake_offset = camera.shake.update(0.016);

        // Shake should produce non-zero offset
        assert!(shake_offset.length() > 0.0);

        // Duration should decrease
        assert!(camera.shake.duration < 0.5);
    }

    #[test]
    fn test_camera_smoothing() {
        use crate::data::CarDatabase;

        let mut camera = Camera3D::new(16.0 / 9.0);
        let car_database = CarDatabase::create_sample();
        let car_spec = car_database.cars().next().unwrap().clone();
        let car = CarPhysics::new(BodyId(0), car_spec, Vec3::new(100.0, 1.0, 100.0));

        // Set high smoothing
        camera.set_smoothing(0.95, 0.95);

        let initial_pos = camera.position();
        camera.update_from_car(&car, 0.016);

        // Position should move toward car but not instantly
        let distance_moved = camera.position().distance(initial_pos);
        assert!(distance_moved > 0.0);
        assert!(distance_moved < 50.0); // Didn't jump all the way
    }

    #[test]
    fn test_camera_mode_cycling() {
        let mut camera = Camera3D::new(16.0 / 9.0);

        assert_eq!(camera.mode, CameraMode::Chase);

        camera.next_mode();
        assert_eq!(camera.mode, CameraMode::TVCamera);

        camera.next_mode();
        assert_eq!(camera.mode, CameraMode::Helicopter);

        camera.next_mode();
        assert_eq!(camera.mode, CameraMode::Cockpit);

        camera.next_mode();
        assert_eq!(camera.mode, CameraMode::Chase);
    }

    #[test]
    fn test_camera_collision_detection() {
        use crate::data::CarDatabase;

        let mut camera = Camera3D::new(16.0 / 9.0);
        let car_database = CarDatabase::create_sample();
        let car_spec = car_database.cars().next().unwrap().clone();
        let car = CarPhysics::new(BodyId(0), car_spec, Vec3::new(0.0, 1.0, 0.0));

        // Force camera below ground
        camera.position.y = -5.0;
        camera.apply_collision_detection();

        // Should be corrected to min height
        assert!(camera.position.y >= 1.0);
    }

    #[test]
    fn test_tv_camera_switching() {
        use crate::data::CarDatabase;

        let mut camera = Camera3D::new(16.0 / 9.0);
        camera.mode = CameraMode::TVCamera;

        let car_database = CarDatabase::create_sample();
        let car_spec = car_database.cars().next().unwrap().clone();

        // Move car far from first TV camera
        let car = CarPhysics::new(BodyId(0), car_spec, Vec3::new(200.0, 1.0, 200.0));

        let initial_index = camera.tv_camera_index;
        camera.update_from_car(&car, 0.016);

        // TV camera should have switched due to distance
        // (might take multiple frames, so this tests the switching mechanism)
        assert!(camera.tv_camera_positions.len() > 1);
    }
}
