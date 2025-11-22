use std::collections::VecDeque;
use rand::Rng;

/// Represents the four possible directions the snake can move
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns true if the given direction is opposite to this one
    pub fn is_opposite(&self, other: &Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

/// Represents a position on the game board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    /// Returns a new position after moving in the given direction
    pub fn move_in_direction(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}

/// Represents the snake in the game
#[derive(Debug, Clone)]
pub struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
    next_direction: Direction,
    growing: bool,
}

impl Snake {
    /// Creates a new snake at the given starting position
    pub fn new(start_pos: Position, direction: Direction) -> Self {
        let mut body = VecDeque::new();
        body.push_back(start_pos);
        body.push_back(Position::new(start_pos.x - 1, start_pos.y));
        body.push_back(Position::new(start_pos.x - 2, start_pos.y));

        Snake {
            body,
            direction,
            next_direction: direction,
            growing: false,
        }
    }

    /// Returns the current head position
    ///
    /// # Panics
    /// Never panics - the snake always has at least one segment (initialized with 3)
    pub fn head(&self) -> Position {
        *self.body.front().unwrap()
    }

    /// Returns a reference to the snake's body
    pub fn body(&self) -> &VecDeque<Position> {
        &self.body
    }

    /// Returns the current direction
    #[allow(dead_code)]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Sets the next direction (will be applied on next move if valid)
    pub fn set_direction(&mut self, direction: Direction) {
        // Prevent 180-degree turns
        if !self.direction.is_opposite(&direction) {
            self.next_direction = direction;
        }
    }

    /// Marks the snake to grow on the next move
    pub fn grow(&mut self) {
        self.growing = true;
    }

    /// Moves the snake one step in its current direction
    pub fn advance(&mut self) {
        self.direction = self.next_direction;
        let new_head = self.head().move_in_direction(self.direction);
        self.body.push_front(new_head);

        if !self.growing {
            self.body.pop_back();
        } else {
            self.growing = false;
        }
    }

    /// Returns true if the snake collides with itself
    pub fn self_collision(&self) -> bool {
        let head = self.head();
        self.body.iter().skip(1).any(|&pos| pos == head)
    }

    /// Returns the length of the snake
    pub fn len(&self) -> usize {
        self.body.len()
    }

    /// Returns true if the snake has no body segments (should never happen in practice)
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

/// Represents the difficulty level of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
}

impl Difficulty {
    /// Returns the initial speed (milliseconds per tick) for this difficulty
    pub fn initial_speed(&self) -> u64 {
        match self {
            Difficulty::Easy => 150,
            Difficulty::Medium => 100,
            Difficulty::Hard => 70,
            Difficulty::Extreme => 50,
        }
    }

    /// Returns the minimum speed (fastest possible) for this difficulty
    pub fn min_speed(&self) -> u64 {
        match self {
            Difficulty::Easy => 80,
            Difficulty::Medium => 50,
            Difficulty::Hard => 30,
            Difficulty::Extreme => 20,
        }
    }

    /// Returns the name of the difficulty as a string
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Extreme => "Extreme",
        }
    }
}

/// Represents the current state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
}

/// The main game controller
pub struct Game {
    snake: Snake,
    food: Position,
    width: i32,
    height: i32,
    score: u32,
    state: GameState,
    difficulty: Difficulty,
    speed: u64,
}

impl Game {
    /// Creates a new game with the specified dimensions and difficulty
    pub fn new(width: i32, height: i32, difficulty: Difficulty) -> Self {
        let start_pos = Position::new(width / 2, height / 2);
        let snake = Snake::new(start_pos, Direction::Right);
        let speed = difficulty.initial_speed();

        let mut game = Game {
            snake,
            food: Position::new(0, 0),
            width,
            height,
            score: 0,
            state: GameState::Running,
            difficulty,
            speed,
        };

        game.spawn_food();
        game
    }

