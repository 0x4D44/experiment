use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub segments: VecDeque<Position>,
    pub direction: Direction,
    pub grow_pending: usize,
}

impl Snake {
    pub fn new(start_pos: Position, direction: Direction) -> Self {
        let mut segments = VecDeque::new();
        segments.push_back(start_pos);
        Self {
            segments,
            direction,
            grow_pending: 0,
        }
    }

    pub fn head(&self) -> Position {
        *self.segments.front().unwrap()
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        // Prevent 180-degree turns
        if new_direction != self.direction.opposite() {
            self.direction = new_direction;
        }
    }

    pub fn move_forward(&mut self) {
        let (dx, dy) = self.direction.to_delta();
        let head = self.head();
        let new_head = Position::new(head.x + dx, head.y + dy);

        self.segments.push_front(new_head);

        if self.grow_pending > 0 {
            self.grow_pending -= 1;
        } else {
            self.segments.pop_back();
        }
    }

    pub fn grow(&mut self, amount: usize) {
        self.grow_pending += amount;
    }

    pub fn collides_with_self(&self) -> bool {
        let head = self.head();
        self.segments.iter().skip(1).any(|&pos| pos == head)
    }

    pub fn contains_position(&self, pos: Position) -> bool {
        self.segments.iter().any(|&p| p == pos)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub pos: Position,
    pub velocity: (f32, f32),
}

impl Bullet {
    pub fn new(pos: Position, velocity: (f32, f32)) -> Self {
        Self { pos, velocity }
    }

    pub fn update(&mut self) {
        self.pos.x = (self.pos.x as f32 + self.velocity.0).round() as i32;
        self.pos.y = (self.pos.y as f32 + self.velocity.1).round() as i32;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub pos: Position,
    pub shoot_cooldown: f32,
    pub pattern_type: usize,
}

impl Enemy {
    pub fn new(pos: Position, pattern_type: usize) -> Self {
        Self {
            pos,
            shoot_cooldown: 0.0,
            pattern_type,
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.shoot_cooldown <= 0.0
    }

    pub fn update_cooldown(&mut self, delta: f32) {
        self.shoot_cooldown -= delta;
    }

    pub fn reset_cooldown(&mut self, cooldown: f32) {
        self.shoot_cooldown = cooldown;
    }
}

pub struct GameState {
    pub snake: Snake,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub width: i32,
    pub height: i32,
    pub score: u32,
    pub game_over: bool,
}

impl GameState {
    pub fn new(width: i32, height: i32) -> Self {
        let start_x = width / 2;
        let start_y = height / 2;
        let snake = Snake::new(Position::new(start_x, start_y), Direction::Right);

        Self {
            snake,
            enemies: Vec::new(),
            bullets: Vec::new(),
            width,
            height,
            score: 0,
            game_over: false,
        }
    }

    pub fn is_in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    pub fn check_wall_collision(&self) -> bool {
        !self.is_in_bounds(self.snake.head())
    }

    pub fn check_bullet_collision(&self) -> bool {
        let head = self.snake.head();
        self.bullets.iter().any(|bullet| bullet.pos == head)
    }

