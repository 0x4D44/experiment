mod game;
mod physics;
mod renderer;
mod input;
mod level;
mod powerup;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};
use std::time::{Duration, Instant};

use game::{Game, GameState};
use renderer::Renderer;

const TARGET_FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS);

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Run game
    let result = run_game();

    // Restore terminal
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    result
}

fn run_game() -> io::Result<()> {
    let mut game = Game::new();
    let mut renderer = Renderer::new()?;
    let mut last_frame = Instant::now();

    loop {
        // Handle input
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('p') if game.state == GameState::Playing => {
                        game.state = GameState::Paused;
                    }
                    KeyCode::Char('p') if game.state == GameState::Paused => {
                        game.state = GameState::Playing;
                    }
                    KeyCode::Char(' ') if game.state == GameState::Menu => {
                        game.start();
                    }
                    KeyCode::Char('r') if game.state == GameState::GameOver || game.state == GameState::Victory => {
                        game = Game::new();
                    }
                    KeyCode::Left => game.move_paddle_left(),
                    KeyCode::Right => game.move_paddle_right(),
                    _ => {}
                }
            }
        }

        // Update game logic
        let now = Instant::now();
        let delta = now.duration_since(last_frame);

        if delta >= FRAME_DURATION {
            game.update(delta.as_secs_f32());
            renderer.render(&game)?;
            last_frame = now;
        }

        // Small sleep to prevent CPU spinning
        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
