//! Board rendering with colors and formatting

use crate::cell::Cell;
use crate::game::{Game, GameStatus};
use super::colors::Color;

/// Render the game board to a string
pub fn render_board(game: &Game, cursor_x: usize, cursor_y: usize) -> String {
    let board = game.board();
    let mut output = String::new();

    // Header
    output.push_str("╔════════════════════════════════════════╗\n");
    output.push_str("║        MINESWEEPER CLI GAME           ║\n");

    // Status line
    let mine_counter = game.mine_counter();
    let timer = game.timer().formatted();
    output.push_str(&format!(
        "║  Mines: {:03}  Time: {}              ║\n",
        mine_counter, timer
    ));

    output.push_str("╠════════════════════════════════════════╣\n");

    // Board
    for y in 0..board.height() {
        output.push_str("║ ");

        for x in 0..board.width() {
            let cell = board.get_cell(x, y);
            let is_cursor = x == cursor_x && y == cursor_y;

            let symbol = format_cell(cell);

            if is_cursor {
                output.push_str(&format!("{}{}{}{}",
                    "\x1b[7m",  // Inverse video
                    Color::for_mine_count(0),
                    symbol,
                    "\x1b[0m"   // Reset
                ));
            } else {
                output.push_str(&format!("{}{}{}",
                    color_code_for_cell(cell),
                    symbol,
                    "\x1b[0m"
                ));
            }

            output.push(' ');
        }

        output.push_str("║\n");
    }

    output.push_str("╚════════════════════════════════════════╝\n");

    // Position and help text
    output.push_str(&format!(
        "Position: ({}, {})  [Space: Reveal] [F: Flag] [C: Chord] [H: Hint] [R: Restart] [Q: Quit]\n",
        cursor_x, cursor_y
    ));

    // Game status message
    match game.status() {
        GameStatus::Playing => {
            output.push_str("Game in progress...\n");
        }
        GameStatus::Won => {
            output.push_str(&format!("{}You Won! Time: {}{}",
                "\x1b[32m", game.timer().formatted(), "\x1b[0m\n"));
        }
        GameStatus::Lost { revealed_mine } => {
            output.push_str(&format!("{}Game Over! Mine revealed at ({}, {}){}",
                "\x1b[31m", revealed_mine.0, revealed_mine.1, "\x1b[0m\n"));
        }
    }

    output
}

/// Format a cell for display
fn format_cell(cell: &Cell) -> String {
    match cell {
        Cell::Unrevealed => "▢".to_string(),
        Cell::Revealed(0) => "·".to_string(),
        Cell::Revealed(n) => n.to_string(),
        Cell::Flagged => "⚑".to_string(),
        Cell::QuestionMarked => "?".to_string(),
    }
}

/// Get color code for a cell
fn color_code_for_cell(cell: &Cell) -> &'static str {
    match cell {
        Cell::Unrevealed => "\x1b[90m", // Gray
        Cell::Revealed(n) => Color::for_mine_count(*n),
        Cell::Flagged => "\x1b[31m",
        Cell::QuestionMarked => "\x1b[33m",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difficulty::Difficulty;

    #[test]
    fn test_render_board_creation() {
        let game = Game::new(Difficulty::beginner());
        let rendered = render_board(&game, 0, 0);
        assert!(!rendered.is_empty());
    }

    #[test]
    fn test_render_board_includes_cursor() {
        let game = Game::new(Difficulty::beginner());
        let rendered = render_board(&game, 0, 0);
        assert!(rendered.contains("Position"));
    }

    #[test]
    fn test_format_cell_unrevealed() {
        assert_eq!(format_cell(&Cell::Unrevealed), "▢");
    }

    #[test]
    fn test_format_cell_revealed_zero() {
        assert_eq!(format_cell(&Cell::Revealed(0)), "·");
    }

    #[test]
    fn test_format_cell_revealed_number() {
        assert_eq!(format_cell(&Cell::Revealed(5)), "5");
    }

    #[test]
    fn test_format_cell_flagged() {
        assert_eq!(format_cell(&Cell::Flagged), "⚑");
    }

    #[test]
    fn test_format_cell_question_mark() {
        assert_eq!(format_cell(&Cell::QuestionMarked), "?");
    }
}
