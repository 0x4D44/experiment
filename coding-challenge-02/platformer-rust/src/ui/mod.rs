use macroquad::prelude::*;

pub struct Background {
    layers: Vec<BackgroundLayer>,
}

struct BackgroundLayer {
    color: Color,
    parallax_speed: f32,
    shapes: Vec<(f32, f32, f32)>, // (x, y, size)
}

impl Background {
    pub fn new() -> Self {
        let mut layers = Vec::new();

        // Sky layer
        layers.push(BackgroundLayer {
            color: SKYBLUE,
            parallax_speed: 0.0,
            shapes: vec![],
        });

        // Far mountains
        let mut far_mountains = Vec::new();
        for i in 0..10 {
            far_mountains.push((i as f32 * 300.0, 400.0, 200.0));
        }
        layers.push(BackgroundLayer {
            color: DARKBLUE,
            parallax_speed: 0.2,
            shapes: far_mountains,
        });

        // Near hills
        let mut near_hills = Vec::new();
        for i in 0..15 {
            near_hills.push((i as f32 * 200.0, 450.0, 150.0));
        }
        layers.push(BackgroundLayer {
            color: DARKGREEN,
            parallax_speed: 0.5,
            shapes: near_hills,
        });

        Self { layers }
    }

    pub fn draw(&self, camera_x: f32) {
        // Draw sky
        clear_background(SKYBLUE);

        // Draw clouds
        let cloud_offset = (camera_x * 0.1) % 800.0;
        for i in 0..5 {
            let x = i as f32 * 300.0 - cloud_offset;
            draw_circle(x, 80.0, 30.0, WHITE);
            draw_circle(x + 20.0, 80.0, 40.0, WHITE);
            draw_circle(x + 40.0, 80.0, 30.0, WHITE);
        }

        // Draw parallax layers
        for layer in &self.layers[1..] {
            let offset = camera_x * layer.parallax_speed;
            for (x, y, size) in &layer.shapes {
                let draw_x = x - offset;
                // Draw triangle for mountains/hills
                draw_triangle(
                    Vec2::new(draw_x, *y),
                    Vec2::new(draw_x - size / 2.0, *y + *size),
                    Vec2::new(draw_x + size / 2.0, *y + *size),
                    layer.color,
                );
            }
        }
    }
}

impl Default for Background {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HUD {
    pub flash_timer: f32,
    pub flash_message: String,
}

impl HUD {
    pub fn new() -> Self {
        Self {
            flash_timer: 0.0,
            flash_message: String::new(),
        }
    }

    pub fn show_message(&mut self, message: &str, duration: f32) {
        self.flash_message = message.to_string();
        self.flash_timer = duration;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.flash_timer > 0.0 {
            self.flash_timer -= delta_time;
        }
    }

    pub fn draw(&self, health: i32, lives: i32, score: i32, coins: i32, level_num: usize) {
        let margin = 20.0;
        let y_start = 20.0;

        // Draw semi-transparent background
        draw_rectangle(0.0, 0.0, 300.0, 120.0, Color::new(0.0, 0.0, 0.0, 0.5));

        // Draw health hearts
        draw_text("Health:", margin, y_start, 20.0, WHITE);
        for i in 0..health {
            draw_circle(margin + 80.0 + i as f32 * 25.0, y_start - 5.0, 8.0, RED);
        }

        // Draw lives
        draw_text(
            &format!("Lives: {}", lives),
            margin,
            y_start + 25.0,
            20.0,
            WHITE,
        );

        // Draw score
        draw_text(
            &format!("Score: {}", score),
            margin,
            y_start + 50.0,
            20.0,
            GOLD,
        );

        // Draw coins
        draw_text(
            &format!("Coins: {}", coins),
            margin,
            y_start + 75.0,
            20.0,
            YELLOW,
        );

        // Draw level number
        let level_text = format!("Level {}", level_num);
        let text_width = measure_text(&level_text, None, 30, 1.0).width;
        draw_text(
            &level_text,
            screen_width() - text_width - margin,
            y_start + 10.0,
            30.0,
            WHITE,
        );

        // Draw flash message
        if self.flash_timer > 0.0 {
            let alpha = (self.flash_timer * 3.0).sin().abs();
            let text_width = measure_text(&self.flash_message, None, 40, 1.0).width;
            draw_text(
                &self.flash_message,
                screen_width() / 2.0 - text_width / 2.0,
                screen_height() / 2.0 - 100.0,
                40.0,
                Color::new(1.0, 1.0, 0.0, alpha),
            );
        }
    }
}

impl Default for HUD {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Menu {
    pub selected: usize,
    pub options: Vec<String>,
}

impl Menu {
    pub fn new(options: Vec<&str>) -> Self {
        Self {
            selected: 0,
            options: options.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn handle_input(&mut self) -> Option<usize> {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.selected = if self.selected == 0 {
                self.options.len() - 1
            } else {
                self.selected - 1
            };
        }

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.selected = (self.selected + 1) % self.options.len();
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            return Some(self.selected);
        }

        None
    }

