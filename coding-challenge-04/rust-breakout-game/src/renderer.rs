use crate::game::{Game, GameState, GAME_HEIGHT, GAME_WIDTH};
use crate::powerup::PowerUpType;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout, Write};

pub struct Renderer {
    buffer: Vec<Vec<char>>,
    color_buffer: Vec<Vec<Color>>,
}

impl Renderer {
    pub fn new() -> io::Result<Self> {
        execute!(stdout(), Hide)?;
        Ok(Self {
            buffer: Vec::new(),
            color_buffer: Vec::new(),
        })
    }

    pub fn render(&mut self, game: &Game) -> io::Result<()> {
        let width = GAME_WIDTH as usize;
        let height = GAME_HEIGHT as usize;

        // Initialize buffers
        self.buffer = vec![vec![' '; width]; height];
        self.color_buffer = vec![vec![Color::White; width]; height];

        match game.state {
            GameState::Menu => self.render_menu(),
            GameState::Playing | GameState::Paused => {
                self.render_game(game);
                if game.state == GameState::Paused {
                    self.render_pause_overlay();
                }
            }
            GameState::GameOver => self.render_game_over(game),
            GameState::Victory => self.render_victory(game),
        }

        self.draw_to_terminal()?;
        Ok(())
    }

    fn render_menu(&mut self) {
        let messages = vec![
            "",
            "",
            "  ██████╗ ██████╗ ███████╗ █████╗ ██╗  ██╗ ██████╗ ██╗   ██╗████████╗",
            "  ██╔══██╗██╔══██╗██╔════╝██╔══██╗██║ ██╔╝██╔═══██╗██║   ██║╚══██╔══╝",
            "  ██████╔╝██████╔╝█████╗  ███████║█████╔╝ ██║   ██║██║   ██║   ██║   ",
            "  ██╔══██╗██╔══██╗██╔══╝  ██╔══██║██╔═██╗ ██║   ██║██║   ██║   ██║   ",
            "  ██████╔╝██║  ██║███████╗██║  ██║██║  ██╗╚██████╔╝╚██████╔╝   ██║   ",
            "  ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝  ╚═════╝    ╚═╝   ",
            "",
            "",
            "                     Press SPACE to Start",
            "",
            "                     Arrow Keys to Move",
            "                     P to Pause",
            "                     Q to Quit",
            "",
            "",
            "           Power-ups: [W] Wide  [M] Multi  [S] Slow  [+] Life  [L] Laser",
            "",
            "",
        ];

        for (i, msg) in messages.iter().enumerate() {
            if i < self.buffer.len() {
                for (j, ch) in msg.chars().enumerate() {
                    if j < self.buffer[i].len() {
                        self.buffer[i][j] = ch;
                        self.color_buffer[i][j] = Color::Cyan;
                    }
                }
            }
        }
    }

    fn render_game(&mut self, game: &Game) {
        // Draw border
        self.draw_border();

        // Draw bricks
        for brick in &game.level.bricks {
            if !brick.active {
                continue;
            }

            let color = self.get_brick_color(brick.color_index());
            let symbol = brick.brick_type.symbol(brick.hits);

            let x = brick.rect.x as usize;
            let y = brick.rect.y as usize;
            let width = brick.rect.width as usize;

            for dy in 0..(brick.rect.height as usize) {
                let row = y + dy;
                if row < self.buffer.len() {
                    for dx in 0..width {
                        let col = x + dx;
                        if col < self.buffer[row].len() {
                            self.buffer[row][col] = symbol.chars().next().unwrap_or('█');
                            self.color_buffer[row][col] = color;
                        }
                    }
                }
            }
        }

        // Draw paddle
        let paddle_y = game.paddle.rect.y as usize;
        let paddle_x = game.paddle.rect.x as usize;
        let paddle_width = game.paddle.rect.width as usize;

        if paddle_y < self.buffer.len() {
            for i in 0..paddle_width {
                let col = paddle_x + i;
                if col < self.buffer[paddle_y].len() {
                    self.buffer[paddle_y][col] = '═';
                    self.color_buffer[paddle_y][col] = if game.paddle.is_wide {
                        Color::Yellow
                    } else {
                        Color::Green
                    };
                }
            }
        }

        // Draw balls
        for ball in &game.balls {
            if !ball.active {
                continue;
            }

            let ball_x = ball.circle.center.x as usize;
            let ball_y = ball.circle.center.y as usize;

            if ball_y < self.buffer.len() && ball_x < self.buffer[ball_y].len() {
                self.buffer[ball_y][ball_x] = '●';
                self.color_buffer[ball_y][ball_x] = Color::Red;
            }
        }

        // Draw power-ups
        for powerup in &game.powerups {
            if !powerup.active {
                continue;
            }

            let x = powerup.position.x as usize;
            let y = powerup.position.y as usize;

            if y < self.buffer.len() && x < self.buffer[y].len() {
                self.buffer[y][x] = powerup.power_type.symbol().chars().next().unwrap_or('?');
                self.color_buffer[y][x] = self.get_powerup_color(&powerup.power_type);
            }
        }

        // Draw HUD
        self.draw_hud(game);
    }

