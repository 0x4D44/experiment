use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    pub fn new(row: i8, col: i8) -> Option<Self> {
        if (0..8).contains(&row) && (0..8).contains(&col) {
            Some(Position { row, col })
        } else {
            None
        }
    }

    pub fn from_algebraic(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }

        let col = (bytes[0] as i8) - ('a' as i8);
        let row = (bytes[1] as i8) - ('1' as i8);

        Position::new(row, col)
    }

    pub fn to_algebraic(&self) -> String {
        let col_char = (b'a' + self.col as u8) as char;
        let row_char = (b'1' + self.row as u8) as char;
        format!("{}{}", col_char, row_char)
    }

    pub fn is_valid(&self) -> bool {
        self.row >= 0 && self.row < 8 && self.col >= 0 && self.col < 8
    }

    pub fn offset(&self, row_delta: i8, col_delta: i8) -> Option<Self> {
        Position::new(self.row + row_delta, self.col + col_delta)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        assert!(Position::new(0, 0).is_some());
        assert!(Position::new(7, 7).is_some());
        assert!(Position::new(8, 0).is_none());
        assert!(Position::new(-1, 0).is_none());
    }

    #[test]
    fn test_algebraic_notation() {
        let pos = Position::from_algebraic("e4").unwrap();
        assert_eq!(pos.row, 3);
        assert_eq!(pos.col, 4);
        assert_eq!(pos.to_algebraic(), "e4");

        let pos2 = Position::from_algebraic("a1").unwrap();
        assert_eq!(pos2.to_algebraic(), "a1");

        assert!(Position::from_algebraic("z9").is_none());
        assert!(Position::from_algebraic("abc").is_none());
    }

    #[test]
    fn test_offset() {
        let pos = Position::new(4, 4).unwrap();
        let new_pos = pos.offset(1, 1).unwrap();
        assert_eq!(new_pos.row, 5);
        assert_eq!(new_pos.col, 5);

        assert!(pos.offset(10, 0).is_none());
    }
}
