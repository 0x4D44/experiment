//! Graphics system test application
//!
//! Tests SDL2 rendering, camera system, and basic shapes.

use anyhow::Result;
use f1gp_port::platform::{Color, Rect, Renderer, SdlRenderer};
use f1gp_port::render::Camera;
use glam::Vec2;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const TARGET_FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / TARGET_FPS);

fn main() -> Result<()> {
    env_logger::init();

    log::info!("Starting graphics test application");
    log::info!("Window size: {}x{}", WINDOW_WIDTH, WINDOW_HEIGHT);
    log::info!("Target FPS: {}", TARGET_FPS);

    // Create renderer
    let mut renderer = SdlRenderer::new("F1GP Graphics Test", WINDOW_WIDTH, WINDOW_HEIGHT)?;
    log::info!("SDL2 renderer initialized successfully");

    // Create camera
    let mut camera = Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    log::info!("Camera system initialized");

    // Animation state
    let mut frame_count = 0u64;
    let start_time = Instant::now();
    let mut last_frame = Instant::now();

    log::info!("Entering main loop (Press ESC to exit)");

    // Main loop
    while renderer.poll_events() {
        let frame_start = Instant::now();

        // Clear screen
        renderer.clear(Color::rgb(32, 32, 48));

        // Draw test shapes
        draw_test_shapes(&mut renderer, &camera, frame_count)?;

        // Draw FPS counter info
        draw_info(&mut renderer, frame_count, start_time)?;

        // Present frame
        renderer.present();

        // Frame timing
        frame_count += 1;
        let frame_elapsed = frame_start.elapsed();
        if frame_elapsed < FRAME_TIME {
            renderer.delay(FRAME_TIME - frame_elapsed);
        }

        last_frame = Instant::now();
    }

    let total_time = start_time.elapsed();
    let avg_fps = frame_count as f64 / total_time.as_secs_f64();
    log::info!("Shutting down");
    log::info!("Total frames: {}, Average FPS: {:.2}", frame_count, avg_fps);

    Ok(())
}

fn draw_test_shapes(renderer: &mut impl Renderer, _camera: &Camera, frame: u64) -> Result<()> {
    let (width, height) = renderer.viewport_size();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;

    // Animated offset
    let t = (frame as f32 * 0.02).sin();
    let offset = t * 50.0;

    // Draw rectangles
    renderer.draw_filled_rect(
        Rect::new(center_x - 200.0 + offset, 100.0, 150.0, 100.0),
        Color::RED,
    )?;

    renderer.draw_rect(
        Rect::new(center_x - 50.0, 100.0, 150.0, 100.0),
        Color::GREEN,
    )?;

    renderer.draw_filled_rect(
        Rect::new(center_x + 100.0 - offset, 100.0, 150.0, 100.0),
        Color::BLUE,
    )?;

    // Draw circles
    let circle_y = center_y;
    renderer.draw_filled_circle(
        Vec2::new(center_x - 200.0, circle_y + offset),
        50.0,
        Color::YELLOW,
    )?;

    renderer.draw_circle(Vec2::new(center_x, circle_y), 50.0, Color::CYAN)?;

    renderer.draw_filled_circle(
        Vec2::new(center_x + 200.0, circle_y - offset),
        50.0,
        Color::MAGENTA,
    )?;

    // Draw lines forming a star
    let star_center = Vec2::new(center_x, center_y + 150.0);
    let star_points = 5;
    let star_radius = 80.0;
    let rotation = (frame as f32 * 0.01) % (std::f32::consts::PI * 2.0);

    for i in 0..star_points {
        let angle1 = rotation + (i as f32 * 2.0 * std::f32::consts::PI / star_points as f32);
        let angle2 = rotation + ((i + 2) as f32 * 2.0 * std::f32::consts::PI / star_points as f32);

        let p1 = star_center + Vec2::new(angle1.cos() * star_radius, angle1.sin() * star_radius);
        let p2 = star_center + Vec2::new(angle2.cos() * star_radius, angle2.sin() * star_radius);

        renderer.draw_line(p1, p2, Color::WHITE)?;
    }

    // Draw grid
    for i in 0..10 {
        let x = (i as f32 * width as f32 / 10.0) as f32;
        renderer.draw_line(
            Vec2::new(x, 0.0),
            Vec2::new(x, height as f32),
            Color::DARK_GRAY,
        )?;
    }
    for i in 0..10 {
        let y = (i as f32 * height as f32 / 10.0) as f32;
        renderer.draw_line(
            Vec2::new(0.0, y),
            Vec2::new(width as f32, y),
            Color::DARK_GRAY,
        )?;
    }

    Ok(())
}

fn draw_info(renderer: &mut impl Renderer, frame: u64, start_time: Instant) -> Result<()> {
    let elapsed = start_time.elapsed();
    let fps = frame as f64 / elapsed.as_secs_f64();

    // Draw info box
    let info_rect = Rect::new(10.0, 10.0, 250.0, 80.0);
    renderer.draw_filled_rect(info_rect, Color::new(0, 0, 0, 200))?;
    renderer.draw_rect(info_rect, Color::WHITE)?;

    // Note: Text rendering would require SDL2_ttf
    // For now, just draw a simple indicator
    let indicator_size = 10.0;
    renderer.draw_filled_rect(
        Rect::new(20.0, 20.0, indicator_size, indicator_size),
        Color::GREEN,
    )?;

    log::debug!("Frame: {}, FPS: {:.2}", frame, fps);

    Ok(())
}