    fn draw_border(&mut self) {
        // Top border
        for x in 0..self.buffer[0].len() {
            self.buffer[0][x] = '═';
            self.color_buffer[0][x] = Color::DarkGrey;
        }

        // Side borders
        for y in 0..self.buffer.len() {
            if !self.buffer[y].is_empty() {
                self.buffer[y][0] = '║';
                self.color_buffer[y][0] = Color::DarkGrey;

                let last = self.buffer[y].len() - 1;
                self.buffer[y][last] = '║';
                self.color_buffer[y][last] = Color::DarkGrey;
            }
        }

        // Corners
        if !self.buffer.is_empty() && !self.buffer[0].is_empty() {
            let last_col = self.buffer[0].len() - 1;
            self.buffer[0][0] = '╔';
            self.buffer[0][last_col] = '╗';
        }
    }

    fn draw_hud(&mut self, game: &Game) {
        let hud_y = 1;

        // Score
        let score_text = format!(" Score: {}", game.score);
        for (i, ch) in score_text.chars().enumerate() {
            if i + 2 < self.buffer[hud_y].len() {
                self.buffer[hud_y][i + 2] = ch;
                self.color_buffer[hud_y][i + 2] = Color::Yellow;
            }
        }

        // Lives
        let lives_text = format!("Lives: {}", game.lives);
        let lives_start = 25;
        for (i, ch) in lives_text.chars().enumerate() {
            if lives_start + i < self.buffer[hud_y].len() {
                self.buffer[hud_y][lives_start + i] = ch;
                self.color_buffer[hud_y][lives_start + i] = Color::Magenta;
            }
        }

        // Level
        let level_text = format!("Level: {}", game.level_number);
        let level_start = 42;
        for (i, ch) in level_text.chars().enumerate() {
            if level_start + i < self.buffer[hud_y].len() {
                self.buffer[hud_y][level_start + i] = ch;
                self.color_buffer[hud_y][level_start + i] = Color::Cyan;
            }
        }

        // Active power-ups
        let powerup_start = 58;
        let mut offset = 0;
        for powerup in &game.active_powerups {
            let text = format!("[{}]", powerup.power_type.symbol());
            for (i, ch) in text.chars().enumerate() {
                if powerup_start + offset + i < self.buffer[hud_y].len() {
                    self.buffer[hud_y][powerup_start + offset + i] = ch;
                    self.color_buffer[hud_y][powerup_start + offset + i] =
                        self.get_powerup_color(&powerup.power_type);
                }
            }
            offset += text.len() + 1;
        }
    }

    fn render_pause_overlay(&mut self) {
        let messages = ["", "           PAUSED", "", "     Press P to Resume"];

        let start_y = (self.buffer.len() / 2).saturating_sub(2);

        for (i, msg) in messages.iter().enumerate() {
            let y = start_y + i;
            if y < self.buffer.len() {
                let start_x = (self.buffer[y].len() / 2).saturating_sub(msg.len() / 2);
                for (j, ch) in msg.chars().enumerate() {
                    if start_x + j < self.buffer[y].len() {
                        self.buffer[y][start_x + j] = ch;
                        self.color_buffer[y][start_x + j] = Color::Yellow;
                    }
                }
            }
        }
    }

