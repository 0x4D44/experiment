//! Mine placement algorithm with fair distribution

use super::Board;
use rand::Rng;

/// Place mines randomly on the board
pub fn place_mines(board: &mut Board, num_mines: usize) {
    let mut rng = rand::thread_rng();
    let mut placed = 0;

    while placed < num_mines {
        let x = rng.gen_range(0..board.width);
        let y = rng.gen_range(0..board.height);

        if !board.cells[y][x].is_mine {
            board.cells[y][x].is_mine = true;
            placed += 1;
        }
    }
}

/// Place mines with a safe zone around the first click
pub fn place_mines_with_safe_zone(
    board: &mut Board,
    num_mines: usize,
    safe_x: usize,
    safe_y: usize,
) {
    let mut rng = rand::thread_rng();
    let mut placed = 0;

    while placed < num_mines {
        let x = rng.gen_range(0..board.width);
        let y = rng.gen_range(0..board.height);

        // Check if in safe zone (3x3 area around first click)
        let in_safe_zone = (x as i32 - safe_x as i32).abs() <= 1
            && (y as i32 - safe_y as i32).abs() <= 1;

        if !board.cells[y][x].is_mine && !in_safe_zone {
            board.cells[y][x].is_mine = true;
            placed += 1;
        }
    }

    // Calculate adjacent mine counts
    board.calculate_adjacent_counts();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_mines_count() {
        let mut board = Board::new(10, 10, 0);
        place_mines(&mut board, 15);

        let mut count = 0;
        for y in 0..10 {
            for x in 0..10 {
                if board.is_mine(x, y) {
                    count += 1;
                }
            }
        }

        assert_eq!(count, 15);
    }

    #[test]
    fn test_place_mines_with_safe_zone() {
        let mut board = Board::new(10, 10, 0);
        place_mines_with_safe_zone(&mut board, 20, 5, 5);

        // Verify no mines in safe zone
        for y in 4..=6 {
            for x in 4..=6 {
                if x < board.width && y < board.height {
                    assert!(!board.is_mine(x, y), "Mine found in safe zone at ({}, {})", x, y);
                }
            }
        }
    }

    #[test]
    fn test_place_mines_no_duplicates() {
        let mut board = Board::new(15, 15, 0);
        place_mines(&mut board, 30);

        let mut mine_positions = std::collections::HashSet::new();
        for y in 0..15 {
            for x in 0..15 {
                if board.is_mine(x, y) {
                    assert!(mine_positions.insert((x, y)), "Duplicate mine at ({}, {})", x, y);
                }
            }
        }

        assert_eq!(mine_positions.len(), 30);
    }
}
