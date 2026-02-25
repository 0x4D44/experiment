use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    // Movement
    Go(Direction),
    Look,
    Examine(String),

    // Investigation
    Touch(String),
    Focus(String),
    Remember(String),
    Compare(String, String),

    // Interaction
    Talk(String),
    AskAbout(String, String),
    Show(String, String),
    Confront(String, String),
    Accuse(String),

    // Management
    Inventory,
    Timeline,
    Notes,
    Help,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
            Direction::Up => "up",
            Direction::Down => "down",
        }
    }
}
