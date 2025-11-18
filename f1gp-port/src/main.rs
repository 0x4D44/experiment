//! F1GP Modern Port - Main Executable
//!
//! A modern reimplementation of Formula 1 Grand Prix (1991) by Geoff Crammond

use anyhow::Result;
use f1gp_port::audio::SoundEngine;
use f1gp_port::data::Track;
use f1gp_port::game::GameState;
use f1gp_port::platform::{Color, Renderer, SdlRenderer};
use f1gp_port::parse_track;
use glam::Vec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs;
use std::time::Instant;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const TARGET_FPS: u64 = 60;
const FRAME_TIME: std::time::Duration = std::time::Duration::from_micros(1_000_000 / TARGET_FPS);

/// Track metadata
struct TrackInfo {
    filename: &'static str,
    name: &'static str,
}

/// All 15 working F1GP tracks
const TRACKS: &[TrackInfo] = &[
    TrackInfo { filename: "F1CT01.DAT", name: "Phoenix" },
    TrackInfo { filename: "F1CT02.DAT", name: "Interlagos" },
    TrackInfo { filename: "F1CT03.DAT", name: "Imola" },
    TrackInfo { filename: "F1CT04.DAT", name: "Monaco" },
    // Skip F1CT05 (Montreal - parser issue)
    TrackInfo { filename: "F1CT06.DAT", name: "Mexico" },
    TrackInfo { filename: "F1CT07.DAT", name: "Magny-Cours" },
    TrackInfo { filename: "F1CT08.DAT", name: "Silverstone" },
    TrackInfo { filename: "F1CT09.DAT", name: "Hockenheim" },
    TrackInfo { filename: "F1CT10.DAT", name: "Hungaroring" },
    TrackInfo { filename: "F1CT11.DAT", name: "Spa-Francorchamps" },
    TrackInfo { filename: "F1CT12.DAT", name: "Monza" },
    TrackInfo { filename: "F1CT13.DAT", name: "Estoril" },
    TrackInfo { filename: "F1CT14.DAT", name: "Barcelona" },
    TrackInfo { filename: "F1CT15.DAT", name: "Suzuka" },
    TrackInfo { filename: "F1CT16.DAT", name: "Adelaide" },
];

/// Game screen state
enum Screen {
    MainMenu,
    TrackSelect,
    Racing,
    Paused,
}

