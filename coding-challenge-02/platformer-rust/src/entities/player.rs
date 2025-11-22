use crate::physics::{PhysicsBody, JUMP_VELOCITY, DOUBLE_JUMP_VELOCITY, PLAYER_SPEED};
use macroquad::prelude::*;

/// Player state and abilities
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerState {
    Idle,
    Running,
    Jumping,
    Falling,
    Dead,
}

pub struct Player {
    pub body: PhysicsBody,
    pub state: PlayerState,
    pub facing_right: bool,
    pub can_double_jump: bool,
    pub has_double_jumped: bool,
    pub health: i32,
    pub max_health: i32,
    pub lives: i32,
    pub score: i32,
    pub coins: i32,
    pub invulnerable: bool,
    pub invulnerable_timer: f32,
    pub animation_timer: f32,
    pub animation_frame: usize,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            body: PhysicsBody::new(x, y, 24.0, 32.0),
            state: PlayerState::Idle,
            facing_right: true,
            can_double_jump: true,
            has_double_jumped: false,
            health: 3,
            max_health: 3,
            lives: 3,
            score: 0,
            coins: 0,
            invulnerable: false,
            invulnerable_timer: 0.0,
            animation_timer: 0.0,
            animation_frame: 0,
        }
    }

    pub fn reset_position(&mut self, x: f32, y: f32) {
        self.body.position = Vec2::new(x, y);
        self.body.velocity = Vec2::ZERO;
        self.state = PlayerState::Idle;
        self.invulnerable = true;
        self.invulnerable_timer = 2.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update invulnerability timer
        if self.invulnerable {
            self.invulnerable_timer -= delta_time;
            if self.invulnerable_timer <= 0.0 {
                self.invulnerable = false;
            }
        }

        // Update animation
        self.animation_timer += delta_time;
        if self.animation_timer > 0.1 {
            self.animation_timer = 0.0;
            self.animation_frame = (self.animation_frame + 1) % 4;
        }

        // Update state based on velocity
        if self.state != PlayerState::Dead {
            if self.body.on_ground {
                self.has_double_jumped = false;
                if self.body.velocity.x.abs() > 10.0 {
                    self.state = PlayerState::Running;
                } else {
                    self.state = PlayerState::Idle;
                }
            } else {
                if self.body.velocity.y < 0.0 {
                    self.state = PlayerState::Jumping;
                } else {
                    self.state = PlayerState::Falling;
                }
            }
        }
    }

    pub fn handle_input(&mut self) {
        if self.state == PlayerState::Dead {
            return;
        }

        // Horizontal movement
        let mut move_x = 0.0;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            move_x -= 1.0;
            self.facing_right = false;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            move_x += 1.0;
            self.facing_right = true;
        }

        self.body.velocity.x = move_x * PLAYER_SPEED;

        // Jumping
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            if self.body.on_ground {
                self.body.velocity.y = JUMP_VELOCITY;
                self.has_double_jumped = false;
            } else if self.can_double_jump && !self.has_double_jumped {
                self.body.velocity.y = DOUBLE_JUMP_VELOCITY;
                self.has_double_jumped = true;
            }
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.invulnerable || self.state == PlayerState::Dead {
            return;
        }

        self.health -= damage;
        self.invulnerable = true;
        self.invulnerable_timer = 1.5;

        if self.health <= 0 {
            self.die();
        }
    }

    pub fn die(&mut self) {
        self.state = PlayerState::Dead;
        self.body.velocity = Vec2::new(0.0, -300.0);
        self.lives -= 1;
    }

    pub fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }

    pub fn respawn(&mut self, x: f32, y: f32) {
        self.health = self.max_health;
        self.state = PlayerState::Idle;
        self.reset_position(x, y);
    }

    pub fn add_score(&mut self, points: i32) {
        self.score += points;
    }

    pub fn collect_coin(&mut self) {
        self.coins += 1;
        self.add_score(100);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn add_life(&mut self) {
        self.lives += 1;
    }

    pub fn draw(&self) {
        let pos = self.body.position;
        let size = self.body.size;

        // Flicker when invulnerable
        if self.invulnerable && (self.invulnerable_timer * 10.0) as i32 % 2 == 0 {
            return;
        }

        let color = if self.state == PlayerState::Dead {
            GRAY
        } else {
            match self.state {
                PlayerState::Idle => BLUE,
                PlayerState::Running => SKYBLUE,
                PlayerState::Jumping => GREEN,
                PlayerState::Falling => YELLOW,
                PlayerState::Dead => GRAY,
            }
        };

        // Draw player body
        draw_rectangle(pos.x, pos.y, size.x, size.y, color);

        // Draw face direction indicator
        let eye_offset = if self.facing_right { size.x * 0.6 } else { size.x * 0.3 };
        draw_circle(pos.x + eye_offset, pos.y + size.y * 0.3, 3.0, WHITE);

        // Draw animation indicator (bobbing effect)
        let bob = (self.animation_frame as f32 * 0.5).sin() * 2.0;
        draw_circle(pos.x + size.x * 0.5, pos.y + bob - 5.0, 2.0, GOLD);
    }
}