    /// Returns the current game state
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Returns the current score
    pub fn score(&self) -> u32 {
        self.score
    }

    /// Returns the current speed (milliseconds per tick)
    pub fn speed(&self) -> u64 {
        self.speed
    }

    /// Returns a reference to the snake
    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    /// Returns the food position
    pub fn food(&self) -> Position {
        self.food
    }

    /// Returns the game board width
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Returns the game board height
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Returns the difficulty level
    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    /// Sets the snake's direction
    pub fn set_direction(&mut self, direction: Direction) {
        self.snake.set_direction(direction);
    }

    /// Toggles pause state
    pub fn toggle_pause(&mut self) {
        if self.state == GameState::Running {
            self.state = GameState::Paused;
        } else if self.state == GameState::Paused {
            self.state = GameState::Running;
        }
    }

    /// Updates the game state by one tick
    pub fn update(&mut self) {
        if self.state != GameState::Running {
            return;
        }

        self.snake.advance();

        // Check for wall collision
        let head = self.snake.head();
        if head.x < 0 || head.x >= self.width || head.y < 0 || head.y >= self.height {
            self.state = GameState::GameOver;
            return;
        }

        // Check for self collision
        if self.snake.self_collision() {
            self.state = GameState::GameOver;
            return;
        }

        // Check for food collision
        if head == self.food {
            self.snake.grow();
            self.score = self.score.saturating_add(10);
            self.spawn_food();
            self.increase_speed();
        }
    }

    /// Spawns food at a random position not occupied by the snake
    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        let max_attempts = (self.width * self.height) * 2;

        for _ in 0..max_attempts {
            let pos = Position::new(
                rng.gen_range(0..self.width),
                rng.gen_range(0..self.height),
            );

            if !self.snake.body().contains(&pos) {
                self.food = pos;
                return;
            }
        }