    fn render_game_over(&mut self, game: &Game) {
        let messages = vec![
            "",
            "",
            "   ██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ██╗   ██╗███████╗██████╗ ",
            "  ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██║   ██║██╔════╝██╔══██╗",
            "  ██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║██║   ██║█████╗  ██████╔╝",
            "  ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗",
            "  ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║",
            "   ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝",
            "",
            "",
        ];

        for (i, msg) in messages.iter().enumerate() {
            if i < self.buffer.len() {
                for (j, ch) in msg.chars().enumerate() {
                    if j < self.buffer[i].len() {
                        self.buffer[i][j] = ch;
                        self.color_buffer[i][j] = Color::Red;
                    }
                }
            }
        }

        let score_line = format!("Final Score: {}", game.score);
        let y = 12;
        let start_x = (self.buffer[y].len() / 2).saturating_sub(score_line.len() / 2);
        for (i, ch) in score_line.chars().enumerate() {
            if start_x + i < self.buffer[y].len() {
                self.buffer[y][start_x + i] = ch;
                self.color_buffer[y][start_x + i] = Color::Yellow;
            }
        }

        let restart_msg = "Press R to Restart or Q to Quit";
        let y = 14;
        let start_x = (self.buffer[y].len() / 2).saturating_sub(restart_msg.len() / 2);
        for (i, ch) in restart_msg.chars().enumerate() {
            if start_x + i < self.buffer[y].len() {
                self.buffer[y][start_x + i] = ch;
                self.color_buffer[y][start_x + i] = Color::White;
            }
        }
    }

    fn render_victory(&mut self, game: &Game) {
        let messages = vec![
            "",
            "",
            "  ██╗   ██╗██╗ ██████╗████████╗ ██████╗ ██████╗ ██╗   ██╗██╗",
            "  ██║   ██║██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗╚██╗ ██╔╝██║",
            "  ██║   ██║██║██║        ██║   ██║   ██║██████╔╝ ╚████╔╝ ██║",
            "  ╚██╗ ██╔╝██║██║        ██║   ██║   ██║██╔══██╗  ╚██╔╝  ╚═╝",
            "   ╚████╔╝ ██║╚██████╗   ██║   ╚██████╔╝██║  ██║   ██║   ██╗",
            "    ╚═══╝  ╚═╝ ╚═════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝",
            "",
            "",
        ];

        for (i, msg) in messages.iter().enumerate() {
            if i < self.buffer.len() {
                for (j, ch) in msg.chars().enumerate() {
                    if j < self.buffer[i].len() {
                        self.buffer[i][j] = ch;
                        self.color_buffer[i][j] = Color::Green;
                    }
                }
            }
        }

        let score_line = format!("Final Score: {}", game.score);
        let y = 12;
        let start_x = (self.buffer[y].len() / 2).saturating_sub(score_line.len() / 2);
        for (i, ch) in score_line.chars().enumerate() {
            if start_x + i < self.buffer[y].len() {
                self.buffer[y][start_x + i] = ch;
                self.color_buffer[y][start_x + i] = Color::Yellow;
            }
        }

        let restart_msg = "Press R to Restart or Q to Quit";
        let y = 14;
        let start_x = (self.buffer[y].len() / 2).saturating_sub(restart_msg.len() / 2);
        for (i, ch) in restart_msg.chars().enumerate() {
            if start_x + i < self.buffer[y].len() {
                self.buffer[y][start_x + i] = ch;
                self.color_buffer[y][start_x + i] = Color::White;
            }
        }
    }

    fn get_brick_color(&self, index: usize) -> Color {
        match index {
            0 => Color::Blue,      // Normal
            1 => Color::Magenta,   // Strong (full health)
            2 => Color::DarkMagenta, // Strong (damaged)
            3 => Color::DarkGrey,  // Unbreakable
            4 => Color::Yellow,    // Bonus
            _ => Color::White,
        }
    }

    fn get_powerup_color(&self, powerup_type: &PowerUpType) -> Color {
        match powerup_type {
            PowerUpType::WidePaddle => Color::Yellow,
            PowerUpType::MultiBall => Color::Cyan,
            PowerUpType::SlowBall => Color::Blue,
            PowerUpType::ExtraLife => Color::Green,
            PowerUpType::LaserPaddle => Color::Red,
        }
    }

    fn draw_to_terminal(&self) -> io::Result<()> {
        let mut stdout = stdout();

        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        for (y, row) in self.buffer.iter().enumerate() {
            execute!(stdout, MoveTo(0, y as u16))?;

            let mut current_color = Color::White;
            let mut text_buffer = String::new();

            for (x, &ch) in row.iter().enumerate() {
                let color = self.color_buffer[y][x];

                if color != current_color {
                    if !text_buffer.is_empty() {
                        execute!(
                            stdout,
                            SetForegroundColor(current_color),
                            Print(&text_buffer)
                        )?;
                        text_buffer.clear();
                    }
                    current_color = color;
                }

                text_buffer.push(ch);
            }

            if !text_buffer.is_empty() {
                execute!(
                    stdout,
                    SetForegroundColor(current_color),
                    Print(&text_buffer)
                )?;
            }
        }

        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = execute!(stdout(), Show, ResetColor);
    }
}
