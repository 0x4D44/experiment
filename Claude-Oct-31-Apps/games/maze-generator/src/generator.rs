use crate::maze::{Maze, Algorithm, Direction};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashSet;

/// Trait for maze generation algorithms
pub trait MazeGenerator {
    fn generate(&mut self) -> Maze;
}

/// Recursive Backtracker algorithm - DFS based
pub struct RecursiveBacktracker {
    width: usize,
    height: usize,
    seed: u64,
}

impl RecursiveBacktracker {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        RecursiveBacktracker { width, height, seed }
    }
}

impl MazeGenerator for RecursiveBacktracker {
    fn generate(&mut self) -> Maze {
        let mut maze = Maze::new(self.width, self.height, self.seed, Algorithm::RecursiveBacktracker);
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut visited = HashSet::new();

        fn carve(
            maze: &mut Maze,
            x: usize,
            y: usize,
            visited: &mut HashSet<(usize, usize)>,
            rng: &mut StdRng,
        ) {
            visited.insert((x, y));
            let mut directions = vec![Direction::North, Direction::East, Direction::South, Direction::West];

            // Shuffle directions
            for i in 0..directions.len() {
                let j = rng.gen_range(0..directions.len());
                directions.swap(i, j);
            }

            for dir in directions {
                let (nx, ny) = match dir {
                    Direction::North => (x, y.saturating_sub(1)),
                    Direction::East => (x + 1, y),
                    Direction::South => (x, y + 1),
                    Direction::West => (x.saturating_sub(1), y),
                };

                if nx < maze.width && ny < maze.height && !visited.contains(&(nx, ny)) {
                    maze.carve_passage((x, y), (nx, ny));
                    carve(maze, nx, ny, visited, rng);
                }
            }
        }

        carve(&mut maze, 0, 0, &mut visited, &mut rng);
        maze
    }
}

/// Kruskal's algorithm - MST based
pub struct Kruskal {
    width: usize,
    height: usize,
    seed: u64,
}

impl Kruskal {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        Kruskal { width, height, seed }
    }
}

impl MazeGenerator for Kruskal {
    fn generate(&mut self) -> Maze {
        let mut maze = Maze::new(self.width, self.height, self.seed, Algorithm::Kruskal);
        let mut rng = StdRng::seed_from_u64(self.seed);

        // Create union-find structure
        let total_cells = self.width * self.height;
        let mut parent = (0..total_cells).collect::<Vec<_>>();

        fn find(parent: &mut [usize], x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut [usize], x: usize, y: usize) -> bool {
            let px = find(parent, x);
            let py = find(parent, y);
            if px != py {
                parent[px] = py;
                true
            } else {
                false
            }
        }

        // Create list of all walls
        let mut walls = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if x < self.width - 1 {
                    walls.push((x, y, Direction::East));
                }
                if y < self.height - 1 {
                    walls.push((x, y, Direction::South));
                }
            }
        }

        // Shuffle walls
        for i in 0..walls.len() {
            let j = rng.gen_range(0..walls.len());
            walls.swap(i, j);
        }

        // Remove walls to create spanning tree
        for (x, y, dir) in walls {
            let (nx, ny) = match dir {
                Direction::East => (x + 1, y),
                Direction::South => (x, y + 1),
                _ => continue,
            };

            let idx1 = y * self.width + x;
            let idx2 = ny * self.width + nx;

            if union(&mut parent, idx1, idx2) {
                maze.carve_passage((x, y), (nx, ny));
            }
        }

        maze
    }
}

/// Prim's algorithm - frontier expansion
pub struct Prim {
    width: usize,
    height: usize,
    seed: u64,
}

impl Prim {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        Prim { width, height, seed }
    }
}

impl MazeGenerator for Prim {
    fn generate(&mut self) -> Maze {
        let mut maze = Maze::new(self.width, self.height, self.seed, Algorithm::Prim);
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut visited = HashSet::new();
        let mut frontier = Vec::new();

        visited.insert((0, 0));

        // Add initial neighbors to frontier
        if self.width > 1 {
            frontier.push((0, 0, Direction::East));
        }
        if self.height > 1 {
            frontier.push((0, 0, Direction::South));
        }

        while !frontier.is_empty() {
            let idx = rng.gen_range(0..frontier.len());
            let (x, y, dir) = frontier.swap_remove(idx);

            let (nx, ny) = match dir {
                Direction::East => (x + 1, y),
                Direction::South => (x, y + 1),
                Direction::North => (x, y - 1),
                Direction::West => (x - 1, y),
            };

            if nx >= self.width || ny >= self.height || visited.contains(&(nx, ny)) {
                continue;
            }

            visited.insert((nx, ny));
            maze.carve_passage((x, y), (nx, ny));

            // Add new frontier walls
            let neighbors = maze.get_all_neighbors(nx, ny);
            for (nnx, nny) in neighbors {
                if !visited.contains(&(nnx, nny)) {
                    let dir = if nnx == nx + 1 {
                        Direction::East
                    } else if nnx + 1 == nx {
                        Direction::West
                    } else if nny == ny + 1 {
                        Direction::South
                    } else {
                        Direction::North
                    };
                    frontier.push((nx, ny, dir));
                }
            }
        }

        maze
    }
}

