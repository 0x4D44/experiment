use super::cell::CellState;
use super::topology::Topology;

/// 2D position in grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    /// Convert to linear index in a grid
    pub fn to_index(&self, width: usize) -> usize {
        self.row * width + self.col
    }

    /// Create position from linear index
    pub fn from_index(index: usize, width: usize) -> Self {
        Position {
            row: index / width,
            col: index % width,
        }
    }
}

/// Grid of cells with topology
#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<CellState>,
    topology: Topology,
}

impl Grid {
    /// Create a new grid with all cells at state 0
    pub fn new(width: usize, height: usize, topology: Topology) -> Self {
        assert!(width > 0 && height > 0, "Grid dimensions must be positive");
        Grid {
            width,
            height,
            cells: vec![CellState::default(); width * height],
            topology,
        }
    }

    /// Create a grid from initial state
    pub fn from_state(
        width: usize,
        height: usize,
        initial_state: Vec<CellState>,
        topology: Topology,
    ) -> Self {
        assert_eq!(
            initial_state.len(),
            width * height,
            "Initial state length must match grid size"
        );
        Grid {
            width,
            height,
            cells: initial_state,
            topology,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn topology(&self) -> &Topology {
        &self.topology
    }

    /// Get cell state at position
    pub fn get(&self, pos: Position) -> Option<CellState> {
        if self.is_valid_position(pos) {
            Some(self.cells[pos.to_index(self.width)])
        } else {
            None
        }
    }

    /// Set cell state at position
    pub fn set(&mut self, pos: Position, state: CellState) {
        if self.is_valid_position(pos) {
            self.cells[pos.to_index(self.width)] = state;
        }
    }

    /// Core game mechanic: Click a cell
    /// Increments the cell and all its neighbors by 1 (mod 4)
    pub fn click(&mut self, pos: Position) {
        if !self.is_valid_position(pos) {
            return;
        }

        // Increment the clicked cell
        let index = pos.to_index(self.width);
        self.cells[index].increment();

        // Increment all neighbors
        for neighbor in self.neighbors(pos) {
            let neighbor_index = neighbor.to_index(self.width);
            self.cells[neighbor_index].increment();
        }
    }

    /// Get neighbors of a position
    pub fn neighbors(&self, pos: Position) -> Vec<Position> {
        self.topology.neighbors(pos, self.width, self.height)
    }

    /// Check if all cells are at state 0
    pub fn is_all_zeros(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_zero())
    }

    /// Check if grid matches target state
    pub fn matches_target(&self, target: &[CellState]) -> bool {
        if target.len() != self.cells.len() {
            return false;
        }
        self.cells.iter().zip(target.iter()).all(|(a, b)| a == b)
    }

    /// Get all cell states as a slice
    pub fn cells(&self) -> &[CellState] {
        &self.cells
    }

    fn is_valid_position(&self, pos: Position) -> bool {
        pos.row < self.height && pos.col < self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_index() {
        assert_eq!(Position::new(0, 0).to_index(3), 0);
        assert_eq!(Position::new(0, 1).to_index(3), 1);
        assert_eq!(Position::new(1, 0).to_index(3), 3);
        assert_eq!(Position::new(1, 1).to_index(3), 4);
    }

    #[test]
    fn test_position_from_index() {
        assert_eq!(Position::from_index(0, 3), Position::new(0, 0));
        assert_eq!(Position::from_index(1, 3), Position::new(0, 1));
        assert_eq!(Position::from_index(3, 3), Position::new(1, 0));
        assert_eq!(Position::from_index(4, 3), Position::new(1, 1));
    }

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(3, 3, Topology::Rectangular);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert!(grid.is_all_zeros());
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid = Grid::new(3, 3, Topology::Rectangular);
        let pos = Position::new(1, 1);

        assert_eq!(grid.get(pos), Some(CellState::new(0)));

        grid.set(pos, CellState::new(2));
        assert_eq!(grid.get(pos), Some(CellState::new(2)));
    }

    #[test]
    fn test_click_center_cell() {
        let mut grid = Grid::new(3, 3, Topology::Rectangular);
        let center = Position::new(1, 1);

        // Click center cell
        grid.click(center);

        // Center should be 1
        assert_eq!(grid.get(center), Some(CellState::new(1)));

        // All 4 neighbors should be 1
        assert_eq!(grid.get(Position::new(0, 1)), Some(CellState::new(1))); // up
        assert_eq!(grid.get(Position::new(2, 1)), Some(CellState::new(1))); // down
        assert_eq!(grid.get(Position::new(1, 0)), Some(CellState::new(1))); // left
        assert_eq!(grid.get(Position::new(1, 2)), Some(CellState::new(1))); // right

        // Corners should still be 0
        assert_eq!(grid.get(Position::new(0, 0)), Some(CellState::new(0)));
        assert_eq!(grid.get(Position::new(0, 2)), Some(CellState::new(0)));
        assert_eq!(grid.get(Position::new(2, 0)), Some(CellState::new(0)));
        assert_eq!(grid.get(Position::new(2, 2)), Some(CellState::new(0)));
    }

    #[test]
    fn test_click_corner_cell() {
        let mut grid = Grid::new(3, 3, Topology::Rectangular);
        let corner = Position::new(0, 0);

        grid.click(corner);

        // Corner should be 1
        assert_eq!(grid.get(corner), Some(CellState::new(1)));

        // Its 2 neighbors should be 1
        assert_eq!(grid.get(Position::new(0, 1)), Some(CellState::new(1))); // right
        assert_eq!(grid.get(Position::new(1, 0)), Some(CellState::new(1))); // down
    }

    #[test]
    fn test_click_wraps_around() {
        let mut grid = Grid::new(2, 2, Topology::Rectangular);

        // Set all cells to 3
        for row in 0..2 {
            for col in 0..2 {
                grid.set(Position::new(row, col), CellState::new(3));
            }
        }

        // Click center-ish (doesn't matter in 2x2)
        grid.click(Position::new(0, 0));

        // Clicked cell and neighbors should wrap to 0
        assert_eq!(grid.get(Position::new(0, 0)), Some(CellState::new(0)));
        assert_eq!(grid.get(Position::new(0, 1)), Some(CellState::new(0)));
        assert_eq!(grid.get(Position::new(1, 0)), Some(CellState::new(0)));
    }

    #[test]
    fn test_multiple_clicks_commute() {
        let mut grid1 = Grid::new(3, 3, Topology::Rectangular);
        let mut grid2 = Grid::new(3, 3, Topology::Rectangular);

        // Click in different orders
        grid1.click(Position::new(0, 0));
        grid1.click(Position::new(1, 1));

        grid2.click(Position::new(1, 1));
        grid2.click(Position::new(0, 0));

        // Should result in same state
        assert_eq!(grid1.cells(), grid2.cells());
    }

    #[test]
    fn test_four_clicks_is_identity() {
        let mut grid = Grid::new(3, 3, Topology::Rectangular);
        let initial_state = grid.cells().to_vec();

        // Click same position 4 times
        let pos = Position::new(1, 1);
        grid.click(pos);
        grid.click(pos);
        grid.click(pos);
        grid.click(pos);

        // Should be back to initial state
        assert_eq!(grid.cells(), &initial_state[..]);
    }

    #[test]
    fn test_is_all_zeros() {
        let mut grid = Grid::new(2, 2, Topology::Rectangular);
        assert!(grid.is_all_zeros());

        grid.set(Position::new(0, 0), CellState::new(1));
        assert!(!grid.is_all_zeros());
    }

    #[test]
    fn test_matches_target() {
        let grid = Grid::new(2, 2, Topology::Rectangular);
        let target = vec![CellState::new(0); 4];
        assert!(grid.matches_target(&target));

        let wrong_target = vec![CellState::new(1); 4];
        assert!(!grid.matches_target(&wrong_target));
    }

    #[test]
    fn test_from_state() {
        let initial = vec![
            CellState::new(1),
            CellState::new(2),
            CellState::new(0),
            CellState::new(3),
        ];
        let grid = Grid::from_state(2, 2, initial.clone(), Topology::Rectangular);

        assert_eq!(grid.get(Position::new(0, 0)), Some(CellState::new(1)));
        assert_eq!(grid.get(Position::new(0, 1)), Some(CellState::new(2)));
        assert_eq!(grid.get(Position::new(1, 0)), Some(CellState::new(0)));
        assert_eq!(grid.get(Position::new(1, 1)), Some(CellState::new(3)));
    }
}
