use crate::level::Level;
use crate::physics::{Circle, Rect, Vec2, reflect_velocity};
use crate::powerup::{ActivePowerUp, PowerUp, PowerUpType};

pub const GAME_WIDTH: f32 = 80.0;
pub const GAME_HEIGHT: f32 = 30.0;

const PADDLE_SPEED: f32 = 40.0;
const BALL_SPEED: f32 = 20.0;
const BALL_SPEED_SLOW: f32 = 12.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
    Victory,
}

pub struct Paddle {
    pub rect: Rect,
    pub normal_width: f32,
    pub wide_width: f32,
    pub is_wide: bool,
}

impl Paddle {
    pub fn new() -> Self {
        let normal_width = 10.0;
        let wide_width = 16.0;
        let x = (GAME_WIDTH - normal_width) / 2.0;
        let y = GAME_HEIGHT - 3.0;

        Self {
            rect: Rect::new(x, y, normal_width, 1.0),
            normal_width,
            wide_width,
            is_wide: false,
        }
    }

    pub fn move_left(&mut self, delta: f32) {
        self.rect.x -= PADDLE_SPEED * delta;
        self.rect.x = self.rect.x.max(0.0);
    }

    pub fn move_right(&mut self, delta: f32) {
        self.rect.x += PADDLE_SPEED * delta;
        self.rect.x = self.rect.x.min(GAME_WIDTH - self.rect.width);
    }

    pub fn set_wide(&mut self, wide: bool) {
        if wide != self.is_wide {
            let center = self.rect.x + self.rect.width / 2.0;
            self.is_wide = wide;
            self.rect.width = if wide { self.wide_width } else { self.normal_width };
            self.rect.x = center - self.rect.width / 2.0;
            self.rect.x = self.rect.x.clamp(0.0, GAME_WIDTH - self.rect.width);
        }
    }
}

pub struct Ball {
    pub circle: Circle,
    pub velocity: Vec2,
    pub active: bool,
    pub attached: bool,
}

impl Ball {
    pub fn new(position: Vec2) -> Self {
        Self {
            circle: Circle::new(position, 0.5),
            velocity: Vec2::new(BALL_SPEED * 0.7, -BALL_SPEED * 0.7),
            active: true,
            attached: true,
        }
    }

    pub fn launch(&mut self) {
        self.attached = false;
    }

    pub fn update(&mut self, delta: f32) {
        if !self.attached && self.active {
            self.circle.center.x += self.velocity.x * delta;
            self.circle.center.y += self.velocity.y * delta;
        }
    }

    pub fn attach_to_paddle(&mut self, paddle: &Paddle) {
        self.attached = true;
        self.circle.center.x = paddle.rect.x + paddle.rect.width / 2.0;
        self.circle.center.y = paddle.rect.y - 1.0;
    }
}

pub struct Game {
    pub state: GameState,
    pub paddle: Paddle,
    pub balls: Vec<Ball>,
    pub level: Level,
    pub powerups: Vec<PowerUp>,
    pub active_powerups: Vec<ActivePowerUp>,
    pub score: u32,
    pub lives: u32,
    pub level_number: u32,
    pub move_left: bool,
    pub move_right: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut balls = Vec::new();
        let paddle = Paddle::new();
        let mut ball = Ball::new(Vec2::new(
            paddle.rect.x + paddle.rect.width / 2.0,
            paddle.rect.y - 1.0,
        ));
        ball.attached = true;
        balls.push(ball);

