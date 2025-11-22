//! F1GP Playable Demo
//!
//! A playable demo of the F1GP Modern Port with physics, rendering, and input.

use anyhow::Result;
use f1gp_port::data::track::{AIBehavior, RacingLine, SurfaceType, Track, TrackSection};
use f1gp_port::game::GameState;
use f1gp_port::platform::{Renderer, SdlRenderer};
use glam::Vec3;
use sdl2::event::Event;
use std::time::Instant;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const TARGET_FPS: u64 = 60;
const FRAME_TIME: std::time::Duration = std::time::Duration::from_micros(1_000_000 / TARGET_FPS);

fn main() -> Result<()> {
    env_logger::init();

    log::info!("F1GP Playable Demo Starting");
    log::info!("Window size: {}x{}", WINDOW_WIDTH, WINDOW_HEIGHT);
    log::info!("Target FPS: {}", TARGET_FPS);
    log::info!("");
    log::info!("Controls:");
    log::info!("  Arrow Keys / WASD - Steer, Throttle, Brake");
    log::info!("  Z - Shift Down");
    log::info!("  X - Shift Up");
    log::info!("  P - Pause");
    log::info!("  R - Reset");
    log::info!("  ESC - Quit");
    log::info!("");

    // Create renderer
    let mut renderer = SdlRenderer::new("F1GP Playable Demo", WINDOW_WIDTH, WINDOW_HEIGHT)?;
    log::info!("SDL2 renderer initialized");

    // Create game state
    let mut game = GameState::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    log::info!("Game state created");

    // Create a simple test track (since we don't have track files loaded yet)
    let test_track = create_test_track();
    game.load_track(test_track);
    log::info!("Test track loaded");

    // Set camera zoom to see more of the track
    game.set_camera_zoom(0.5);

    // Main loop
    let start_time = Instant::now();
    let mut last_frame = Instant::now();
    let mut frame_count = 0u64;
    let mut last_fps_print = Instant::now();

    log::info!("Entering main loop");

    while renderer.poll_events() {
        let frame_start = Instant::now();
        let delta_time = last_frame.elapsed().as_secs_f32();
        last_frame = frame_start;

        // Handle events
        for event in renderer.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(()),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == sdl2::keyboard::Keycode::Escape {
                        return Ok(());
                    }
                    game.handle_key_down(keycode);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    game.handle_key_up(keycode);
                }
                _ => {}
            }
        }

        // Update game
        game.update(delta_time);

        // Render
        game.render(&mut renderer)?;

        // Present
        renderer.present();

        // Frame timing
        frame_count += 1;
        let frame_elapsed = frame_start.elapsed();
        if frame_elapsed < FRAME_TIME {
            renderer.delay(FRAME_TIME - frame_elapsed);
        }

        // Print FPS every second
        if last_fps_print.elapsed().as_secs() >= 1 {
            let fps = frame_count as f64 / start_time.elapsed().as_secs_f64();
            log::info!(
                "Speed: {:.1} km/h | Gear: {} | RPM: {:.0} | FPS: {:.1}",
                game.get_speed_kmh(),
                game.get_gear(),
                game.get_rpm(),
                fps
            );
            last_fps_print = Instant::now();
        }
    }

    let total_time = start_time.elapsed();
    let avg_fps = frame_count as f64 / total_time.as_secs_f64();
    log::info!("Shutting down");
    log::info!("Total frames: {}, Average FPS: {:.2}", frame_count, avg_fps);

    Ok(())
}

/// Create a simple test track for the demo
fn create_test_track() -> Track {
    // Create a circular test track
    let mut sections = Vec::new();
    let radius = 500.0;
    let num_segments = 32;

    for i in 0..num_segments {
        let angle = (i as f32) * 2.0 * std::f32::consts::PI / (num_segments as f32);
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        sections.push(TrackSection {
            position: Vec3::new(x, 0.0, z),
            width: 15.0,
            banking: 0.0,
            elevation: 0.0,
            surface: SurfaceType::Track,
            length: 2.0 * std::f32::consts::PI * radius / (num_segments as f32),
        });
    }

    Track {
        name: "Test Track".to_string(),
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
