mod game;
mod renderer;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;
use std::time::{Duration, Instant};

use game::{Difficulty, Direction, Game, GameState};
use renderer::Renderer;

/// Main entry point for the Snake game
fn main() -> io::Result<()> {
    let difficulty = show_menu()?;
    run_game(difficulty)?;
    Ok(())
}

/// Displays the difficulty selection menu and returns the chosen difficulty
fn show_menu() -> io::Result<Difficulty> {
    let mut renderer = Renderer::new()?;
    let mut selected: usize = 1; // Default to Medium

    loop {
        renderer.render_menu(selected)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        selected = selected.saturating_add(1).min(3);
                    }
                    KeyCode::Enter => {
                        return Ok(match selected {
                            0 => Difficulty::Easy,
                            1 => Difficulty::Medium,
                            2 => Difficulty::Hard,
                            3 => Difficulty::Extreme,
                            _ => Difficulty::Medium,
                        });
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Runs the main game loop with the specified difficulty
fn run_game(difficulty: Difficulty) -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut game = Game::new(30, 20, difficulty);
    let mut last_update = Instant::now();

    loop {
        renderer.render(&game)?;

        // Handle input
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                match game.state() {
                    GameState::Running | GameState::Paused => {
                        if !handle_input(&mut game, key) {
                            return Ok(()); // User quit
                        }
                    }
                    GameState::GameOver => {
                        match key.code {
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                // Restart the game
                                game = Game::new(30, 20, difficulty);
                                last_update = Instant::now();
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                                return Ok(()); // User quit
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Update game state based on speed
        let elapsed = last_update.elapsed();
        if elapsed >= Duration::from_millis(game.speed()) {
            game.update();
            last_update = Instant::now();
        }
    }
}

/// Handles keyboard input and returns false if the user wants to quit
fn handle_input(game: &mut Game, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Up => {
            game.set_direction(Direction::Up);
        }
        KeyCode::Down => {
            game.set_direction(Direction::Down);
        }
        KeyCode::Left => {
            game.set_direction(Direction::Left);
        }
        KeyCode::Right => {
            game.set_direction(Direction::Right);
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            game.toggle_pause();
        }
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            return false; // Signal to quit
        }
        _ => {}
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_initialization() {
        let game = Game::new(30, 20, Difficulty::Medium);
        assert_eq!(game.state(), GameState::Running);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn test_direction_handling() {
        let mut game = Game::new(30, 20, Difficulty::Medium);
        let initial_direction = game.snake().direction();

        game.set_direction(Direction::Up);
        // Direction should be updated (will take effect on next update)
        game.update();
        // After update, the direction should have changed
        assert_ne!(game.snake().direction(), initial_direction);
    }

    #[test]
    fn test_pause_functionality() {
        let mut game = Game::new(30, 20, Difficulty::Medium);
        assert_eq!(game.state(), GameState::Running);

        game.toggle_pause();
        assert_eq!(game.state(), GameState::Paused);

        // Game shouldn't update while paused
        let head_before = game.snake().head();
        game.update();
        assert_eq!(game.snake().head(), head_before);

        game.toggle_pause();
        assert_eq!(game.state(), GameState::Running);
    }
}
