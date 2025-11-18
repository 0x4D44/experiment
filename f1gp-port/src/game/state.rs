//! Game state management
//!
//! Manages the overall game state, integrating physics, rendering, and input.

use crate::ai::{AIDriver, DriverPersonality, NearbyCarInfo, RacingLineFollower};
use crate::data::car::CarDatabase;
use crate::data::track::Track;
use crate::game::input::{CarInput, InputManager};
use crate::game::session::RaceSession;
use crate::physics::{BodyId, CarPhysics, PhysicsWorld, TrackCollision};
use crate::platform::{Color, Renderer};
use crate::render::{Camera, CarRenderer, CarState, Hud, Telemetry, TrackRenderer};
use crate::render3d::{Camera3D, CameraMode, Renderer3D};
use crate::ui::{Menu, MenuAction};
use anyhow::Result;
use glam::{Vec2, Vec3};

/// Game screen state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameScreen {
    /// Main menu
    MainMenu,

    /// Race setup screen
    RaceSetup,

    /// In-game (racing)
    InGame,

    /// Pause menu
    Paused,

    /// Race results
    Results,
}

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

    /// AI opponent cars
    ai_cars: Vec<CarPhysics>,

    /// AI drivers controlling opponent cars
    ai_drivers: Vec<AIDriver>,

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

    /// Optional 3D renderer
    renderer_3d: Option<Renderer3D>,

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

    /// Previous sections for AI cars (for lap counting)
    ai_prev_sections: Vec<usize>,

    /// Race session manager
    race_session: Option<RaceSession>,

    /// Current screen
    screen: GameScreen,

    /// Current menu
    menu: Option<Menu>,

    /// Number of AI opponents for race setup
    num_opponents: usize,

    /// Viewport dimensions
    viewport_width: u32,
    viewport_height: u32,
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

        // Create main menu
        let menu = Menu::main_menu(viewport_width, viewport_height);

        Self {
            physics_world,
            player_car,
            ai_cars: Vec::new(),
            ai_drivers: Vec::new(),
            car_database,
            track: None,
            track_renderer: None,
            track_collision: None,
            car_renderer,
            hud,
            camera,
            renderer_3d: None,
            input_manager,
            mode: GameMode::Practice,
            paused: false,
            total_time: 0.0,
            lap_time: 0.0,
            best_lap: None,
            current_lap: 1,
            prev_section: 0,
            ai_prev_sections: Vec::new(),
            race_session: None,
            screen: GameScreen::MainMenu,
            menu: Some(menu),
            num_opponents: 5,
            viewport_width,
            viewport_height,
        }
    }

    /// Load a track
    pub fn load_track(&mut self, track: Track) {
        // Create track renderer
        let track_renderer = TrackRenderer::new(&track);

        // Fit camera to track bounds
        self.camera.fit_bounds(track_renderer.bounds);

        // Enable isometric 2.5D view (like original F1GP)
        self.camera.set_isometric();

        // Create track collision detector
        let track_collision = TrackCollision::new(track.clone());

        self.track_renderer = Some(track_renderer);
        self.track_collision = Some(track_collision);
        self.track = Some(track);
    }

    /// Spawn AI opponents for race mode
    pub fn spawn_ai_opponents(&mut self, num_opponents: usize) {
        // Clear existing AI
        self.ai_cars.clear();
        self.ai_drivers.clear();
        self.ai_prev_sections.clear();

        // Get available cars from database
        let available_cars: Vec<_> = self.car_database.cars().cloned().collect();

        // Get track for racing line
        let track = match &self.track {
            Some(t) => t,
            None => {
                log::warn!("Cannot spawn AI without loaded track");
                return;
            }
        };

        // Pre-defined AI personalities (rotate through famous drivers)
        let personalities = [
            ("Ayrton Senna", DriverPersonality::senna()),
            ("Nigel Mansell", DriverPersonality::mansell()),
            ("Alain Prost", DriverPersonality::prost()),
            ("Michael Schumacher", DriverPersonality::average()),
            ("Gerhard Berger", DriverPersonality::average()),
        ];

        // Spawn AI cars at staggered positions
        for i in 0..num_opponents.min(5) {
            let car_idx = (i + 1) % available_cars.len();
            let car_spec = available_cars[car_idx].clone();

            // Position AI cars behind player with spacing
            let z_offset = -20.0 * (i as f32 + 1.0);
            let x_offset = if i % 2 == 0 { -3.0 } else { 3.0 }; // Alternate left/right
            let position = Vec3::new(x_offset, 1.0, z_offset);

            // Create AI car
            let car_id = BodyId(i + 1);
            let ai_car = CarPhysics::new(car_id, car_spec, position);

            // Create AI driver
            let (name, personality) = personalities[i % personalities.len()];
            let mut ai_driver = AIDriver::new(name.to_string(), personality);

            // Set up racing line for AI
            let racing_line = RacingLineFollower::new(track, 20.0);
            ai_driver.set_racing_line(racing_line);

            self.ai_cars.push(ai_car);
            self.ai_drivers.push(ai_driver);
            self.ai_prev_sections.push(0);
        }

        log::info!("Spawned {} AI opponents", num_opponents.min(5));
        self.mode = GameMode::Race;

        // Create race session (player + AI opponents, 5 laps)
        let num_drivers = 1 + self.ai_drivers.len();
        self.race_session = Some(RaceSession::new(num_drivers, 5));
    }

    /// Start the race countdown sequence
    pub fn start_race(&mut self) {
        if let Some(ref mut session) = self.race_session {
            session.start_countdown();
            log::info!("Race countdown started");
        }
    }

    /// Get race session (if active)
    pub fn race_session(&self) -> Option<&RaceSession> {
        self.race_session.as_ref()
    }

    /// Handle menu action
    fn handle_menu_action(&mut self, action: MenuAction) {
        match action {
            MenuAction::StartRace => {
                // Transition from menu to in-game
                if self.screen == GameScreen::RaceSetup {
                    // Spawn AI opponents and start race
                    self.spawn_ai_opponents(self.num_opponents);
                    self.start_race();
                    self.screen = GameScreen::InGame;
                    self.menu = None;
                    log::info!("Starting race with {} opponents", self.num_opponents);
                } else {
                    // Go to race setup
                    self.screen = GameScreen::RaceSetup;
                    self.menu = Some(Menu::race_setup_menu(
                        self.viewport_width,
                        self.viewport_height,
                        self.num_opponents,
                    ));
                }
            }

            MenuAction::Resume => {
                // Resume from pause
                self.screen = GameScreen::InGame;
                self.menu = None;
                self.paused = false;
            }

            MenuAction::Restart => {
                // Restart race
                self.reset();
                if !self.ai_drivers.is_empty() {
                    self.spawn_ai_opponents(self.num_opponents);
                    self.start_race();
                }
                self.screen = GameScreen::InGame;
                self.menu = None;
            }

            MenuAction::MainMenu => {
                // Return to main menu
                self.reset();
                self.screen = GameScreen::MainMenu;
                self.menu = Some(Menu::main_menu(self.viewport_width, self.viewport_height));
            }

            MenuAction::Exit => {
                log::info!("Exit requested");
                // TODO: Signal to main loop to exit
            }

            _ => {}
        }
    }

    /// Handle menu navigation key press
    pub fn handle_menu_key(&mut self, key: sdl2::keyboard::Keycode) {
        if let Some(ref mut menu) = self.menu {
            match key {
                sdl2::keyboard::Keycode::Up => menu.move_up(),
                sdl2::keyboard::Keycode::Down => menu.move_down(),
                sdl2::keyboard::Keycode::Return => {
                    let action = menu.get_selected_action();
                    self.handle_menu_action(action);
                }
                sdl2::keyboard::Keycode::Escape => {
                    // Back/Pause behavior
                    match self.screen {
                        GameScreen::MainMenu => {
                            self.handle_menu_action(MenuAction::Exit);
                        }
                        GameScreen::RaceSetup => {
                            self.handle_menu_action(MenuAction::MainMenu);
                        }
                        GameScreen::InGame => {
                            // Pause game
                            self.screen = GameScreen::Paused;
                            self.paused = true;
                            self.menu = Some(Menu::pause_menu(
                                self.viewport_width,
                                self.viewport_height,
                            ));
                        }
                        GameScreen::Paused => {
                            self.handle_menu_action(MenuAction::Resume);
                        }
                        GameScreen::Results => {
                            self.handle_menu_action(MenuAction::MainMenu);
                        }
                    }
                }
                sdl2::keyboard::Keycode::Left => {
                    // Adjust number of opponents in race setup
                    if self.screen == GameScreen::RaceSetup && self.num_opponents > 0 {
                        self.num_opponents -= 1;
                        if let Some(ref mut menu) = self.menu {
                            menu.update_item_text(0, format!("OPPONENTS: {}", self.num_opponents));
                        }
                    }
                }
                sdl2::keyboard::Keycode::Right => {
                    // Adjust number of opponents in race setup
                    if self.screen == GameScreen::RaceSetup && self.num_opponents < 5 {
                        self.num_opponents += 1;
                        if let Some(ref mut menu) = self.menu {
                            menu.update_item_text(0, format!("OPPONENTS: {}", self.num_opponents));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Update game state
    pub fn update(&mut self, delta_time: f32) {
        // Only update game logic when in-game and not paused
        if self.screen != GameScreen::InGame || self.paused {
            return;
        }

        // Update input manager
        self.input_manager.update();

        // Get player input
        let input = self.input_manager.get_car_input();

        // Apply input to player car
        self.apply_input(&input);

        // Update AI drivers
        self.update_ai(delta_time);

        // Update physics
        self.update_physics(delta_time);

        // Update camera
        self.update_camera(delta_time);

        // Update race session
        if let Some(ref mut session) = self.race_session {
            // Build driver names (player + AI)
            let mut driver_names = vec!["Player".to_string()];
            for ai_driver in &self.ai_drivers {
                driver_names.push(ai_driver.name.clone());
            }
            session.update(delta_time, &driver_names);

            // Check if race is finished
            if session.state == crate::game::session::RaceState::Finished {
                self.screen = GameScreen::Results;
                self.menu = Some(Menu::results_menu(self.viewport_width, self.viewport_height));
                log::info!("Race finished!");
            }
        }

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

    /// Update AI drivers
    fn update_ai(&mut self, delta_time: f32) {
        // Update each AI driver and apply their inputs to their cars
        for i in 0..self.ai_drivers.len() {
            // Gather nearby car information for this AI
            let ai_position = self.ai_cars[i].body.position;
            let mut nearby_cars = Vec::new();

            // Add player car
            let distance_to_player = (ai_position - self.player_car.body.position).length();
            if distance_to_player < 100.0 {
                // Determine if player is ahead or behind
                let to_player = self.player_car.body.position - ai_position;
                let ai_forward_3d = self.ai_cars[i].body.orientation * glam::Vec3::X;
                let is_ahead = to_player.dot(ai_forward_3d) > 0.0;

                nearby_cars.push(NearbyCarInfo {
                    position: self.player_car.body.position,
                    velocity: self.player_car.body.velocity,
                    distance: distance_to_player,
                    is_ahead,
                });
            }

            // Add other AI cars
            for (j, other_car) in self.ai_cars.iter().enumerate() {
                if i == j {
                    continue; // Skip self
                }

                let distance = (ai_position - other_car.body.position).length();
                if distance < 100.0 {
                    // Determine if other car is ahead or behind
                    let to_other = other_car.body.position - ai_position;
                    let ai_forward_3d = self.ai_cars[i].body.orientation * glam::Vec3::X;
                    let is_ahead = to_other.dot(ai_forward_3d) > 0.0;

                    nearby_cars.push(NearbyCarInfo {
                        position: other_car.body.position,
                        velocity: other_car.body.velocity,
                        distance,
                        is_ahead,
                    });
                }
            }

            // Get AI input
            let ai_input = self.ai_drivers[i].update(&self.ai_cars[i], &nearby_cars, delta_time);

            // Apply AI input to car
            self.ai_cars[i].set_throttle(ai_input.throttle);
            self.ai_cars[i].set_brake(ai_input.brake);
            self.ai_cars[i].set_steering(ai_input.steering);

            if ai_input.shift_up {
                self.ai_cars[i].shift_up();
            }
            if ai_input.shift_down {
                self.ai_cars[i].shift_down();
            }

            // Update AI car physics
            self.ai_cars[i].update(delta_time);

            // Apply collision detection for AI cars
            if let Some(collision_detector) = &self.track_collision {
                let collision_result = collision_detector.check_collision(self.ai_cars[i].body.position);
                self.ai_cars[i].apply_surface_grip(collision_result.grip_multiplier);
                self.ai_cars[i].on_track = collision_result.on_track;

                // Check for lap crossing (AI driver index = i + 1, since player is 0)
                if collision_detector.check_lap_crossing(self.ai_prev_sections[i], collision_result.nearest_section) {
                    // Notify race session
                    if let Some(ref mut session) = self.race_session {
                        session.complete_lap(i + 1);
                    }
                }

                self.ai_prev_sections[i] = collision_result.nearest_section;
            }
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

                    // Notify race session (player is driver index 0)
                    if let Some(ref mut session) = self.race_session {
                        session.complete_lap(0);
                    }
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

        // Render based on current screen
        match self.screen {
            GameScreen::MainMenu | GameScreen::RaceSetup => {
                // Render menu only
                if let Some(ref menu) = self.menu {
                    menu.render(renderer)?;
                }
            }

            GameScreen::InGame | GameScreen::Paused => {
                // Render game world
                // Render track if loaded
                if let Some(track_renderer) = &self.track_renderer {
                    track_renderer.render(renderer, &self.camera)?;
                }

                // Collect all cars for depth-sorted rendering (important for isometric view)
                let mut all_cars: Vec<CarState> = Vec::with_capacity(1 + self.ai_cars.len());

                // Add player car
                all_cars.push(CarState {
                    position: self.player_car.body.position,
                    rotation: self.get_car_rotation(self.player_car.body.orientation),
                    velocity: self.player_car.body.velocity.truncate(),
                    spec: self.player_car.spec.clone(),
                    driver_name: "Player".to_string(),
                });

                // Add AI opponent cars
                for (ai_car, ai_driver) in self.ai_cars.iter().zip(self.ai_drivers.iter()) {
                    all_cars.push(CarState {
                        position: ai_car.body.position,
                        rotation: self.get_car_rotation(ai_car.body.orientation),
                        velocity: ai_car.body.velocity.truncate(),
                        spec: ai_car.spec.clone(),
                        driver_name: ai_driver.name.clone(),
                    });
                }

                // Sort cars by Y coordinate for proper depth rendering in isometric view
                // Cars with lower Y (further "back" in isometric) are drawn first
                if self.camera.is_isometric() {
                    all_cars.sort_by(|a, b| {
                        a.position.y.partial_cmp(&b.position.y).unwrap_or(std::cmp::Ordering::Equal)
                    });
                }

                // Render all cars in sorted order
                for car_state in &all_cars {
                    self.car_renderer.render_car(renderer, car_state, &self.camera)?;
                }

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

                // Render pause menu overlay
                if self.screen == GameScreen::Paused {
                    if let Some(ref menu) = self.menu {
                        menu.render(renderer)?;
                    }
                }
            }

            GameScreen::Results => {
                // Render race results
                self.render_results(renderer)?;

                // Render results menu
                if let Some(ref menu) = self.menu {
                    menu.render(renderer)?;
                }
            }
        }

        Ok(())
    }

    /// Render race results screen
    fn render_results(&self, renderer: &mut impl Renderer) -> Result<()> {
        if let Some(ref session) = self.race_session {
            let center_x = self.viewport_width as f32 / 2.0;
            let mut y = 100.0;

            // Draw title
            renderer.draw_text(
                "RACE RESULTS",
                Vec2::new(center_x - 120.0, y),
                32.0,
                Color::WHITE,
            )?;
            y += 80.0;

            // Draw results
            for result in &session.results {
                let position_text = format!("{}. {}", result.position, result.name);
                let time_text = if result.finished {
                    format!("{:.2}s", result.race_time)
                } else {
                    "DNF".to_string()
                };
                let best_lap_text = result.best_lap
                    .map(|t| format!("Best: {:.2}s", t))
                    .unwrap_or_else(|| "".to_string());

                renderer.draw_text(
                    &position_text,
                    Vec2::new(center_x - 200.0, y),
                    20.0,
                    Color::WHITE,
                )?;
                renderer.draw_text(
                    &time_text,
                    Vec2::new(center_x + 50.0, y),
                    20.0,
                    Color::WHITE,
                )?;
                renderer.draw_text(
                    &best_lap_text,
                    Vec2::new(center_x + 150.0, y),
                    16.0,
                    Color::rgba(200, 200, 200, 255),
                )?;

                y += 35.0;
            }
        }

        Ok(())
    }

    /// Get car rotation angle from quaternion
    fn get_car_rotation(&self, orientation: glam::Quat) -> f32 {
        // Extract yaw from quaternion
        let q = orientation;
        let yaw = (2.0 * (q.w * q.z + q.x * q.y)).atan2(1.0 - 2.0 * (q.y * q.y + q.z * q.z));
        yaw
    }

    /// Handle keyboard input
    pub fn handle_key_down(&mut self, keycode: sdl2::keyboard::Keycode) {
        use sdl2::keyboard::Keycode;

        // Route to menu if menu is active
        if self.menu.is_some() {
            self.handle_menu_key(keycode);
            return;
        }

        // In-game controls
        match keycode {
            Keycode::P | Keycode::Escape => self.toggle_pause(),
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

    /// Get player car reference
    pub fn player_car(&self) -> &CarPhysics {
        &self.player_car
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

    /// Set 3D renderer
    pub fn set_renderer_3d(&mut self, renderer_3d: Renderer3D) {
        self.renderer_3d = Some(renderer_3d);
    }

    /// Get mutable reference to 3D renderer
    pub fn renderer_3d_mut(&mut self) -> Option<&mut Renderer3D> {
        self.renderer_3d.as_mut()
    }

    /// Get reference to 3D renderer
    pub fn renderer_3d(&self) -> Option<&Renderer3D> {
        self.renderer_3d.as_ref()
    }

    /// Update 3D camera to follow player car
    pub fn update_3d_camera(&mut self, delta_time: f32) {
        if let Some(renderer_3d) = &mut self.renderer_3d {
            // Update camera from car
            renderer_3d.camera.update_from_car(&self.player_car, delta_time);
        }
    }

    /// Get AI cars for 3D rendering
    pub fn ai_cars(&self) -> &[CarPhysics] {
        &self.ai_cars
    }

    /// Get current track for 3D rendering
    pub fn track(&self) -> Option<&Track> {
        self.track.as_ref()
    }

    /// Get player car RPM (for audio)
    pub fn get_player_rpm(&self) -> f32 {
        self.player_car.engine_rpm
    }

    /// Get player car gear (for audio)
    pub fn get_player_gear(&self) -> i32 {
        self.player_car.gear as i32
    }

    /// Get tire squeal intensity (0.0 to 1.0) based on car sliding
    pub fn get_tire_squeal_intensity(&self) -> f32 {
        // Calculate sliding by comparing velocity direction with car heading
        let velocity = self.player_car.body.velocity;
        let speed = velocity.length();

        // No squeal if moving very slow
        if speed < 5.0 {
            return 0.0;
        }

        // Get car's forward direction
        let forward = self.player_car.body.orientation * glam::Vec3::NEG_Z;

        // Get velocity direction
        let velocity_dir = velocity.normalize();

        // Calculate angle between forward and velocity (lateral slip)
        let dot = forward.dot(velocity_dir);
        let angle = dot.acos().abs();

        // Normalize angle to 0-1 range (0-90 degrees)
        // Max squeal at 45 degrees
        let normalized_angle = (angle / (std::f32::consts::PI / 4.0)).clamp(0.0, 2.0);
        let slip_factor = if normalized_angle <= 1.0 {
            normalized_angle
        } else {
            2.0 - normalized_angle
        };

        // Scale by speed (more squeal at higher speeds)
        let speed_factor = (speed / 50.0).clamp(0.0, 1.0);

        slip_factor * speed_factor
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
