use macroquad::prelude::*;

/// Player spaceship entity
#[derive(Debug, Clone)]
pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Vec2,
    pub health: i32,
    pub max_health: i32,
    pub shield: i32,
    pub max_shield: i32,
    pub lives: i32,
    pub weapon_level: usize,
    pub speed: f32,
    pub invulnerable_timer: f32,
    pub is_alive: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: vec2(400.0, 700.0),
            velocity: Vec2::ZERO,
            size: vec2(40.0, 40.0),
            health: 100,
            max_health: 100,
            shield: 100,
            max_shield: 100,
            lives: 3,
            weapon_level: 0,
            speed: 350.0,
            invulnerable_timer: 0.0,
            is_alive: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Apply velocity
        self.position += self.velocity * dt;

        // Clamp to screen bounds
        let half_size = self.size / 2.0;
        self.position.x = self.position.x.clamp(half_size.x, 800.0 - half_size.x);
        self.position.y = self.position.y.clamp(half_size.y, 900.0 - half_size.y);

        // Update invulnerability timer
        if self.invulnerable_timer > 0.0 {
            self.invulnerable_timer -= dt;
        }

        // Reset velocity for next frame
        self.velocity = Vec2::ZERO;
    }

    pub fn move_direction(&mut self, direction: Vec2) {
        self.velocity += direction.normalize_or_zero() * self.speed;
    }

    pub fn take_damage(&mut self, damage: i32) -> bool {
        if self.invulnerable_timer > 0.0 {
            return false;
        }

        // Shield absorbs damage first
        if self.shield > 0 {
            self.shield -= damage;
            if self.shield < 0 {
                self.health += self.shield; // Overflow damage to health
                self.shield = 0;
            }
        } else {
            self.health -= damage;
        }

        if self.health <= 0 {
            self.health = 0;
            self.lives -= 1;
            if self.lives > 0 {
                // Respawn
                self.health = self.max_health;
                self.shield = self.max_shield / 2;
                self.position = vec2(400.0, 700.0);
                self.invulnerable_timer = 3.0;
                return true;
            } else {
                self.is_alive = false;
                return true;
            }
        }

        self.invulnerable_timer = 0.5;
        true
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn recharge_shield(&mut self, amount: i32) {
        self.shield = (self.shield + amount).min(self.max_shield);
    }

    pub fn upgrade_weapon(&mut self) {
        self.weapon_level = (self.weapon_level + 1).min(4);
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.position.x - self.size.x / 2.0,
            self.position.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
        )
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

/// Enemy entity
#[derive(Debug, Clone)]
pub struct Enemy {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Vec2,
    pub health: i32,
    pub max_health: i32,
    pub enemy_type: EnemyType,
    pub shoot_timer: f32,
    pub shoot_cooldown: f32,
    pub is_alive: bool,
    pub score_value: u32,
    pub movement_pattern: MovementPattern,
    pub pattern_timer: f32,
    pub original_x: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    BasicFighter,
    HeavyCruiser,
    Kamikaze,
    Boss1,
    Boss2,
}

#[derive(Debug, Clone, Copy)]
pub enum MovementPattern {
    Straight,
    Zigzag,
    Circle,
    Stationary,
    Chase,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, position: Vec2) -> Self {
        let (size, health, shoot_cooldown, score_value, movement_pattern) = match enemy_type {
            EnemyType::BasicFighter => (vec2(30.0, 30.0), 20, 1.5, 100, MovementPattern::Straight),
            EnemyType::HeavyCruiser => (vec2(50.0, 50.0), 100, 1.0, 500, MovementPattern::Zigzag),
            EnemyType::Kamikaze => (vec2(25.0, 25.0), 10, 999.0, 200, MovementPattern::Chase),
            EnemyType::Boss1 => (vec2(100.0, 100.0), 1000, 0.5, 5000, MovementPattern::Circle),
            EnemyType::Boss2 => (vec2(120.0, 120.0), 1500, 0.3, 10000, MovementPattern::Stationary),
        };

        Self {
            position,
            velocity: vec2(0.0, 50.0),
            size,
            health,
            max_health: health,
            enemy_type,
            shoot_timer: 0.0,
            shoot_cooldown,
            is_alive: true,
            score_value,
            movement_pattern,
            pattern_timer: 0.0,
            original_x: position.x,
        }
    }

    pub fn update(&mut self, dt: f32, player_pos: Vec2) {
        self.shoot_timer += dt;
        self.pattern_timer += dt;

        // Update movement based on pattern
        match self.movement_pattern {
            MovementPattern::Straight => {
                self.position += self.velocity * dt;
            }
            MovementPattern::Zigzag => {
                let zigzag_speed = 100.0;
                let zigzag_amount = (self.pattern_timer * 2.0).sin() * zigzag_speed;
                self.position.x = self.original_x + zigzag_amount;
                self.position.y += 50.0 * dt;
            }
            MovementPattern::Circle => {
                let radius = 150.0;
                let speed = 1.0;
                self.position.x = 400.0 + (self.pattern_timer * speed).cos() * radius;
                self.position.y = 200.0 + (self.pattern_timer * speed).sin() * radius * 0.5;
            }
            MovementPattern::Stationary => {
                // Stay in place, maybe slight hover
                self.position.y = 150.0 + (self.pattern_timer * 2.0).sin() * 10.0;
            }
            MovementPattern::Chase => {
                let direction = (player_pos - self.position).normalize_or_zero();
                self.position += direction * 200.0 * dt;
            }
        }
    }

    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health -= damage;
        if self.health <= 0 {
            self.is_alive = false;
            return true;
        }
        false
    }

    pub fn can_shoot(&self) -> bool {
        self.shoot_timer >= self.shoot_cooldown
    }

    pub fn reset_shoot_timer(&mut self) {
        self.shoot_timer = 0.0;
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.position.x - self.size.x / 2.0,
            self.position.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
        )
    }

    pub fn is_off_screen(&self) -> bool {
        self.position.y > 950.0 || self.position.y < -50.0 ||
        self.position.x > 850.0 || self.position.x < -50.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new();
        assert_eq!(player.lives, 3);
        assert_eq!(player.health, 100);
        assert!(player.is_alive);
    }

    #[test]
    fn test_player_damage() {
        let mut player = Player::new();
        player.take_damage(50);
        assert_eq!(player.shield, 50);
        assert_eq!(player.health, 100);
    }

    #[test]
    fn test_player_shield_overflow() {
        let mut player = Player::new();
        player.take_damage(150);
        assert_eq!(player.shield, 0);
        assert_eq!(player.health, 50);
    }

    #[test]
    fn test_enemy_creation() {
        let enemy = Enemy::new(EnemyType::BasicFighter, vec2(100.0, 100.0));
        assert_eq!(enemy.health, 20);
        assert_eq!(enemy.score_value, 100);
    }

    #[test]
    fn test_enemy_damage() {
        let mut enemy = Enemy::new(EnemyType::BasicFighter, vec2(100.0, 100.0));
        let killed = enemy.take_damage(15);
        assert!(!killed);
        assert_eq!(enemy.health, 5);

        let killed = enemy.take_damage(10);
        assert!(killed);
        assert!(!enemy.is_alive);
    }
}