    pub fn draw(&self, title: &str) {
        clear_background(BLACK);

        // Draw title
        let title_size = 60.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(
            title,
            screen_width() / 2.0 - title_width / 2.0,
            screen_height() / 3.0,
            title_size,
            GOLD,
        );

        // Draw options
        let option_size = 30.0;
        let start_y = screen_height() / 2.0;
        let spacing = 50.0;

        for (i, option) in self.options.iter().enumerate() {
            let color = if i == self.selected { YELLOW } else { WHITE };
            let prefix = if i == self.selected { "> " } else { "  " };
            let text = format!("{}{}", prefix, option);
            let text_width = measure_text(&text, None, option_size as u16, 1.0).width;
            draw_text(
                &text,
                screen_width() / 2.0 - text_width / 2.0,
                start_y + i as f32 * spacing,
                option_size,
                color,
            );
        }

        // Draw controls hint
        draw_text(
            "Use Arrow Keys/WASD to navigate, Enter/Space to select",
            50.0,
            screen_height() - 30.0,
            20.0,
            GRAY,
        );
    }
}

pub fn draw_game_over(score: i32, final_level: usize) {
    clear_background(BLACK);

    let title = "GAME OVER";
    let title_width = measure_text(title, None, 60, 1.0).width;
    draw_text(
        title,
        screen_width() / 2.0 - title_width / 2.0,
        screen_height() / 3.0,
        60.0,
        RED,
    );

    let score_text = format!("Final Score: {}", score);
    let score_width = measure_text(&score_text, None, 30, 1.0).width;
    draw_text(
        &score_text,
        screen_width() / 2.0 - score_width / 2.0,
        screen_height() / 2.0,
        30.0,
        WHITE,
    );

    let level_text = format!("Level Reached: {}", final_level);
    let level_width = measure_text(&level_text, None, 30, 1.0).width;
    draw_text(
        &level_text,
        screen_width() / 2.0 - level_width / 2.0,
        screen_height() / 2.0 + 50.0,
        30.0,
        WHITE,
    );

    let restart_text = "Press ENTER to return to menu";
    let restart_width = measure_text(restart_text, None, 20, 1.0).width;
    draw_text(
        restart_text,
        screen_width() / 2.0 - restart_width / 2.0,
        screen_height() - 100.0,
        20.0,
        GRAY,
    );
}

pub fn draw_victory(score: i32, total_levels: usize) {
    clear_background(Color::new(0.1, 0.1, 0.3, 1.0));

    // Draw stars
    for i in 0..50 {
        let x = (i as f32 * 123.456) % screen_width();
        let y = (i as f32 * 234.567) % screen_height();
        draw_circle(x, y, 2.0, YELLOW);
    }

    let title = "VICTORY!";
    let title_width = measure_text(title, None, 70, 1.0).width;
    draw_text(
        title,
        screen_width() / 2.0 - title_width / 2.0,
        screen_height() / 3.0,
        70.0,
        GOLD,
    );

    let subtitle = format!("You completed all {} levels!", total_levels);
    let subtitle_width = measure_text(&subtitle, None, 30, 1.0).width;
    draw_text(
        &subtitle,
        screen_width() / 2.0 - subtitle_width / 2.0,
        screen_height() / 3.0 + 70.0,
        30.0,
        WHITE,
    );

    let score_text = format!("Final Score: {}", score);
    let score_width = measure_text(&score_text, None, 40, 1.0).width;
    draw_text(
        &score_text,
        screen_width() / 2.0 - score_width / 2.0,
        screen_height() / 2.0 + 50.0,
        40.0,
        YELLOW,
    );

    let restart_text = "Press ENTER to return to menu";
    let restart_width = measure_text(restart_text, None, 20, 1.0).width;
    draw_text(
        restart_text,
        screen_width() / 2.0 - restart_width / 2.0,
        screen_height() - 100.0,
        20.0,
        GRAY,
    );
}
