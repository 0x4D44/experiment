use crate::entities::*;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub spawn_x: f32,
    pub spawn_y: f32,
    pub platforms: Vec<PlatformData>,
    pub enemies: Vec<EnemyData>,
    pub collectibles: Vec<CollectibleData>,
    pub checkpoints: Vec<CheckpointData>,
    pub goal_x: f32,
    pub goal_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub platform_type: PlatformType,
    pub end_x: Option<f32>,
    pub end_y: Option<f32>,
    pub speed: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyData {
    pub x: f32,
    pub y: f32,
    pub enemy_type: EnemyType,
    pub patrol_start: Option<f32>,
    pub patrol_end: Option<f32>,
    pub radius: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectibleData {
    pub x: f32,
    pub y: f32,
    pub collectible_type: CollectibleType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointData {
    pub x: f32,
    pub y: f32,
}

pub struct Level {
    pub data: LevelData,
    pub platforms: Vec<Platform>,
    pub enemies: Vec<Enemy>,
    pub collectibles: Vec<Collectible>,
    pub checkpoints: Vec<Checkpoint>,
    pub completed: bool,
    pub last_checkpoint: usize,
}

impl Level {
    pub fn from_data(data: LevelData) -> Self {
        let platforms = data
            .platforms
            .iter()
            .map(|p| {
                if p.platform_type == PlatformType::Moving {
                    Platform::new_moving(
                        p.x,
                        p.y,
                        p.width,
                        p.height,
                        p.end_x.unwrap_or(p.x),
                        p.end_y.unwrap_or(p.y),
                        p.speed.unwrap_or(50.0),
                    )
                } else {
                    Platform::new(p.x, p.y, p.width, p.height, p.platform_type)
                }
            })
            .collect();

        let enemies = data
            .enemies
            .iter()
            .map(|e| match e.enemy_type {
                EnemyType::Walker => Enemy::new_walker(
                    e.x,
                    e.y,
                    e.patrol_start.unwrap_or(e.x - 100.0),
                    e.patrol_end.unwrap_or(e.x + 100.0),
                ),
                EnemyType::Flyer => Enemy::new_flyer(e.x, e.y, e.radius.unwrap_or(50.0)),
                EnemyType::Patroller => Enemy::new_patroller(
                    e.x,
                    e.y,
                    e.patrol_start.unwrap_or(e.x - 150.0),
                    e.patrol_end.unwrap_or(e.x + 150.0),
                ),
            })
            .collect();

        let collectibles = data
            .collectibles
            .iter()
            .map(|c| Collectible::new(c.x, c.y, c.collectible_type))
            .collect();

        let checkpoints = data
            .checkpoints
            .iter()
            .map(|c| Checkpoint::new(c.x, c.y))
            .collect();

        Self {
            data,
            platforms,
            enemies,
            collectibles,
            checkpoints,
            completed: false,
            last_checkpoint: 0,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read level file: {}", e))?;
        let data: LevelData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse level JSON: {}", e))?;
        Ok(Self::from_data(data))
    }

    pub fn update(&mut self, delta_time: f32) {
        for platform in &mut self.platforms {
            platform.update(delta_time);
        }

        for enemy in &mut self.enemies {
            enemy.update(delta_time);
        }

        for collectible in &mut self.collectibles {
            collectible.update(delta_time);
        }

        for checkpoint in &mut self.checkpoints {
            checkpoint.update(delta_time);
        }
    }

    pub fn reset(&mut self) {
        // Reset all enemies
        for (i, enemy_data) in self.data.enemies.iter().enumerate() {
            if i < self.enemies.len() {
                let enemy = match enemy_data.enemy_type {
                    EnemyType::Walker => Enemy::new_walker(
                        enemy_data.x,
                        enemy_data.y,
                        enemy_data.patrol_start.unwrap_or(enemy_data.x - 100.0),
                        enemy_data.patrol_end.unwrap_or(enemy_data.x + 100.0),
                    ),
                    EnemyType::Flyer => Enemy::new_flyer(
                        enemy_data.x,
                        enemy_data.y,
                        enemy_data.radius.unwrap_or(50.0),
                    ),
                    EnemyType::Patroller => Enemy::new_patroller(
                        enemy_data.x,
                        enemy_data.y,
                        enemy_data.patrol_start.unwrap_or(enemy_data.x - 150.0),
                        enemy_data.patrol_end.unwrap_or(enemy_data.x + 150.0),
                    ),
                };
                self.enemies[i] = enemy;
            }
        }

        // Reset collectibles
        for collectible in &mut self.collectibles {
            collectible.collected = false;
        }
    }

    pub fn get_spawn_point(&self) -> (f32, f32) {
        if self.last_checkpoint > 0 && self.last_checkpoint <= self.checkpoints.len() {
            let cp = &self.checkpoints[self.last_checkpoint - 1];
            (cp.position.x, cp.position.y - 50.0)
        } else {
            (self.data.spawn_x, self.data.spawn_y)
        }
    }

    pub fn draw(&self) {
        // Draw platforms
        for platform in &self.platforms {
            platform.draw();
        }

        // Draw enemies
        for enemy in &self.enemies {
            enemy.draw();
        }

        // Draw collectibles
        for collectible in &self.collectibles {
            collectible.draw();
        }

        // Draw checkpoints
        for checkpoint in &self.checkpoints {
            checkpoint.draw();
        }

        // Draw goal
        let glow = (get_time() * 2.0).sin() as f32 * 0.3 + 0.7;
        draw_circle(
            self.data.goal_x,
            self.data.goal_y,
            30.0,
            Color::new(1.0, 1.0, 0.0, glow),
        );
        draw_text(
            "GOAL",
            self.data.goal_x - 20.0,
            self.data.goal_y + 5.0,
            20.0,
            BLACK,
        );
    }
}