        Self {
            state: GameState::Menu,
            paddle,
            balls,
            level: Level::load(1, GAME_WIDTH),
            powerups: Vec::new(),
            active_powerups: Vec::new(),
            score: 0,
            lives: 5,
            level_number: 1,
            move_left: false,
            move_right: false,
        }
    }

    pub fn start(&mut self) {
        self.state = GameState::Playing;
        if self.balls.iter().all(|b| b.attached) {
            for ball in &mut self.balls {
                ball.launch();
            }
        }
    }

    pub fn move_paddle_left(&mut self) {
        self.move_left = true;
    }

    pub fn move_paddle_right(&mut self) {
        self.move_right = true;
    }

    pub fn update(&mut self, delta: f32) {
        if self.state != GameState::Playing {
            return;
        }

        // Update paddle movement
        if self.move_left {
            self.paddle.move_left(delta);
        }
        if self.move_right {
            self.paddle.move_right(delta);
        }
        self.move_left = false;
        self.move_right = false;

        // Update active power-ups
        self.active_powerups.retain_mut(|powerup| {
            let still_active = powerup.update(delta);
            if !still_active {
                // Power-up expired
                match powerup.power_type {
                    PowerUpType::WidePaddle => self.paddle.set_wide(false),
                    PowerUpType::SlowBall => {
                        for ball in &mut self.balls {
                            let speed = ball.velocity.length();
                            if speed < BALL_SPEED * 0.9 {
                                let normalized = ball.velocity.normalize();
                                ball.velocity.x = normalized.x * BALL_SPEED;
                                ball.velocity.y = normalized.y * BALL_SPEED;
                            }
                        }
                    }
                    _ => {}
                }
            }
            still_active
        });

        // Update balls
        for ball in &mut self.balls {
            if ball.attached {
                ball.attach_to_paddle(&self.paddle);
                continue;
            }

            ball.update(delta);

            // Wall collisions
            if ball.circle.center.x - ball.circle.radius <= 0.0
                || ball.circle.center.x + ball.circle.radius >= GAME_WIDTH
            {
                ball.velocity.x = -ball.velocity.x;
                ball.circle.center.x = ball.circle.center.x.clamp(
                    ball.circle.radius,
                    GAME_WIDTH - ball.circle.radius,
                );
            }

            if ball.circle.center.y - ball.circle.radius <= 0.0 {
                ball.velocity.y = -ball.velocity.y;
                ball.circle.center.y = ball.circle.radius;
            }

            // Bottom boundary (lose life)
            if ball.circle.center.y > GAME_HEIGHT {
                ball.active = false;
            }

            // Paddle collision
            if let Some(normal) = ball.circle.collides_rect(&self.paddle.rect) {
                // Calculate hit position on paddle (-1 to 1)
                let hit_pos = (ball.circle.center.x - self.paddle.rect.x) / self.paddle.rect.width;
                let hit_pos = (hit_pos * 2.0 - 1.0).clamp(-1.0, 1.0);

                // Adjust angle based on hit position
                let angle_adjustment = hit_pos * 0.8;
                let speed = ball.velocity.length();
                let new_velocity = reflect_velocity(ball.velocity, normal);

                ball.velocity.x = new_velocity.x + angle_adjustment * speed * 0.5;
                ball.velocity.y = new_velocity.y;

                // Normalize and maintain speed
                let normalized = ball.velocity.normalize();
                ball.velocity.x = normalized.x * speed;
                ball.velocity.y = normalized.y * speed;

                // Prevent ball from getting stuck
                ball.circle.center.y = self.paddle.rect.y - ball.circle.radius - 0.1;
            }

            // Brick collisions
            for brick in &mut self.level.bricks {
                if !brick.active {
                    continue;
                }

                if let Some(normal) = ball.circle.collides_rect(&brick.rect) {
                    let destroyed = brick.hit();

                    if destroyed {
                        self.score += brick.brick_type.points();

                        // Spawn power-up
                        if brick.brick_type.drops_powerup() {
                            let powerup_type = PowerUpType::random();
                            let powerup_pos = brick.rect.center();
                            self.powerups.push(PowerUp::new(powerup_pos, powerup_type));
                        }
                    }

                    // Reflect ball
                    ball.velocity = reflect_velocity(ball.velocity, normal);

                    // Move ball slightly to prevent multiple collisions
                    ball.circle.center.x += normal.x * 0.2;
                    ball.circle.center.y += normal.y * 0.2;

                    break; // Only hit one brick per frame
                }
            }
        }

        // Remove inactive balls
        self.balls.retain(|b| b.active);

        // Check if all balls lost
        if self.balls.is_empty() {
            self.lives -= 1;
            if self.lives == 0 {
                self.state = GameState::GameOver;
            } else {
                self.reset_ball();
            }
        }

        // Update power-ups
        let mut collected_powerups = Vec::new();
        for powerup in &mut self.powerups {
            powerup.update(delta);

            // Check collection
            if powerup.active {
                let powerup_rect = Rect::new(
                    powerup.position.x - 1.0,
                    powerup.position.y - 1.0,
                    2.0,
                    2.0,
                );

                if self.paddle.rect.intersects(&powerup_rect) {
                    collected_powerups.push(powerup.power_type);
                    powerup.active = false;
                }
            }
        }

        // Apply collected power-ups
        for power_type in collected_powerups {
            self.collect_powerup(power_type);
        }

        // Remove inactive or off-screen power-ups
        self.powerups.retain(|p| p.active && !p.is_off_screen(GAME_HEIGHT));

        // Check level completion
        if self.level.all_breakable_destroyed() {
            self.level_number += 1;
            if self.level_number > 5 {
                self.state = GameState::Victory;
            } else {
                self.next_level();
            }
        }
    }

    fn collect_powerup(&mut self, power_type: PowerUpType) {
        match power_type {
            PowerUpType::WidePaddle => {
                self.paddle.set_wide(true);
                self.active_powerups.retain(|p| p.power_type != PowerUpType::WidePaddle);
                self.active_powerups.push(ActivePowerUp::new(power_type));
            }
            PowerUpType::MultiBall => {
                // Duplicate all existing balls
                let new_balls: Vec<Ball> = self
                    .balls
                    .iter()
                    .filter(|b| !b.attached)
                    .map(|ball| {
                        let mut new_ball = Ball::new(ball.circle.center);
                        new_ball.attached = false;
                        new_ball.velocity = Vec2::new(-ball.velocity.x, ball.velocity.y);
                        new_ball
                    })
                    .collect();

                self.balls.extend(new_balls);
            }
            PowerUpType::SlowBall => {
                for ball in &mut self.balls {
                    if !ball.attached {
                        let normalized = ball.velocity.normalize();
                        ball.velocity.x = normalized.x * BALL_SPEED_SLOW;
                        ball.velocity.y = normalized.y * BALL_SPEED_SLOW;
                    }
                }
                self.active_powerups.retain(|p| p.power_type != PowerUpType::SlowBall);
                self.active_powerups.push(ActivePowerUp::new(power_type));
            }
            PowerUpType::ExtraLife => {
                self.lives += 1;
            }
            PowerUpType::LaserPaddle => {
                self.active_powerups.retain(|p| p.power_type != PowerUpType::LaserPaddle);
                self.active_powerups.push(ActivePowerUp::new(power_type));
            }
        }
    }

    fn reset_ball(&mut self) {
        self.balls.clear();
        let mut ball = Ball::new(Vec2::new(
            self.paddle.rect.x + self.paddle.rect.width / 2.0,
            self.paddle.rect.y - 1.0,
        ));
        ball.attached = true;
        self.balls.push(ball);
    }

    fn next_level(&mut self) {
        self.level = Level::load(self.level_number, GAME_WIDTH);
        self.reset_ball();
        self.powerups.clear();
        self.active_powerups.clear();
        self.paddle.set_wide(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new();
        assert_eq!(game.state, GameState::Menu);
        assert_eq!(game.lives, 5);
        assert_eq!(game.level_number, 1);
        assert_eq!(game.balls.len(), 1);
    }

    #[test]
    fn test_paddle_movement() {
        let mut game = Game::new();
        game.state = GameState::Playing;
        let initial_x = game.paddle.rect.x;

        game.move_paddle_left();
        game.update(0.1);
        assert!(game.paddle.rect.x < initial_x);

        let after_left = game.paddle.rect.x;
        game.move_paddle_right();
        game.update(0.1);
        assert!(game.paddle.rect.x > after_left);
    }

    #[test]
    fn test_paddle_boundaries() {
        let mut game = Game::new();
        game.state = GameState::Playing;

        // Move far left
        for _ in 0..100 {
            game.move_paddle_left();
            game.update(0.1);
        }
        assert_eq!(game.paddle.rect.x, 0.0);

        // Move far right
        for _ in 0..200 {
            game.move_paddle_right();
            game.update(0.1);
        }
        assert!(game.paddle.rect.x + game.paddle.rect.width <= GAME_WIDTH);
    }

    #[test]
    fn test_ball_launch() {
        let mut game = Game::new();
        game.start();

        assert_eq!(game.state, GameState::Playing);
        assert!(!game.balls[0].attached);
    }

    #[test]
    fn test_powerup_collection() {
        let mut game = Game::new();
        game.state = GameState::Playing;

        let initial_width = game.paddle.rect.width;
        game.collect_powerup(PowerUpType::WidePaddle);

        assert!(game.paddle.rect.width > initial_width);
        assert!(game.active_powerups.iter().any(|p| p.power_type == PowerUpType::WidePaddle));
    }

    #[test]
    fn test_extra_life_powerup() {
        let mut game = Game::new();
        let initial_lives = game.lives;

        game.collect_powerup(PowerUpType::ExtraLife);
        assert_eq!(game.lives, initial_lives + 1);
    }

    #[test]
    fn test_multiball_powerup() {
        let mut game = Game::new();
        game.state = GameState::Playing;
        game.balls[0].launch();

        let initial_ball_count = game.balls.len();
        game.collect_powerup(PowerUpType::MultiBall);

        assert_eq!(game.balls.len(), initial_ball_count * 2);
    }

    #[test]
    fn test_score_increases() {
        let mut game = Game::new();
        let initial_score = game.score;

        game.level.bricks[0].hit();
        let brick_points = game.level.bricks[0].brick_type.points();
        game.score += brick_points;

        assert!(game.score > initial_score);
    }
}