        // Fallback: if we can't find a random spot (board is almost full),
        // do an exhaustive search for any empty space
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x, y);
                if !self.snake.body().contains(&pos) {
                    self.food = pos;
                    return;
                }
            }
        }

        // If we get here, the board is completely full - this is a win condition!
        // Just place food at (0,0) - the player has won anyway
        self.food = Position::new(0, 0);
    }

    /// Increases the game speed based on score (progressive difficulty)
    fn increase_speed(&mut self) {
        let min_speed = self.difficulty.min_speed();
        // Decrease delay by 2ms every 5 points, but don't go below min_speed
        let speed_decrease = (self.score / 50) * 2;
        let new_speed = self.difficulty.initial_speed().saturating_sub(speed_decrease as u64);
        self.speed = new_speed.max(min_speed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_opposite() {
        assert!(Direction::Up.is_opposite(&Direction::Down));
        assert!(Direction::Down.is_opposite(&Direction::Up));
        assert!(Direction::Left.is_opposite(&Direction::Right));
        assert!(Direction::Right.is_opposite(&Direction::Left));
        assert!(!Direction::Up.is_opposite(&Direction::Left));
        assert!(!Direction::Up.is_opposite(&Direction::Right));
    }

    #[test]
    fn test_position_move() {
        let pos = Position::new(5, 5);
        assert_eq!(pos.move_in_direction(Direction::Up), Position::new(5, 4));
        assert_eq!(pos.move_in_direction(Direction::Down), Position::new(5, 6));
        assert_eq!(pos.move_in_direction(Direction::Left), Position::new(4, 5));
        assert_eq!(pos.move_in_direction(Direction::Right), Position::new(6, 5));
    }

    #[test]
    fn test_snake_creation() {
        let snake = Snake::new(Position::new(10, 10), Direction::Right);
        assert_eq!(snake.len(), 3);
        assert_eq!(snake.head(), Position::new(10, 10));
        assert_eq!(snake.direction(), Direction::Right);
    }

    #[test]
    fn test_snake_movement() {
        let mut snake = Snake::new(Position::new(10, 10), Direction::Right);
        let initial_len = snake.len();
        snake.advance();
        assert_eq!(snake.head(), Position::new(11, 10));
        assert_eq!(snake.len(), initial_len); // Length shouldn't change
    }

    #[test]
    fn test_snake_growing() {
        let mut snake = Snake::new(Position::new(10, 10), Direction::Right);
        let initial_len = snake.len();
        snake.grow();
        snake.advance();
        assert_eq!(snake.len(), initial_len + 1);
    }

    #[test]
    fn test_snake_direction_change() {
        let mut snake = Snake::new(Position::new(10, 10), Direction::Right);
        snake.set_direction(Direction::Up);
        snake.advance();
        assert_eq!(snake.head(), Position::new(10, 9));
    }

    #[test]
    fn test_snake_no_180_turn() {
        let mut snake = Snake::new(Position::new(10, 10), Direction::Right);
        snake.set_direction(Direction::Left); // Try to go opposite
        snake.advance();
        assert_eq!(snake.head(), Position::new(11, 10)); // Should still go right
    }

    #[test]
    fn test_snake_self_collision() {
        let mut snake = Snake::new(Position::new(10, 10), Direction::Right);
        assert!(!snake.self_collision());

        // Create a scenario where snake will collide with itself
        for _ in 0..3 {
            snake.grow();
            snake.advance();
        }
        snake.set_direction(Direction::Down);
        snake.advance();
        snake.set_direction(Direction::Left);
        snake.advance();
        snake.set_direction(Direction::Up);
        snake.advance();

        assert!(snake.self_collision());
    }

    #[test]
    fn test_game_creation() {
        let game = Game::new(20, 20, Difficulty::Medium);
        assert_eq!(game.state(), GameState::Running);
        assert_eq!(game.score(), 0);
        assert_eq!(game.width(), 20);
        assert_eq!(game.height(), 20);
    }

    #[test]
    fn test_game_wall_collision() {
        let mut game = Game::new(20, 20, Difficulty::Medium);
        // Move snake to the right edge
        for _ in 0..20 {
            game.update();
        }
        assert_eq!(game.state(), GameState::GameOver);
    }

    #[test]
    fn test_game_pause() {
        let mut game = Game::new(20, 20, Difficulty::Medium);
        assert_eq!(game.state(), GameState::Running);
        game.toggle_pause();
        assert_eq!(game.state(), GameState::Paused);
        game.toggle_pause();
        assert_eq!(game.state(), GameState::Running);
    }

    #[test]
    fn test_difficulty_speeds() {
        assert_eq!(Difficulty::Easy.initial_speed(), 150);
        assert_eq!(Difficulty::Medium.initial_speed(), 100);
        assert_eq!(Difficulty::Hard.initial_speed(), 70);
        assert_eq!(Difficulty::Extreme.initial_speed(), 50);
    }

    #[test]
    fn test_food_not_on_snake() {
        let game = Game::new(20, 20, Difficulty::Medium);
        let food_pos = game.food();
        assert!(!game.snake().body().contains(&food_pos));
    }

    #[test]
    fn test_score_increases_on_food() {
        let mut game = Game::new(20, 20, Difficulty::Medium);
        let initial_score = game.score();

        // Manually position food in front of snake
        let head = game.snake().head();
        let food_pos = head.move_in_direction(game.snake().direction());
        game.food = food_pos;

        game.update();
        assert_eq!(game.score(), initial_score + 10);
    }

    #[test]
    fn test_snake_grows_on_food() {
        let mut game = Game::new(20, 20, Difficulty::Medium);
        let initial_len = game.snake().len();

        // Position food in front of snake
        let head = game.snake().head();
        let food_pos = head.move_in_direction(game.snake().direction());
        game.food = food_pos;

        game.update();
        game.update(); // One more update to see the growth
        assert!(game.snake().len() > initial_len);
    }
}
