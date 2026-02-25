// Temporary debug file
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
}

fn main() {
    let mut snake = Snake::new(Position::new(5, 5), Direction::Right);
    snake.grow(5);

    println!("Initial: {:?}", snake.segments);

    snake.move_forward();
    println!("After move 1: {:?}", snake.segments);

    snake.move_forward();
    println!("After move 2: {:?}", snake.segments);

    snake.move_forward();
    println!("After move 3: {:?}", snake.segments);

    snake.move_forward();
    println!("After move 4: {:?}", snake.segments);

    snake.move_forward();
    println!("After move 5: {:?}", snake.segments);
    println!("grow_pending after 5 moves: {}", snake.grow_pending);

    snake.change_direction(Direction::Up);
    snake.move_forward();
    println!("After up: {:?}", snake.segments);

    snake.change_direction(Direction::Left);
    snake.move_forward();
    println!("After left 1: {:?}", snake.segments);

    snake.move_forward();
    println!("After left 2: {:?}", snake.segments);

    snake.change_direction(Direction::Down);
    snake.move_forward();
    println!("After down: {:?}", snake.segments);

    println!("Collides with self? {}", snake.collides_with_self());
}
