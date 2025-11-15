//! Game state management
//!
//! Manages the overall game state, integrating physics, rendering, and input.

use crate::data::car::CarDatabase;
use crate::data::track::Track;
use crate::game::input::{CarInput, InputManager};
use crate::physics::{BodyId, CarPhysics, PhysicsWorld, TrackCollision};
use crate::platform::{Color, Renderer};
use crate::render::{Camera, CarRenderer, CarState, Hud, Telemetry, TrackRenderer};
use anyhow::Result;
use glam::Vec3;

/// Game mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    /// Practice mode (single car)
    Practice,
    /// Race mode (with AI opponents)
    Race,
    /// Time trial mode
    TimeTrial,
}

/// Main game state
pub struct GameState {
    /// Physics world
    physics_world: PhysicsWorld,

    /// Player car physics
    player_car: CarPhysics,

    /// Car database
    car_database: CarDatabase,

    /// Current track
    track: Option<Track>,

    /// Track renderer
    track_renderer: Option<TrackRenderer>,

    /// Track collision detector
    track_collision: Option<TrackCollision>,

    /// Car renderer
    car_renderer: CarRenderer,

    /// HUD renderer
    hud: Hud,

    /// Camera
    camera: Camera,

    /// Input manager
    input_manager: InputManager,

    /// Game mode
    mode: GameMode,

    /// Is game paused?
    paused: bool,

    /// Total game time elapsed
    total_time: f32,

    /// Current lap time
    lap_time: f32,

    /// Best lap time
    best_lap: Option<f32>,

    /// Current lap number
    current_lap: u32,

    /// Previous track section (for lap counting)
    prev_section: usize,
}

impl GameState {
    /// Create a new game state
    pub fn new(viewport_width: u32, viewport_height: u32) -> Self {
        let physics_world = PhysicsWorld::new();
        let car_database = CarDatabase::create_sample();

        // Create player car with first available car spec
        let car_spec = car_database.cars().next().unwrap().clone();
        let player_car = CarPhysics::new(BodyId(0), car_spec, Vec3::new(0.0, 1.0, 0.0));

        let camera = Camera::new(viewport_width, viewport_height);
        let car_renderer = CarRenderer::new();
        let hud = Hud::new(viewport_width, viewport_height);
        let input_manager = InputManager::new();

        Self {
            physics_world,
            player_car,
            car_database,
            track: None,
            track_renderer: None,
            track_collision: None,
            car_renderer,
            hud,
            camera,
            input_manager,
            mode: GameMode::Practice,
            paused: false,
            total_time: 0.0,
            lap_time: 0.0,
            best_lap: None,
            current_lap: 1,
            prev_section: 0,
        }
    }

    /// Load a track
    pub fn load_track(&mut self, track: Track) {
        // Create track renderer
        let track_renderer = TrackRenderer::new(&track);

        // Fit camera to track bounds
        self.camera.fit_bounds(track_renderer.bounds);

        // Create track collision detector
        let track_collision = TrackCollision::new(track.clone());

        self.track_renderer = Some(track_renderer);
        self.track_collision = Some(track_collision);
        self.track = Some(track);
    }

    /// Update game state
    pub fn update(&mut self, delta_time: f32) {
        if self.paused {
            return;
        }

        // Update input manager
        self.input_manager.update();

        // Get player input
        let input = self.input_manager.get_car_input();

        // Apply input to player car
        self.apply_input(&input);

        // Update physics
        self.update_physics(delta_time);

        // Update camera
        self.update_camera(delta_time);

        // Update timers
        self.total_time += delta_time;
        self.lap_time += delta_time;
    }

    /// Apply input to player car
    fn apply_input(&mut self, input: &CarInput) {
        self.player_car.set_throttle(input.throttle);
        self.player_car.set_brake(input.brake);
        self.player_car.set_steering(input.steering);

        if input.shift_up {
            self.player_car.shift_up();
        }
        if input.shift_down {
            self.player_car.shift_down();
        }
    }

    /// Update physics simulation
    fn update_physics(&mut self, delta_time: f32) {
        // Check collision and apply surface physics
        if let Some(collision_detector) = &self.track_collision {
            let collision_result = collision_detector.check_collision(self.player_car.body.position);

            // Apply surface grip to car
            self.player_car.apply_surface_grip(collision_result.grip_multiplier);
            self.player_car.on_track = collision_result.on_track;

            // Check for lap crossing
            if collision_detector.check_lap_crossing(self.prev_section, collision_result.nearest_section) {
                // Record lap time
                if self.lap_time > 1.0 {
                    self.set_best_lap(self.lap_time);
                    log::info!("Lap {} completed: {:.2}s", self.current_lap, self.lap_time);
                    self.lap_time = 0.0;
                    self.current_lap += 1;
                }
            }

            self.prev_section = collision_result.nearest_section;
        }

        // Update player car physics
        self.player_car.update(delta_time);

        // Add player car to physics world for this frame
        // (In a real implementation, we'd keep it in the world)
        self.physics_world.step(delta_time);
    }