/// Main application state
struct App {
    screen: Screen,
    selected_track: usize,
    menu_selection: usize,
    load_track_request: Option<usize>,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::MainMenu,
            selected_track: 3, // Default to Monaco
            menu_selection: 0,
            load_track_request: None,
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    // Print banner
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║          F1GP MODERN PORT - v0.5 Preview                ║");
    println!("║     Classic Formula 1 Grand Prix Reimplementation       ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    log::info!("F1GP Modern Port v0.5 Starting");
    log::info!("Window: {}x{} | Target: {} FPS", WINDOW_WIDTH, WINDOW_HEIGHT, TARGET_FPS);
    log::info!("");
    log::info!("Controls:");
    log::info!("  Menu: ↑/↓ Navigate, ENTER Select, ESC Back");
    log::info!("  Game: Arrow Keys/WASD Drive, Z/X Shift, P Pause, R Reset");
    log::info!("");

    // Initialize SDL2 context
    let sdl_context = sdl2::init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize SDL2: {}", e))?;

    // Initialize SDL2 renderer
    let mut renderer = SdlRenderer::new("F1GP Modern Port", WINDOW_WIDTH, WINDOW_HEIGHT)?;
    log::info!("SDL2 initialized");

    // Initialize audio subsystem
    let audio_subsystem = sdl_context.audio()
        .map_err(|e| anyhow::anyhow!("Failed to initialize audio subsystem: {}", e))?;
    let sound_engine = match SoundEngine::new(&audio_subsystem) {
        Ok(engine) => {
            log::info!("Audio system initialized");
            Some(engine)
        }
        Err(e) => {
            log::warn!("Failed to initialize audio: {}", e);
            log::warn!("Continuing without sound");
            None
        }
    };

    // Create application state
    let mut app = App::new();
    let mut game: Option<GameState> = None;
    let mut last_gear = 1;

    // Main loop
    let start_time = Instant::now();
    let mut last_frame = Instant::now();
    let mut frame_count = 0u64;
    let mut last_fps_print = Instant::now();

    log::info!("Entering main loop");

    'main: loop {
        let frame_start = Instant::now();
        let delta_time = last_frame.elapsed().as_secs_f32();
        last_frame = frame_start;

        // Handle events
        for event in renderer.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    // Global audio mute toggle
                    if keycode == Keycode::M {
                        if let Some(ref audio) = sound_engine {
                            audio.toggle_mute();
                        }
                    }
                    // Global quit
                    else if keycode == Keycode::Escape {
                        match app.screen {
                            Screen::MainMenu => break 'main,
                            Screen::TrackSelect => app.screen = Screen::MainMenu,
                            Screen::Racing => app.screen = Screen::Paused,
                            Screen::Paused => app.screen = Screen::Racing,
                        }
                    } else {
                        // Handle screen-specific input
                        match &mut app.screen {
                            Screen::MainMenu => handle_main_menu(&mut app, keycode, sound_engine.as_ref()),
                            Screen::TrackSelect => handle_track_select(&mut app, keycode, sound_engine.as_ref()),
                            Screen::Racing => {
                                if let Some(ref mut g) = game {
                                    if keycode == Keycode::P {
                                        app.screen = Screen::Paused;
                                    } else {
                                        g.handle_key_down(keycode);
                                    }
                                }
                            }
                            Screen::Paused => {
                                if keycode == Keycode::P {
                                    app.screen = Screen::Racing;
                                }
                            }
                        }
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Screen::Racing = app.screen {
                        if let Some(ref mut g) = game {
                            g.handle_key_up(keycode);
                        }
                    }
                }
                _ => {}
            }
        }

        // Handle track loading request
        if let Some(track_idx) = app.load_track_request.take() {
            match load_track(track_idx) {
                Ok(track) => {
                    let mut new_game = GameState::new(WINDOW_WIDTH, WINDOW_HEIGHT);
                    new_game.load_track(track);
                    new_game.set_camera_zoom(0.5);

                    // Spawn AI opponents
                    new_game.spawn_ai_opponents(3);
                    new_game.start_race();

                    game = Some(new_game);
                    app.screen = Screen::Racing;
                    log::info!("Race started!");
                }
                Err(e) => {
                    log::error!("Failed to load track: {}", e);
                    app.screen = Screen::TrackSelect;
                }
            }
        }

        // Update game if racing
        if let Screen::Racing = app.screen {
            if let Some(ref mut g) = game {
                g.update(delta_time);

                // Update audio with engine RPM
                if let Some(ref audio) = sound_engine {
                    let rpm = g.get_player_rpm();
                    audio.set_rpm(rpm);

                    // Detect gear shifts
                    let current_gear = g.get_player_gear();
                    if current_gear != last_gear {
                        audio.play_gear_shift();
                        last_gear = current_gear;
                    }

                    // Update tire squeal based on sliding
                    let squeal_intensity = g.get_tire_squeal_intensity();
                    audio.set_tire_squeal(squeal_intensity);
                }
            }
        }

        // Render current screen
        renderer.clear(Color::BLACK);

        match &app.screen {
            Screen::MainMenu => render_main_menu(&mut renderer, &app)?,
            Screen::TrackSelect => render_track_select(&mut renderer, &app)?,
            Screen::Racing => {
                if let Some(ref mut g) = game {
                    g.render(&mut renderer)?;
                }
            }
            Screen::Paused => {
                // Render game background
                if let Some(ref mut g) = game {
                    g.render(&mut renderer)?;
                }
                // Render pause overlay
                render_pause_overlay(&mut renderer)?;
            }
        }

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

            if let Some(ref g) = game {
                if let Screen::Racing = app.screen {
                    log::info!(
                        "Speed: {:.1} km/h | Gear: {} | RPM: {:.0} | FPS: {:.1}",
                        g.get_speed_kmh(),
                        g.get_gear(),
                        g.get_rpm(),
                        fps
                    );
                }
            }

            last_fps_print = Instant::now();
        }
    }

    let total_time = start_time.elapsed();
    let avg_fps = frame_count as f64 / total_time.as_secs_f64();
    log::info!("Shutting down");
    log::info!("Total frames: {}, Average FPS: {:.2}", frame_count, avg_fps);

    Ok(())
}

/// Handle main menu input
fn handle_main_menu(app: &mut App, keycode: Keycode, audio: Option<&SoundEngine>) {
    match keycode {
        Keycode::Up => {
            if app.menu_selection > 0 {
                app.menu_selection -= 1;
                if let Some(audio) = audio {
                    audio.play_menu_beep();
                }
            }
        }
        Keycode::Down => {
            if app.menu_selection < 2 {
                app.menu_selection += 1;
                if let Some(audio) = audio {
                    audio.play_menu_beep();
                }
            }
        }
        Keycode::Return => {
            if let Some(audio) = audio {
                audio.play_menu_beep();
            }
            match app.menu_selection {
                0 => app.screen = Screen::TrackSelect, // Start Race
                1 => {}, // Options (not implemented yet)
                2 => std::process::exit(0), // Quit
                _ => {}
            }
        }
        _ => {}
    }
}

