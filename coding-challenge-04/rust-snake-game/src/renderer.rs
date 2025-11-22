use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Stdout, Write};

use crate::game::{Game, GameState};

/// Handles rendering the game to the terminal
pub struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    /// Creates a new renderer
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        execute!(stdout, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        execute!(stdout, cursor::Hide)?;

        Ok(Renderer { stdout })
    }

    /// Renders the game state to the terminal
    pub fn render(&mut self, game: &Game) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All)
        )?;

        self.render_title()?;
        self.render_info(game)?;
        self.render_board(game)?;
        self.render_instructions(game)?;

        self.stdout.flush()?;
        Ok(())
    }

    /// Renders the game title
    fn render_title(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Green),
            Print("╔═══════════════════════════════════════════════════════════════════╗\n"),
            Print("║          🐍  RUST SNAKE GAME - Terminal Edition  🐍              ║\n"),
            Print("╚═══════════════════════════════════════════════════════════════════╝"),
            ResetColor
        )?;
        Ok(())
    }

    /// Renders game information (score, difficulty, speed)
    fn render_info(&mut self, game: &Game) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::MoveTo(0, 3),
            SetForegroundColor(Color::Yellow),
            Print(format!("  Score: {} ", game.score())),
            SetForegroundColor(Color::Cyan),
            Print(format!("| Length: {} ", game.snake().len())),
            SetForegroundColor(Color::Magenta),
            Print(format!("| Difficulty: {} ", game.difficulty().name())),
            SetForegroundColor(Color::White),
            Print(format!("| Speed: {}ms", game.speed())),
            ResetColor
        )?;
        Ok(())
    }

    /// Renders the game board with borders
    fn render_board(&mut self, game: &Game) -> io::Result<()> {
        let width = game.width() as u16;
        let height = game.height() as u16;
        let offset_x = 2;
        let offset_y = 5;

        // Draw top border
        execute!(
            self.stdout,
            cursor::MoveTo(offset_x, offset_y),
            SetForegroundColor(Color::DarkGrey),
            Print("┌"),
            Print("─".repeat(width as usize * 2)),
            Print("┐"),
            ResetColor
        )?;

        // Draw game area
        for y in 0..height {
            execute!(
                self.stdout,
                cursor::MoveTo(offset_x, offset_y + y + 1),
                SetForegroundColor(Color::DarkGrey),
                Print("│"),
                ResetColor
            )?;

            for x in 0..width {
                let pos = crate::game::Position::new(x as i32, y as i32);

                if game.snake().head() == pos {
                    // Snake head
                    execute!(
                        self.stdout,
                        SetForegroundColor(Color::Green),
                        SetBackgroundColor(Color::DarkGreen),
                        Print("🟢"),
                        ResetColor
                    )?;
                } else if game.snake().body().contains(&pos) {
                    // Snake body
                    execute!(
                        self.stdout,
                        SetForegroundColor(Color::Green),
                        Print("██"),
                        ResetColor
                    )?;
                } else if game.food() == pos {
                    // Food
                    execute!(
                        self.stdout,
                        SetForegroundColor(Color::Red),
                        Print("🍎"),
                        ResetColor
                    )?;
                } else {
                    // Empty space
                    execute!(self.stdout, Print("  "))?;
                }
            }

            execute!(
                self.stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("│"),
                ResetColor
            )?;
        }

        // Draw bottom border
        execute!(
            self.stdout,
            cursor::MoveTo(offset_x, offset_y + height + 1),
            SetForegroundColor(Color::DarkGrey),
            Print("└"),
            Print("─".repeat(width as usize * 2)),
            Print("┘"),
            ResetColor
        )?;

        Ok(())
    }

    /// Renders game instructions and state messages
    fn render_instructions(&mut self, game: &Game) -> io::Result<()> {
        let offset_y = 5 + game.height() as u16 + 3;

        match game.state() {
            GameState::Running => {
                execute!(
                    self.stdout,
                    cursor::MoveTo(2, offset_y),
                    SetForegroundColor(Color::White),
                    Print("Controls: "),
                    SetForegroundColor(Color::Cyan),
                    Print("Arrow Keys"),
                    SetForegroundColor(Color::White),
                    Print(" to move | "),
                    SetForegroundColor(Color::Cyan),
                    Print("P"),
                    SetForegroundColor(Color::White),
                    Print(" to pause | "),
                    SetForegroundColor(Color::Cyan),
                    Print("Q"),
                    SetForegroundColor(Color::White),
                    Print(" to quit"),
                    ResetColor
                )?;
            }
            GameState::Paused => {
                execute!(
                    self.stdout,
                    cursor::MoveTo(2, offset_y),
                    SetForegroundColor(Color::Yellow),
                    Print("⏸  PAUSED - Press "),
                    SetForegroundColor(Color::Cyan),
                    Print("P"),
                    SetForegroundColor(Color::Yellow),
                    Print(" to resume or "),
                    SetForegroundColor(Color::Cyan),
                    Print("Q"),
                    SetForegroundColor(Color::Yellow),
                    Print(" to quit"),
                    ResetColor
                )?;
            }
            GameState::GameOver => {
                execute!(
                    self.stdout,
                    cursor::MoveTo(2, offset_y),
                    SetForegroundColor(Color::Red),
                    Print("💀 GAME OVER! "),
                    SetForegroundColor(Color::White),
                    Print(format!("Final Score: {} ", game.score())),
                    Print("| Press "),
                    SetForegroundColor(Color::Cyan),
                    Print("R"),
                    SetForegroundColor(Color::White),
                    Print(" to restart or "),
                    SetForegroundColor(Color::Cyan),
                    Print("Q"),
                    SetForegroundColor(Color::White),
                    Print(" to quit"),
                    ResetColor
                )?;
            }
        }

        Ok(())
    }

    /// Renders the difficulty selection menu
    pub fn render_menu(&mut self, selected: usize) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All)
        )?;

        let difficulties = [
            ("Easy", "Relaxed pace, perfect for beginners"),
            ("Medium", "Balanced challenge for casual players"),
            ("Hard", "Fast-paced action for experienced players"),
            ("Extreme", "Blazing speed for true snake masters"),
        ];

        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Green),
            Print("╔═══════════════════════════════════════════════════════════════════╗\n"),
            Print("║          🐍  RUST SNAKE GAME - Terminal Edition  🐍              ║\n"),
            Print("╚═══════════════════════════════════════════════════════════════════╝\n\n"),
            ResetColor
        )?;

        execute!(
            self.stdout,
            cursor::MoveTo(2, 4),
            SetForegroundColor(Color::Cyan),
            Print("Select Difficulty Level:\n\n"),
            ResetColor
        )?;

        for (i, (name, description)) in difficulties.iter().enumerate() {
            let y = 6 + (i * 2) as u16;

            if i == selected {
                execute!(
                    self.stdout,
                    cursor::MoveTo(4, y),
                    SetForegroundColor(Color::Green),
                    SetBackgroundColor(Color::DarkGreen),
                    Print(format!(" ▶ {} ", name)),
                    ResetColor,
                    SetForegroundColor(Color::White),
                    Print(format!(" - {}", description)),
                    ResetColor
                )?;
            } else {
                execute!(
                    self.stdout,
                    cursor::MoveTo(4, y),
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("   {} ", name)),
                    ResetColor,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!(" - {}", description)),
                    ResetColor
                )?;
            }
        }

        execute!(
            self.stdout,
            cursor::MoveTo(2, 16),
            SetForegroundColor(Color::Yellow),
            Print("Use "),
            SetForegroundColor(Color::Cyan),
            Print("↑/↓"),
            SetForegroundColor(Color::Yellow),
            Print(" to select, "),
            SetForegroundColor(Color::Cyan),
            Print("Enter"),
            SetForegroundColor(Color::Yellow),
            Print(" to start, "),
            SetForegroundColor(Color::Cyan),
            Print("Q"),
            SetForegroundColor(Color::Yellow),
            Print(" to quit"),
            ResetColor
        )?;

        self.stdout.flush()?;
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = execute!(self.stdout, cursor::Show);
        let _ = execute!(self.stdout, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}