/// Binary Tree algorithm - fast, directional bias
pub struct BinaryTree {
    width: usize,
    height: usize,
    seed: u64,
}

impl BinaryTree {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        BinaryTree { width, height, seed }
    }
}

impl MazeGenerator for BinaryTree {
    fn generate(&mut self) -> Maze {
        let mut maze = Maze::new(self.width, self.height, self.seed, Algorithm::BinaryTree);
        let mut rng = StdRng::seed_from_u64(self.seed);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut neighbors = Vec::new();

                if x < self.width - 1 {
                    neighbors.push((x + 1, y));
                }
                if y < self.height - 1 {
                    neighbors.push((x, y + 1));
                }

                if !neighbors.is_empty() {
                    let neighbor = neighbors[rng.gen_range(0..neighbors.len())];
                    maze.carve_passage((x, y), neighbor);
                }
            }
        }

        maze
    }
}

/// Aldous-Broder algorithm - unbiased uniform generation
pub struct AldousBroder {
    width: usize,
    height: usize,
    seed: u64,
}

impl AldousBroder {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        AldousBroder { width, height, seed }
    }
}

impl MazeGenerator for AldousBroder {
    fn generate(&mut self) -> Maze {
        let mut maze = Maze::new(self.width, self.height, self.seed, Algorithm::AldousBroder);
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut visited = HashSet::new();
        let mut current = (0, 0);
        visited.insert(current);

        while visited.len() < self.width * self.height {
            let neighbors = maze.get_all_neighbors(current.0, current.1);
            let next = neighbors[rng.gen_range(0..neighbors.len())];

            if !visited.contains(&next) {
                visited.insert(next);
                maze.carve_passage(current, next);
            }

            current = next;
        }

        maze
    }
}

/// Wilson's algorithm - loop-erased random walk
pub struct Wilson {
    width: usize,
    height: usize,
    seed: u64,
}

impl Wilson {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        Wilson { width, height, seed }
    }
}

impl MazeGenerator for Wilson {
    fn generate(&mut self) -> Maze {
        let mut maze = Maze::new(self.width, self.height, self.seed, Algorithm::Wilson);
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut visited = HashSet::new();
        visited.insert((0, 0));

        for start_y in 0..self.height {
            for start_x in 0..self.width {
                if visited.contains(&(start_x, start_y)) {
                    continue;
                }

                let mut path = Vec::new();
                let mut current = (start_x, start_y);
                path.push(current);

                while !visited.contains(&current) {
                    let neighbors = maze.get_all_neighbors(current.0, current.1);
                    let next = neighbors[rng.gen_range(0..neighbors.len())];

                    if let Some(pos) = path.iter().position(|&x| x == next) {
                        // Loop detected, erase loop
                        path.truncate(pos + 1);
                    } else {
                        path.push(next);
                    }

                    current = next;
                }

                // Add path to maze
                for i in 0..path.len() - 1 {
                    visited.insert(path[i]);
                    maze.carve_passage(path[i], path[i + 1]);
                }
                visited.insert(current);
            }
        }

        maze
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_backtracker_generates() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);
        assert_eq!(maze.cells.len(), 100);
    }

    #[test]
    fn test_kruskal_generates() {
        let mut gen = Kruskal::new(10, 10, 42);
        let maze = gen.generate();
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);
    }

    #[test]
    fn test_prim_generates() {
        let mut gen = Prim::new(10, 10, 42);
        let maze = gen.generate();
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);
    }

    #[test]
    fn test_binary_tree_generates() {
        let mut gen = BinaryTree::new(10, 10, 42);
        let maze = gen.generate();
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);
    }

    #[test]
    fn test_aldous_broder_generates() {
        let mut gen = AldousBroder::new(10, 10, 42);
        let maze = gen.generate();
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);
    }

    #[test]
    fn test_wilson_generates() {
        let mut gen = Wilson::new(10, 10, 42);
        let maze = gen.generate();
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);
    }
}
