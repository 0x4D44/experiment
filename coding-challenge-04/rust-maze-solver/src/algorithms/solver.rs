use crate::maze::Maze;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathfindingAlgorithm {
    AStar,
    BFS,
    DFS,
    Dijkstra,
}

#[derive(Debug, Clone)]
pub struct SolutionResult {
    pub path: Vec<(usize, usize)>,
    pub visited: HashSet<(usize, usize)>,
    pub path_length: usize,
    pub nodes_explored: usize,
}

pub struct MazeSolver;

impl MazeSolver {
    pub fn solve(maze: &Maze, algorithm: PathfindingAlgorithm) -> Option<SolutionResult> {
        match algorithm {
            PathfindingAlgorithm::AStar => Self::a_star(maze),
            PathfindingAlgorithm::BFS => Self::bfs(maze),
            PathfindingAlgorithm::DFS => Self::dfs(maze),
            PathfindingAlgorithm::Dijkstra => Self::dijkstra(maze),
        }
    }

    /// A* Algorithm - Optimal pathfinding with heuristic
    fn a_star(maze: &Maze) -> Option<SolutionResult> {
        let start = maze.start;
        let goal = maze.end;
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut g_score: HashMap<(usize, usize), f64> = HashMap::new();
        let mut visited = HashSet::new();

        g_score.insert(start, 0.0);
        open_set.push(AStarNode {
            position: start,
            f_score: Self::heuristic(start, goal),
        });

        while let Some(AStarNode { position: current, .. }) = open_set.pop() {
            visited.insert(current);

            if current == goal {
                let path = Self::reconstruct_path(&came_from, current);
                return Some(SolutionResult {
                    path_length: path.len(),
                    nodes_explored: visited.len(),
                    path,
                    visited,
                });
            }

            for neighbor in maze.neighbors(current.0, current.1) {
                if let Some(cell) = maze.get(neighbor.0, neighbor.1) {
                    if !cell.is_walkable() && neighbor != goal {
                        continue;
                    }

                    let tentative_g_score = g_score.get(&current).unwrap_or(&f64::MAX) + 1.0;

                    if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::MAX) {
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);
                        let f_score = tentative_g_score + Self::heuristic(neighbor, goal);
                        open_set.push(AStarNode {
                            position: neighbor,
                            f_score,
                        });
                    }
                }
            }
        }

        None
    }

    /// BFS - Guarantees shortest path, explores layer by layer
    fn bfs(maze: &Maze) -> Option<SolutionResult> {
        let start = maze.start;
        let goal = maze.end;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == goal {
                let path = Self::reconstruct_path(&came_from, current);
                return Some(SolutionResult {
                    path_length: path.len(),
                    nodes_explored: visited.len(),
                    path,
                    visited,
                });
            }

            for neighbor in maze.neighbors(current.0, current.1) {
                if let Some(cell) = maze.get(neighbor.0, neighbor.1) {
                    if (!cell.is_walkable() && neighbor != goal) || visited.contains(&neighbor) {
                        continue;
                    }

                    visited.insert(neighbor);
                    came_from.insert(neighbor, current);
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    /// DFS - Depth-first search, may not find shortest path
    fn dfs(maze: &Maze) -> Option<SolutionResult> {
        let start = maze.start;
        let goal = maze.end;
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        stack.push(start);

        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            if current == goal {
                let path = Self::reconstruct_path(&came_from, current);
                return Some(SolutionResult {
                    path_length: path.len(),
                    nodes_explored: visited.len(),
                    path,
                    visited,
                });
            }

            for neighbor in maze.neighbors(current.0, current.1) {
                if let Some(cell) = maze.get(neighbor.0, neighbor.1) {
                    if (!cell.is_walkable() && neighbor != goal) || visited.contains(&neighbor) {
                        continue;
                    }

                    if let std::collections::hash_map::Entry::Vacant(e) = came_from.entry(neighbor) {
                        e.insert(current);
                        stack.push(neighbor);
                    }
                }
            }
        }

        None
    }

    /// Dijkstra's Algorithm - Similar to A* but without heuristic
    fn dijkstra(maze: &Maze) -> Option<SolutionResult> {
        let start = maze.start;
        let goal = maze.end;
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut distances: HashMap<(usize, usize), f64> = HashMap::new();
        let mut visited = HashSet::new();

        distances.insert(start, 0.0);
        open_set.push(DijkstraNode {
            position: start,
            distance: 0.0,
        });

        while let Some(DijkstraNode { position: current, .. }) = open_set.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            if current == goal {
                let path = Self::reconstruct_path(&came_from, current);
                return Some(SolutionResult {
                    path_length: path.len(),
                    nodes_explored: visited.len(),
                    path,
                    visited,
                });
            }

            for neighbor in maze.neighbors(current.0, current.1) {
                if let Some(cell) = maze.get(neighbor.0, neighbor.1) {
                    if !cell.is_walkable() && neighbor != goal {
                        continue;
                    }

                    let tentative_distance = distances.get(&current).unwrap_or(&f64::MAX) + 1.0;

                    if tentative_distance < *distances.get(&neighbor).unwrap_or(&f64::MAX) {
                        came_from.insert(neighbor, current);
                        distances.insert(neighbor, tentative_distance);
                        open_set.push(DijkstraNode {
                            position: neighbor,
                            distance: tentative_distance,
                        });
                    }
                }
            }
        }

        None
    }

    fn heuristic(from: (usize, usize), to: (usize, usize)) -> f64 {
        // Manhattan distance
        let dx = (from.0 as i32 - to.0 as i32).abs() as f64;
        let dy = (from.1 as i32 - to.1 as i32).abs() as f64;
        dx + dy
    }

    fn reconstruct_path(
        came_from: &HashMap<(usize, usize), (usize, usize)>,
        mut current: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut path = vec![current];

        while let Some(&prev) = came_from.get(&current) {
            path.push(prev);
            current = prev;
        }

        path.reverse();
        path
    }
}

