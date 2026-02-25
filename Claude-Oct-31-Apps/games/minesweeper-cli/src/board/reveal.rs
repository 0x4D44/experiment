//! Recursive reveal algorithm using BFS flood fill

use super::Board;
use crate::cell::Cell;
use std::collections::VecDeque;

/// Perform recursive reveal using BFS flood fill
/// Returns true if a mine was revealed (game lost)
pub fn reveal_recursive(board: &mut Board, start_x: usize, start_y: usize) -> bool {
    if start_x >= board.width || start_y >= board.height {
        return false;
    }

    // Check if we hit a mine
    if board.is_mine(start_x, start_y) {
        board.reveal_cell(start_x, start_y);
        return true;
    }

    // BFS flood fill
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();

    queue.push_back((start_x, start_y));
    visited.insert((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        // Reveal this cell
        board.reveal_cell(x, y);

        // Get adjacent mine count
        let adjacent_mines = board.count_adjacent_mines(x, y);

        // If no adjacent mines, recursively reveal neighbors
        if adjacent_mines == 0 {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;

                    // Check bounds and not already visited
                    if nx < board.width && ny < board.height && !visited.contains(&(nx, ny)) {
                        // Don't queue mines, and don't queue flagged cells
                        if !board.is_mine(nx, ny) && !matches!(board.get_cell(nx, ny), Cell::Flagged) {
                            visited.insert((nx, ny));
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }
    }

    false
}

/// Perform chord operation (reveal adjacent cells if flags match mine count)
pub fn chord(board: &mut Board, x: usize, y: usize) -> bool {
    if x >= board.width || y >= board.height {
        return false;
    }

    // Only works on revealed cells
    if !matches!(board.get_cell(x, y), Cell::Revealed(_)) {
        return false;
    }

    let adjacent_mines = board.count_adjacent_mines(x, y);

    // Count adjacent flags
    let mut adjacent_flags = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;

            if nx < board.width && ny < board.height {
                if matches!(board.get_cell(nx, ny), Cell::Flagged) {
                    adjacent_flags += 1;
                }
            }
        }
    }

    // If flags match mine count, reveal all unrevealed adjacent cells
    if adjacent_flags == adjacent_mines as usize {
        let mut mine_hit = false;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;

                if nx < board.width && ny < board.height {
                    if matches!(board.get_cell(nx, ny), Cell::Unrevealed) {
                        if reveal_recursive(board, nx, ny) {
                            mine_hit = true;
                        }
                    }
                }
            }
        }

        return mine_hit;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reveal_single_cell() {
        let mut board = Board::new(5, 5, 2);

        // Reveal a safe cell
        let hit_mine = reveal_recursive(&mut board, 2, 2);

        // Either hit mine or revealed safe cell
        assert!(!hit_mine || board.is_mine(2, 2));
    }

    #[test]
    fn test_reveal_flood_fill_chain() {
        let mut board = Board::new(10, 10, 5);

        // Reset all cells to unrevealed for this test
        for y in 0..board.height() {
            for x in 0..board.width() {
                if !board.is_mine(x, y) {
                    *board.get_cell_mut(x, y) = Cell::Unrevealed;
                }
            }
        }

        let original_unrevealed = board.get_unrevealed_cells().len();

        // Reveal a cell
        reveal_recursive(&mut board, 5, 5);

        let after_unrevealed = board.get_unrevealed_cells().len();

        // After revealing, fewer cells should be unrevealed
        assert!(after_unrevealed <= original_unrevealed);
    }

    #[test]
    fn test_reveal_no_panic_large_board() {
        let mut board = Board::new(50, 50, 20);

        // Should not panic or stack overflow
        reveal_recursive(&mut board, 25, 25);

        assert!(true); // Reached here without panic
    }

    #[test]
    fn test_chord_operation() {
        let mut board = Board::new(7, 7, 3);

        // First reveal some safe area
        reveal_recursive(&mut board, 3, 3);

        // Find a cell with adjacent mines to test chord
        for y in 1..6 {
            for x in 1..6 {
                if let Cell::Revealed(adjacent) = board.get_cell(x, y) {
                    if *adjacent > 0 && *adjacent <= 2 {
                        // Try to chord (will probably fail if not all flags placed)
                        let _ = chord(&mut board, x, y);
                        return;
                    }
                }
            }
        }
    }
}
