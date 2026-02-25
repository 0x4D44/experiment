use crate::game::GameState;
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

pub struct Renderer {
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn render(&mut self, game_state: &GameState) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(cursor::Hide)?;

        // Draw border
        for x in 0..=self.width + 1 {
            stdout
                .queue(cursor::MoveTo(x, 0))?
                .queue(SetForegroundColor(Color::White))?
                .queue(Print("#"))?;
            stdout
                .queue(cursor::MoveTo(x, self.height + 1))?
                .queue(SetForegroundColor(Color::White))?
                .queue(Print("#"))?;
        }

        for y in 0..=self.height + 1 {
            stdout
                .queue(cursor::MoveTo(0, y))?
                .queue(SetForegroundColor(Color::White))?
                .queue(Print("#"))?;
            stdout
                .queue(cursor::MoveTo(self.width + 1, y))?
                .queue(SetForegroundColor(Color::White))?
                .queue(Print("#"))?;
        }

        // Draw bullets
        for bullet in &game_state.bullets {
            if game_state.is_in_bounds(bullet.pos) {
                stdout
                    .queue(cursor::MoveTo(
                        (bullet.pos.x + 1) as u16,
                        (bullet.pos.y + 1) as u16,
                    ))?
                    .queue(SetForegroundColor(Color::Red))?
                    .queue(Print("*"))?;
            }
        }

        // Draw enemies
        for enemy in &game_state.enemies {
            stdout
                .queue(cursor::MoveTo(
                    (enemy.pos.x + 1) as u16,
                    (enemy.pos.y + 1) as u16,
                ))?
                .queue(SetForegroundColor(Color::Yellow))?
                .queue(Print("@"))?;
        }

        // Draw snake
        for (i, segment) in game_state.snake.segments.iter().enumerate() {
            let symbol = if i == 0 { "O" } else { "o" };
            stdout
                .queue(cursor::MoveTo(
                    (segment.x + 1) as u16,
                    (segment.y + 1) as u16,
                ))?
                .queue(SetForegroundColor(Color::Green))?
                .queue(Print(symbol))?;
        }

        // Draw UI
        stdout
            .queue(cursor::MoveTo(0, self.height + 3))?
            .queue(ResetColor)?
            .queue(Print(format!(
                "Score: {}  |  Length: {}  |  Controls: WASD/Arrows | Q: Quit",
                game_state.score,
                game_state.snake.segments.len()
            )))?;

        stdout.flush()?;
        Ok(())
    }

    pub fn render_game_over(&mut self, game_state: &GameState) -> io::Result<()> {
        let mut stdout = io::stdout();

        let msg1 = "GAME OVER!";
        let msg2 = format!("Final Score: {}", game_state.score);
        let msg3 = format!("Final Length: {}", game_state.snake.segments.len());

        let x = (self.width / 2).saturating_sub(msg1.len() as u16 / 2);
        let y = self.height / 2;

        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(SetForegroundColor(Color::Red))?
            .queue(Print(msg1))?;

        stdout
            .queue(cursor::MoveTo(
                (self.width / 2).saturating_sub(msg2.len() as u16 / 2),
                y + 2,
            ))?
            .queue(SetForegroundColor(Color::White))?
            .queue(Print(msg2))?;

        stdout
            .queue(cursor::MoveTo(
                (self.width / 2).saturating_sub(msg3.len() as u16 / 2),
                y + 3,
            ))?
            .queue(SetForegroundColor(Color::White))?
            .queue(Print(msg3))?;

        stdout.flush()?;
        Ok(())
    }
}
