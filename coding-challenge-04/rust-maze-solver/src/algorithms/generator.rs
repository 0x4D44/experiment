use crate::maze::{CellType, Maze};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratorAlgorithm {
    RecursiveBacktracker,
    Prims,
    Kruskals,
    AldousBroder,
}

pub struct MazeGenerator;

impl MazeGenerator {
    pub fn generate(
        width: usize,
        height: usize,
        algorithm: GeneratorAlgorithm,
    ) -> Maze {
        match algorithm {
            GeneratorAlgorithm::RecursiveBacktracker => {
                Self::recursive_backtracker(width, height)
            }
            GeneratorAlgorithm::Prims => Self::prims(width, height),
            GeneratorAlgorithm::Kruskals => Self::kruskals(width, height),
            GeneratorAlgorithm::AldousBroder => Self::aldous_broder(width, height),
        }
    }

    /// Recursive Backtracker (DFS-based) - Creates perfect mazes with long corridors
    fn recursive_backtracker(width: usize, height: usize) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        // Start from random position
        let start_row = rng.gen_range(0..height);
        let start_col = rng.gen_range(0..width);

        stack.push((start_row, start_col));
        visited.insert((start_row, start_col));

        while let Some((row, col)) = stack.last().copied() {
            maze.set_cell_type(row, col, CellType::Path);

            // Get unvisited neighbors
            let neighbors = Self::get_unvisited_neighbors(&maze, row, col, &visited);

            if neighbors.is_empty() {
                stack.pop();
            } else {
                let &(next_row, next_col) = neighbors.choose(&mut rng).unwrap();
                Self::carve_path(&mut maze, row, col, next_row, next_col);
                visited.insert((next_row, next_col));
                stack.push((next_row, next_col));
            }
        }

