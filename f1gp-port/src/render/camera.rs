//! Camera system for 2D rendering
//!
//! Provides camera transformations between world and screen coordinates.

use crate::platform::Rect;
use glam::{Mat4, Vec2, Vec3};

/// Camera modes for different views
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    /// Free camera that can pan and zoom
    Free,
    /// Camera that follows a target
    Follow,
    /// Top-down overview
    TopDown,
    /// Track overview (fits entire track)
    TrackOverview,
}

/// 2D Camera for viewing the game world
#[derive(Debug, Clone)]
pub struct Camera {
    /// Camera position in world space
    pub position: Vec3,

    /// Camera target (for follow mode)
    pub target: Vec3,

    /// Zoom level (1.0 = normal, >1.0 = zoomed in, <1.0 = zoomed out)
    pub zoom: f32,

    /// Viewport rectangle (screen space)
    pub viewport: Rect,

    /// Camera mode
    pub mode: CameraMode,

    /// Smoothing factor for camera movement (0.0 = instant, 1.0 = very smooth)
    pub smoothing: f32,
}

impl Camera {
    /// Create a new camera with default settings
    pub fn new(viewport_width: u32, viewport_height: u32) -> Self {
        Self {
            position: Vec3::ZERO,
            target: Vec3::ZERO,
            zoom: 1.0,
            viewport: Rect::new(0.0, 0.0, viewport_width as f32, viewport_height as f32),
            mode: CameraMode::Free,
            smoothing: 0.1,
        }
    }

    /// Create a camera in track overview mode
    pub fn track_overview(
        viewport_width: u32,
        viewport_height: u32,
        track_bounds: Rect,
    ) -> Self {
        let mut camera = Self::new(viewport_width, viewport_height);
        camera.mode = CameraMode::TrackOverview;
        camera.fit_bounds(track_bounds);
        camera
    }

    /// Update camera position (for follow mode)
    pub fn update(&mut self, _delta_time: f32) {
        match self.mode {
            CameraMode::Follow => {
                // Smoothly move camera towards target
                let delta = self.target - self.position;
                self.position += delta * self.smoothing.min(1.0);
            }
            _ => {}
        }
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_pos: Vec3) -> Vec2 {
        let centered_x = world_pos.x - self.position.x;
        let centered_y = world_pos.y - self.position.y;

        let screen_x = (centered_x * self.zoom) + self.viewport.width / 2.0;
        let screen_y = (centered_y * self.zoom) + self.viewport.height / 2.0;

        Vec2::new(screen_x, screen_y)
    }

    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec3 {
        let centered_x = screen_pos.x - self.viewport.width / 2.0;
        let centered_y = screen_pos.y - self.viewport.height / 2.0;

        let world_x = (centered_x / self.zoom) + self.position.x;
        let world_y = (centered_y / self.zoom) + self.position.y;

        Vec3::new(world_x, world_y, 0.0)
    }

    /// Pan the camera by a screen-space offset
    pub fn pan(&mut self, delta: Vec2) {
        let world_delta = delta / self.zoom;
        self.position.x -= world_delta.x;
        self.position.y -= world_delta.y;
    }

    /// Zoom the camera by a factor (centered on screen center)
    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom *= factor;
        self.zoom = self.zoom.clamp(0.1, 10.0);
    }

    /// Zoom the camera to a specific level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.1, 10.0);
    }

    /// Center the camera on a world position
    pub fn center_on(&mut self, world_pos: Vec3) {
        self.position = world_pos;
    }

    /// Set camera to follow a target
    pub fn follow(&mut self, target: Vec3) {
        self.mode = CameraMode::Follow;
        self.target = target;
    }

    /// Fit the camera to show a bounding box
    pub fn fit_bounds(&mut self, bounds: Rect) {
        // Center on bounds
        self.position.x = bounds.x + bounds.width / 2.0;
        self.position.y = bounds.y + bounds.height / 2.0;

        // Calculate zoom to fit bounds in viewport
        let zoom_x = self.viewport.width / bounds.width;
        let zoom_y = self.viewport.height / bounds.height;
        self.zoom = zoom_x.min(zoom_y) * 0.9; // 0.9 for some padding
    }

    /// Update viewport size (when window is resized)
    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        self.viewport.width = width as f32;
        self.viewport.height = height as f32;
    }

    /// Get view matrix for rendering
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::new(self.zoom, self.zoom, 1.0),
            glam::Quat::IDENTITY,
            -self.position,
        )
    }

    /// Get projection matrix (orthographic)
    pub fn projection_matrix(&self) -> Mat4 {
        let half_width = self.viewport.width / 2.0;
        let half_height = self.viewport.height / 2.0;
        Mat4::orthographic_rh(
            -half_width,
            half_width,
            -half_height,
            half_height,
            -100.0,
            100.0,
        )
    }

    /// Check if a world position is visible in the viewport
    pub fn is_visible(&self, world_pos: Vec3, margin: f32) -> bool {
        let screen_pos = self.world_to_screen(world_pos);
        screen_pos.x >= -margin
            && screen_pos.x <= self.viewport.width + margin
            && screen_pos.y >= -margin
            && screen_pos.y <= self.viewport.height + margin
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(1920, 1080);
        assert_eq!(camera.zoom, 1.0);
        assert_eq!(camera.viewport.width, 1920.0);
        assert_eq!(camera.viewport.height, 1080.0);
    }

    #[test]
    fn test_world_to_screen() {
        let camera = Camera::new(1920, 1080);
        let world_pos = Vec3::new(0.0, 0.0, 0.0);
        let screen_pos = camera.world_to_screen(world_pos);
        assert_eq!(screen_pos.x, 960.0); // Half of 1920
        assert_eq!(screen_pos.y, 540.0); // Half of 1080
    }

    #[test]
    fn test_screen_to_world() {
        let camera = Camera::new(1920, 1080);
        let screen_pos = Vec2::new(960.0, 540.0);
        let world_pos = camera.screen_to_world(screen_pos);
        assert_eq!(world_pos.x, 0.0);
        assert_eq!(world_pos.y, 0.0);
    }

    #[test]
    fn test_zoom() {
        let mut camera = Camera::new(1920, 1080);
        camera.zoom_by(2.0);
        assert_eq!(camera.zoom, 2.0);

        camera.zoom_by(10.0);
        assert_eq!(camera.zoom, 10.0); // Clamped to max

        camera.set_zoom(0.05);
        assert_eq!(camera.zoom, 0.1); // Clamped to min
    }

    #[test]
    fn test_pan() {
        let mut camera = Camera::new(1920, 1080);
        camera.pan(Vec2::new(100.0, 50.0));
        assert_eq!(camera.position.x, -100.0);
        assert_eq!(camera.position.y, -50.0);
    }

    #[test]
    fn test_center_on() {
        let mut camera = Camera::new(1920, 1080);
        camera.center_on(Vec3::new(500.0, 300.0, 0.0));
        assert_eq!(camera.position.x, 500.0);
        assert_eq!(camera.position.y, 300.0);
    }

    #[test]
    fn test_visibility() {
        let camera = Camera::new(1920, 1080);
        // Point at center should be visible
        assert!(camera.is_visible(Vec3::ZERO, 0.0));

        // Point far away should not be visible
        assert!(!camera.is_visible(Vec3::new(10000.0, 10000.0, 0.0), 0.0));
    }
}
