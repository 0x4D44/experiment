use crate::physics::AABB;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PlatformType {
    Solid,
    Moving,
    Disappearing,
}

pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub platform_type: PlatformType,
    pub active: bool,

    // For moving platforms
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub move_speed: f32,
    pub moving_forward: bool,

    // For disappearing platforms
    pub disappear_timer: f32,
    pub respawn_timer: f32,
    pub triggered: bool,
}

impl Platform {
    pub fn new(x: f32, y: f32, width: f32, height: f32, platform_type: PlatformType) -> Self {
        Self {
            x,
            y,
            width,
            height,
            platform_type,
            active: true,
            start_pos: Vec2::new(x, y),
            end_pos: Vec2::new(x, y),
            move_speed: 50.0,
            moving_forward: true,
            disappear_timer: 0.0,
            respawn_timer: 0.0,
            triggered: false,
        }
    }

    pub fn new_moving(x: f32, y: f32, width: f32, height: f32, end_x: f32, end_y: f32, speed: f32) -> Self {
        let mut platform = Self::new(x, y, width, height, PlatformType::Moving);
        platform.start_pos = Vec2::new(x, y);
        platform.end_pos = Vec2::new(end_x, end_y);
        platform.move_speed = speed;
        platform
    }

    pub fn new_disappearing(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self::new(x, y, width, height, PlatformType::Disappearing)
    }

    pub fn update(&mut self, delta_time: f32) {
        match self.platform_type {
            PlatformType::Moving => {
                let current_pos = Vec2::new(self.x, self.y);
                let target_pos = if self.moving_forward {
                    self.end_pos
                } else {
                    self.start_pos
                };

                let direction = (target_pos - current_pos).normalize_or_zero();
                let movement = direction * self.move_speed * delta_time;

                self.x += movement.x;
                self.y += movement.y;

                // Check if reached target
                let distance_to_target = current_pos.distance(target_pos);
                if distance_to_target < 5.0 {
                    self.moving_forward = !self.moving_forward;
                }
            }
            PlatformType::Disappearing => {
                if self.triggered && self.active {
                    self.disappear_timer += delta_time;
                    if self.disappear_timer >= 0.8 {
                        self.active = false;
                        self.respawn_timer = 3.0;
                    }
                } else if !self.active {
                    self.respawn_timer -= delta_time;
                    if self.respawn_timer <= 0.0 {
                        self.active = true;
                        self.triggered = false;
                        self.disappear_timer = 0.0;
                    }
                }
            }
            PlatformType::Solid => {}
        }
    }

    pub fn trigger(&mut self) {
        if self.platform_type == PlatformType::Disappearing && !self.triggered {
            self.triggered = true;
        }
    }

    pub fn aabb(&self) -> AABB {
        AABB::new(self.x, self.y, self.width, self.height)
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }

        let color = match self.platform_type {
            PlatformType::Solid => DARKBROWN,
            PlatformType::Moving => ORANGE,
            PlatformType::Disappearing => {
                if self.triggered {
                    // Fade out effect
                    let alpha = 1.0 - (self.disappear_timer / 0.8).min(1.0);
                    Color::new(1.0, 0.0, 1.0, alpha)
                } else {
                    PURPLE
                }
            }
        };

        draw_rectangle(self.x, self.y, self.width, self.height, color);

        // Draw platform edge highlight
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0,
            Color::new(color.r * 0.7, color.g * 0.7, color.b * 0.7, color.a));
    }
}
