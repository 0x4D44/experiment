use crate::physics::AABB;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CollectibleType {
    Coin,
    Gem,
    HealthPack,
    ExtraLife,
    DoubleJump,
}

pub struct Collectible {
    pub position: Vec2,
    pub collectible_type: CollectibleType,
    pub collected: bool,
    pub animation_timer: f32,
    pub bob_offset: f32,
    pub size: f32,
}

impl Collectible {
    pub fn new(x: f32, y: f32, collectible_type: CollectibleType) -> Self {
        let size = match collectible_type {
            CollectibleType::Coin => 12.0,
            CollectibleType::Gem => 16.0,
            CollectibleType::HealthPack => 14.0,
            CollectibleType::ExtraLife => 18.0,
            CollectibleType::DoubleJump => 14.0,
        };

        Self {
            position: Vec2::new(x, y),
            collectible_type,
            collected: false,
            animation_timer: 0.0,
            bob_offset: 0.0,
            size,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.collected {
            return;
        }

        self.animation_timer += delta_time * 3.0;
        self.bob_offset = self.animation_timer.sin() * 5.0;
    }

    pub fn aabb(&self) -> AABB {
        AABB::new(
            self.position.x - self.size / 2.0,
            self.position.y - self.size / 2.0 + self.bob_offset,
            self.size,
            self.size,
        )
    }

    pub fn collect(&mut self) {
        self.collected = true;
    }

    pub fn score_value(&self) -> i32 {
        match self.collectible_type {
            CollectibleType::Coin => 100,
            CollectibleType::Gem => 500,
            CollectibleType::HealthPack => 50,
            CollectibleType::ExtraLife => 1000,
            CollectibleType::DoubleJump => 200,
        }
    }

    pub fn draw(&self) {
        if self.collected {
            return;
        }

        let x = self.position.x;
        let y = self.position.y + self.bob_offset;
        let rotation = self.animation_timer;

        match self.collectible_type {
            CollectibleType::Coin => {
                // Draw spinning coin
                let scale = (rotation.cos() * 0.5 + 0.5).max(0.2);
                draw_circle(x, y, self.size * scale, GOLD);
                draw_circle_lines(x, y, self.size * scale, 2.0, ORANGE);
            }
            CollectibleType::Gem => {
                // Draw diamond shape
                let points = [
                    Vec2::new(x, y - self.size),
                    Vec2::new(x + self.size * 0.7, y),
                    Vec2::new(x, y + self.size),
                    Vec2::new(x - self.size * 0.7, y),
                ];

                for i in 0..4 {
                    let next = (i + 1) % 4;
                    draw_line(points[i].x, points[i].y, points[next].x, points[next].y, 3.0, BLUE);
                }
                draw_circle(x, y, self.size * 0.3, SKYBLUE);
            }
            CollectibleType::HealthPack => {
                // Draw health cross
                draw_rectangle(x - self.size * 0.5, y - self.size * 0.2,
                    self.size, self.size * 0.4, RED);
                draw_rectangle(x - self.size * 0.2, y - self.size * 0.5,
                    self.size * 0.4, self.size, RED);
                draw_rectangle_lines(x - self.size * 0.5, y - self.size * 0.5,
                    self.size, self.size, 2.0, WHITE);
            }
            CollectibleType::ExtraLife => {
                // Draw heart
                draw_circle(x - self.size * 0.25, y - self.size * 0.2, self.size * 0.4, PINK);
                draw_circle(x + self.size * 0.25, y - self.size * 0.2, self.size * 0.4, PINK);
                draw_triangle(
                    Vec2::new(x - self.size * 0.6, y - self.size * 0.1),
                    Vec2::new(x + self.size * 0.6, y - self.size * 0.1),
                    Vec2::new(x, y + self.size * 0.6),
                    PINK,
                );
            }
            CollectibleType::DoubleJump => {
                // Draw boot with upward arrow
                draw_rectangle(x - self.size * 0.4, y - self.size * 0.2,
                    self.size * 0.8, self.size * 0.6, DARKGREEN);
                draw_triangle(
                    Vec2::new(x, y - self.size * 0.7),
                    Vec2::new(x - self.size * 0.3, y - self.size * 0.2),
                    Vec2::new(x + self.size * 0.3, y - self.size * 0.2),
                    GREEN,
                );
            }
        }

        // Draw glow effect
        let glow_radius = self.size + (rotation.sin() * 2.0).abs();
        let glow_color = match self.collectible_type {
            CollectibleType::Coin => Color::new(1.0, 0.84, 0.0, 0.2),
            CollectibleType::Gem => Color::new(0.0, 0.5, 1.0, 0.2),
            CollectibleType::HealthPack => Color::new(1.0, 0.0, 0.0, 0.2),
            CollectibleType::ExtraLife => Color::new(1.0, 0.7, 0.8, 0.2),
            CollectibleType::DoubleJump => Color::new(0.0, 1.0, 0.0, 0.2),
        };
        draw_circle(x, y, glow_radius, glow_color);
    }
}
