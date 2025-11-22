use crate::physics::Vec2;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerUpType {
    WidePaddle,
    MultiBall,
    SlowBall,
    ExtraLife,
    LaserPaddle,
}

impl PowerUpType {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) {
            0 => PowerUpType::WidePaddle,
            1 => PowerUpType::MultiBall,
            2 => PowerUpType::SlowBall,
            3 => PowerUpType::ExtraLife,
            _ => PowerUpType::LaserPaddle,
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            PowerUpType::WidePaddle => "W",
            PowerUpType::MultiBall => "M",
            PowerUpType::SlowBall => "S",
            PowerUpType::ExtraLife => "+",
            PowerUpType::LaserPaddle => "L",
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            PowerUpType::WidePaddle => "Wide Paddle",
            PowerUpType::MultiBall => "Multi-Ball",
            PowerUpType::SlowBall => "Slow Ball",
            PowerUpType::ExtraLife => "Extra Life",
            PowerUpType::LaserPaddle => "Laser Paddle",
        }
    }

    pub fn duration(&self) -> Option<f32> {
        match self {
            PowerUpType::WidePaddle => Some(10.0),
            PowerUpType::SlowBall => Some(8.0),
            PowerUpType::LaserPaddle => Some(12.0),
            PowerUpType::MultiBall => None,
            PowerUpType::ExtraLife => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerUp {
    pub position: Vec2,
    pub velocity: Vec2,
    pub power_type: PowerUpType,
    pub active: bool,
}

impl PowerUp {
    pub fn new(position: Vec2, power_type: PowerUpType) -> Self {
        Self {
            position,
            velocity: Vec2::new(0.0, 3.0), // Falls downward
            power_type,
            active: true,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.active {
            self.position.x += self.velocity.x * delta;
            self.position.y += self.velocity.y * delta;
        }
    }

    pub fn is_off_screen(&self, screen_height: f32) -> bool {
        self.position.y > screen_height
    }
}

#[derive(Debug, Clone)]
pub struct ActivePowerUp {
    pub power_type: PowerUpType,
    pub remaining_time: f32,
}

impl ActivePowerUp {
    pub fn new(power_type: PowerUpType) -> Self {
        Self {
            power_type,
            remaining_time: power_type.duration().unwrap_or(0.0),
        }
    }

    pub fn update(&mut self, delta: f32) -> bool {
        if self.power_type.duration().is_some() {
            self.remaining_time -= delta;
            self.remaining_time > 0.0
        } else {
            true // Permanent power-ups
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerup_creation() {
        let powerup = PowerUp::new(Vec2::new(10.0, 10.0), PowerUpType::WidePaddle);
        assert_eq!(powerup.power_type, PowerUpType::WidePaddle);
        assert!(powerup.active);
    }

    #[test]
    fn test_powerup_falls() {
        let mut powerup = PowerUp::new(Vec2::new(10.0, 10.0), PowerUpType::MultiBall);
        let initial_y = powerup.position.y;
        powerup.update(1.0);
        assert!(powerup.position.y > initial_y);
    }

    #[test]
    fn test_active_powerup_duration() {
        let mut active = ActivePowerUp::new(PowerUpType::WidePaddle);
        assert!(active.remaining_time > 0.0);

        let still_active = active.update(5.0);
        assert!(still_active);
        assert!(active.remaining_time > 0.0);

        active.update(10.0);
        assert!(active.remaining_time <= 0.0);
    }
}