        maze.mark_start();
        maze.mark_end();
        maze
    }

    /// Prim's Algorithm - Creates mazes with many short dead ends
    fn prims(width: usize, height: usize) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();
        let mut walls = Vec::new();
        let mut visited = HashSet::new();

        // Start from random cell
        let start_row = rng.gen_range(0..height);
        let start_col = rng.gen_range(0..width);

        visited.insert((start_row, start_col));
        maze.set_cell_type(start_row, start_col, CellType::Path);

        // Add walls of starting cell
        for (nr, nc) in maze.neighbors(start_row, start_col) {
            if !visited.contains(&(nr, nc)) {
                walls.push((start_row, start_col, nr, nc));
            }
        }

        while !walls.is_empty() {
            let idx = rng.gen_range(0..walls.len());
            let (from_row, from_col, to_row, to_col) = walls.swap_remove(idx);

            if !visited.contains(&(to_row, to_col)) {
                visited.insert((to_row, to_col));
                Self::carve_path(&mut maze, from_row, from_col, to_row, to_col);

                // Add new walls
                for (nr, nc) in maze.neighbors(to_row, to_col) {
                    if !visited.contains(&(nr, nc)) {
                        walls.push((to_row, to_col, nr, nc));
                    }
                }
            }
        }

        maze.mark_start();
        maze.mark_end();
        maze
    }

    /// Kruskal's Algorithm - Creates uniform mazes
    fn kruskals(width: usize, height: usize) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();

        // Initialize all cells as paths in their own set
        let mut parent: Vec<Vec<(usize, usize)>> = (0..height)
            .map(|r| (0..width).map(|c| (r, c)).collect())
            .collect();

        for row in 0..height {
            for col in 0..width {
                maze.set_cell_type(row, col, CellType::Path);
            }
        }

        // Create list of all possible edges
        let mut edges = Vec::new();
        for row in 0..height {
            for col in 0..width {
                if row + 1 < height {
                    edges.push((row, col, row + 1, col));
                }
                if col + 1 < width {
                    edges.push((row, col, row, col + 1));
                }
            }
        }

        edges.shuffle(&mut rng);

        // Process edges
        for (r1, c1, r2, c2) in edges {
            let root1 = Self::find_root(&parent, r1, c1);
            let root2 = Self::find_root(&parent, r2, c2);

            if root1 != root2 {
                Self::carve_path(&mut maze, r1, c1, r2, c2);
                parent[root2.0][root2.1] = root1;
            }
        }

        maze.mark_start();
        maze.mark_end();
        maze
    }

    /// Aldous-Broder Algorithm - Random walk that creates uniform spanning trees
    fn aldous_broder(width: usize, height: usize) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();
        let mut visited = HashSet::new();
        let total_cells = width * height;

        // Start from random position
        let mut current_row = rng.gen_range(0..height);
        let mut current_col = rng.gen_range(0..width);

        visited.insert((current_row, current_col));
        maze.set_cell_type(current_row, current_col, CellType::Path);

        while visited.len() < total_cells {
            let neighbors = maze.neighbors(current_row, current_col);
            let &(next_row, next_col) = neighbors.choose(&mut rng).unwrap();

            if !visited.contains(&(next_row, next_col)) {
                Self::carve_path(&mut maze, current_row, current_col, next_row, next_col);
                visited.insert((next_row, next_col));
            }

            current_row = next_row;
            current_col = next_col;
        }

        maze.mark_start();
        maze.mark_end();
        maze
    }

    fn get_unvisited_neighbors(
        maze: &Maze,
        row: usize,
        col: usize,
        visited: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        maze.neighbors(row, col)
            .into_iter()
            .filter(|pos| !visited.contains(pos))
            .collect()
    }

    fn carve_path(maze: &mut Maze, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        maze.set_cell_type(to_row, to_col, CellType::Path);

        // Remove walls between cells (for visualization purposes if needed)
        // We need to do this in two separate steps to avoid mutable borrow issues
        if from_row == to_row {
            // Horizontal movement
            if from_col < to_col {
                if let Some(from_cell) = maze.get_mut(from_row, from_col) {
                    from_cell.walls.east = false;
                }
                if let Some(to_cell) = maze.get_mut(to_row, to_col) {
                    to_cell.walls.west = false;
                }
            } else {
                if let Some(from_cell) = maze.get_mut(from_row, from_col) {
                    from_cell.walls.west = false;
                }
                if let Some(to_cell) = maze.get_mut(to_row, to_col) {
                    to_cell.walls.east = false;
                }
            }
        } else {
            // Vertical movement
            if from_row < to_row {
                if let Some(from_cell) = maze.get_mut(from_row, from_col) {
                    from_cell.walls.south = false;
                }
                if let Some(to_cell) = maze.get_mut(to_row, to_col) {
                    to_cell.walls.north = false;
                }
            } else {
                if let Some(from_cell) = maze.get_mut(from_row, from_col) {
                    from_cell.walls.north = false;
                }
                if let Some(to_cell) = maze.get_mut(to_row, to_col) {
                    to_cell.walls.south = false;
                }
            }
        }
    }

    fn find_root(parent: &[Vec<(usize, usize)>], row: usize, col: usize) -> (usize, usize) {
        let mut current = (row, col);
        while parent[current.0][current.1] != current {
            current = parent[current.0][current.1];
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_backtracker_generates_maze() {
        let maze = MazeGenerator::generate(10, 10, GeneratorAlgorithm::RecursiveBacktracker);
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);

        // Should have at least some path cells
        let path_count = maze.count_cells(CellType::Path);
        assert!(path_count > 0);
    }

    #[test]
    fn test_prims_generates_maze() {
        let maze = MazeGenerator::generate(15, 15, GeneratorAlgorithm::Prims);
        assert_eq!(maze.width, 15);
        assert_eq!(maze.height, 15);

        let path_count = maze.count_cells(CellType::Path);
        assert!(path_count > 0);
    }

    #[test]
    fn test_kruskals_generates_maze() {
        let maze = MazeGenerator::generate(12, 12, GeneratorAlgorithm::Kruskals);
        assert_eq!(maze.width, 12);
        assert_eq!(maze.height, 12);

        let path_count = maze.count_cells(CellType::Path);
        assert!(path_count > 0);
    }

    #[test]
    fn test_aldous_broder_generates_maze() {
        let maze = MazeGenerator::generate(8, 8, GeneratorAlgorithm::AldousBroder);
        assert_eq!(maze.width, 8);
        assert_eq!(maze.height, 8);

        let path_count = maze.count_cells(CellType::Path);
        assert!(path_count > 0);
    }

    #[test]
    fn test_maze_has_start_and_end() {
        let maze = MazeGenerator::generate(10, 10, GeneratorAlgorithm::RecursiveBacktracker);
        assert_eq!(maze.count_cells(CellType::Start), 1);
        assert_eq!(maze.count_cells(CellType::End), 1);
    }

    #[test]
    fn test_different_algorithms_work() {
        // Just verify that different algorithms can generate valid mazes
        let algorithms = vec![
            GeneratorAlgorithm::RecursiveBacktracker,
            GeneratorAlgorithm::Prims,
            GeneratorAlgorithm::Kruskals,
            GeneratorAlgorithm::AldousBroder,
        ];

        for algo in algorithms {
            let maze = MazeGenerator::generate(15, 15, algo);
            assert_eq!(maze.width, 15);
            assert_eq!(maze.height, 15);
            // Verify it has paths
            let path_count = maze.count_cells(CellType::Path);
            assert!(path_count > 0, "Algorithm {:?} should create paths", algo);
        }
    }
}
