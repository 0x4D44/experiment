use crate::audio::AudioSystem;
use crate::collision;
use crate::enemies::EnemySystem;
use crate::entities::{Enemy, Player};
use crate::particles::ParticleSystem;
use crate::powerups::PowerUp;
use crate::rendering::Renderer;
use crate::score::ScoreSystem;
use crate::state::GameState;
use crate::waves::WaveSystem;
use crate::weapons::{Bullet, WeaponSystem};
use macroquad::prelude::*;

/// Main game state and logic
pub struct Game {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    powerups: Vec<PowerUp>,
    weapon_system: WeaponSystem,
    particle_system: ParticleSystem,
    wave_system: WaveSystem,
    score_system: ScoreSystem,
    audio_system: AudioSystem,
    renderer: Renderer,
    game_over_timer: f32,
    wave_clear_timer: f32,
    next_wave_delay: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Menu,
            player: Player::new(),
            enemies: Vec::new(),
            bullets: Vec::new(),
            powerups: Vec::new(),
            weapon_system: WeaponSystem::new(),
            particle_system: ParticleSystem::new(),
            wave_system: WaveSystem::new(),
            score_system: ScoreSystem::new(),
            audio_system: AudioSystem::new(),
            renderer: Renderer::new(),
            game_over_timer: 0.0,
            wave_clear_timer: 0.0,
            next_wave_delay: 3.0,
        }
    }

    pub fn handle_input(&mut self) {
        match self.state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    self.start_game();
                }
            }
            GameState::Playing => {
                self.handle_playing_input();
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    self.start_game();
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            }
            GameState::Victory => {
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    self.start_game();
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            }
        }
    }

    fn handle_playing_input(&mut self) {
        // Pause
        if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Paused;
            return;
        }

        // Movement - 8 directional
        let mut direction = Vec2::ZERO;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            self.player.move_direction(direction);
        }

        // Shooting
        if is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
            let new_bullets = self.weapon_system.fire(self.player.position, self.player.weapon_level);
            if !new_bullets.is_empty() {
                self.bullets.extend(new_bullets);
                self.audio_system.play_shoot();

                // Engine trail when shooting
                self.particle_system.create_trail(
                    self.player.position + vec2(-10.0, 15.0),
                    Color::new(0.0, 0.5, 1.0, 0.8),
                );
                self.particle_system.create_trail(
                    self.player.position + vec2(10.0, 15.0),
                    Color::new(0.0, 0.5, 1.0, 0.8),
                );
            }
        }
    }

    fn start_game(&mut self) {
        self.state = GameState::Playing;
        self.player = Player::new();
        self.enemies.clear();
        self.bullets.clear();
        self.powerups.clear();
        self.particle_system.clear();
        self.wave_system = WaveSystem::new();
        self.score_system.reset();
        self.game_over_timer = 0.0;
        self.wave_clear_timer = 0.0;

        // Start wave 1
        self.wave_system.start_wave(1);
    }

    pub fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Menu | GameState::Paused | GameState::GameOver | GameState::Victory => {
                // Update background effects only
                self.renderer.update(dt);
                self.particle_system.update(dt);
                return;
            }
            GameState::Playing => {
                self.update_playing(dt);
            }
        }
    }

    fn update_playing(&mut self, dt: f32) {
        // Update systems
        self.player.update(dt);
        self.weapon_system.update(dt);
        self.wave_system.update(dt);
        self.score_system.update(dt);
        self.renderer.update(dt);
        self.particle_system.update(dt);

        // Spawn enemies
        if self.wave_system.should_spawn() {
            if let Some(enemy) = self.wave_system.spawn_enemy() {
                let is_boss = matches!(
                    enemy.enemy_type,
                    crate::entities::EnemyType::Boss1 | crate::entities::EnemyType::Boss2
                );
                self.enemies.push(enemy);

                // Boss warning
                if is_boss {
                    self.audio_system.play_boss_warning();
                }
            }
        }

        // Update enemies
        for enemy in &mut self.enemies {
            enemy.update(dt, self.player.position);

            // Enemy shooting
            if enemy.can_shoot() {
                enemy.reset_shoot_timer();
                let enemy_bullets = EnemySystem::generate_bullets(enemy);
                self.bullets.extend(enemy_bullets);
            }
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }

        // Update powerups
        for powerup in &mut self.powerups {
            powerup.update(dt);
        }

        // Collision detection
        self.check_collisions();

        // Cleanup
        self.bullets.retain(|b| !b.is_off_screen());
        self.enemies.retain(|e| e.is_alive && !e.is_off_screen());
        self.powerups.retain(|p| !p.is_expired());

        // Check wave completion
        if self.wave_system.check_wave_complete(self.enemies.len()) {
            self.wave_clear_timer = 0.0;
        }

        // Handle wave transitions
        if self.wave_system.wave_cleared {
            self.wave_clear_timer += dt;
            if self.wave_clear_timer >= self.next_wave_delay {
                if self.wave_system.current_wave >= 10 {
                    self.state = GameState::Victory;
                    self.score_system.save_high_score();
                } else {
                    self.wave_system.start_wave(self.wave_system.current_wave + 1);
                }
            }
        }

        // Check game over
        if !self.player.is_alive {
            self.game_over_timer += dt;
            if self.game_over_timer >= 3.0 {
                self.state = GameState::GameOver;
                self.score_system.save_high_score();
            }
        }

        // Always spawn engine trail particles
        if self.player.is_alive && rand::gen_range(0, 3) == 0 {
            self.particle_system.create_trail(
                self.player.position + vec2(-10.0, 15.0),
                Color::new(1.0, 0.5, 0.0, 0.6),
            );
            self.particle_system.create_trail(
                self.player.position + vec2(10.0, 15.0),
                Color::new(1.0, 0.5, 0.0, 0.6),
            );
        }
    }

    fn check_collisions(&mut self) {
        // Player bullets vs enemies
        let mut bullets_to_remove = Vec::new();
        let mut enemies_to_damage: Vec<(usize, i32)> = Vec::new();

        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            if !bullet.is_player_bullet {
                continue;
            }

            for (enemy_idx, enemy) in self.enemies.iter().enumerate() {
                if collision::check_collision(bullet.get_rect(), enemy.get_rect()) {
                    bullets_to_remove.push(bullet_idx);
                    enemies_to_damage.push((enemy_idx, bullet.damage));
                    break;
                }
            }
        }

        // Apply damage to enemies
        for (enemy_idx, damage) in enemies_to_damage {
            if let Some(enemy) = self.enemies.get_mut(enemy_idx) {
                let killed = enemy.take_damage(damage);
                self.audio_system.play_hit();
                self.particle_system.create_impact(enemy.position, 5);

                if killed {
                    self.score_system.add_kill(enemy.score_value);
                    self.audio_system.play_explosion();
                    self.particle_system.create_explosion(enemy.position, 30, ORANGE);
                    self.renderer.add_screen_shake(3.0);

                    // Chance to drop powerup
                    if rand::gen_range(0, 100) < 15 {
                        self.powerups.push(PowerUp::random(enemy.position));
                    }
                }
            }
        }

        // Remove hit bullets
        bullets_to_remove.sort_unstable();
        bullets_to_remove.dedup();
        for &idx in bullets_to_remove.iter().rev() {
            if idx < self.bullets.len() {
                self.bullets.remove(idx);
            }
        }

        // Enemy bullets vs player
        self.bullets.retain(|bullet| {
            if bullet.is_player_bullet {
                return true;
            }

            if collision::check_collision(bullet.get_rect(), self.player.get_rect()) {
                if self.player.take_damage(bullet.damage) {
                    self.audio_system.play_explosion();
                    self.particle_system.create_explosion(self.player.position, 40, SKYBLUE);
                    self.renderer.add_screen_shake(5.0);
                }
                return false;
            }
            true
        });

        // Enemies vs player (collision damage)
        for enemy in &mut self.enemies {
            if collision::check_collision(enemy.get_rect(), self.player.get_rect()) {
                if self.player.take_damage(30) {
                    self.audio_system.play_explosion();
                    self.particle_system.create_explosion(self.player.position, 40, SKYBLUE);
                    self.renderer.add_screen_shake(5.0);
                }

                // Kamikaze enemies die on impact
                if matches!(enemy.enemy_type, crate::entities::EnemyType::Kamikaze) {
                    enemy.is_alive = false;
                    self.particle_system.create_explosion(enemy.position, 20, RED);
                }
            }
        }

        // Powerups vs player
        let player_rect = self.player.get_rect();
        let mut collected_powerups = Vec::new();

        for (idx, powerup) in self.powerups.iter().enumerate() {
            if collision::check_collision(powerup.get_rect(), player_rect) {
                collected_powerups.push((idx, powerup.clone()));
            }
        }

        // Process collected powerups in reverse order to maintain indices
        for (idx, powerup) in collected_powerups.iter().rev() {
            self.apply_powerup(powerup);
            self.audio_system.play_powerup();
            self.particle_system.create_explosion(powerup.position, 15, GOLD);
            self.powerups.remove(*idx);
        }
    }

    fn apply_powerup(&mut self, powerup: &PowerUp) {
        match powerup.powerup_type {
            crate::powerups::PowerUpType::Health => {
                self.player.heal(50);
            }
            crate::powerups::PowerUpType::Shield => {
                self.player.recharge_shield(50);
            }
            crate::powerups::PowerUpType::WeaponUpgrade => {
                self.player.upgrade_weapon();
            }
            crate::powerups::PowerUpType::ScoreMultiplier => {
                self.score_system.set_multiplier(2.0, 10.0);
                self.score_system.add_score(1000);
            }
        }
    }

    pub fn render(&mut self) {
        let shake_offset = self.renderer.get_shake_offset();

        // Background
        self.renderer.render_background();

        // Particles (behind)
        for particle in &self.particle_system.particles {
            if particle.size < 4.0 {
                self.renderer.render_particle(particle, shake_offset);
            }
        }

        // Game entities
        match self.state {
            GameState::Menu => {
                self.render_menu();
            }
            GameState::Playing => {
                self.render_playing(shake_offset);
            }
            GameState::Paused => {
                self.render_playing(shake_offset);
                self.render_pause_overlay();
            }
            GameState::GameOver => {
                self.render_playing(shake_offset);
                self.render_game_over();
            }
            GameState::Victory => {
                self.render_playing(shake_offset);
                self.render_victory();
            }
        }
    }

    fn render_playing(&self, shake_offset: Vec2) {
        // Render bullets
        for bullet in &self.bullets {
            self.renderer.render_bullet(bullet, shake_offset);
        }

        // Render enemies
        for enemy in &self.enemies {
            self.renderer.render_enemy(enemy, shake_offset);
        }

        // Render powerups
        for powerup in &self.powerups {
            self.renderer.render_powerup(powerup, shake_offset);
        }

        // Render player
        self.renderer.render_player(&self.player, shake_offset);

        // Particles (front)
        for particle in &self.particle_system.particles {
            if particle.size >= 4.0 {
                self.renderer.render_particle(particle, shake_offset);
            }
        }

        // HUD
        self.renderer.render_hud(
            &self.player,
            self.score_system.score,
            self.score_system.combo,
            self.wave_system.current_wave,
            self.score_system.high_score,
        );

        // Wave clear message
        if self.wave_system.wave_cleared {
            let alpha = (1.0 - (self.wave_clear_timer / self.next_wave_delay)).max(0.0);
            draw_text(
                "WAVE CLEAR!",
                250.0,
                400.0,
                60.0,
                Color::new(0.0, 1.0, 0.0, alpha),
            );

            if self.wave_system.current_wave < 10 {
                draw_text(
                    &format!("Next Wave: {}", self.wave_system.current_wave + 1),
                    280.0,
                    450.0,
                    30.0,
                    Color::new(1.0, 1.0, 1.0, alpha),
                );
            }
        }
    }

    fn render_menu(&self) {
        draw_text("SPACE SHOOTER", 200.0, 300.0, 80.0, SKYBLUE);
        draw_text("Competition Edition", 280.0, 350.0, 30.0, WHITE);

        draw_text("CONTROLS:", 300.0, 450.0, 30.0, YELLOW);
        draw_text("WASD / Arrows - Move", 250.0, 490.0, 25.0, WHITE);
        draw_text("SPACE / Mouse - Shoot", 250.0, 520.0, 25.0, WHITE);
        draw_text("P / ESC - Pause", 250.0, 550.0, 25.0, WHITE);

        draw_text("Press SPACE to Start", 240.0, 650.0, 35.0, GREEN);

        // Animated features
        let pulse = (get_time() as f32 * 2.0).sin() * 0.2 + 1.0;
        draw_text("10 WAVES", 50.0, 750.0, 20.0 * pulse, ORANGE);
        draw_text("2 BOSSES", 250.0, 750.0, 20.0 * pulse, RED);
        draw_text("POWER-UPS", 450.0, 750.0, 20.0 * pulse, PURPLE);
        draw_text("COMBOS", 650.0, 750.0, 20.0 * pulse, YELLOW);
    }

    fn render_pause_overlay(&self) {
        draw_rectangle(0.0, 0.0, 800.0, 900.0, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_text("PAUSED", 300.0, 400.0, 60.0, WHITE);
        draw_text("Press P or ESC to Resume", 220.0, 500.0, 30.0, GRAY);
    }

    fn render_game_over(&self) {
        draw_rectangle(0.0, 0.0, 800.0, 900.0, Color::new(0.5, 0.0, 0.0, 0.7));
        draw_text("GAME OVER", 240.0, 400.0, 70.0, RED);
        draw_text(
            &format!("Final Score: {}", self.score_system.score),
            250.0,
            480.0,
            35.0,
            WHITE,
        );
        draw_text(
            &format!("High Score: {}", self.score_system.high_score),
            250.0,
            520.0,
            30.0,
            YELLOW,
        );
        draw_text("Press SPACE to Retry", 240.0, 600.0, 30.0, WHITE);
        draw_text("Press ESC for Menu", 250.0, 640.0, 25.0, GRAY);
    }

    fn render_victory(&self) {
        draw_rectangle(0.0, 0.0, 800.0, 900.0, Color::new(0.0, 0.5, 0.0, 0.7));
        draw_text("VICTORY!", 270.0, 350.0, 80.0, GOLD);
        draw_text("All Waves Complete!", 230.0, 430.0, 40.0, WHITE);
        draw_text(
            &format!("Final Score: {}", self.score_system.score),
            250.0,
            510.0,
            35.0,
            WHITE,
        );
        draw_text(
            &format!("High Score: {}", self.score_system.high_score),
            250.0,
            550.0,
            30.0,
            YELLOW,
        );
        draw_text("Press SPACE to Play Again", 210.0, 650.0, 30.0, WHITE);
        draw_text("Press ESC for Menu", 250.0, 690.0, 25.0, GRAY);
    }
}
