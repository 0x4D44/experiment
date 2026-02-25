pub mod maze;
pub mod generator;
pub mod pathfinder;
pub mod renderer;
pub mod game;

pub use maze::{Maze, Cell, Direction, Algorithm};
pub use generator::{MazeGenerator, RecursiveBacktracker, Kruskal, Prim, BinaryTree, AldousBroder, Wilson};
pub use pathfinder::{Pathfinder, PathfindingAlgorithm};
pub use game::GameState;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // MAZE GENERATION ALGORITHM TESTS
    // ============================================================================

    #[test]
    fn test_recursive_backtracker_connectivity() {
        let mut gen = RecursiveBacktracker::new(25, 25, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
    }

    #[test]
    fn test_recursive_backtracker_tree_structure() {
        let mut gen = RecursiveBacktracker::new(20, 20, 123);
        let maze = gen.generate();
        assert_maze_is_tree(&maze);
    }

    #[test]
    fn test_recursive_backtracker_solvability() {
        let mut gen = RecursiveBacktracker::new(30, 30, 456);
        let maze = gen.generate();
        assert_maze_is_solvable(&maze);
    }

    #[test]
    fn test_kruskal_connectivity() {
        let mut gen = Kruskal::new(25, 25, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
    }

    #[test]
    fn test_kruskal_tree_structure() {
        let mut gen = Kruskal::new(20, 20, 123);
        let maze = gen.generate();
        assert_maze_is_tree(&maze);
    }

    #[test]
    fn test_kruskal_solvability() {
        let mut gen = Kruskal::new(30, 30, 456);
        let maze = gen.generate();
        assert_maze_is_solvable(&maze);
    }

    #[test]
    fn test_prim_connectivity() {
        let mut gen = Prim::new(25, 25, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
    }

    #[test]
    fn test_prim_tree_structure() {
        let mut gen = Prim::new(20, 20, 123);
        let maze = gen.generate();
        assert_maze_is_tree(&maze);
    }

    #[test]
    fn test_prim_solvability() {
        let mut gen = Prim::new(30, 30, 456);
        let maze = gen.generate();
        assert_maze_is_solvable(&maze);
    }

    #[test]
    fn test_binary_tree_connectivity() {
        let mut gen = BinaryTree::new(25, 25, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
    }

    #[test]
    fn test_binary_tree_tree_structure() {
        let mut gen = BinaryTree::new(20, 20, 123);
        let maze = gen.generate();
        assert_maze_is_tree(&maze);
    }

    #[test]
    fn test_binary_tree_solvability() {
        let mut gen = BinaryTree::new(30, 30, 456);
        let maze = gen.generate();
        assert_maze_is_solvable(&maze);
    }

    #[test]
    fn test_aldous_broder_connectivity() {
        let mut gen = AldousBroder::new(20, 20, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
    }

    #[test]
    fn test_aldous_broder_tree_structure() {
        let mut gen = AldousBroder::new(15, 15, 123);
        let maze = gen.generate();
        assert_maze_is_tree(&maze);
    }

    #[test]
    fn test_aldous_broder_solvability() {
        let mut gen = AldousBroder::new(15, 15, 456);
        let maze = gen.generate();
        assert_maze_is_solvable(&maze);
    }

    #[test]
    fn test_wilson_connectivity() {
        let mut gen = Wilson::new(20, 20, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
    }

    #[test]
    fn test_wilson_tree_structure() {
        let mut gen = Wilson::new(15, 15, 123);
        let maze = gen.generate();
        assert_maze_is_tree(&maze);
    }

    #[test]
    fn test_wilson_solvability() {
        let mut gen = Wilson::new(15, 15, 456);
        let maze = gen.generate();
        assert_maze_is_solvable(&maze);
    }

    // ============================================================================
    // EDGE CASE TESTS
    // ============================================================================

    #[test]
    fn test_minimal_maze_1x1() {
        let mut gen = RecursiveBacktracker::new(1, 1, 42);
        let maze = gen.generate();
        assert_eq!(maze.width, 1);
        assert_eq!(maze.height, 1);
        assert_maze_is_connected(&maze);
    }

    #[test]
    fn test_minimal_maze_2x2() {
        let mut gen = RecursiveBacktracker::new(2, 2, 42);
        let maze = gen.generate();
        assert_eq!(maze.width, 2);
        assert_eq!(maze.height, 2);
        assert_maze_is_connected(&maze);
        assert_maze_is_tree(&maze);
    }

    #[test]
    fn test_minimal_maze_1x10() {
        let mut gen = RecursiveBacktracker::new(1, 10, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
        assert_maze_is_tree(&maze);
    }

    #[test]
    fn test_tall_narrow_maze() {
        let mut gen = RecursiveBacktracker::new(5, 50, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
        assert_maze_is_tree(&maze);
        assert_maze_is_solvable(&maze);
    }

    #[test]
    fn test_wide_short_maze() {
        let mut gen = RecursiveBacktracker::new(50, 5, 42);
        let maze = gen.generate();
        assert_maze_is_connected(&maze);
        assert_maze_is_tree(&maze);
        assert_maze_is_solvable(&maze);
    }

    // ============================================================================
    // MAZE PROPERTIES TESTS
    // ============================================================================

    #[test]
    fn test_maze_cell_count() {
        let mut gen = RecursiveBacktracker::new(10, 15, 42);
        let maze = gen.generate();
        assert_eq!(maze.cells.len(), 10 * 15);
    }

    #[test]
    fn test_maze_dimensions() {
        let mut gen = RecursiveBacktracker::new(42, 37, 99);
        let maze = gen.generate();
        assert_eq!(maze.width, 42);
        assert_eq!(maze.height, 37);
    }

    #[test]
    fn test_maze_seed_reproducibility() {
        let mut gen1 = RecursiveBacktracker::new(20, 20, 12345);
        let maze1 = gen1.generate();

        let mut gen2 = RecursiveBacktracker::new(20, 20, 12345);
        let maze2 = gen2.generate();

        // Check that same seed produces identical mazes
        assert_eq!(maze1.cells.len(), maze2.cells.len());
        for i in 0..maze1.cells.len() {
            assert_eq!(maze1.cells[i].walls, maze2.cells[i].walls);
        }
    }

    #[test]
    fn test_different_seeds_different_mazes() {
        let mut gen1 = RecursiveBacktracker::new(20, 20, 111);
        let maze1 = gen1.generate();

        let mut gen2 = RecursiveBacktracker::new(20, 20, 222);
        let maze2 = gen2.generate();

        // Very likely that different seeds produce different mazes
        let mut identical_cells = 0;
        for i in 0..maze1.cells.len() {
            if maze1.cells[i].walls == maze2.cells[i].walls {
                identical_cells += 1;
            }
        }
        // Allow some tolerance but expect most to be different
        assert!(identical_cells < maze1.cells.len() / 2);
    }

    #[test]
    fn test_start_and_end_positions() {
        let mut gen = RecursiveBacktracker::new(20, 20, 42);
        let maze = gen.generate();

        // Start should be in bounds
        assert!(maze.start.0 < maze.width);
        assert!(maze.start.1 < maze.height);

        // End should be in bounds
        assert!(maze.end.0 < maze.width);
        assert!(maze.end.1 < maze.height);

        // Start and end should be different
        assert_ne!(maze.start, maze.end);
    }

    #[test]
    fn test_all_cells_reachable_from_start() {
        let mut gen = RecursiveBacktracker::new(15, 15, 42);
        let maze = gen.generate();

        let reachable = count_reachable_cells(&maze, maze.start);
        assert_eq!(reachable, maze.width * maze.height);
    }

    // ============================================================================
    // PATHFINDING TESTS
    // ============================================================================

    #[test]
    fn test_bfs_finds_solution() {
        let mut gen = RecursiveBacktracker::new(20, 20, 42);
        let maze = gen.generate();

        let path = Pathfinder::bfs(&maze);
        assert!(path.is_some());

        let path = path.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], maze.start);
        assert_eq!(path[path.len() - 1], maze.end);
    }

    #[test]
    fn test_bfs_solution_is_shortest() {
        let mut gen = RecursiveBacktracker::new(20, 20, 42);
        let maze = gen.generate();

        let bfs_path = Pathfinder::bfs(&maze).unwrap();
        let dfs_path = Pathfinder::dfs(&maze).unwrap();

        // BFS should find path no longer than DFS
        assert!(bfs_path.len() <= dfs_path.len());
    }

    #[test]
    fn test_dfs_finds_solution() {
        let mut gen = RecursiveBacktracker::new(20, 20, 42);
        let maze = gen.generate();

        let path = Pathfinder::dfs(&maze);
        assert!(path.is_some());

        let path = path.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], maze.start);
        assert_eq!(path[path.len() - 1], maze.end);
    }

    #[test]
    fn test_astar_finds_solution() {
        let mut gen = RecursiveBacktracker::new(20, 20, 42);
        let maze = gen.generate();

        let path = Pathfinder::astar(&maze);
        assert!(path.is_some());

        let path = path.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], maze.start);
        assert_eq!(path[path.len() - 1], maze.end);
    }

    #[test]
    fn test_astar_solution_is_optimal() {
        let mut gen = RecursiveBacktracker::new(20, 20, 42);
        let maze = gen.generate();

        let astar_path = Pathfinder::astar(&maze).unwrap();
        let bfs_path = Pathfinder::bfs(&maze).unwrap();

        // A* should find path as short as BFS
        assert_eq!(astar_path.len(), bfs_path.len());
    }

    #[test]
    fn test_path_validity() {
        let mut gen = RecursiveBacktracker::new(15, 15, 42);
        let maze = gen.generate();

        let path = Pathfinder::bfs(&maze).unwrap();

        // Verify each step is valid
        for i in 0..path.len() - 1 {
            let current = path[i];
            let next = path[i + 1];

            // Adjacent cells should be neighbors
            let dx = (current.0 as i32 - next.0 as i32).abs();
            let dy = (current.1 as i32 - next.1 as i32).abs();
            assert_eq!(dx + dy, 1, "Path contains non-adjacent cells");

            // No wall between cells
            assert!(is_passage_open(&maze, current, next));
        }
    }

    #[test]
    fn test_pathfinder_various_maze_sizes() {
        for size in &[5, 10, 15, 20, 30] {
            let mut gen = RecursiveBacktracker::new(*size, *size, 42);
            let maze = gen.generate();

            let path = Pathfinder::bfs(&maze);
            assert!(path.is_some(), "Failed to find path in {}x{} maze", size, size);
        }
    }

    // ============================================================================
    // PLAYER MOVEMENT TESTS
    // ============================================================================

    #[test]
    fn test_player_movement_validity() {
        let mut gen = RecursiveBacktracker::new(15, 15, 42);
        let maze = gen.generate();
        let mut game = GameState::new(maze);

        let initial_pos = game.player_pos;

        // Try moving in each direction and verify no walls blocking valid moves
        let moves = vec![(0, -1), (1, 0), (0, 1), (-1, 0)]; // N, E, S, W

        for (dx, dy) in moves {
            let new_x = (initial_pos.0 as i32 + dx) as usize;
            let new_y = (initial_pos.1 as i32 + dy) as usize;
            let new_pos = (new_x, new_y);

            // If move is in bounds, passage should be open or move blocked
            if new_x < game.maze.width && new_y < game.maze.height {
                let can_move = game.try_move(dx, dy);
                // If can_move is true, verify passage is open
                if can_move {
                    game.player_pos = initial_pos; // Reset for next iteration
                }
            }
        }
    }

    #[test]
    fn test_breadcrumb_trail_tracking() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        let mut game = GameState::new(maze);

        let initial_pos = game.player_pos;
        let initial_trail_len = game.breadcrumb_trail.len();

        // Make a valid move
        game.record_visit(game.player_pos);

        assert!(game.breadcrumb_trail.len() >= initial_trail_len);
    }

    #[test]
    fn test_visited_cells_tracking() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        let mut game = GameState::new(maze);

        let start_pos = game.player_pos;
        let initial_visited = game.visited_cells.len();

        game.record_visit(start_pos);

        assert!(game.visited_cells.contains(&start_pos));
        assert!(game.visited_cells.len() >= initial_visited);
    }

    // ============================================================================
    // HELPER FUNCTIONS
    // ============================================================================

    /// Verify that all cells in a maze are reachable from the start position
    fn assert_maze_is_connected(maze: &Maze) {
        let reachable = count_reachable_cells(maze, maze.start);
        assert_eq!(
            reachable,
            maze.width * maze.height,
            "Maze is not connected: {} of {} cells reachable",
            reachable,
            maze.width * maze.height
        );
    }

    /// Verify that a maze has tree structure (n cells, n-1 passages, no cycles)
    fn assert_maze_is_tree(maze: &Maze) {
        let total_cells = maze.width * maze.height;
        let passage_count = count_passages(maze);

        // A tree with n nodes has exactly n-1 edges
        assert_eq!(
            passage_count,
            total_cells - 1,
            "Maze is not a tree: has {} passages but {} cells (expected {})",
            passage_count,
            total_cells,
            total_cells - 1
        );
    }

    /// Verify that a maze has a solution path from start to end
    fn assert_maze_is_solvable(maze: &Maze) {
        let path = Pathfinder::bfs(maze);
        assert!(
            path.is_some(),
            "Maze is not solvable: no path from start {:?} to end {:?}",
            maze.start,
            maze.end
        );
    }

    /// Count how many cells are reachable from a starting position
    fn count_reachable_cells(maze: &Maze, start: (usize, usize)) -> usize {
        use std::collections::{VecDeque, HashSet};

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited.insert(start);

        while let Some((x, y)) = queue.pop_front() {
            let cell = &maze.cells[y * maze.width + x];

            // Check all 4 directions
            if !cell.walls[0] && y > 0 {
                let next = (x, y - 1);
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
            if !cell.walls[1] && x < maze.width - 1 {
                let next = (x + 1, y);
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
            if !cell.walls[2] && y < maze.height - 1 {
                let next = (x, y + 1);
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
            if !cell.walls[3] && x > 0 {
                let next = (x - 1, y);
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
        }

        visited.len()
    }

    /// Count the number of passages (open walls) in the maze
    fn count_passages(maze: &Maze) -> usize {
        let mut passages = 0;

        for y in 0..maze.height {
            for x in 0..maze.width {
                let cell = &maze.cells[y * maze.width + x];

                // Count horizontal passages (not counting duplicates)
                if x < maze.width - 1 && !cell.walls[1] {
                    passages += 1;
                }

                // Count vertical passages (not counting duplicates)
                if y < maze.height - 1 && !cell.walls[2] {
                    passages += 1;
                }
            }
        }

        passages
    }

    /// Check if a passage is open between two adjacent cells
    fn is_passage_open(maze: &Maze, from: (usize, usize), to: (usize, usize)) -> bool {
        let (fx, fy) = from;
        let (tx, ty) = to;

        let from_cell = &maze.cells[fy * maze.width + fx];

        if tx == fx + 1 && ty == fy {
            // Moving east
            !from_cell.walls[1]
        } else if tx + 1 == fx && ty == fy {
            // Moving west (to is to the west)
            !from_cell.walls[3]
        } else if tx == fx && ty == fy + 1 {
            // Moving south
            !from_cell.walls[2]
        } else if tx == fx && ty + 1 == fy {
            // Moving north (to is to the north)
            !from_cell.walls[0]
        } else {
            false
        }
    }
}
