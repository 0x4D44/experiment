use crate::core::{CellState, Grid, Position};
use crate::levels::Level;
use crossterm::{
    cursor, execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

pub struct Renderer {
    stdout: io::Stdout,
}

impl Renderer {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;
        Ok(Renderer { stdout })
    }

    pub fn render(
        &mut self,
        level: &Level,
        grid: &Grid,
        cursor_pos: Position,
        moves: usize,
    ) -> io::Result<()> {
        execute!(
            self.stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Title
        writeln!(
            self.stdout,
            "╔═══════════════════════════════════════════════╗"
        )?;
        writeln!(
            self.stdout,
            "║              FLUX - Level {}               ║",
            level.id + 1
        )?;
        writeln!(
            self.stdout,
            "╚═══════════════════════════════════════════════╝"
        )?;
        writeln!(self.stdout)?;

        // Level info
        writeln!(self.stdout, "  {}", level.name)?;
        writeln!(self.stdout, "  {}", level.description)?;
        writeln!(self.stdout)?;
        writeln!(self.stdout, "  Moves: {}", moves)?;
        writeln!(self.stdout)?;

        // Grid
        self.render_grid(grid, cursor_pos)?;

        writeln!(self.stdout)?;
        writeln!(self.stdout, "Controls:")?;
        writeln!(self.stdout, "  WASD/Arrow Keys/HJKL: Move cursor")?;
        writeln!(self.stdout, "  Enter/Space: Click cell")?;
        writeln!(self.stdout, "  R: Reset   N: Next   P: Previous   Q: Quit")?;

        self.stdout.flush()?;
        Ok(())
    }

    fn render_grid(&mut self, grid: &Grid, cursor_pos: Position) -> io::Result<()> {
        let cell_height = 2;

        // Top border
        write!(self.stdout, "  ┌")?;
        for _ in 0..grid.width() {
            write!(self.stdout, "────")?;
        }
        writeln!(self.stdout, "┐")?;

        // Cells
        for row in 0..grid.height() {
            for line in 0..cell_height {
                write!(self.stdout, "  │")?;
                for col in 0..grid.width() {
                    let pos = Position::new(row, col);
                    let cell = grid.get(pos).unwrap();
                    let is_cursor = pos == cursor_pos;

                    if line == 0 {
                        // Top half of cell
                        if is_cursor {
                            write!(self.stdout, "┌──┐")?;
                        } else {
                            write!(self.stdout, "    ")?;
                        }
                    } else {
                        // Bottom half with cell value
                        if is_cursor {
                            write!(self.stdout, "│")?;
                        } else {
                            write!(self.stdout, " ")?;
                        }
                        self.render_cell(cell)?;
                        if is_cursor {
                            write!(self.stdout, "│")?;
                        } else {
                            write!(self.stdout, " ")?;
                        }
                    }
                }
                writeln!(self.stdout, "│")?;
            }
        }

        // Bottom border
        write!(self.stdout, "  └")?;
        for _ in 0..grid.width() {
            write!(self.stdout, "────")?;
        }
        writeln!(self.stdout, "┘")?;

        Ok(())
    }

    fn render_cell(&mut self, cell: CellState) -> io::Result<()> {
        let (color, symbol) = match cell.value() {
            0 => (Color::DarkGrey, " "),
            1 => (Color::Blue, "█"),
            2 => (Color::Green, "█"),
            3 => (Color::Yellow, "█"),
            _ => (Color::White, "?"),
        };

        execute!(self.stdout, SetForegroundColor(color))?;
        write!(self.stdout, "{}{}", symbol, symbol)?;
        execute!(self.stdout, ResetColor)?;
        Ok(())
    }

    pub fn show_win_screen(&mut self, level_id: usize, moves: usize) -> io::Result<()> {
        execute!(
            self.stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        writeln!(self.stdout)?;
        writeln!(
            self.stdout,
            "╔═══════════════════════════════════════════════╗"
        )?;
        writeln!(
            self.stdout,
            "║             LEVEL COMPLETE!                   ║"
        )?;
        writeln!(
            self.stdout,
            "╚═══════════════════════════════════════════════╝"
        )?;
        writeln!(self.stdout)?;
        writeln!(
            self.stdout,
            "  Level {} completed in {} moves!",
            level_id + 1,
            moves
        )?;
        writeln!(self.stdout)?;
        writeln!(self.stdout, "  Press N for next level, Q to quit")?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn show_game_complete(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        writeln!(self.stdout)?;
        writeln!(
            self.stdout,
            "╔═══════════════════════════════════════════════╗"
        )?;
        writeln!(
            self.stdout,
            "║         CONGRATULATIONS!                      ║"
        )?;
        writeln!(
            self.stdout,
            "║                                               ║"
        )?;
        writeln!(
            self.stdout,
            "║     You've completed all levels!              ║"
        )?;
        writeln!(
            self.stdout,
            "╚═══════════════════════════════════════════════╝"
        )?;
        writeln!(self.stdout)?;
        writeln!(self.stdout, "  You are a FLUX master!")?;
        writeln!(self.stdout)?;
        writeln!(self.stdout, "  Press Q to quit")?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            ResetColor,
            cursor::Show,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}
