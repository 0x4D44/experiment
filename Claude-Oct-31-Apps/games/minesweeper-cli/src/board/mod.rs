//! Board module with mine placement and cell management

use crate::cell::{Cell, CellData};

pub mod generator;
pub mod reveal;

/// Represents the game board with cells and mine layout
#[derive(Clone, Debug)]
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellData>>,
    mines_count: usize,
}

impl Board {
    /// Create a new board with mines placed randomly
    pub fn new(width: usize, height: usize, mines_count: usize) -> Self {
        let mut cells = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(CellData::new(false));
            }
            cells.push(row);
        }

        let mut board = Board {
            width,
            height,
            cells,
            mines_count,
        };

        // Place mines randomly
        generator::place_mines(&mut board, mines_count);

        board
    }

    /// Create a board with a safe zone around first click
    pub fn new_with_safe_zone(
        width: usize,
        height: usize,
        mines_count: usize,
        safe_x: usize,
        safe_y: usize,
    ) -> Self {
        let mut cells = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(CellData::new(false));
            }
            cells.push(row);
        }

        let mut board = Board {
            width,
            height,
            cells,
            mines_count,
        };

        generator::place_mines_with_safe_zone(&mut board, mines_count, safe_x, safe_y);

        // Calculate adjacent mine counts
        board.calculate_adjacent_counts();

        board
    }

    /// Get board width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get board height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get total number of cells
    pub fn total_cells(&self) -> usize {
        self.width * self.height
    }

    /// Get number of mines
    pub fn mines_count(&self) -> usize {
        self.mines_count
    }

    /// Check if cell at (x, y) contains a mine
    pub fn is_mine(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.cells[y][x].is_mine
    }

    /// Get cell state at (x, y)
    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x].cell
    }

    /// Get mutable cell reference
    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x].cell
    }

    /// Reveal a cell and return whether it's a mine
    pub fn reveal_cell(&mut self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let is_mine = self.cells[y][x].is_mine;
        let already_revealed = !matches!(self.cells[y][x].cell, Cell::Unrevealed);

        if is_mine {
            self.cells[y][x].cell = Cell::Revealed(9); // Use 9 to represent mine in revealed state
            return true;
        }

        if already_revealed {
            return false; // Already revealed
        }

        let count = self.count_adjacent_mines(x, y);
        self.cells[y][x].cell = Cell::Revealed(count);
        false
    }

    /// Set cell display to a number (adjacent mine count)
    pub fn set_cell_revealed(&mut self, x: usize, y: usize, count: u8) {
        if x < self.width && y < self.height {
            self.cells[y][x].cell = Cell::Revealed(count);
        }
    }

    /// Count adjacent mines for a cell
    pub fn count_adjacent_mines(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;

                if nx < self.width && ny < self.height && self.is_mine(nx, ny) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Calculate and cache adjacent mine counts for all cells
    pub fn calculate_adjacent_counts(&mut self) {
        let width = self.width;
        let height = self.height;

        for y in 0..height {
            for x in 0..width {
                if !self.cells[y][x].is_mine {
                    let count = self.count_adjacent_mines(x, y);
                    self.cells[y][x].cell = Cell::Revealed(count);
                }
            }
        }
    }

    /// Flag a cell
    pub fn flag_cell(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            if matches!(self.cells[y][x].cell, Cell::Unrevealed) {
                self.cells[y][x].cell = Cell::Flagged;
            }
        }
    }

    /// Unflag a cell
    pub fn unflag_cell(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            if matches!(self.cells[y][x].cell, Cell::Flagged | Cell::QuestionMarked) {
                self.cells[y][x].cell = Cell::Unrevealed;
            }
        }
    }

    /// Toggle flag on a cell (cycle through states)
    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            return;
        }

        match &self.cells[y][x].cell {
            Cell::Unrevealed => {
                self.cells[y][x].cell = Cell::Flagged;
            }
            Cell::Flagged => {
                self.cells[y][x].cell = Cell::QuestionMarked;
            }
            Cell::QuestionMarked => {
                self.cells[y][x].cell = Cell::Unrevealed;
            }
            _ => {} // Can't flag revealed cells
        }
    }

    /// Get all unrevealed cells
    pub fn get_unrevealed_cells(&self) -> Vec<(usize, usize)> {
        let mut unrevealed = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.cells[y][x].cell, Cell::Unrevealed) {
                    unrevealed.push((x, y));
                }
            }
        }

        unrevealed
    }

    /// Count flagged cells
    pub fn count_flagged(&self) -> usize {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.cells[y][x].cell, Cell::Flagged) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Check if all non-mine cells are revealed
    pub fn is_complete(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.cells[y][x].is_mine {
                    if matches!(self.cells[y][x].cell, Cell::Unrevealed) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Reveal all mines (called on game loss)
    pub fn reveal_all_mines(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x].is_mine {
                    self.cells[y][x].cell = Cell::Revealed(9);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_dimensions() {
        let board = Board::new(15, 15, 30);
        assert_eq!(board.width(), 15);
        assert_eq!(board.height(), 15);
        assert_eq!(board.total_cells(), 225);
    }

    #[test]
    fn test_board_mine_count() {
        let board = Board::new(10, 10, 20);
        assert_eq!(board.mines_count(), 20);
    }

    #[test]
    fn test_mine_placement() {
        let board = Board::new(10, 10, 10);
        let mut count = 0;

        for y in 0..10 {
            for x in 0..10 {
                if board.is_mine(x, y) {
                    count += 1;
                }
            }
        }

        assert_eq!(count, 10);
    }

    #[test]
    fn test_adjacent_mine_counting() {
        let board = Board::new(5, 5, 2);

        for y in 0..5 {
            for x in 0..5 {
                let count = board.count_adjacent_mines(x, y);
                assert!(count <= 8);
            }
        }
    }

    #[test]
    fn test_flag_toggle() {
        let mut board = Board::new(5, 5, 1);

        // Find a non-mine cell that's unrevealed
        for y in 0..5 {
            for x in 0..5 {
                if !board.is_mine(x, y) {
                    board.toggle_flag(x, y);
                    assert!(matches!(board.get_cell(x, y), Cell::Flagged));

                    board.toggle_flag(x, y);
                    assert!(matches!(board.get_cell(x, y), Cell::QuestionMarked));

                    board.toggle_flag(x, y);
                    assert!(matches!(board.get_cell(x, y), Cell::Unrevealed));
                    return;
                }
            }
        }
        panic!("No non-mine cells found");
    }

    #[test]
    fn test_count_flagged() {
        let mut board = Board::new(5, 5, 1);

        // Find non-mine cells to flag
        let mut non_mine_cells = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
                if !board.is_mine(x, y) {
                    non_mine_cells.push((x, y));
                }
            }
        }

        assert_eq!(board.count_flagged(), 0);

        if non_mine_cells.len() >= 1 {
            let (x, y) = non_mine_cells[0];
            board.toggle_flag(x, y);
            assert_eq!(board.count_flagged(), 1);
        }

        if non_mine_cells.len() >= 2 {
            let (x, y) = non_mine_cells[1];
            board.toggle_flag(x, y);
            assert_eq!(board.count_flagged(), 2);
        }

        if non_mine_cells.len() >= 1 {
            let (x, y) = non_mine_cells[0];
            board.toggle_flag(x, y); // Change to question mark
            assert_eq!(board.count_flagged(), 1);
        }
    }

    #[test]
    fn test_is_complete() {
        let mut board = Board::new(3, 3, 1);

        // Reveal all non-mine cells
        for y in 0..3 {
            for x in 0..3 {
                if !board.is_mine(x, y) {
                    board.set_cell_revealed(x, y, board.count_adjacent_mines(x, y));
                }
            }
        }

        assert!(board.is_complete());
    }
}
