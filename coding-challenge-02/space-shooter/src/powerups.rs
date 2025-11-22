use macroquad::prelude::*;

/// Power-up entity
#[derive(Debug, Clone)]
pub struct PowerUp {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Vec2,
    pub powerup_type: PowerUpType,
    pub lifetime: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUpType {
    Health,
    Shield,
    WeaponUpgrade,
    ScoreMultiplier,
}

impl PowerUp {
    pub fn new(position: Vec2, powerup_type: PowerUpType) -> Self {
        Self {
            position,
            velocity: vec2(0.0, 100.0),
            size: vec2(30.0, 30.0),
            powerup_type,
            lifetime: 8.0,
        }
    }

    pub fn random(position: Vec2) -> Self {
        let powerup_type = match rand::gen_range(0, 4) {
            0 => PowerUpType::Health,
            1 => PowerUpType::Shield,
            2 => PowerUpType::WeaponUpgrade,
            _ => PowerUpType::ScoreMultiplier,
        };
        Self::new(position, powerup_type)
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime <= 0.0 || self.position.y > 920.0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerup_creation() {
        let powerup = PowerUp::new(vec2(100.0, 100.0), PowerUpType::Health);
        assert_eq!(powerup.powerup_type, PowerUpType::Health);
        assert_eq!(powerup.lifetime, 8.0);
    }

    #[test]
    fn test_powerup_expiration() {
        let mut powerup = PowerUp::new(vec2(100.0, 100.0), PowerUpType::Shield);
        assert!(!powerup.is_expired());

        powerup.lifetime = 0.0;
        assert!(powerup.is_expired());
    }
}
