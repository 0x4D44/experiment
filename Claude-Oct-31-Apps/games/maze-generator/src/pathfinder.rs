use crate::maze::Maze;
use std::collections::{VecDeque, HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathfindingAlgorithm {
    BreadthFirstSearch,
    DepthFirstSearch,
    AStar,
}

/// Pathfinding algorithms for maze solving
pub struct Pathfinder;

impl Pathfinder {
    /// Breadth-First Search - finds shortest path
    pub fn bfs(maze: &Maze) -> Option<Vec<(usize, usize)>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = std::collections::HashMap::new();

        queue.push_back(maze.start);
        visited.insert(maze.start);

        while let Some(current) = queue.pop_front() {
            if current == maze.end {
                return Some(reconstruct_path(&parent, maze.start, maze.end));
            }

            let neighbors = maze.get_valid_neighbors(current.0, current.1);
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    parent.insert(neighbor, current);
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    /// Depth-First Search - finds a path (not necessarily shortest)
    pub fn dfs(maze: &Maze) -> Option<Vec<(usize, usize)>> {
        let mut stack = vec![maze.start];
        let mut visited = HashSet::new();
        let mut parent = std::collections::HashMap::new();

        visited.insert(maze.start);

        while let Some(current) = stack.pop() {
            if current == maze.end {
                return Some(reconstruct_path(&parent, maze.start, maze.end));
            }

            let neighbors = maze.get_valid_neighbors(current.0, current.1);
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    parent.insert(neighbor, current);
                    stack.push(neighbor);
                }
            }
        }

        None
    }

    /// A* Search - optimal pathfinding with heuristic
    pub fn astar(maze: &Maze) -> Option<Vec<(usize, usize)>> {
        let mut open_set = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut parent = std::collections::HashMap::new();
        let mut g_score = std::collections::HashMap::new();

        g_score.insert(maze.start, 0);
        let f_score = heuristic(maze.start, maze.end, maze);
        open_set.push(AStarNode {
            pos: maze.start,
            f_score,
        });

        while let Some(current_node) = open_set.pop() {
            let current = current_node.pos;

            if current == maze.end {
                return Some(reconstruct_path(&parent, maze.start, maze.end));
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            let neighbors = maze.get_valid_neighbors(current.0, current.1);
            for neighbor in neighbors {
                if visited.contains(&neighbor) {
                    continue;
                }

                let tentative_g = g_score.get(&current).copied().unwrap_or(usize::MAX) + 1;
                let neighbor_g = g_score.get(&neighbor).copied().unwrap_or(usize::MAX);

                if tentative_g < neighbor_g {
                    parent.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g);

                    let f = tentative_g + heuristic(neighbor, maze.end, maze);
                    open_set.push(AStarNode {
                        pos: neighbor,
                        f_score: f,
                    });
                }
            }
        }

        None
    }

    /// Find the next step toward the goal using A*
    pub fn next_step(maze: &Maze, current: (usize, usize)) -> Option<(usize, usize)> {
        let path = Pathfinder::astar(maze)?;

        // Find current position in path
        for i in 0..path.len() - 1 {
            if path[i] == current {
                return Some(path[i + 1]);
            }
        }

        None
    }

    /// Get hint path (next 3-5 steps)
    pub fn hint_path(maze: &Maze, current: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let path = Pathfinder::astar(maze)?;

        // Find current position in path
        for i in 0..path.len() {
            if path[i] == current {
                let end = std::cmp::min(i + 5, path.len());
                return Some(path[i..end].to_vec());
            }
        }

        None
    }
}

// ============================================================================
// HELPER STRUCTURES AND FUNCTIONS
// ============================================================================

#[derive(Debug, Clone, Eq, PartialEq)]
struct AStarNode {
    pos: (usize, usize),
    f_score: usize,
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Manhattan distance heuristic
fn heuristic(from: (usize, usize), to: (usize, usize), _maze: &Maze) -> usize {
    let dx = (from.0 as i32 - to.0 as i32).abs() as usize;
    let dy = (from.1 as i32 - to.1 as i32).abs() as usize;
    dx + dy
}

/// Reconstruct path from parent map
fn reconstruct_path(
    parent: &std::collections::HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path = vec![end];
    let mut current = end;

    while current != start {
        if let Some(&prev) = parent.get(&current) {
            path.push(prev);
            current = prev;
        } else {
            break;
        }
    }

    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::RecursiveBacktracker;
    use crate::MazeGenerator;

    #[test]
    fn test_bfs_finds_path() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();

        let path = Pathfinder::bfs(&maze);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path[0], maze.start);
        assert_eq!(path[path.len() - 1], maze.end);
    }

    #[test]
    fn test_dfs_finds_path() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();

        let path = Pathfinder::dfs(&maze);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path[0], maze.start);
        assert_eq!(path[path.len() - 1], maze.end);
    }

    #[test]
    fn test_astar_finds_path() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();

        let path = Pathfinder::astar(&maze);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path[0], maze.start);
        assert_eq!(path[path.len() - 1], maze.end);
    }

    #[test]
    fn test_bfs_optimal() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();

        let bfs_path = Pathfinder::bfs(&maze).unwrap();
        let dfs_path = Pathfinder::dfs(&maze).unwrap();

        assert!(bfs_path.len() <= dfs_path.len());
    }

    #[test]
    fn test_next_step() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();

        let next = Pathfinder::next_step(&maze, maze.start);
        assert!(next.is_some());

        let next = next.unwrap();
        // Should be adjacent to start
        let dx = (maze.start.0 as i32 - next.0 as i32).abs();
        let dy = (maze.start.1 as i32 - next.1 as i32).abs();
        assert_eq!(dx + dy, 1);
    }

    #[test]
    fn test_hint_path() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();

        let hint = Pathfinder::hint_path(&maze, maze.start);
        assert!(hint.is_some());

        let hint = hint.unwrap();
        assert!(hint.len() <= 5);
        assert_eq!(hint[0], maze.start);
    }
}
