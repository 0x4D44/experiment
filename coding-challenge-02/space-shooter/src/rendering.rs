use crate::entities::{Enemy, EnemyType, Player};
use crate::particles::Particle;
use crate::powerups::{PowerUp, PowerUpType};
use crate::weapons::{Bullet, BulletType};
use macroquad::prelude::*;

/// Rendering system for all game entities
pub struct Renderer {
    pub screen_shake: f32,
    pub stars: Vec<Star>,
}

#[derive(Clone)]
struct Star {
    position: Vec2,
    size: f32,
    speed: f32,
}

impl Renderer {
    pub fn new() -> Self {
        let mut stars = Vec::new();
        for _ in 0..150 {
            stars.push(Star {
                position: vec2(rand::gen_range(0.0, 800.0), rand::gen_range(0.0, 900.0)),
                size: rand::gen_range(1.0, 3.0),
                speed: rand::gen_range(20.0, 100.0),
            });
        }

        Self {
            screen_shake: 0.0,
            stars,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update screen shake
        if self.screen_shake > 0.0 {
            self.screen_shake -= dt * 10.0;
            if self.screen_shake < 0.0 {
                self.screen_shake = 0.0;
            }
        }

        // Update stars
        for star in &mut self.stars {
            star.position.y += star.speed * dt;
            if star.position.y > 900.0 {
                star.position.y = 0.0;
                star.position.x = rand::gen_range(0.0, 800.0);
            }
        }
    }

    pub fn add_screen_shake(&mut self, amount: f32) {
        self.screen_shake = (self.screen_shake + amount).min(10.0);
    }

    pub fn get_shake_offset(&self) -> Vec2 {
        if self.screen_shake > 0.0 {
            vec2(
                rand::gen_range(-self.screen_shake, self.screen_shake),
                rand::gen_range(-self.screen_shake, self.screen_shake),
            )
        } else {
            Vec2::ZERO
        }
    }

    pub fn render_background(&self) {
        // Render stars
        for star in &self.stars {
            let brightness = star.size / 3.0;
            draw_circle(
                star.position.x,
                star.position.y,
                star.size,
                Color::new(brightness, brightness, brightness, 1.0),
            );
        }
    }

    pub fn render_player(&self, player: &Player, shake_offset: Vec2) {
        if !player.is_alive {
            return;
        }

        let pos = player.position + shake_offset;

        // Flicker when invulnerable
        if player.invulnerable_timer > 0.0 {
            if (player.invulnerable_timer * 10.0) as i32 % 2 == 0 {
                return;
            }
        }

        // Draw player ship (triangle)
        let p1 = pos + vec2(0.0, -20.0);
        let p2 = pos + vec2(-15.0, 20.0);
        let p3 = pos + vec2(15.0, 20.0);
        draw_triangle(p1, p2, p3, SKYBLUE);

        // Draw cockpit
        draw_circle(pos.x, pos.y, 8.0, BLUE);

        // Draw engine glow
        draw_circle(pos.x - 10.0, pos.y + 15.0, 4.0, Color::new(1.0, 0.5, 0.0, 0.8));
        draw_circle(pos.x + 10.0, pos.y + 15.0, 4.0, Color::new(1.0, 0.5, 0.0, 0.8));

        // Draw shield if active
        if player.shield > 0 {
            let alpha = (player.shield as f32 / player.max_shield as f32) * 0.5;
            draw_circle_lines(pos.x, pos.y, 25.0, 2.0, Color::new(0.0, 1.0, 1.0, alpha));
        }
    }

    pub fn render_enemy(&self, enemy: &Enemy, shake_offset: Vec2) {
        let pos = enemy.position + shake_offset;
        let health_percent = enemy.health as f32 / enemy.max_health as f32;

        match enemy.enemy_type {
            EnemyType::BasicFighter => {
                // Triangle enemy
                let p1 = pos + vec2(0.0, 15.0);
                let p2 = pos + vec2(-12.0, -15.0);
                let p3 = pos + vec2(12.0, -15.0);
                draw_triangle(p1, p2, p3, RED);
                draw_circle(pos.x, pos.y, 6.0, DARKPURPLE);
            }
            EnemyType::HeavyCruiser => {
                // Rectangle enemy
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, MAROON);
                draw_rectangle(pos.x - 20.0, pos.y - 20.0, 40.0, 40.0, RED);
                draw_circle(pos.x, pos.y, 10.0, ORANGE);
            }
            EnemyType::Kamikaze => {
                // Diamond enemy
                let p1 = pos + vec2(0.0, -12.0);
                let p2 = pos + vec2(-12.0, 0.0);
                let p3 = pos + vec2(0.0, 12.0);
                let p4 = pos + vec2(12.0, 0.0);
                draw_triangle(p1, p2, p3, ORANGE);
                draw_triangle(p1, p3, p4, ORANGE);
            }
            EnemyType::Boss1 => {
                // Large boss
                draw_circle(pos.x, pos.y, 50.0, DARKPURPLE);
                draw_circle(pos.x, pos.y, 40.0, PURPLE);
                draw_circle(pos.x, pos.y, 25.0, RED);

                // Boss eyes
                draw_circle(pos.x - 15.0, pos.y - 10.0, 8.0, YELLOW);
                draw_circle(pos.x + 15.0, pos.y - 10.0, 8.0, YELLOW);
                draw_circle(pos.x - 15.0, pos.y - 10.0, 4.0, RED);
                draw_circle(pos.x + 15.0, pos.y - 10.0, 4.0, RED);
            }
            EnemyType::Boss2 => {
                // Ultra boss
                draw_rectangle(pos.x - 60.0, pos.y - 60.0, 120.0, 120.0, DARKPURPLE);
                draw_rectangle(pos.x - 50.0, pos.y - 50.0, 100.0, 100.0, PURPLE);
                draw_rectangle(pos.x - 35.0, pos.y - 35.0, 70.0, 70.0, RED);

                // Boss core
                draw_circle(pos.x, pos.y, 20.0, ORANGE);
                draw_circle(pos.x, pos.y, 12.0, YELLOW);
            }
        }

        // Health bar for bosses and heavy cruisers
        if matches!(
            enemy.enemy_type,
            EnemyType::Boss1 | EnemyType::Boss2 | EnemyType::HeavyCruiser
        ) {
            let bar_width = enemy.size.x * 1.2;
            let bar_height = 6.0;
            let bar_x = pos.x - bar_width / 2.0;
            let bar_y = pos.y - enemy.size.y / 2.0 - 15.0;

            draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);
            draw_rectangle(bar_x, bar_y, bar_width * health_percent, bar_height, RED);
        }
    }

    pub fn render_bullet(&self, bullet: &Bullet, shake_offset: Vec2) {
        let pos = bullet.position + shake_offset;

        match bullet.bullet_type {
            BulletType::Normal => {
                draw_rectangle(
                    pos.x - bullet.size.x / 2.0,
                    pos.y - bullet.size.y / 2.0,
                    bullet.size.x,
                    bullet.size.y,
                    YELLOW,
                );
            }
            BulletType::Spread => {
                draw_rectangle(
                    pos.x - bullet.size.x / 2.0,
                    pos.y - bullet.size.y / 2.0,
                    bullet.size.x,
                    bullet.size.y,
                    ORANGE,
                );
            }
            BulletType::Laser => {
                draw_rectangle(
                    pos.x - bullet.size.x / 2.0,
                    pos.y - bullet.size.y / 2.0,
                    bullet.size.x,
                    bullet.size.y,
                    SKYBLUE,
                );
                draw_rectangle(
                    pos.x - bullet.size.x / 4.0,
                    pos.y - bullet.size.y / 2.0,
                    bullet.size.x / 2.0,
                    bullet.size.y,
                    WHITE,
                );
            }
            BulletType::Missile => {
                draw_rectangle(
                    pos.x - bullet.size.x / 2.0,
                    pos.y - bullet.size.y / 2.0,
                    bullet.size.x,
                    bullet.size.y,
                    RED,
                );
                draw_circle(pos.x, pos.y + bullet.size.y / 2.0, 3.0, ORANGE);
            }
            BulletType::Enemy => {
                draw_circle(pos.x, pos.y, bullet.size.x, RED);
                draw_circle(pos.x, pos.y, bullet.size.x * 0.6, ORANGE);
            }
        }
    }

    pub fn render_powerup(&self, powerup: &PowerUp, shake_offset: Vec2) {
        let pos = powerup.position + shake_offset;
        let pulse = (powerup.lifetime * 5.0).sin() * 0.2 + 1.0;

        let (color, symbol) = match powerup.powerup_type {
            PowerUpType::Health => (GREEN, "+"),
            PowerUpType::Shield => (SKYBLUE, "S"),
            PowerUpType::WeaponUpgrade => (YELLOW, "W"),
            PowerUpType::ScoreMultiplier => (PURPLE, "X"),
        };

        draw_circle(pos.x, pos.y, powerup.size.x / 2.0 * pulse, color);
        draw_text(
            symbol,
            pos.x - 8.0,
            pos.y + 8.0,
            30.0,
            WHITE,
        );
    }

    pub fn render_particle(&self, particle: &Particle, shake_offset: Vec2) {
        let pos = particle.position + shake_offset;
        let alpha = particle.alpha();
        let mut color = particle.color;
        color.a = alpha;

        draw_circle(pos.x, pos.y, particle.size, color);
    }

    pub fn render_hud(&self, player: &Player, score: u32, combo: u32, wave: usize, high_score: u32) {
        // Score
        draw_text(&format!("SCORE: {}", score), 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("HIGH: {}", high_score), 10.0, 60.0, 20.0, GRAY);

        // Combo
        if combo > 1 {
            let combo_color = if combo > 10 {
                RED
            } else if combo > 5 {
                ORANGE
            } else {
                YELLOW
            };
            draw_text(&format!("COMBO x{}", combo), 10.0, 90.0, 25.0, combo_color);
        }

        // Wave
        draw_text(&format!("WAVE {}", wave), 650.0, 30.0, 30.0, WHITE);

        // Health bar
        let health_percent = player.health as f32 / player.max_health as f32;
        draw_rectangle(10.0, 850.0, 200.0, 20.0, DARKGRAY);
        draw_rectangle(10.0, 850.0, 200.0 * health_percent, 20.0, GREEN);
        draw_text("HEALTH", 10.0, 845.0, 20.0, WHITE);

        // Shield bar
        let shield_percent = player.shield as f32 / player.max_shield as f32;
        draw_rectangle(220.0, 850.0, 200.0, 20.0, DARKGRAY);
        draw_rectangle(220.0, 850.0, 200.0 * shield_percent, 20.0, SKYBLUE);
        draw_text("SHIELD", 220.0, 845.0, 20.0, WHITE);

        // Lives
        for i in 0..player.lives.max(0) as usize {
            let x = 650.0 + (i as f32 * 30.0);
            draw_triangle(
                vec2(x + 10.0, 860.0),
                vec2(x, 875.0),
                vec2(x + 20.0, 875.0),
                SKYBLUE,
            );
        }
        draw_text("LIVES", 650.0, 845.0, 20.0, WHITE);

        // Weapon level
        draw_text(
            &format!("WEAPON LVL {}", player.weapon_level + 1),
            430.0,
            30.0,
            20.0,
            YELLOW,
        );
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
