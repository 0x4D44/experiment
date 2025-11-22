use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }

    /// Get the Unicode character for this piece
    pub fn to_unicode(&self) -> char {
        match (self.color, self.piece_type) {
            (Color::White, PieceType::King) => '♔',
            (Color::White, PieceType::Queen) => '♕',
            (Color::White, PieceType::Rook) => '♖',
            (Color::White, PieceType::Bishop) => '♗',
            (Color::White, PieceType::Knight) => '♘',
            (Color::White, PieceType::Pawn) => '♙',
            (Color::Black, PieceType::King) => '♚',
            (Color::Black, PieceType::Queen) => '♛',
            (Color::Black, PieceType::Rook) => '♜',
            (Color::Black, PieceType::Bishop) => '♝',
            (Color::Black, PieceType::Knight) => '♞',
            (Color::Black, PieceType::Pawn) => '♟',
        }
    }

    /// Get the piece value for evaluation (in centipawns)
    pub fn value(&self) -> i32 {
        match self.piece_type {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_opposite() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
    }

    #[test]
    fn test_piece_values() {
        let pawn = Piece::new(PieceType::Pawn, Color::White);
        let queen = Piece::new(PieceType::Queen, Color::Black);
        assert_eq!(pawn.value(), 100);
        assert_eq!(queen.value(), 900);
    }

    #[test]
    fn test_unicode_representation() {
        let white_king = Piece::new(PieceType::King, Color::White);
        let black_pawn = Piece::new(PieceType::Pawn, Color::Black);
        assert_eq!(white_king.to_unicode(), '♔');
        assert_eq!(black_pawn.to_unicode(), '♟');
    }
}