#[derive(Clone, Copy)]
struct AStarNode {
    position: (usize, usize),
    f_score: f64,
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl Eq for AStarNode {}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

#[derive(Clone, Copy)]
struct DijkstraNode {
    position: (usize, usize),
    distance: f64,
}

impl PartialEq for DijkstraNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for DijkstraNode {}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.distance.partial_cmp(&self.distance).unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::MazeGenerator;
    use crate::algorithms::GeneratorAlgorithm;

    #[test]
    fn test_a_star_finds_solution() {
        let maze = MazeGenerator::generate(15, 15, GeneratorAlgorithm::RecursiveBacktracker);
        let result = MazeSolver::solve(&maze, PathfindingAlgorithm::AStar);

        assert!(result.is_some());
        let solution = result.unwrap();
        assert!(solution.path_length > 0);
        assert_eq!(solution.path.first().unwrap(), &maze.start);
        assert_eq!(solution.path.last().unwrap(), &maze.end);
    }

    #[test]
    fn test_bfs_finds_solution() {
        let maze = MazeGenerator::generate(15, 15, GeneratorAlgorithm::Prims);
        let result = MazeSolver::solve(&maze, PathfindingAlgorithm::BFS);

        assert!(result.is_some());
        let solution = result.unwrap();
        assert!(solution.path_length > 0);
    }

    #[test]
    fn test_dfs_finds_solution() {
        let maze = MazeGenerator::generate(15, 15, GeneratorAlgorithm::Kruskals);
        let result = MazeSolver::solve(&maze, PathfindingAlgorithm::DFS);

        assert!(result.is_some());
        let solution = result.unwrap();
        assert!(solution.path_length > 0);
    }

    #[test]
    fn test_dijkstra_finds_solution() {
        let maze = MazeGenerator::generate(15, 15, GeneratorAlgorithm::RecursiveBacktracker);
        let result = MazeSolver::solve(&maze, PathfindingAlgorithm::Dijkstra);

        assert!(result.is_some());
        let solution = result.unwrap();
        assert!(solution.path_length > 0);
    }

    #[test]
    fn test_heuristic_calculation() {
        assert_eq!(MazeSolver::heuristic((0, 0), (3, 4)), 7.0);
        assert_eq!(MazeSolver::heuristic((5, 5), (5, 5)), 0.0);
    }

    #[test]
    fn test_solution_statistics() {
        let maze = MazeGenerator::generate(20, 20, GeneratorAlgorithm::RecursiveBacktracker);
        let result = MazeSolver::solve(&maze, PathfindingAlgorithm::AStar).unwrap();

        assert!(result.nodes_explored > 0);
        assert!(result.path_length > 0);
        assert!(result.path_length <= result.nodes_explored);
    }
}
