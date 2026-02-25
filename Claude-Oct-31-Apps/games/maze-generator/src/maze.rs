use std::fmt;

/// Represents the four cardinal directions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

/// Maze generation algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    RecursiveBacktracker,
    Kruskal,
    Prim,
    BinaryTree,
    AldousBroder,
    Wilson,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Algorithm::RecursiveBacktracker => write!(f, "Recursive Backtracker"),
            Algorithm::Kruskal => write!(f, "Kruskal's Algorithm"),
            Algorithm::Prim => write!(f, "Prim's Algorithm"),
            Algorithm::BinaryTree => write!(f, "Binary Tree"),
            Algorithm::AldousBroder => write!(f, "Aldous-Broder"),
            Algorithm::Wilson => write!(f, "Wilson's Algorithm"),
        }
    }
}

/// Represents a single cell in the maze
/// Walls are stored as [North, East, South, West] booleans
/// true = wall exists, false = passage open
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub walls: [bool; 4], // [N, E, S, W]
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Cell {
            x,
            y,
            walls: [true, true, true, true], // All walls initially
        }
    }

    pub fn has_wall(&self, direction: Direction) -> bool {
        self.walls[direction as usize]
    }

    pub fn remove_wall(&mut self, direction: Direction) {
        self.walls[direction as usize] = false;
    }

    pub fn add_wall(&mut self, direction: Direction) {
        self.walls[direction as usize] = true;
    }

    pub fn opposite_direction(direction: Direction) -> Direction {
        match direction {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

/// Represents a complete maze
#[derive(Debug, Clone)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub seed: u64,
    pub algorithm: Algorithm,
}

impl Maze {
    /// Create a new maze with all walls initially present
    pub fn new(width: usize, height: usize, seed: u64, algorithm: Algorithm) -> Self {
        let mut cells = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x, y));
            }
        }

        // Default start at top-left, end at bottom-right
        let start = (0, 0);
        let end = (width - 1, height - 1);

        Maze {
            width,
            height,
            cells,
            start,
            end,
            seed,
            algorithm,
        }
    }

    /// Get a cell reference by coordinates
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }

    /// Get a mutable cell reference by coordinates
    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            Some(&mut self.cells[y * self.width + x])
        } else {
            None
        }
    }

    /// Check if a passage exists between two adjacent cells
    pub fn is_passage_open(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (fx, fy) = from;
        let (tx, ty) = to;

        if let Some(from_cell) = self.get_cell(fx, fy) {
            if tx == fx + 1 && ty == fy {
                // Moving east
                !from_cell.walls[Direction::East as usize]
            } else if tx == fx && ty == fy + 1 {
                // Moving south
                !from_cell.walls[Direction::South as usize]
            } else if tx + 1 == fx && ty == fy {
                // Moving west (check from origin)
                !from_cell.walls[Direction::West as usize]
            } else if tx == fx && ty + 1 == fy {
                // Moving north (check from origin)
                !from_cell.walls[Direction::North as usize]
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get valid neighboring cells (cells with open passages)
    pub fn get_valid_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if let Some(cell) = self.get_cell(x, y) {
            // North
            if !cell.walls[Direction::North as usize] && y > 0 {
                neighbors.push((x, y.saturating_sub(1)));
            }
            // East
            if !cell.walls[Direction::East as usize] && x < self.width.saturating_sub(1) {
                neighbors.push((x + 1, y));
            }
            // South
            if !cell.walls[Direction::South as usize] && y < self.height.saturating_sub(1) {
                neighbors.push((x, y + 1));
            }
            // West
            if !cell.walls[Direction::West as usize] && x > 0 {
                neighbors.push((x.saturating_sub(1), y));
            }
        }

        neighbors
    }

    /// Get all neighboring cell positions (regardless of walls)
    pub fn get_all_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if y > 0 {
            neighbors.push((x, y - 1)); // North
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y)); // East
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1)); // South
        }
        if x > 0 {
            neighbors.push((x - 1, y)); // West
        }

        neighbors
    }

    /// Remove wall between two adjacent cells (create passage)
    pub fn carve_passage(&mut self, from: (usize, usize), to: (usize, usize)) {
        let (fx, fy) = from;
        let (tx, ty) = to;

        if let Some(from_cell) = self.get_cell_mut(fx, fy) {
            if tx == fx + 1 && ty == fy {
                // Moving east
                from_cell.remove_wall(Direction::East);
            } else if tx == fx && ty == fy + 1 {
                // Moving south
                from_cell.remove_wall(Direction::South);
            } else if tx + 1 == fx && ty == fy {
                // Moving west (tx is to the west)
                from_cell.remove_wall(Direction::West);
            } else if tx == fx && ty + 1 == fy {
                // Moving north (ty is to the north)
                from_cell.remove_wall(Direction::North);
            }
        }

        // Also remove wall from the other side
        if let Some(to_cell) = self.get_cell_mut(tx, ty) {
            if fx == tx + 1 && fy == ty {
                // from is to the east
                to_cell.remove_wall(Direction::East);
            } else if fx == tx && fy == ty + 1 {
                // from is to the south
                to_cell.remove_wall(Direction::South);
            } else if fx + 1 == tx && fy == ty {
                // from is to the west
                to_cell.remove_wall(Direction::West);
            } else if fx == tx && fy + 1 == ty {
                // from is to the north
                to_cell.remove_wall(Direction::North);
            }
        }
    }

    /// Check if a point is within maze bounds
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Get Manhattan distance between two points
    pub fn manhattan_distance(&self, from: (usize, usize), to: (usize, usize)) -> usize {
        let dx = (from.0 as i32 - to.0 as i32).abs() as usize;
        let dy = (from.1 as i32 - to.1 as i32).abs() as usize;
        dx + dy
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Maze({}x{}, algorithm: {}, seed: {})",
            self.width, self.height, self.algorithm, self.seed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new(5, 10);
        assert_eq!(cell.x, 5);
        assert_eq!(cell.y, 10);
        assert!(cell.has_wall(Direction::North));
        assert!(cell.has_wall(Direction::East));
        assert!(cell.has_wall(Direction::South));
        assert!(cell.has_wall(Direction::West));
    }

    #[test]
    fn test_wall_manipulation() {
        let mut cell = Cell::new(0, 0);
        assert!(cell.has_wall(Direction::North));

        cell.remove_wall(Direction::North);
        assert!(!cell.has_wall(Direction::North));

        cell.add_wall(Direction::North);
        assert!(cell.has_wall(Direction::North));
    }

    #[test]
    fn test_opposite_direction() {
        assert_eq!(Cell::opposite_direction(Direction::North), Direction::South);
        assert_eq!(Cell::opposite_direction(Direction::South), Direction::North);
        assert_eq!(Cell::opposite_direction(Direction::East), Direction::West);
        assert_eq!(Cell::opposite_direction(Direction::West), Direction::East);
    }

    #[test]
    fn test_maze_creation() {
        let maze = Maze::new(10, 15, 42, Algorithm::RecursiveBacktracker);
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 15);
        assert_eq!(maze.cells.len(), 150);
        assert_eq!(maze.seed, 42);
        assert_eq!(maze.start, (0, 0));
        assert_eq!(maze.end, (9, 14));
    }

    #[test]
    fn test_get_cell() {
        let maze = Maze::new(5, 5, 42, Algorithm::RecursiveBacktracker);

        assert!(maze.get_cell(0, 0).is_some());
        assert!(maze.get_cell(4, 4).is_some());
        assert!(maze.get_cell(5, 5).is_none());
        assert!(maze.get_cell(0, 5).is_none());
    }

    #[test]
    fn test_is_in_bounds() {
        let maze = Maze::new(10, 10, 42, Algorithm::RecursiveBacktracker);

        assert!(maze.is_in_bounds(0, 0));
        assert!(maze.is_in_bounds(9, 9));
        assert!(!maze.is_in_bounds(10, 10));
        assert!(!maze.is_in_bounds(0, 10));
    }

    #[test]
    fn test_manhattan_distance() {
        let maze = Maze::new(20, 20, 42, Algorithm::RecursiveBacktracker);

        assert_eq!(maze.manhattan_distance((0, 0), (0, 0)), 0);
        assert_eq!(maze.manhattan_distance((0, 0), (3, 4)), 7);
        assert_eq!(maze.manhattan_distance((5, 5), (10, 10)), 10);
    }

    #[test]
    fn test_get_all_neighbors() {
        let maze = Maze::new(10, 10, 42, Algorithm::RecursiveBacktracker);

        // Corner cell
        let neighbors = maze.get_all_neighbors(0, 0);
        assert_eq!(neighbors.len(), 2);

        // Interior cell
        let neighbors = maze.get_all_neighbors(5, 5);
        assert_eq!(neighbors.len(), 4);

        // Edge cell
        let neighbors = maze.get_all_neighbors(0, 5);
        assert_eq!(neighbors.len(), 3);
    }

    #[test]
    fn test_carve_passage() {
        let mut maze = Maze::new(5, 5, 42, Algorithm::RecursiveBacktracker);

        // Initially all walls
        let cell = maze.get_cell(2, 2).unwrap();
        assert!(cell.has_wall(Direction::East));

        // Carve passage
        maze.carve_passage((2, 2), (3, 2));

        // Check both sides
        let cell = maze.get_cell(2, 2).unwrap();
        assert!(!cell.has_wall(Direction::East));

        let cell = maze.get_cell(3, 2).unwrap();
        assert!(!cell.has_wall(Direction::West));
    }
}