    pub fn check_enemy_collision(&self) -> Option<usize> {
        let head = self.snake.head();
        self.enemies.iter().position(|enemy| enemy.pos == head)
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.game_over {
            return;
        }

        // Move snake
        self.snake.move_forward();

        // Check collisions
        if self.check_wall_collision() || self.snake.collides_with_self() || self.check_bullet_collision() {
            self.game_over = true;
            return;
        }

        // Check enemy collision
        if let Some(enemy_idx) = self.check_enemy_collision() {
            self.enemies.remove(enemy_idx);
            self.snake.grow(3);
            self.score += 100;
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update();
        }

        // Remove out-of-bounds bullets
        let width = self.width;
        let height = self.height;
        self.bullets.retain(|bullet| {
            bullet.pos.x >= 0 && bullet.pos.x < width && bullet.pos.y >= 0 && bullet.pos.y < height
        });

        // Update enemy cooldowns
        for enemy in &mut self.enemies {
            enemy.update_cooldown(delta_time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
    }

    #[test]
    fn test_snake_creation() {
        let snake = Snake::new(Position::new(5, 5), Direction::Right);
        assert_eq!(snake.head(), Position::new(5, 5));
        assert_eq!(snake.segments.len(), 1);
        assert_eq!(snake.direction, Direction::Right);
    }

    #[test]
    fn test_snake_movement() {
        let mut snake = Snake::new(Position::new(5, 5), Direction::Right);
        snake.move_forward();
        assert_eq!(snake.head(), Position::new(6, 5));
        assert_eq!(snake.segments.len(), 1);
    }

    #[test]
    fn test_snake_direction_change() {
        let mut snake = Snake::new(Position::new(5, 5), Direction::Right);
        snake.change_direction(Direction::Up);
        assert_eq!(snake.direction, Direction::Up);

        // Should not allow 180-degree turn
        snake.change_direction(Direction::Down);
        assert_eq!(snake.direction, Direction::Up);
    }

    #[test]
    fn test_snake_growth() {
        let mut snake = Snake::new(Position::new(5, 5), Direction::Right);
        snake.grow(2);

        snake.move_forward(); // Should grow
        assert_eq!(snake.segments.len(), 2);

        snake.move_forward(); // Should grow again
        assert_eq!(snake.segments.len(), 3);

        snake.move_forward(); // Should not grow
        assert_eq!(snake.segments.len(), 3);
    }

    #[test]
    fn test_snake_self_collision() {
        let mut snake = Snake::new(Position::new(5, 5), Direction::Right);
        snake.grow(10); // Need enough length to create collision

        // Build up a long snake
        for _ in 0..10 {
            snake.move_forward();
        }

        // Now create collision: turn in a tight square
        snake.change_direction(Direction::Up);
        snake.move_forward();
        snake.move_forward();

        snake.change_direction(Direction::Left);
        snake.move_forward();
        snake.move_forward();

        snake.change_direction(Direction::Down);
        snake.move_forward(); // Should hit body

        assert!(snake.collides_with_self());
    }

    #[test]
    fn test_game_state_creation() {
        let game = GameState::new(40, 20);
        assert_eq!(game.width, 40);
        assert_eq!(game.height, 20);
        assert_eq!(game.score, 0);
        assert!(!game.game_over);
        assert_eq!(game.snake.head(), Position::new(20, 10));
    }

    #[test]
    fn test_wall_collision() {
        let mut game = GameState::new(40, 20);
        game.snake = Snake::new(Position::new(0, 0), Direction::Left);
        game.snake.move_forward();
        assert!(game.check_wall_collision());
    }

    #[test]
    fn test_bullet_collision() {
        let mut game = GameState::new(40, 20);
        game.snake = Snake::new(Position::new(5, 5), Direction::Right);
        game.bullets.push(Bullet::new(Position::new(5, 5), (0.0, 0.0)));
        assert!(game.check_bullet_collision());
    }

    #[test]
    fn test_enemy_collision() {
        let mut game = GameState::new(40, 20);
        game.snake = Snake::new(Position::new(5, 5), Direction::Right);
        game.enemies.push(Enemy::new(Position::new(6, 5), 0));

        game.update(0.016);

        assert_eq!(game.enemies.len(), 0); // Enemy should be removed
        assert_eq!(game.score, 100);
        assert!(game.snake.grow_pending > 0);
    }

    #[test]
    fn test_bullet_update() {
        let mut bullet = Bullet::new(Position::new(5, 5), (1.0, 0.5));
        bullet.update();
        assert_eq!(bullet.pos, Position::new(6, 6));
    }

    #[test]
    fn test_bullets_removed_when_out_of_bounds() {
        let mut game = GameState::new(40, 20);
        game.bullets.push(Bullet::new(Position::new(5, 5), (10.0, 0.0)));

        // Update multiple times to move bullet out of bounds
        for _ in 0..10 {
            for bullet in &mut game.bullets {
                bullet.update();
            }
            let width = game.width;
            let height = game.height;
            game.bullets.retain(|bullet| {
                bullet.pos.x >= 0 && bullet.pos.x < width && bullet.pos.y >= 0 && bullet.pos.y < height
            });
        }

        assert_eq!(game.bullets.len(), 0);
    }

    #[test]
    fn test_game_over_on_wall_collision() {
        let mut game = GameState::new(10, 10);
        game.snake = Snake::new(Position::new(0, 5), Direction::Left);
        game.update(0.016);
        assert!(game.game_over);
    }

    #[test]
    fn test_game_over_on_self_collision() {
        let mut game = GameState::new(40, 20);
        game.snake = Snake::new(Position::new(5, 5), Direction::Right);
        game.snake.grow(5);

        // Move in a pattern that causes self-collision
        for _ in 0..3 {
            game.snake.move_forward();
        }
        game.snake.change_direction(Direction::Up);
        game.snake.move_forward();
        game.snake.change_direction(Direction::Left);
        game.snake.move_forward();
        game.snake.change_direction(Direction::Down);

        game.update(0.016);
        assert!(game.game_over);
    }
}
