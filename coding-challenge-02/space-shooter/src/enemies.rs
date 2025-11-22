use crate::entities::Enemy;
use crate::weapons::{Bullet, BulletType};
use macroquad::prelude::*;

/// Enemy AI and behavior system
pub struct EnemySystem;

impl EnemySystem {
    /// Generate enemy bullet patterns based on enemy type
    pub fn generate_bullets(enemy: &Enemy) -> Vec<Bullet> {
        let mut bullets = vec![];

        match enemy.enemy_type {
            crate::entities::EnemyType::BasicFighter => {
                // Single shot
                bullets.push(Bullet::new(
                    enemy.position,
                    vec2(0.0, 300.0),
                    BulletType::Enemy,
                    false,
                ));
            }
            crate::entities::EnemyType::HeavyCruiser => {
                // Triple shot spread
                bullets.push(Bullet::new(
                    enemy.position,
                    vec2(0.0, 350.0),
                    BulletType::Enemy,
                    false,
                ));
                bullets.push(Bullet::new(
                    enemy.position,
                    vec2(-100.0, 350.0),
                    BulletType::Enemy,
                    false,
                ));
                bullets.push(Bullet::new(
                    enemy.position,
                    vec2(100.0, 350.0),
                    BulletType::Enemy,
                    false,
                ));
            }
            crate::entities::EnemyType::Kamikaze => {
                // No shooting, just kamikaze
            }
            crate::entities::EnemyType::Boss1 => {
                // Spiral pattern
                let base_angle = rand::gen_range(0.0, std::f32::consts::TAU);
                for i in 0..8 {
                    let angle = base_angle + (i as f32 * std::f32::consts::TAU / 8.0);
                    let velocity = vec2(angle.cos() * 300.0, angle.sin() * 300.0);
                    bullets.push(Bullet::new(
                        enemy.position,
                        velocity,
                        BulletType::Enemy,
                        false,
                    ));
                }
            }
            crate::entities::EnemyType::Boss2 => {
                // Bullet hell pattern
                let base_angle = rand::gen_range(0.0, std::f32::consts::TAU);
                for i in 0..16 {
                    let angle = base_angle + (i as f32 * std::f32::consts::TAU / 16.0);
                    let velocity = vec2(angle.cos() * 250.0, angle.sin() * 250.0);
                    bullets.push(Bullet::new(
                        enemy.position,
                        velocity,
                        BulletType::Enemy,
                        false,
                    ));
                }
            }
        }

        bullets
    }
}
