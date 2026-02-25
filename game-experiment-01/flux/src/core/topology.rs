use super::grid::Position;
use std::collections::HashMap;

/// Defines neighbor relationships for cells in a grid
#[derive(Debug, Clone)]
pub enum Topology {
    /// Standard 4-connected grid (up, down, left, right)
    Rectangular,
    /// 8-connected grid (includes diagonals)
    RectangularWithDiagonals,
    /// Custom neighbor map
    Custom(HashMap<Position, Vec<Position>>),
}

impl Topology {
    /// Get neighbors for a position in a grid of given dimensions
    pub fn neighbors(&self, pos: Position, width: usize, height: usize) -> Vec<Position> {
        match self {
            Topology::Rectangular => Self::rectangular_neighbors(pos, width, height),
            Topology::RectangularWithDiagonals => {
                Self::rectangular_with_diagonals_neighbors(pos, width, height)
            }
            Topology::Custom(map) => map.get(&pos).cloned().unwrap_or_default(),
        }
    }

    fn rectangular_neighbors(pos: Position, width: usize, height: usize) -> Vec<Position> {
        let mut neighbors = Vec::new();

        // Up
        if pos.row > 0 {
            neighbors.push(Position::new(pos.row - 1, pos.col));
        }
        // Down
        if pos.row < height - 1 {
            neighbors.push(Position::new(pos.row + 1, pos.col));
        }
        // Left
        if pos.col > 0 {
            neighbors.push(Position::new(pos.row, pos.col - 1));
        }
        // Right
        if pos.col < width - 1 {
            neighbors.push(Position::new(pos.row, pos.col + 1));
        }

        neighbors
    }

    fn rectangular_with_diagonals_neighbors(
        pos: Position,
        width: usize,
        height: usize,
    ) -> Vec<Position> {
        let mut neighbors = Self::rectangular_neighbors(pos, width, height);

        // Add diagonals
        // Up-left
        if pos.row > 0 && pos.col > 0 {
            neighbors.push(Position::new(pos.row - 1, pos.col - 1));
        }
        // Up-right
        if pos.row > 0 && pos.col < width - 1 {
            neighbors.push(Position::new(pos.row - 1, pos.col + 1));
        }
        // Down-left
        if pos.row < height - 1 && pos.col > 0 {
            neighbors.push(Position::new(pos.row + 1, pos.col - 1));
        }
        // Down-right
        if pos.row < height - 1 && pos.col < width - 1 {
            neighbors.push(Position::new(pos.row + 1, pos.col + 1));
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangular_center() {
        let topology = Topology::Rectangular;
        let neighbors = topology.neighbors(Position::new(1, 1), 3, 3);
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&Position::new(0, 1))); // up
        assert!(neighbors.contains(&Position::new(2, 1))); // down
        assert!(neighbors.contains(&Position::new(1, 0))); // left
        assert!(neighbors.contains(&Position::new(1, 2))); // right
    }

    #[test]
    fn test_rectangular_corner() {
        let topology = Topology::Rectangular;
        let neighbors = topology.neighbors(Position::new(0, 0), 3, 3);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position::new(1, 0))); // down
        assert!(neighbors.contains(&Position::new(0, 1))); // right
    }

    #[test]
    fn test_rectangular_edge() {
        let topology = Topology::Rectangular;
        let neighbors = topology.neighbors(Position::new(0, 1), 3, 3);
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&Position::new(1, 1))); // down
        assert!(neighbors.contains(&Position::new(0, 0))); // left
        assert!(neighbors.contains(&Position::new(0, 2))); // right
    }

    #[test]
    fn test_rectangular_with_diagonals_center() {
        let topology = Topology::RectangularWithDiagonals;
        let neighbors = topology.neighbors(Position::new(1, 1), 3, 3);
        assert_eq!(neighbors.len(), 8); // 4 orthogonal + 4 diagonal
    }

    #[test]
    fn test_rectangular_with_diagonals_corner() {
        let topology = Topology::RectangularWithDiagonals;
        let neighbors = topology.neighbors(Position::new(0, 0), 3, 3);
        assert_eq!(neighbors.len(), 3); // down, right, down-right
        assert!(neighbors.contains(&Position::new(1, 0))); // down
        assert!(neighbors.contains(&Position::new(0, 1))); // right
        assert!(neighbors.contains(&Position::new(1, 1))); // down-right
    }

    #[test]
    fn test_custom_topology() {
        let mut map = HashMap::new();
        map.insert(Position::new(0, 0), vec![Position::new(2, 2)]);
        map.insert(
            Position::new(1, 1),
            vec![Position::new(0, 0), Position::new(2, 2)],
        );

        let topology = Topology::Custom(map);
        let neighbors = topology.neighbors(Position::new(0, 0), 3, 3);
        assert_eq!(neighbors.len(), 1);
        assert!(neighbors.contains(&Position::new(2, 2)));

        let neighbors = topology.neighbors(Position::new(1, 1), 3, 3);
        assert_eq!(neighbors.len(), 2);
    }
}
