//! F1GP 3D Demo
//!
//! A demo of the F1GP Modern Port with 3D rendering using wgpu.

mod track_loader;

use anyhow::Result;
use f1gp_port::game::GameState;
use f1gp_port::render3d::{HudRenderer, Renderer3D};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

struct App {
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    config: Option<wgpu::SurfaceConfiguration>,
    renderer_3d: Option<Renderer3D>,
    hud: Option<HudRenderer>,
    game: Option<GameState>,
    last_frame: Instant,
    frame_count: u64,
    last_fps_print: Instant,
    fps: f64,
    pressed_keys: HashSet<KeyCode>,
    current_track_index: usize,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            surface: None,
            device: None,
            queue: None,
            config: None,
            renderer_3d: None,
            hud: None,
            game: None,
            last_frame: Instant::now(),
            frame_count: 0,
            last_fps_print: Instant::now(),
            fps: 0.0,
            pressed_keys: HashSet::new(),
            current_track_index: 0,
        }
    }

    fn init_graphics(&mut self) -> Result<()> {
        let window = self.window.as_ref().unwrap();

        // Initialize wgpu
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone())?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        log::info!("Adapter: {:?}", adapter.get_info());

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))?;

        log::info!("Device and queue created");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);
        log::info!("Surface configured");

        // Create 3D renderer
        let renderer_3d = Renderer3D::new(&device, &config)?;
        log::info!("3D renderer created");

        // Create HUD renderer
        let hud = HudRenderer::new(&device, &queue, &config)?;
        log::info!("HUD renderer created");

        // Create game state
        let mut game = GameState::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        log::info!("Game state created");

        // Load initial track
        let track = track_loader::get_track(self.current_track_index).unwrap();
        log::info!("Loading track: {}", track.name);
        game.load_track(track);
        log::info!("Track loaded");

        // Load track mesh into renderer
        if let Some(track) = game.track() {
            let mut renderer_for_loading = renderer_3d;
            renderer_for_loading.load_track(&device, track);
            log::info!("Track mesh loaded into renderer");
            self.renderer_3d = Some(renderer_for_loading);
        } else {
            self.renderer_3d = Some(renderer_3d);
        }

        // Spawn some AI opponents
        game.spawn_ai_opponents(3);
        game.start_race();
        log::info!("AI opponents spawned");

        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);
        self.hud = Some(hud);
        self.game = Some(game);

        Ok(())
    }

    fn switch_track(&mut self, track_index: usize) {
        if track_index >= track_loader::get_track_count() {
            log::warn!("Invalid track index: {}", track_index);
            return;
        }

        self.current_track_index = track_index;

        if let (Some(game), Some(device), Some(renderer_3d)) =
            (&mut self.game, &self.device, &mut self.renderer_3d)
        {
            // Load new track
            if let Some(track) = track_loader::get_track(track_index) {
                log::info!("Switching to track: {}", track.name);
                game.load_track(track);

                // Reload track mesh into renderer
                if let Some(track) = game.track() {
                    renderer_3d.load_track(device, track);
                    log::info!("Track mesh reloaded");
                }

                // Reset game
                game.reset();
            }
        }
    }

    fn render(&mut self) -> Result<()> {
        let game = self.game.as_mut().unwrap();
        let renderer_3d = self.renderer_3d.as_mut().unwrap();
        let device = self.device.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();
        let surface = self.surface.as_ref().unwrap();

        // Calculate delta time
        let delta_time = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();

        // Handle free camera movement with keyboard
        use f1gp_port::render3d::CameraMode;
        if renderer_3d.camera.mode == CameraMode::Free {
            let mut forward = 0.0f32;
            let mut right = 0.0f32;
            let mut up = 0.0f32;

            // WASD for movement
            if self.pressed_keys.contains(&KeyCode::KeyW)
                || self.pressed_keys.contains(&KeyCode::ArrowUp)
            {
                forward += 1.0;
            }
            if self.pressed_keys.contains(&KeyCode::KeyS)
                || self.pressed_keys.contains(&KeyCode::ArrowDown)
            {
                forward -= 1.0;
            }
            if self.pressed_keys.contains(&KeyCode::KeyA)
                || self.pressed_keys.contains(&KeyCode::ArrowLeft)
            {
                right -= 1.0;
            }
            if self.pressed_keys.contains(&KeyCode::KeyD)
                || self.pressed_keys.contains(&KeyCode::ArrowRight)
            {
                right += 1.0;
            }
            if self.pressed_keys.contains(&KeyCode::Space) {
                up += 1.0;
            }
            if self.pressed_keys.contains(&KeyCode::ControlLeft)
                || self.pressed_keys.contains(&KeyCode::ControlRight)
            {
                up -= 1.0;
            }

            // Apply speed modifier for Shift
            let speed_mult = if self.pressed_keys.contains(&KeyCode::ShiftLeft)
                || self.pressed_keys.contains(&KeyCode::ShiftRight)
            {
                3.0
            } else {
                1.0
            };

            // Move camera
            renderer_3d.camera.move_free_camera(
                forward * speed_mult,
                right * speed_mult,
                up * speed_mult,
                delta_time,
            );
        }

        // Update game
        game.update(delta_time);

        // Update 3D camera
        game.update_3d_camera(delta_time);

        // Update renderer from game state
        renderer_3d.update(game, queue);

        // Get surface texture
        let output = surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Render 3D scene (skybox + track)
        renderer_3d.render(&mut encoder, &view)?;

        // Render cars
        renderer_3d.render_cars(device, &mut encoder, &view, game, queue)?;

        // Render HUD overlay
        if let Some(hud) = &mut self.hud {
            let player_car = game.player_car();
            let on_track_icon = if player_car.on_track { "ON" } else { "OFF" };
            let camera_mode = self
                .renderer_3d
                .as_ref()
                .map(|r| format!("{:?}", r.camera.mode))
                .unwrap_or_else(|| "Unknown".to_string());

            // Build HUD text lines (text, x, y, scale, color)
            let hud_lines = vec![
                // Top-left: Telemetry
                (
                    format!("FPS: {:.1}", self.fps),
                    10.0,
                    10.0,
                    1.5,
                    [0.0, 1.0, 0.0, 1.0], // Green
                ),
                (
                    format!("Speed: {:.0} km/h", game.get_speed_kmh()),
                    10.0,
                    34.0,
                    1.5,
                    [1.0, 1.0, 1.0, 1.0], // White
                ),
                (
                    format!("Gear: {}", game.get_gear()),
                    10.0,
                    58.0,
                    1.5,
                    [1.0, 1.0, 1.0, 1.0],
                ),
                (
                    format!("RPM: {:.0}", game.get_rpm()),
                    10.0,
                    82.0,
                    1.5,
                    [1.0, 1.0, 1.0, 1.0],
                ),
                (
                    format!("Track: {}", on_track_icon),
                    10.0,
                    106.0,
                    1.5,
                    if player_car.on_track {
                        [0.0, 1.0, 0.0, 1.0] // Green
                    } else {
                        [1.0, 0.0, 0.0, 1.0] // Red
                    },
                ),
                (
                    format!("Camera: {}", camera_mode),
                    10.0,
                    130.0,
                    1.5,
                    [0.5, 0.5, 1.0, 1.0], // Light blue
                ),
                (
                    format!("AI Opponents: {}", game.ai_cars().len()),
                    10.0,
                    154.0,
                    1.5,
                    [1.0, 1.0, 0.0, 1.0], // Yellow
                ),
                (
                    format!(
                        "Track: {}",
                        game.track().map(|t| t.name.as_str()).unwrap_or("Unknown")
                    ),
                    10.0,
                    178.0,
                    1.5,
                    [0.0, 1.0, 1.0, 1.0], // Cyan
                ),
                // Top-right: Controls
                (
                    "Controls:".to_string(),
                    920.0,
                    10.0,
                    1.5,
                    [1.0, 1.0, 1.0, 1.0],
                ),
                (
                    "Arrows/WASD: Drive".to_string(),
                    920.0,
                    34.0,
                    1.0,
                    [0.8, 0.8, 0.8, 1.0],
                ),
                (
                    "C: Camera Mode".to_string(),
                    920.0,
                    50.0,
                    1.0,
                    [0.8, 0.8, 0.8, 1.0],
                ),
                (
                    "P: Pause".to_string(),
                    920.0,
                    66.0,
                    1.0,
                    [0.8, 0.8, 0.8, 1.0],
                ),
                (
                    "R: Reset".to_string(),
                    920.0,
                    82.0,
                    1.0,
                    [0.8, 0.8, 0.8, 1.0],
                ),
                (
                    "1-5: Change Track".to_string(),
                    920.0,
                    98.0,
                    1.0,
                    [0.8, 0.8, 0.8, 1.0],
                ),
                (
                    "ESC: Exit".to_string(),
                    920.0,
                    114.0,
                    1.0,
                    [0.8, 0.8, 0.8, 1.0],
                ),
            ];

            hud.render(device, queue, &mut encoder, &view, &hud_lines);
        }

        // Submit commands and present
        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // Print enhanced stats every second
        self.frame_count += 1;
        if self.last_fps_print.elapsed().as_secs() >= 1 {
            self.fps = self.frame_count as f64 / self.last_fps_print.elapsed().as_secs_f64();
            let player_car = game.player_car();
            let on_track = if player_car.on_track { "✓" } else { "✗" };
            let camera_mode = self
                .renderer_3d
                .as_ref()
                .map(|r| format!("{:?}", r.camera.mode))
                .unwrap_or_else(|| "Unknown".to_string());

            log::info!(
                "FPS: {:.1} | Speed: {:.0} km/h | Gear: {} | RPM: {:.0} | Track: {} | Camera: {} | AI: {}",
                self.fps,
                game.get_speed_kmh(),
                game.get_gear(),
                game.get_rpm(),
                on_track,
                camera_mode,
                game.ai_cars().len()
            );
            self.last_fps_print = Instant::now();
            self.frame_count = 0;
        }

        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("F1GP 3D Demo")
                .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.window = Some(window);

            if let Err(e) = self.init_graphics() {
                log::error!("Failed to initialize graphics: {}", e);
                event_loop.exit();
            }

            log::info!("Graphics initialized");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("Close requested");
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                // Track pressed keys for continuous movement
                self.pressed_keys.insert(key);

                match key {
                    KeyCode::Escape => {
                        log::info!("Escape pressed");
                        event_loop.exit();
                    }
                    KeyCode::KeyC => {
                        // Cycle camera mode
                        if let Some(renderer_3d) = &mut self.renderer_3d {
                            renderer_3d.camera.next_mode();
                            log::info!("Camera mode: {:?}", renderer_3d.camera.mode);
                        }
                    }
                    KeyCode::KeyF => {
                        // Toggle free camera mode
                        if let Some(renderer_3d) = &mut self.renderer_3d {
                            renderer_3d.camera.toggle_free_mode();
                            log::info!("Camera mode: {:?}", renderer_3d.camera.mode);
                        }
                    }
                    KeyCode::Home => {
                        // Reset camera
                        if let Some(renderer_3d) = &mut self.renderer_3d {
                            renderer_3d.camera.reset();
                            log::info!("Camera reset to Chase mode");
                        }
                    }
                    KeyCode::KeyP => {
                        if let Some(game) = &mut self.game {
                            game.toggle_pause();
                            log::info!("Pause toggled");
                        }
                    }
                    KeyCode::KeyR => {
                        if let Some(game) = &mut self.game {
                            game.reset();
                            log::info!("Game reset");
                        }
                    }
                    // Track selection (all 16 F1GP tracks)
                    KeyCode::Digit1 => self.switch_track(0), // Phoenix
                    KeyCode::Digit2 => self.switch_track(1), // Interlagos
                    KeyCode::Digit3 => self.switch_track(2), // Imola
                    KeyCode::Digit4 => self.switch_track(3), // Monaco
                    KeyCode::Digit5 => self.switch_track(4), // Montreal
                    KeyCode::Digit6 => self.switch_track(5), // Mexico
                    KeyCode::Digit7 => self.switch_track(6), // Magny-Cours
                    KeyCode::Digit8 => self.switch_track(7), // Silverstone
                    KeyCode::Digit9 => self.switch_track(8), // Hockenheim
                    KeyCode::Digit0 => self.switch_track(9), // Hungaroring
                    KeyCode::Minus => self.switch_track(10), // Spa
                    KeyCode::Equal => self.switch_track(11), // Monza
                    // Could add more but 12 tracks is plenty for now
                    _ => {
                        // Handle game input
                        if let Some(game) = &mut self.game {
                            if let Some(sdl_key) = winit_to_sdl_keycode(key) {
                                game.handle_key_down(sdl_key);
                            }
                        }
                    }
                }
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Released,
                        ..
                    },
                ..
            } => {
                // Track released keys
                self.pressed_keys.remove(&key);

                if let Some(game) = &mut self.game {
                    if let Some(sdl_key) = winit_to_sdl_keycode(key) {
                        game.handle_key_up(sdl_key);
                    }
                }
            }

            WindowEvent::Resized(physical_size) => {
                if let (
                    Some(config),
                    Some(surface),
                    Some(device),
                    Some(renderer_3d),
                    Some(hud),
                    Some(game),
                ) = (
                    &mut self.config,
                    &self.surface,
                    &self.device,
                    &mut self.renderer_3d,
                    &mut self.hud,
                    &mut self.game,
                ) {
                    config.width = physical_size.width;
                    config.height = physical_size.height;
                    surface.configure(device, config);

                    // Update game viewport
                    game.set_viewport_size(physical_size.width, physical_size.height);

                    // Resize renderer depth buffer
                    renderer_3d.resize(device, physical_size.width, physical_size.height);

                    // Resize HUD
                    hud.resize(physical_size.width, physical_size.height);
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                // Handle mouse wheel for zoom
                if let Some(renderer_3d) = &mut self.renderer_3d {
                    let zoom_delta = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_x, y) => y,
                        winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.y / 50.0) as f32,
                    };
                    renderer_3d.camera.handle_zoom(zoom_delta);
                }
            }

            WindowEvent::RedrawRequested => {
                if let Err(e) = self.render() {
                    log::error!("Render error: {}", e);
                }

                // Request next frame
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        // Handle raw mouse motion for free camera
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            if let Some(renderer_3d) = &mut self.renderer_3d {
                renderer_3d.camera.handle_mouse_motion(delta.0, delta.1);
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    log::info!("═══════════════════════════════════════════");
    log::info!("  F1GP 3D Demo - Enhanced Edition");
    log::info!("═══════════════════════════════════════════");
    log::info!(
        "Window: {}x{} | Target: 60 FPS",
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    );
    log::info!("");
    log::info!("Features:");
    log::info!("  ✓ 3D Rendering (wgpu 27.0)");
    log::info!("  ✓ Real-time Physics");
    log::info!("  ✓ AI Opponents (3 cars)");
    log::info!("  ✓ Multiple Camera Modes");
    log::info!("  ✓ Performance Metrics");
    log::info!("");
    log::info!("Available Tracks (Real F1GP Circuits):");
    for (i, name) in track_loader::get_track_names().iter().enumerate() {
        log::info!("  {:2} - {}", i + 1, name);
    }
    log::info!("");
    log::info!("Controls:");
    log::info!("  Arrow Keys / WASD - Steer, Throttle, Brake");
    log::info!("  Z / X             - Shift Down / Up");
    log::info!("  C                 - Cycle Camera Mode");
    log::info!("  F                 - Toggle Free Camera");
    log::info!("  1-9, 0, -, =      - Select Track (16 F1GP circuits)");
    log::info!("  P                 - Pause");
    log::info!("  R                 - Reset");
    log::info!("  ESC               - Quit");
    log::info!("═══════════════════════════════════════════");
    log::info!("");

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

/// Convert winit keycode to SDL2 keycode (for game input compatibility)
fn winit_to_sdl_keycode(key: KeyCode) -> Option<sdl2::keyboard::Keycode> {
    use sdl2::keyboard::Keycode;

    match key {
        KeyCode::ArrowUp => Some(Keycode::Up),
        KeyCode::ArrowDown => Some(Keycode::Down),
        KeyCode::ArrowLeft => Some(Keycode::Left),
        KeyCode::ArrowRight => Some(Keycode::Right),
        KeyCode::KeyW => Some(Keycode::W),
        KeyCode::KeyA => Some(Keycode::A),
        KeyCode::KeyS => Some(Keycode::S),
        KeyCode::KeyD => Some(Keycode::D),
        KeyCode::KeyZ => Some(Keycode::Z),
        KeyCode::KeyX => Some(Keycode::X),
        _ => None,
    }
}
