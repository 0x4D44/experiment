use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use crate::board::{Board, Cell, Player};

/// Terminal UI manager
pub struct UI;

impl UI {
    /// Initialize terminal for the game
    pub fn init() -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }

    /// Cleanup terminal
    pub fn cleanup() -> io::Result<()> {
        execute!(
            io::stdout(),
            cursor::Show,
            terminal::LeaveAlternateScreen,
            ResetColor
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Clear the screen
    pub fn clear() -> io::Result<()> {
        execute!(io::stdout(), terminal::Clear(ClearType::All))?;
        Ok(())
    }

    /// Draw the game board with beautiful colors
    pub fn draw_board(board: &Board, selected_col: Option<usize>) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(stdout, cursor::MoveTo(0, 0))?;

        // Title
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("╔══════════════════════════════════════════╗\n\r"),
            Print("║      CONNECT FOUR - RUST EDITION!       ║\n\r"),
            Print("╚══════════════════════════════════════════╝\n\r"),
            ResetColor,
            Print("\n\r")
        )?;

        // Column numbers with selection indicator
        queue!(stdout, Print("   "))?;
        for col in 0..Board::COLS {
            if Some(col) == selected_col {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(format!(" [{}]", col + 1)),
                    ResetColor
                )?;
            } else {
                queue!(stdout, Print(format!("  {}  ", col + 1)))?;
            }
        }
        queue!(stdout, Print("\n\r"))?;

        // Top border
        queue!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print("   ╔════╦════╦════╦════╦════╦════╦════╗\n\r"),
            ResetColor
        )?;

        // Board rows
        for row in 0..Board::ROWS {
            queue!(stdout, SetForegroundColor(Color::Blue), Print("   ║"))?;

            for col in 0..Board::COLS {
                let piece = match board.grid[row][col] {
                    Cell::Empty => "    ".to_string(),
                    Cell::Occupied(Player::Red) => {
                        format!(
                            "{}{}{}",
                            SetForegroundColor(Color::Red),
                            " ●● ",
                            SetForegroundColor(Color::Blue)
                        )
                    }
                    Cell::Occupied(Player::Yellow) => {
                        format!(
                            "{}{}{}",
                            SetForegroundColor(Color::Yellow),
                            " ●● ",
                            SetForegroundColor(Color::Blue)
                        )
                    }
                };
                queue!(stdout, Print(piece), Print("║"))?;
            }

            queue!(stdout, Print("\n\r"))?;

            // Row separator (except last row)
            if row < Board::ROWS - 1 {
                queue!(stdout, Print("   ╠════╬════╬════╬════╬════╬════╬════╣\n\r"))?;
            }
        }

        // Bottom border
        queue!(
            stdout,
            Print("   ╚════╩════╩════╩════╩════╩════╩════╝\n\r"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display game status
    pub fn draw_status(
        current_player: Player,
        move_count: usize,
        game_mode: &str,
    ) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(stdout, Print("\n\r"))?;

        // Current player
        let player_color = match current_player {
            Player::Red => Color::Red,
            Player::Yellow => Color::Yellow,
        };

        queue!(
            stdout,
            Print("   Current Player: "),
            SetForegroundColor(player_color),
            Print(format!("{}", current_player)),
            ResetColor,
            Print(format!(" | Moves: {} | Mode: {}\n\r", move_count, game_mode))
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display controls help
    pub fn draw_controls() -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            Print("\n\r"),
            SetForegroundColor(Color::DarkGrey),
            Print("   Controls: [1-7] Select Column | [U] Undo | [Q] Quit | [Enter] Drop Piece\n\r"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display win message
    pub fn draw_win_message(winner: Player) -> io::Result<()> {
        let mut stdout = io::stdout();

        let color = match winner {
            Player::Red => Color::Red,
            Player::Yellow => Color::Yellow,
        };

        queue!(stdout, Print("\n\r"))?;
        queue!(
            stdout,
            SetForegroundColor(color),
            SetBackgroundColor(Color::Black),
            Print("   ╔══════════════════════════════════════════╗\n\r"),
            Print(format!(
                "   ║       {} WINS! CONGRATULATIONS!        ║\n\r",
                winner.to_string().to_uppercase()
            )),
            Print("   ╚══════════════════════════════════════════╝\n\r"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display draw message
    pub fn draw_draw_message() -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            Print("\n\r"),
            SetForegroundColor(Color::Cyan),
            Print("   ╔══════════════════════════════════════════╗\n\r"),
            Print("   ║          IT'S A DRAW! WELL PLAYED!       ║\n\r"),
            Print("   ╚══════════════════════════════════════════╝\n\r"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display main menu
    pub fn draw_menu() -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All),
            SetForegroundColor(Color::Cyan),
            Print("\n\r"),
            Print("   ╔══════════════════════════════════════════╗\n\r"),
            Print("   ║      CONNECT FOUR - RUST EDITION!       ║\n\r"),
            Print("   ╚══════════════════════════════════════════╝\n\r"),
            ResetColor,
            Print("\n\r"),
            SetForegroundColor(Color::Yellow),
            Print("   Select Game Mode:\n\r"),
            ResetColor,
            Print("\n\r"),
            Print("   [1] Player vs Player\n\r"),
            Print("   [2] Player vs AI (Easy)\n\r"),
            Print("   [3] Player vs AI (Medium)\n\r"),
            Print("   [4] Player vs AI (Hard)\n\r"),
            Print("   [5] Player vs AI (Expert)\n\r"),
            Print("\n\r"),
            Print("   [Q] Quit\n\r"),
            Print("\n\r"),
            SetForegroundColor(Color::DarkGrey),
            Print("   Press a number to select...\n\r"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display play again prompt
    pub fn draw_play_again() -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            Print("\n\r"),
            SetForegroundColor(Color::Green),
            Print("   Play again? [Y/N]\n\r"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display statistics
    pub fn draw_statistics(red_wins: usize, yellow_wins: usize, draws: usize) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            Print("\n\r"),
            SetForegroundColor(Color::Cyan),
            Print("   ╔══════════════════════════════════════════╗\n\r"),
            Print("   ║            GAME STATISTICS               ║\n\r"),
            Print("   ╚══════════════════════════════════════════╝\n\r"),
            ResetColor,
            Print("\n\r"),
            SetForegroundColor(Color::Red),
            Print(format!("   Red Wins:    {}\n\r", red_wins)),
            SetForegroundColor(Color::Yellow),
            Print(format!("   Yellow Wins: {}\n\r", yellow_wins)),
            SetForegroundColor(Color::White),
            Print(format!("   Draws:       {}\n\r", draws)),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display AI thinking message
    pub fn draw_ai_thinking(difficulty: &str) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            Print("\n\r"),
            SetForegroundColor(Color::Magenta),
            Print(format!("   AI ({}) is thinking...\n\r", difficulty)),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Display error message
    pub fn draw_error(message: &str) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            Print("\n\r"),
            SetForegroundColor(Color::Red),
            Print(format!("   ERROR: {}\n\r", message)),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    /// Read a key press
    pub fn read_key() -> io::Result<KeyEvent> {
        loop {
            if let Event::Key(key_event) = event::read()? {
                return Ok(key_event);
            }
        }
    }

    /// Get column from key press (1-7 keys)
    pub fn key_to_column(key: KeyCode) -> Option<usize> {
        match key {
            KeyCode::Char('1') => Some(0),
            KeyCode::Char('2') => Some(1),
            KeyCode::Char('3') => Some(2),
            KeyCode::Char('4') => Some(3),
            KeyCode::Char('5') => Some(4),
            KeyCode::Char('6') => Some(5),
            KeyCode::Char('7') => Some(6),
            _ => None,
        }
    }
}
