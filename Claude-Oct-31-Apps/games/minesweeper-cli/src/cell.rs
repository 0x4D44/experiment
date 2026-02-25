//! Cell state definitions and operations

use std::fmt;

/// Represents the visual state of a cell on the game board
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    /// Unrevealed cell (not yet clicked)
    Unrevealed,
    /// Revealed cell with adjacent mine count (0-8)
    Revealed(u8),
    /// Flagged as a mine
    Flagged,
    /// Marked with question mark (uncertain)
    QuestionMarked,
}

/// Represents the flag state of a cell
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Flag {
    None,
    Flagged,
    QuestionMarked,
}

/// Complete cell data including state and mine status
#[derive(Clone, Debug)]
pub struct CellData {
    pub cell: Cell,
    pub is_mine: bool,
}

impl CellData {
    /// Create a new cell with given mine status
    pub fn new(is_mine: bool) -> Self {
        CellData {
            cell: Cell::Unrevealed,
            is_mine,
        }
    }

    /// Get the display representation of this cell
    pub fn display_char(&self) -> &'static str {
        match &self.cell {
            Cell::Unrevealed => "▢",
            Cell::Revealed(0) => "·",
            Cell::Revealed(n) => {
                match n {
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    8 => "8",
                    _ => "?",
                }
            }
            Cell::Flagged => "⚑",
            Cell::QuestionMarked => "?",
        }
    }

    /// Get ANSI color code for this cell
    pub fn color_code(&self) -> &'static str {
        match &self.cell {
            Cell::Unrevealed => "\x1b[90m", // Dark gray
            Cell::Revealed(0) => "\x1b[37m", // White
            Cell::Revealed(1) => "\x1b[34m", // Blue
            Cell::Revealed(2) => "\x1b[32m", // Green
            Cell::Revealed(3) => "\x1b[31m", // Red
            Cell::Revealed(4) => "\x1b[44m", // Dark blue background
            Cell::Revealed(5) => "\x1b[35m", // Magenta/Brown
            Cell::Revealed(6) => "\x1b[36m", // Cyan
            Cell::Revealed(7) => "\x1b[30m", // Black
            Cell::Revealed(8) => "\x1b[90m", // Dark gray
            Cell::Revealed(_) => "\x1b[90m", // Dark gray for invalid counts
            Cell::Flagged => "\x1b[31m", // Red
            Cell::QuestionMarked => "\x1b[33m", // Yellow
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_data_creation() {
        let cell = CellData::new(true);
        assert!(cell.is_mine);
        assert_eq!(cell.cell, Cell::Unrevealed);
    }

    #[test]
    fn test_display_char_unrevealed() {
        let cell = CellData::new(false);
        assert_eq!(cell.display_char(), "▢");
    }

    #[test]
    fn test_display_char_revealed_numbers() {
        for n in 0..=8 {
            let mut cell = CellData::new(false);
            cell.cell = Cell::Revealed(n);
            assert!(!cell.display_char().is_empty());
        }
    }

    #[test]
    fn test_display_char_flagged() {
        let mut cell = CellData::new(true);
        cell.cell = Cell::Flagged;
        assert_eq!(cell.display_char(), "⚑");
    }

    #[test]
    fn test_display_char_question_mark() {
        let mut cell = CellData::new(false);
        cell.cell = Cell::QuestionMarked;
        assert_eq!(cell.display_char(), "?");
    }
}
