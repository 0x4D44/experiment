use crate::entities::{Enemy, EnemyType};
use macroquad::prelude::*;

/// Wave configuration and spawning system
#[derive(Debug, Clone)]
pub struct WaveSystem {
    pub current_wave: usize,
    pub wave_timer: f32,
    pub spawn_timer: f32,
    pub enemies_spawned: usize,
    pub enemies_to_spawn: usize,
    pub wave_active: bool,
    pub wave_cleared: bool,
}

impl WaveSystem {
    pub fn new() -> Self {
        Self {
            current_wave: 0,
            wave_timer: 0.0,
            spawn_timer: 0.0,
            enemies_spawned: 0,
            enemies_to_spawn: 0,
            wave_active: false,
            wave_cleared: false,
        }
    }

    pub fn start_wave(&mut self, wave_number: usize) {
        self.current_wave = wave_number;
        self.wave_timer = 0.0;
        self.spawn_timer = 0.0;
        self.enemies_spawned = 0;
        self.enemies_to_spawn = self.calculate_enemy_count(wave_number);
        self.wave_active = true;
        self.wave_cleared = false;
    }

    pub fn update(&mut self, dt: f32) {
        if self.wave_active {
            self.wave_timer += dt;
            self.spawn_timer += dt;
        }
    }

    fn calculate_enemy_count(&self, wave: usize) -> usize {
        // Boss waves
        if wave == 5 || wave == 10 {
            return 1;
        }
        // Regular waves - progressive difficulty
        10 + (wave * 3)
    }

    pub fn should_spawn(&mut self) -> bool {
        if !self.wave_active || self.enemies_spawned >= self.enemies_to_spawn {
            return false;
        }

        let spawn_delay = if self.current_wave == 5 || self.current_wave == 10 {
            5.0 // Delay before boss
        } else {
            0.5 - (self.current_wave as f32 * 0.03).min(0.3)
        };

        if self.spawn_timer >= spawn_delay {
            self.spawn_timer = 0.0;
            return true;
        }

        false
    }

    pub fn spawn_enemy(&mut self) -> Option<Enemy> {
        if self.enemies_spawned >= self.enemies_to_spawn {
            return None;
        }

        self.enemies_spawned += 1;

        let enemy_type = self.determine_enemy_type();
        let spawn_x = if matches!(enemy_type, EnemyType::Boss1 | EnemyType::Boss2) {
            400.0
        } else {
            rand::gen_range(50.0, 750.0)
        };

        Some(Enemy::new(enemy_type, vec2(spawn_x, -50.0)))
    }

    fn determine_enemy_type(&self) -> EnemyType {
        // Boss waves
        if self.current_wave == 5 {
            return EnemyType::Boss1;
        }
        if self.current_wave == 10 {
            return EnemyType::Boss2;
        }

        // Regular waves with progressive difficulty
        let roll = rand::gen_range(0, 100);
        let wave_factor = self.current_wave;

        if wave_factor >= 7 && roll < 10 {
            EnemyType::Kamikaze
        } else if wave_factor >= 4 && roll < 30 {
            EnemyType::HeavyCruiser
        } else if wave_factor >= 2 && roll < 15 {
            EnemyType::Kamikaze
        } else if roll < 70 {
            EnemyType::BasicFighter
        } else {
            EnemyType::HeavyCruiser
        }
    }

    pub fn check_wave_complete(&mut self, enemies_alive: usize) -> bool {
        if self.wave_active
            && self.enemies_spawned >= self.enemies_to_spawn
            && enemies_alive == 0
        {
            self.wave_active = false;
            self.wave_cleared = true;
            return true;
        }
        false
    }

    pub fn is_complete(&self) -> bool {
        self.current_wave > 10
    }
}

impl Default for WaveSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_initialization() {
        let mut waves = WaveSystem::new();
        waves.start_wave(1);
        assert_eq!(waves.current_wave, 1);
        assert!(waves.wave_active);
    }

    #[test]
    fn test_enemy_count_calculation() {
        let waves = WaveSystem::new();
        assert_eq!(waves.calculate_enemy_count(1), 13);
        assert_eq!(waves.calculate_enemy_count(5), 1); // Boss
        assert_eq!(waves.calculate_enemy_count(10), 1); // Boss
    }

    #[test]
    fn test_wave_completion() {
        let mut waves = WaveSystem::new();
        waves.start_wave(1);
        waves.enemies_spawned = waves.enemies_to_spawn;

        assert!(!waves.check_wave_complete(5)); // Still enemies alive
        assert!(waves.check_wave_complete(0)); // All enemies dead
        assert!(waves.wave_cleared);
    }
}
