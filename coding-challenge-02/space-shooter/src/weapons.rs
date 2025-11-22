use macroquad::prelude::*;

/// Bullet entity
#[derive(Debug, Clone)]
pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Vec2,
    pub damage: i32,
    pub bullet_type: BulletType,
    pub is_player_bullet: bool,
    pub lifetime: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletType {
    Normal,
    Spread,
    Laser,
    Missile,
    Enemy,
}

impl Bullet {
    pub fn new(position: Vec2, velocity: Vec2, bullet_type: BulletType, is_player_bullet: bool) -> Self {
        let (size, damage) = match bullet_type {
            BulletType::Normal => (vec2(4.0, 12.0), 10),
            BulletType::Spread => (vec2(3.0, 10.0), 8),
            BulletType::Laser => (vec2(6.0, 30.0), 25),
            BulletType::Missile => (vec2(8.0, 16.0), 50),
            BulletType::Enemy => (vec2(5.0, 10.0), 15),
        };

        Self {
            position,
            velocity,
            size,
            damage,
            bullet_type,
            is_player_bullet,
            lifetime: 3.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;
    }

    pub fn is_off_screen(&self) -> bool {
        self.lifetime <= 0.0 ||
        self.position.y < -20.0 || self.position.y > 920.0 ||
        self.position.x < -20.0 || self.position.x > 820.0
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

/// Weapon system for the player
pub struct WeaponSystem {
    pub fire_rate: f32,
    pub fire_timer: f32,
}

impl WeaponSystem {
    pub fn new() -> Self {
        Self {
            fire_rate: 0.15,
            fire_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.fire_timer += dt;
    }

    pub fn can_fire(&self) -> bool {
        self.fire_timer >= self.fire_rate
    }

    pub fn reset_timer(&mut self) {
        self.fire_timer = 0.0;
    }

    /// Generate bullets based on weapon level
    pub fn fire(&mut self, player_pos: Vec2, weapon_level: usize) -> Vec<Bullet> {
        if !self.can_fire() {
            return vec![];
        }

        self.reset_timer();
        let mut bullets = vec![];

        match weapon_level {
            0 => {
                // Single shot
                bullets.push(Bullet::new(
                    player_pos,
                    vec2(0.0, -600.0),
                    BulletType::Normal,
                    true,
                ));
            }
            1 => {
                // Double shot
                bullets.push(Bullet::new(
                    player_pos + vec2(-10.0, 0.0),
                    vec2(0.0, -600.0),
                    BulletType::Normal,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos + vec2(10.0, 0.0),
                    vec2(0.0, -600.0),
                    BulletType::Normal,
                    true,
                ));
            }
            2 => {
                // Triple spread
                bullets.push(Bullet::new(
                    player_pos,
                    vec2(0.0, -600.0),
                    BulletType::Spread,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos,
                    vec2(-150.0, -600.0),
                    BulletType::Spread,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos,
                    vec2(150.0, -600.0),
                    BulletType::Spread,
                    true,
                ));
            }
            3 => {
                // Quad laser
                bullets.push(Bullet::new(
                    player_pos + vec2(-15.0, 0.0),
                    vec2(0.0, -700.0),
                    BulletType::Laser,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos + vec2(15.0, 0.0),
                    vec2(0.0, -700.0),
                    BulletType::Laser,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos + vec2(-5.0, 0.0),
                    vec2(0.0, -700.0),
                    BulletType::Laser,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos + vec2(5.0, 0.0),
                    vec2(0.0, -700.0),
                    BulletType::Laser,
                    true,
                ));
            }
            _ => {
                // Max level: missiles + lasers
                bullets.push(Bullet::new(
                    player_pos + vec2(-20.0, 0.0),
                    vec2(-50.0, -650.0),
                    BulletType::Missile,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos + vec2(20.0, 0.0),
                    vec2(50.0, -650.0),
                    BulletType::Missile,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos,
                    vec2(0.0, -700.0),
                    BulletType::Laser,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos + vec2(-10.0, 0.0),
                    vec2(0.0, -700.0),
                    BulletType::Laser,
                    true,
                ));
                bullets.push(Bullet::new(
                    player_pos + vec2(10.0, 0.0),
                    vec2(0.0, -700.0),
                    BulletType::Laser,
                    true,
                ));
            }
        }

        bullets
    }
}

impl Default for WeaponSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullet_creation() {
        let bullet = Bullet::new(vec2(100.0, 100.0), vec2(0.0, -600.0), BulletType::Normal, true);
        assert_eq!(bullet.damage, 10);
        assert!(bullet.is_player_bullet);
    }

    #[test]
    fn test_weapon_system_fire_rate() {
        let mut weapon = WeaponSystem::new();
        assert!(!weapon.can_fire());

        weapon.update(0.2);
        assert!(weapon.can_fire());
    }

    #[test]
    fn test_weapon_levels() {
        let mut weapon = WeaponSystem::new();
        weapon.fire_timer = 1.0; // Ensure can fire

        // Level 0: 1 bullet
        let bullets = weapon.fire(vec2(100.0, 100.0), 0);
        assert_eq!(bullets.len(), 1);

        weapon.fire_timer = 1.0;
        // Level 1: 2 bullets
        let bullets = weapon.fire(vec2(100.0, 100.0), 1);
        assert_eq!(bullets.len(), 2);

        weapon.fire_timer = 1.0;
        // Level 4: 5 bullets
        let bullets = weapon.fire(vec2(100.0, 100.0), 4);
        assert_eq!(bullets.len(), 5);
    }
}
