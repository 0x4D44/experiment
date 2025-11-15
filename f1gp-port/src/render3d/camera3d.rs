// Camera3D - 3D camera system for different view modes
// Stage 6.1 & 6.4

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

/// 3D Camera with multiple modes
pub struct Camera3D {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    pub mode: CameraMode,
}

impl Camera3D {
    /// Create a new Camera3D
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 2.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 60.0_f32.to_radians(),
            aspect_ratio,
            near: 0.1,
            far: 1000.0,
            mode: CameraMode::Chase,
        }
    }

    /// Get view matrix (world to camera space)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Get projection matrix (camera space to clip space)
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    /// Update camera from car physics (Stage 6.4 will expand this)
    pub fn update_from_car(&mut self, car: &CarPhysics, _delta_time: f32) {
        match self.mode {
            CameraMode::Cockpit => {
                // Position at driver head (0.5m above car center)
                let offset = Vec3::new(0.0, 0.5, 0.0);
                self.position = car.body.position + offset;

                // Look forward from car's orientation
                let forward = car.body.orientation * Vec3::NEG_Z;
                self.target = self.position + forward;
                self.up = Vec3::Y;
            }
            CameraMode::Chase => {
                // Position 5m behind, 2m above car (basic for now)
                let car_forward = car.body.orientation * Vec3::NEG_Z;
                let offset = -car_forward * 5.0 + Vec3::Y * 2.0;
                self.position = car.body.position + offset;
                self.target = car.body.position + Vec3::Y * 0.5;
                self.up = Vec3::Y;
            }
            CameraMode::TVCamera => {
                // Fixed position for now (Stage 6.4 will improve)
                self.position = Vec3::new(20.0, 10.0, 0.0);
                self.target = car.body.position;
                self.up = Vec3::Y;
            }
            CameraMode::Helicopter => {
                // Overhead view
                self.position = car.body.position + Vec3::new(0.0, 15.0, 5.0);
                self.target = car.body.position;
                self.up = Vec3::Y;
            }
        }
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
}