/// Handle track selection input
fn handle_track_select(app: &mut App, keycode: Keycode, audio: Option<&SoundEngine>) {
    match keycode {
        Keycode::Up => {
            if app.selected_track > 0 {
                app.selected_track -= 1;
                if let Some(audio) = audio {
                    audio.play_menu_beep();
                }
            }
        }
        Keycode::Down => {
            if app.selected_track < TRACKS.len() - 1 {
                app.selected_track += 1;
                if let Some(audio) = audio {
                    audio.play_menu_beep();
                }
            }
        }
        Keycode::Return => {
            // Request track load
            if let Some(audio) = audio {
                audio.play_menu_beep();
            }
            log::info!("Loading track: {}", TRACKS[app.selected_track].name);
            app.load_track_request = Some(app.selected_track);
        }
        _ => {}
    }
}

/// Render main menu
fn render_main_menu(renderer: &mut SdlRenderer, app: &App) -> Result<()> {
    // Title
    renderer.draw_text("F1GP MODERN PORT", Vec2::new(450.0, 100.0), 3.0, Color::WHITE)?;
    renderer.draw_text("v0.5 Preview", Vec2::new(540.0, 140.0), 1.5, Color::LIGHT_GRAY)?;

    // Menu options
    let menu_items = ["Start Race", "Options", "Quit"];
    for (i, item) in menu_items.iter().enumerate() {
        let color = if i == app.menu_selection {
            Color::YELLOW
        } else {
            Color::LIGHT_GRAY
        };

        let prefix = if i == app.menu_selection { "> " } else { "  " };
        renderer.draw_text(
            &format!("{}{}", prefix, item),
            Vec2::new(500.0, 300.0 + (i as f32 * 50.0)),
            2.0,
            color,
        )?;
    }

    // Instructions
    renderer.draw_text("↑/↓: Navigate  ENTER: Select  ESC: Quit",
                      Vec2::new(400.0, 600.0), 1.0, Color::GRAY)?;

    Ok(())
}

/// Render track selection menu
fn render_track_select(renderer: &mut SdlRenderer, app: &App) -> Result<()> {
    // Title
    renderer.draw_text("SELECT TRACK", Vec2::new(480.0, 50.0), 2.5, Color::WHITE)?;

    // Track list (show 10 at a time, scroll if needed)
    let start_idx = if app.selected_track > 5 { app.selected_track - 5 } else { 0 };
    let end_idx = (start_idx + 10).min(TRACKS.len());

    for i in start_idx..end_idx {
        let track = &TRACKS[i];
        let color = if i == app.selected_track {
            Color::YELLOW
        } else {
            Color::LIGHT_GRAY
        };

        let prefix = if i == app.selected_track { "> " } else { "  " };
        renderer.draw_text(
            &format!("{}{:2}. {}", prefix, i + 1, track.name),
            Vec2::new(400.0, 120.0 + ((i - start_idx) as f32 * 40.0)),
            1.5,
            color,
        )?;
    }

    // Instructions
    renderer.draw_text("↑/↓: Navigate  ENTER: Start  ESC: Back",
                      Vec2::new(350.0, 650.0), 1.0, Color::GRAY)?;

    Ok(())
}

/// Render pause overlay
fn render_pause_overlay(renderer: &mut SdlRenderer) -> Result<()> {
    // Semi-transparent overlay (we'll just draw text for now)
    renderer.draw_text("PAUSED", Vec2::new(560.0, 300.0), 3.0, Color::YELLOW)?;
    renderer.draw_text("Press P to Resume", Vec2::new(500.0, 360.0), 1.5, Color::LIGHT_GRAY)?;
    renderer.draw_text("Press ESC for Menu", Vec2::new(490.0, 400.0), 1.5, Color::LIGHT_GRAY)?;

    Ok(())
}

/// Load a track from .DAT file
fn load_track(track_index: usize) -> Result<Track> {
    let track_info = &TRACKS[track_index];
    let path = format!("assets/original/{}", track_info.filename);

    log::info!("Loading track: {} from {}", track_info.name, path);

    let data = fs::read(&path)?;
    let track = parse_track(data, track_info.name.to_string())?;

    log::info!("Track loaded: {} sections, {:.2}km",
               track.sections.len(), track.length / 1000.0);

    Ok(track)
}
