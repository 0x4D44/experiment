use crate::physics::{PhysicsBody, AABB};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EnemyType {
    Walker,
    Flyer,
    Patroller,
}

pub struct Enemy {
    pub body: PhysicsBody,
    pub enemy_type: EnemyType,
    pub alive: bool,
    pub facing_right: bool,

    // For walkers and patrollers
    pub patrol_start: f32,
    pub patrol_end: f32,
    pub move_speed: f32,

    // For flyers
    pub fly_center: Vec2,
    pub fly_radius: f32,
    pub fly_angle: f32,
    pub fly_speed: f32,

    pub animation_timer: f32,
    pub animation_frame: usize,
}

impl Enemy {
    pub fn new_walker(x: f32, y: f32, patrol_start: f32, patrol_end: f32) -> Self {
        let mut body = PhysicsBody::new(x, y, 24.0, 24.0);
        body.gravity_scale = 1.0;

        Self {
            body,
            enemy_type: EnemyType::Walker,
            alive: true,
            facing_right: true,
            patrol_start,
            patrol_end,
            move_speed: 60.0,
            fly_center: Vec2::ZERO,
            fly_radius: 0.0,
            fly_angle: 0.0,
            fly_speed: 0.0,
            animation_timer: 0.0,
            animation_frame: 0,
        }
    }

    pub fn new_flyer(center_x: f32, center_y: f32, radius: f32) -> Self {
        let mut body = PhysicsBody::new(center_x, center_y, 20.0, 20.0);
        body.gravity_scale = 0.0;

        Self {
            body,
            enemy_type: EnemyType::Flyer,
            alive: true,
            facing_right: true,
            patrol_start: 0.0,
            patrol_end: 0.0,
            move_speed: 0.0,
            fly_center: Vec2::new(center_x, center_y),
            fly_radius: radius,
            fly_angle: 0.0,
            fly_speed: 2.0,
            animation_timer: 0.0,
            animation_frame: 0,
        }
    }

    pub fn new_patroller(x: f32, y: f32, patrol_start: f32, patrol_end: f32) -> Self {
        let mut body = PhysicsBody::new(x, y, 28.0, 28.0);
        body.gravity_scale = 1.0;

        Self {
            body,
            enemy_type: EnemyType::Patroller,
            alive: true,
            facing_right: true,
            patrol_start,
            patrol_end,
            move_speed: 80.0,
            fly_center: Vec2::ZERO,
            fly_radius: 0.0,
            fly_angle: 0.0,
            fly_speed: 0.0,
            animation_timer: 0.0,
            animation_frame: 0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.alive {
            return;
        }

        // Update animation
        self.animation_timer += delta_time;
        if self.animation_timer > 0.15 {
            self.animation_timer = 0.0;
            self.animation_frame = (self.animation_frame + 1) % 3;
        }

        match self.enemy_type {
            EnemyType::Walker | EnemyType::Patroller => {
                // Horizontal patrol movement
                if self.facing_right {
                    self.body.velocity.x = self.move_speed;
                    if self.body.position.x >= self.patrol_end {
                        self.facing_right = false;
                    }
                } else {
                    self.body.velocity.x = -self.move_speed;
                    if self.body.position.x <= self.patrol_start {
                        self.facing_right = true;
                    }
                }
            }
            EnemyType::Flyer => {
                // Circular flying pattern
                self.fly_angle += self.fly_speed * delta_time;
                let target_x = self.fly_center.x + self.fly_radius * self.fly_angle.cos();
                let target_y = self.fly_center.y + self.fly_radius * self.fly_angle.sin();

                self.body.position.x = target_x;
                self.body.position.y = target_y;

                self.facing_right = self.fly_angle.cos() > 0.0;
            }
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn aabb(&self) -> AABB {
        self.body.aabb()
    }

    pub fn draw(&self) {
        if !self.alive {
            return;
        }

        let pos = self.body.position;
        let size = self.body.size;

        let color = match self.enemy_type {
            EnemyType::Walker => RED,
            EnemyType::Flyer => PINK,
            EnemyType::Patroller => MAROON,
        };

        // Draw enemy body
        draw_rectangle(pos.x, pos.y, size.x, size.y, color);

        // Draw eyes
        let eye_offset = if self.facing_right { size.x * 0.6 } else { size.x * 0.3 };
        draw_circle(pos.x + eye_offset, pos.y + size.y * 0.3, 2.0, YELLOW);

        // Draw type-specific features
        match self.enemy_type {
            EnemyType::Flyer => {
                // Draw wings
                let wing_y = pos.y + size.y * 0.5;
                draw_line(pos.x, wing_y, pos.x - 5.0, wing_y - 5.0, 2.0, LIGHTGRAY);
                draw_line(pos.x + size.x, wing_y, pos.x + size.x + 5.0, wing_y - 5.0, 2.0, LIGHTGRAY);
            }
            EnemyType::Patroller => {
                // Draw spikes on top
                let spike_count = 3;
                for i in 0..spike_count {
                    let spike_x = pos.x + (i as f32 + 0.5) * size.x / spike_count as f32;
                    draw_triangle(
                        Vec2::new(spike_x - 3.0, pos.y),
                        Vec2::new(spike_x + 3.0, pos.y),
                        Vec2::new(spike_x, pos.y - 6.0),
                        DARKGRAY,
                    );
                }
            }
            _ => {}
        }
    }
}
