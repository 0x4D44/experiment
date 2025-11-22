use super::cell::{Cell, CellType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::new(); width]; height];
        let start = (0, 0);
        let end = (height.saturating_sub(1), width.saturating_sub(1));

        Self {
            width,
            height,
            grid,
            start,
            end,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.grid.get(row).and_then(|r| r.get(col))
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.grid.get_mut(row).and_then(|r| r.get_mut(col))
    }

    pub fn set_cell_type(&mut self, row: usize, col: usize, cell_type: CellType) {
        if let Some(cell) = self.get_mut(row, col) {
            cell.cell_type = cell_type;
        }
    }

    pub fn is_valid_position(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && (row as usize) < self.height && (col as usize) < self.width
    }

    pub fn mark_start(&mut self) {
        let (row, col) = self.start;
        self.set_cell_type(row, col, CellType::Start);
    }

    pub fn mark_end(&mut self) {
        let (row, col) = self.end;
        self.set_cell_type(row, col, CellType::End);
    }

    pub fn reset_solution(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                if cell.cell_type == CellType::Visited || cell.cell_type == CellType::Solution {
                    cell.cell_type = CellType::Path;
                }
            }
        }
        self.mark_start();
        self.mark_end();
    }

    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if self.is_valid_position(new_row, new_col) {
                neighbors.push((new_row as usize, new_col as usize));
            }
        }

        neighbors
    }

    pub fn count_cells(&self, cell_type: CellType) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|cell| cell.cell_type == cell_type)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_creation() {
        let maze = Maze::new(10, 10);
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);
        assert_eq!(maze.grid.len(), 10);
        assert_eq!(maze.grid[0].len(), 10);
    }

    #[test]
    fn test_get_cell() {
        let maze = Maze::new(5, 5);
        assert!(maze.get(0, 0).is_some());
        assert!(maze.get(4, 4).is_some());
        assert!(maze.get(5, 5).is_none());
    }

    #[test]
    fn test_is_valid_position() {
        let maze = Maze::new(10, 10);
        assert!(maze.is_valid_position(0, 0));
        assert!(maze.is_valid_position(9, 9));
        assert!(!maze.is_valid_position(-1, 0));
        assert!(!maze.is_valid_position(0, 10));
        assert!(!maze.is_valid_position(10, 0));
    }

    #[test]
    fn test_neighbors() {
        let maze = Maze::new(5, 5);

        // Corner cell (0,0) should have 2 neighbors
        let neighbors = maze.neighbors(0, 0);
        assert_eq!(neighbors.len(), 2);

        // Center cell should have 4 neighbors
        let neighbors = maze.neighbors(2, 2);
        assert_eq!(neighbors.len(), 4);

        // Edge cell should have 3 neighbors
        let neighbors = maze.neighbors(0, 2);
        assert_eq!(neighbors.len(), 3);
    }

    #[test]
    fn test_set_cell_type() {
        let mut maze = Maze::new(5, 5);
        maze.set_cell_type(2, 2, CellType::Path);

        if let Some(cell) = maze.get(2, 2) {
            assert_eq!(cell.cell_type, CellType::Path);
        } else {
            panic!("Cell should exist");
        }
    }

    #[test]
    fn test_mark_start_end() {
        let mut maze = Maze::new(5, 5);
        maze.mark_start();
        maze.mark_end();

        let start_cell = maze.get(maze.start.0, maze.start.1).unwrap();
        assert_eq!(start_cell.cell_type, CellType::Start);

        let end_cell = maze.get(maze.end.0, maze.end.1).unwrap();
        assert_eq!(end_cell.cell_type, CellType::End);
    }

    #[test]
    fn test_reset_solution() {
        let mut maze = Maze::new(5, 5);
        maze.set_cell_type(1, 1, CellType::Visited);
        maze.set_cell_type(2, 2, CellType::Solution);

        maze.reset_solution();

        assert_eq!(maze.get(1, 1).unwrap().cell_type, CellType::Path);
        assert_eq!(maze.get(2, 2).unwrap().cell_type, CellType::Path);
    }
}
