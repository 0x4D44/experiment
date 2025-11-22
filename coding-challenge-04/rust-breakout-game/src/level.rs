use crate::physics::Rect;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrickType {
    Normal,
    Strong,
    Unbreakable,
    Bonus,
}

impl BrickType {
    pub fn hits_required(&self) -> u32 {
        match self {
            BrickType::Normal => 1,
            BrickType::Strong => 2,
            BrickType::Unbreakable => u32::MAX,
            BrickType::Bonus => 1,
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            BrickType::Normal => 10,
            BrickType::Strong => 25,
            BrickType::Unbreakable => 0,
            BrickType::Bonus => 50,
        }
    }

    pub fn symbol(&self, hits: u32) -> &str {
        match self {
            BrickType::Normal => "█",
            BrickType::Strong if hits == 0 => "▓",
            BrickType::Strong => "▒",
            BrickType::Unbreakable => "▓",
            BrickType::Bonus => "◆",
        }
    }

    pub fn drops_powerup(&self) -> bool {
        match self {
            BrickType::Bonus => true,
            BrickType::Normal => rand::thread_rng().gen_bool(0.15),
            BrickType::Strong => rand::thread_rng().gen_bool(0.25),
            BrickType::Unbreakable => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Brick {
    pub rect: Rect,
    pub brick_type: BrickType,
    pub hits: u32,
    pub active: bool,
}

impl Brick {
    pub fn new(x: f32, y: f32, width: f32, height: f32, brick_type: BrickType) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            brick_type,
            hits: 0,
            active: true,
        }
    }

    pub fn hit(&mut self) -> bool {
        if !self.active || self.brick_type == BrickType::Unbreakable {
            return false;
        }

        self.hits += 1;
        if self.hits >= self.brick_type.hits_required() {
            self.active = false;
            true
        } else {
            false
        }
    }

    pub fn color_index(&self) -> usize {
        match self.brick_type {
            BrickType::Normal => 0,
            BrickType::Strong if self.hits == 0 => 1,
            BrickType::Strong => 2,
            BrickType::Unbreakable => 3,
            BrickType::Bonus => 4,
        }
    }
}

pub struct Level {
    #[allow(dead_code)]
    pub number: u32,
    pub bricks: Vec<Brick>,
}

impl Level {
    pub fn load(level_number: u32, game_width: f32) -> Self {
        let bricks = match level_number {
            1 => Self::create_level_1(game_width),
            2 => Self::create_level_2(game_width),
            3 => Self::create_level_3(game_width),
            4 => Self::create_level_4(game_width),
            5 => Self::create_level_5(game_width),
            _ => Self::create_level_random(level_number, game_width),
        };

        Self {
            number: level_number,
            bricks,
        }
    }

