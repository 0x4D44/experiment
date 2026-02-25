use crate::game::{Bullet, Enemy, GameState, Position};
use rand::Rng;
use std::f32::consts::PI;

const SHOOT_COOLDOWN: f32 = 1.5;

pub fn spawn_enemy(game_state: &mut GameState) {
    let mut rng = rand::thread_rng();

    // Spawn enemy at random position away from snake
    let snake_head = game_state.snake.head();
    let mut pos = loop {
        let x = rng.gen_range(5..game_state.width - 5);
        let y = rng.gen_range(5..game_state.height - 5);
        let test_pos = Position::new(x, y);

        // Make sure not too close to snake
        let dx = (test_pos.x - snake_head.x).abs();
        let dy = (test_pos.y - snake_head.y).abs();
        if dx > 10 || dy > 10 {
            break test_pos;
        }
    };

    // Don't spawn on top of snake
    while game_state.snake.contains_position(pos) {
        pos.x = rng.gen_range(5..game_state.width - 5);
        pos.y = rng.gen_range(5..game_state.height - 5);
    }

    let pattern_type = rng.gen_range(0..3);
    game_state.enemies.push(Enemy::new(pos, pattern_type));
}

pub fn update_enemy_shooting(game_state: &mut GameState, _delta_time: f32) {
    let snake_head = game_state.snake.head();

    for enemy in &mut game_state.enemies {
        if enemy.can_shoot() {
            // Generate bullets based on pattern type
            let new_bullets = match enemy.pattern_type {
                0 => create_aimed_pattern(enemy.pos, snake_head),
                1 => create_spread_pattern(enemy.pos, snake_head),
                2 => create_spiral_pattern(enemy.pos),
                _ => create_aimed_pattern(enemy.pos, snake_head),
            };

            game_state.bullets.extend(new_bullets);
            enemy.reset_cooldown(SHOOT_COOLDOWN);
        }
    }
}

fn create_aimed_pattern(origin: Position, target: Position) -> Vec<Bullet> {
    let dx = (target.x - origin.x) as f32;
    let dy = (target.y - origin.y) as f32;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < 0.1 {
        return vec![];
    }

    let speed = 0.8;
    let vx = (dx / distance) * speed;
    let vy = (dy / distance) * speed;

    vec![Bullet::new(origin, (vx, vy))]
}

fn create_spread_pattern(origin: Position, target: Position) -> Vec<Bullet> {
    let dx = (target.x - origin.x) as f32;
    let dy = (target.y - origin.y) as f32;
    let base_angle = dy.atan2(dx);

    let speed = 0.6;
    let spread = PI / 6.0; // 30 degrees

    vec![
        Bullet::new(
            origin,
            (
                (base_angle - spread).cos() * speed,
                (base_angle - spread).sin() * speed,
            ),
        ),
        Bullet::new(
            origin,
            (base_angle.cos() * speed, base_angle.sin() * speed),
        ),
        Bullet::new(
            origin,
            (
                (base_angle + spread).cos() * speed,
                (base_angle + spread).sin() * speed,
            ),
        ),
    ]
}

fn create_spiral_pattern(origin: Position) -> Vec<Bullet> {
    let mut bullets = Vec::new();
    let speed = 0.5;
    let num_bullets = 8;

    for i in 0..num_bullets {
        let angle = (i as f32 / num_bullets as f32) * 2.0 * PI;
        bullets.push(Bullet::new(
            origin,
            (angle.cos() * speed, angle.sin() * speed),
        ));
    }

    bullets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aimed_pattern_creates_bullet_toward_target() {
        let origin = Position::new(10, 10);
        let target = Position::new(20, 10);
        let bullets = create_aimed_pattern(origin, target);

        assert_eq!(bullets.len(), 1);
        assert!(bullets[0].velocity.0 > 0.0); // Moving right
        assert!(bullets[0].velocity.1.abs() < 0.1); // Not moving vertically much
    }

    #[test]
    fn test_spread_pattern_creates_three_bullets() {
        let origin = Position::new(10, 10);
        let target = Position::new(20, 15);
        let bullets = create_spread_pattern(origin, target);

        assert_eq!(bullets.len(), 3);
    }

    #[test]
    fn test_spiral_pattern_creates_eight_bullets() {
        let origin = Position::new(10, 10);
        let bullets = create_spiral_pattern(origin);

        assert_eq!(bullets.len(), 8);
    }

    #[test]
    fn test_spawn_enemy_adds_enemy_to_game() {
        let mut game = GameState::new(40, 20);
        spawn_enemy(&mut game);

        assert_eq!(game.enemies.len(), 1);
    }

    #[test]
    fn test_enemy_shoots_when_cooldown_ready() {
        let mut game = GameState::new(40, 20);
        game.enemies.push(Enemy::new(Position::new(10, 10), 0));

        update_enemy_shooting(&mut game, 0.016);

        assert!(!game.bullets.is_empty());
        assert!(game.enemies[0].shoot_cooldown > 0.0);
    }

    #[test]
    fn test_enemy_does_not_shoot_during_cooldown() {
        let mut game = GameState::new(40, 20);
        let mut enemy = Enemy::new(Position::new(10, 10), 0);
        enemy.reset_cooldown(1.0);
        game.enemies.push(enemy);

        let initial_bullet_count = game.bullets.len();
        update_enemy_shooting(&mut game, 0.016);

        assert_eq!(game.bullets.len(), initial_bullet_count);
    }
}
