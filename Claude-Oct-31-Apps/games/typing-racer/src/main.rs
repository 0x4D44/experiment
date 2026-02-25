// Typing Speed Racer - Main Game Loop

use std::io::{self, Read, Write};
use std::time::{Duration, Instant};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use typing_racer::{
    config::*,
    difficulty::DifficultyLevel,
    game::{Game, GameState},
    Renderer,
};

const FRAME_TIME: Duration = Duration::from_millis(FRAME_TIME_MS);

fn main() -> io::Result<()> {
    Renderer::init()?;

    let mut game = Game::new();
    let mut renderer = Renderer::new();

    // Show start screen
    show_start_screen(&mut renderer)?;
    wait_for_key()?;

    // Show difficulty selection
    let difficulty = show_difficulty_selection(&mut renderer)?;

    // Start the game
    game.start_new_game(difficulty);

    let mut last_frame = Instant::now();

    // Main game loop
    loop {
        let frame_start = Instant::now();
        let delta_time = last_frame.elapsed().as_secs_f32();
        last_frame = frame_start;

        // Handle input
        if poll(Duration::from_millis(10)).ok() == Some(true) {
            if let Ok(Event::Key(key_event)) = read() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                            break;
                        }
                        if c == ' ' {
                            game.handle_input(' ');
                        } else {
                            game.handle_input(c);
                        }
                    }
                    KeyCode::Backspace => game.handle_input('\u{0008}'),
                    KeyCode::Enter => game.handle_input(' '),
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // Update game
        game.update(delta_time);

        // Render
        let stats = game.score_engine.get_stats();
        renderer.render_frame(
            &game.words,
            game.input.as_str(),
            &stats,
            game.lives,
            &game.difficulty.to_string(),
        )?;

        // Frame rate limiting
        let frame_elapsed = frame_start.elapsed();
        if frame_elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - frame_elapsed);
        }

        // Check if game is over
        if game.state == GameState::GameOver {
            break;
        }
    }

    // Show game over screen
    if game.state == GameState::GameOver {
        let stats = game.score_engine.get_stats();
        renderer.render_game_over(&stats)?;
        wait_for_key()?;

        // Ask to play again
        println!("\nPlay again? (y/n): ");
        let mut input = [0u8; 1];
        io::stdin().read_exact(&mut input)?;
        if input[0] == b'y' || input[0] == b'Y' {
            return main();
        }
    }

    Renderer::cleanup()?;
    Ok(())
}

fn show_start_screen(renderer: &mut Renderer) -> io::Result<()> {
    crossterm::execute!(
        io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    )?;

    renderer.render_start_screen()?;
    Ok(())
}

fn show_difficulty_selection(_renderer: &mut Renderer) -> io::Result<DifficultyLevel> {
    loop {
        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )?;

        let mut buffer = String::new();
        buffer.push_str(&format!(
            "╔{}╗\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));
        buffer.push_str("║                   SELECT DIFFICULTY LEVEL                        ║\n");
        buffer.push_str(&format!(
            "╠{}╣\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));
        buffer.push_str("║                                                                  ║\n");
        buffer.push_str("║ 1. EASY   - Slower words, shorter words (3-5 chars)              ║\n");
        buffer.push_str("║ 2. MEDIUM - Medium speed, medium words (6-8 chars)               ║\n");
        buffer.push_str("║ 3. HARD   - Fast words, longer words (9-12 chars)                ║\n");
        buffer.push_str("║ 4. EXPERT - Very fast, expert words (13-15 chars)                ║\n");
        buffer.push_str("║                                                                  ║\n");
        buffer.push_str(&format!(
            "╠{}╣\n",
            "═".repeat((GAME_WIDTH as usize).saturating_sub(2))
        ));
        buffer.push_str("║ Select difficulty (1-4): ");

        print!("{}", buffer);
        io::stdout().flush()?;

        if poll(Duration::from_secs(30)).ok() == Some(true) {
            if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
                match code {
                    KeyCode::Char('1') => return Ok(DifficultyLevel::Easy),
                    KeyCode::Char('2') => return Ok(DifficultyLevel::Medium),
                    KeyCode::Char('3') => return Ok(DifficultyLevel::Hard),
                    KeyCode::Char('4') => return Ok(DifficultyLevel::Expert),
                    KeyCode::Char('c') | KeyCode::Esc => std::process::exit(0),
                    _ => {}
                }
            }
        }
    }
}

fn wait_for_key() -> io::Result<()> {
    loop {
        if poll(Duration::from_millis(100)).ok() == Some(true) {
            if let Ok(Event::Key(_)) = read() {
                return Ok(());
            }
        }
    }
}
