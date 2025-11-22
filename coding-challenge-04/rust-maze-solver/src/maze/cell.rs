use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellType {
    Wall,
    Path,
    Start,
    End,
    Visited,
    Solution,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: CellType,
    pub walls: Walls,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Walls {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            cell_type: CellType::Wall,
            walls: Walls {
                north: true,
                south: true,
                east: true,
                west: true,
            },
        }
    }

    pub fn empty() -> Self {
        Self {
            cell_type: CellType::Path,
            walls: Walls {
                north: false,
                south: false,
                east: false,
                west: false,
            },
        }
    }

    pub fn is_walkable(&self) -> bool {
        matches!(
            self.cell_type,
            CellType::Path | CellType::Start | CellType::End | CellType::Visited
        )
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl Walls {
    pub fn remove_wall(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.north = false,
            Direction::South => self.south = false,
            Direction::East => self.east = false,
            Direction::West => self.west = false,
        }
    }

    pub fn all_up() -> Self {
        Self {
            north: true,
            south: true,
            east: true,
            west: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new();
        assert_eq!(cell.cell_type, CellType::Wall);
        assert!(cell.walls.north);
        assert!(cell.walls.south);
        assert!(cell.walls.east);
        assert!(cell.walls.west);
    }

    #[test]
    fn test_empty_cell() {
        let cell = Cell::empty();
        assert_eq!(cell.cell_type, CellType::Path);
        assert!(!cell.walls.north);
        assert!(!cell.walls.south);
        assert!(!cell.walls.east);
        assert!(!cell.walls.west);
    }

    #[test]
    fn test_is_walkable() {
        let mut cell = Cell::new();
        assert!(!cell.is_walkable());

        cell.cell_type = CellType::Path;
        assert!(cell.is_walkable());

        cell.cell_type = CellType::Start;
        assert!(cell.is_walkable());
    }

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
    }

    #[test]
    fn test_direction_delta() {
        assert_eq!(Direction::North.delta(), (-1, 0));
        assert_eq!(Direction::South.delta(), (1, 0));
        assert_eq!(Direction::East.delta(), (0, 1));
        assert_eq!(Direction::West.delta(), (0, -1));
    }
}