    /// Update camera to follow player car
    fn update_camera(&mut self, delta_time: f32) {
        // Follow player car
        if self.mode == GameMode::Practice || self.mode == GameMode::Race {
            self.camera.follow(self.player_car.body.position);
            self.camera.update(delta_time);
        }
    }

    /// Render game state
    pub fn render(&self, renderer: &mut impl Renderer) -> Result<()> {
        // Clear screen
        renderer.clear(Color::rgb(20, 80, 20)); // Green background (grass)

        // Render track if loaded
        if let Some(track_renderer) = &self.track_renderer {
            track_renderer.render(renderer, &self.camera)?;
        }

        // Render player car
        let car_state = CarState {
            position: self.player_car.body.position,
            rotation: self.get_car_rotation(),
            velocity: self.player_car.body.velocity.truncate(),
            spec: self.player_car.spec.clone(),
            driver_name: "Player".to_string(),
        };

        self.car_renderer.render_car(renderer, &car_state, &self.camera)?;

        // Render HUD
        let telemetry = Telemetry {
            speed: self.player_car.speed * 3.6, // Convert m/s to km/h
            gear: self.player_car.gear,
            rpm: self.player_car.engine_rpm,
            current_lap: self.current_lap,
            current_lap_time: self.lap_time,
            best_lap_time: self.best_lap,
            delta_time: None, // TODO: Calculate delta vs best lap
            on_track: self.player_car.on_track,
        };

        self.hud.render(renderer, &telemetry)?;

        Ok(())
    }

    /// Get car rotation angle from quaternion
    fn get_car_rotation(&self) -> f32 {
        // Extract yaw from quaternion
        let q = self.player_car.body.orientation;
        let yaw = (2.0 * (q.w * q.z + q.x * q.y)).atan2(1.0 - 2.0 * (q.y * q.y + q.z * q.z));
        yaw
    }

    /// Handle keyboard input
    pub fn handle_key_down(&mut self, keycode: sdl2::keyboard::Keycode) {
        use sdl2::keyboard::Keycode;

        match keycode {
            Keycode::P => self.toggle_pause(),
            Keycode::R => self.reset(),
            _ => self.input_manager.key_down(keycode),
        }
    }

    /// Handle key release
    pub fn handle_key_up(&mut self, keycode: sdl2::keyboard::Keycode) {
        self.input_manager.key_up(keycode);
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Reset game state
    pub fn reset(&mut self) {
        // Reset car position
        self.player_car.body.position = Vec3::new(0.0, 1.0, 0.0);
        self.player_car.body.velocity = Vec3::ZERO;
        self.player_car.body.angular_velocity = Vec3::ZERO;
        self.player_car.engine_rpm = 1000.0;
        self.player_car.gear = 1;

        // Reset timers
        self.lap_time = 0.0;

        // Clear input
        self.input_manager.clear();
    }

    /// Get current speed in km/h
    pub fn get_speed_kmh(&self) -> f32 {
        self.player_car.speed * 3.6
    }

    /// Get current gear
    pub fn get_gear(&self) -> i8 {
        self.player_car.gear
    }

    /// Get current RPM
    pub fn get_rpm(&self) -> f32 {
        self.player_car.engine_rpm
    }

    /// Get lap time in seconds
    pub fn get_lap_time(&self) -> f32 {
        self.lap_time
    }

    /// Is game paused?
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Set camera zoom
    pub fn set_camera_zoom(&mut self, zoom: f32) {
        self.camera.set_zoom(zoom);
    }

    /// Update viewport size (when window is resized)
    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        self.camera.set_viewport_size(width, height);
    }

    /// Get car database
    pub fn car_database(&self) -> &CarDatabase {
        &self.car_database
    }

    /// Get best lap time
    pub fn best_lap(&self) -> Option<f32> {
        self.best_lap
    }

    /// Set best lap time
    pub fn set_best_lap(&mut self, time: f32) {
        if self.best_lap.is_none() || time < self.best_lap.unwrap() {
            self.best_lap = Some(time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let game = GameState::new(1920, 1080);
        assert_eq!(game.mode, GameMode::Practice);
        assert!(!game.paused);
        assert_eq!(game.total_time, 0.0);
    }

    #[test]
    fn test_pause_toggle() {
        let mut game = GameState::new(1920, 1080);
        assert!(!game.is_paused());

        game.toggle_pause();
        assert!(game.is_paused());

        game.toggle_pause();
        assert!(!game.is_paused());
    }

    #[test]
    fn test_reset() {
        let mut game = GameState::new(1920, 1080);
        game.lap_time = 60.0;
        game.player_car.body.velocity = Vec3::new(50.0, 0.0, 0.0);

        game.reset();
        assert_eq!(game.lap_time, 0.0);
        assert_eq!(game.player_car.body.velocity, Vec3::ZERO);
    }

    #[test]
    fn test_speed_conversion() {
        let mut game = GameState::new(1920, 1080);
        game.player_car.speed = 50.0; // 50 m/s
        assert_eq!(game.get_speed_kmh(), 180.0); // Should be 180 km/h
    }
}
