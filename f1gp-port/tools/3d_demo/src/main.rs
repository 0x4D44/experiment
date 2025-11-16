//! F1GP 3D Demo
//!
//! A demo of the F1GP Modern Port with 3D rendering using wgpu.

use anyhow::Result;
use f1gp_port::data::track::{Track, TrackSection, RacingLine, AIBehavior, SurfaceType};
use f1gp_port::game::GameState;
use f1gp_port::render3d::Renderer3D;
use glam::Vec3;
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
    game: Option<GameState>,
    last_frame: Instant,
    frame_count: u64,
    last_fps_print: Instant,
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
            game: None,
            last_frame: Instant::now(),
            frame_count: 0,
            last_fps_print: Instant::now(),
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
        })).unwrap();

        log::info!("Adapter: {:?}", adapter.get_info());

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor::default(),
        ))?;

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

        // Create game state
        let mut game = GameState::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        log::info!("Game state created");

        // Create a test track
        let test_track = create_test_track();
        game.load_track(test_track);
        log::info!("Test track loaded");

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
        self.game = Some(game);

        Ok(())
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

        // Submit commands and present
        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // Print enhanced stats every second
        self.frame_count += 1;
        if self.last_fps_print.elapsed().as_secs() >= 1 {
            let fps = self.frame_count as f64 / self.last_fps_print.elapsed().as_secs_f64();
            let player_car = game.player_car();
            let on_track = if player_car.on_track { "✓" } else { "✗" };
            let camera_mode = self
                .renderer_3d
                .as_ref()
                .map(|r| format!("{:?}", r.camera.mode))
                .unwrap_or_else(|| "Unknown".to_string());

            log::info!(
                "FPS: {:.1} | Speed: {:.0} km/h | Gear: {} | RPM: {:.0} | Track: {} | Camera: {} | AI: {}",
                fps,
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
                if let Some(game) = &mut self.game {
                    if let Some(sdl_key) = winit_to_sdl_keycode(key) {
                        game.handle_key_up(sdl_key);
                    }
                }
            }

            WindowEvent::Resized(physical_size) => {
                if let (Some(config), Some(surface), Some(device), Some(renderer_3d), Some(game)) = (
                    &mut self.config,
                    &self.surface,
                    &self.device,
                    &mut self.renderer_3d,
                    &mut self.game,
                ) {
                    config.width = physical_size.width;
                    config.height = physical_size.height;
                    surface.configure(device, config);

                    // Update game viewport
                    game.set_viewport_size(physical_size.width, physical_size.height);

                    // Resize renderer depth buffer
                    renderer_3d.resize(device, physical_size.width, physical_size.height);
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
}

fn main() -> Result<()> {
    env_logger::init();

    log::info!("═══════════════════════════════════════════");
    log::info!("  F1GP 3D Demo - Enhanced Edition");
    log::info!("═══════════════════════════════════════════");
    log::info!("Window: {}x{} | Target: 60 FPS", WINDOW_WIDTH, WINDOW_HEIGHT);
    log::info!("");
    log::info!("Features:");
    log::info!("  ✓ 3D Rendering (wgpu 27.0)");
    log::info!("  ✓ Real-time Physics");
    log::info!("  ✓ AI Opponents (3 cars)");
    log::info!("  ✓ Multiple Camera Modes");
    log::info!("  ✓ Performance Metrics");
    log::info!("");
    log::info!("Controls:");
    log::info!("  Arrow Keys / WASD - Steer, Throttle, Brake");
    log::info!("  Z / X             - Shift Down / Up");
    log::info!("  C                 - Cycle Camera Mode");
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

/// Create a simple test track for the demo
fn create_test_track() -> Track {
    // Create a circular test track with some elevation and banking
    let mut sections = Vec::new();
    let radius = 500.0;
    let num_segments = 64;

    for i in 0..num_segments {
        let angle = (i as f32) * 2.0 * std::f32::consts::PI / (num_segments as f32);
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        // Add some elevation variation (sine wave)
        let elevation = (angle * 2.0).sin() * 10.0;

        // Add banking in corners (higher at 90, 180, 270 degrees)
        let banking_angle = (angle * 4.0).sin().abs() * 0.3; // Up to ~17 degrees

        sections.push(TrackSection {
            position: Vec3::new(x, elevation, z),
            width: 15.0,
            banking: banking_angle,
            elevation,
            surface: SurfaceType::Track,
            length: 2.0 * std::f32::consts::PI * radius / (num_segments as f32),
        });
    }

    Track {
        name: "Test Circuit".to_string(),
        length: 2.0 * std::f32::consts::PI * radius,
        object_shapes: vec![],
        sections,
        racing_line: RacingLine { points: vec![] },
        ai_behavior: AIBehavior::default(),
        pit_lane: vec![],
        cameras: vec![],
        checksum: 0,
    }
}