    fn create_level_1(game_width: f32) -> Vec<Brick> {
        let mut bricks = Vec::new();
        let brick_width = 8.0;
        let brick_height = 2.0;
        let cols = (game_width / brick_width) as usize;
        let rows = 5;

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * brick_width;
                let y = 3.0 + row as f32 * brick_height;
                let brick_type = if row == 2 && col % 3 == 0 {
                    BrickType::Bonus
                } else {
                    BrickType::Normal
                };
                bricks.push(Brick::new(x, y, brick_width, brick_height, brick_type));
            }
        }
        bricks
    }

    fn create_level_2(game_width: f32) -> Vec<Brick> {
        let mut bricks = Vec::new();
        let brick_width = 8.0;
        let brick_height = 2.0;
        let cols = (game_width / brick_width) as usize;
        let rows = 6;

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * brick_width;
                let y = 3.0 + row as f32 * brick_height;

                let brick_type = if row % 2 == 0 {
                    BrickType::Strong
                } else if col % 4 == 0 {
                    BrickType::Bonus
                } else {
                    BrickType::Normal
                };

                bricks.push(Brick::new(x, y, brick_width, brick_height, brick_type));
            }
        }
        bricks
    }

    fn create_level_3(game_width: f32) -> Vec<Brick> {
        let mut bricks = Vec::new();
        let brick_width = 8.0;
        let brick_height = 2.0;
        let cols = (game_width / brick_width) as usize;
        let rows = 7;

        for row in 0..rows {
            for col in 0..cols {
                // Create a pyramid pattern
                if col >= row && col < cols - row {
                    let x = col as f32 * brick_width;
                    let y = 3.0 + row as f32 * brick_height;

                    let brick_type = if col == row || col == cols - row - 1 {
                        BrickType::Strong
                    } else if row == 0 {
                        BrickType::Bonus
                    } else {
                        BrickType::Normal
                    };

                    bricks.push(Brick::new(x, y, brick_width, brick_height, brick_type));
                }
            }
        }
        bricks
    }

    fn create_level_4(game_width: f32) -> Vec<Brick> {
        let mut bricks = Vec::new();
        let brick_width = 8.0;
        let brick_height = 2.0;
        let cols = (game_width / brick_width) as usize;
        let rows = 8;

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * brick_width;
                let y = 3.0 + row as f32 * brick_height;

                // Create checkerboard with unbreakable blocks
                let brick_type = if (row + col) % 4 == 0 {
                    BrickType::Unbreakable
                } else if (row + col) % 3 == 0 {
                    BrickType::Strong
                } else if col % 5 == 0 {
                    BrickType::Bonus
                } else {
                    BrickType::Normal
                };

                bricks.push(Brick::new(x, y, brick_width, brick_height, brick_type));
            }
        }
        bricks
    }

    fn create_level_5(game_width: f32) -> Vec<Brick> {
        let mut bricks = Vec::new();
        let brick_width = 8.0;
        let brick_height = 2.0;
        let cols = (game_width / brick_width) as usize;
        let rows = 9;

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * brick_width;
                let y = 3.0 + row as f32 * brick_height;

                // Create complex pattern with walls
                let brick_type = if col == 0 || col == cols - 1 {
                    BrickType::Unbreakable
                } else if row % 3 == 0 {
                    BrickType::Strong
                } else if row % 2 == 0 && col % 2 == 0 {
                    BrickType::Bonus
                } else {
                    BrickType::Normal
                };

                bricks.push(Brick::new(x, y, brick_width, brick_height, brick_type));
            }
        }
        bricks
    }

    fn create_level_random(level_number: u32, game_width: f32) -> Vec<Brick> {
        let mut bricks = Vec::new();
        let brick_width = 8.0;
        let brick_height = 2.0;
        let cols = (game_width / brick_width) as usize;
        let rows = (5 + level_number.min(10)) as usize;

        let mut rng = rand::thread_rng();

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * brick_width;
                let y = 3.0 + row as f32 * brick_height;

                let brick_type = match rng.gen_range(0..10) {
                    0..=5 => BrickType::Normal,
                    6..=7 => BrickType::Strong,
                    8 => BrickType::Bonus,
                    _ => BrickType::Unbreakable,
                };

                bricks.push(Brick::new(x, y, brick_width, brick_height, brick_type));
            }
        }
        bricks
    }

    pub fn all_breakable_destroyed(&self) -> bool {
        self.bricks.iter().all(|brick| {
            !brick.active || brick.brick_type == BrickType::Unbreakable
        })
    }

    #[allow(dead_code)]
    pub fn active_brick_count(&self) -> usize {
        self.bricks.iter().filter(|b| b.active).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brick_hit() {
        let mut brick = Brick::new(0.0, 0.0, 10.0, 2.0, BrickType::Normal);
        assert!(brick.active);

        let destroyed = brick.hit();
        assert!(destroyed);
        assert!(!brick.active);
    }

    #[test]
    fn test_strong_brick() {
        let mut brick = Brick::new(0.0, 0.0, 10.0, 2.0, BrickType::Strong);

        let destroyed = brick.hit();
        assert!(!destroyed);
        assert!(brick.active);

        let destroyed = brick.hit();
        assert!(destroyed);
        assert!(!brick.active);
    }

    #[test]
    fn test_unbreakable_brick() {
        let mut brick = Brick::new(0.0, 0.0, 10.0, 2.0, BrickType::Unbreakable);

        for _ in 0..10 {
            let destroyed = brick.hit();
            assert!(!destroyed);
            assert!(brick.active);
        }
    }

    #[test]
    fn test_level_creation() {
        let level = Level::load(1, 80.0);
        assert_eq!(level.number, 1);
        assert!(level.bricks.len() > 0);
    }

    #[test]
    fn test_level_completion() {
        let mut level = Level::load(1, 80.0);

        // Destroy all breakable bricks
        for brick in &mut level.bricks {
            if brick.brick_type != BrickType::Unbreakable {
                while brick.active {
                    brick.hit();
                }
            }
        }

        assert!(level.all_breakable_destroyed());
    }
}
