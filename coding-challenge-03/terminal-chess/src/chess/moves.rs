use super::{Piece, PieceType, Position};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveType {
    Normal,
    Capture,
    EnPassant,
    Castle,
    Promotion(PieceType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
    pub piece: Piece,
    pub captured: Option<Piece>,
}

impl Move {
    pub fn new(from: Position, to: Position, piece: Piece) -> Self {
        Move {
            from,
            to,
            move_type: MoveType::Normal,
            piece,
            captured: None,
        }
    }

    pub fn with_capture(from: Position, to: Position, piece: Piece, captured: Piece) -> Self {
        Move {
            from,
            to,
            move_type: MoveType::Capture,
            piece,
            captured: Some(captured),
        }
    }

    pub fn en_passant(from: Position, to: Position, piece: Piece, captured: Piece) -> Self {
        Move {
            from,
            to,
            move_type: MoveType::EnPassant,
            piece,
            captured: Some(captured),
        }
    }

    pub fn castle(from: Position, to: Position, piece: Piece) -> Self {
        Move {
            from,
            to,
            move_type: MoveType::Castle,
            piece,
            captured: None,
        }
    }

    pub fn promotion(from: Position, to: Position, piece: Piece, promote_to: PieceType, captured: Option<Piece>) -> Self {
        Move {
            from,
            to,
            move_type: MoveType::Promotion(promote_to),
            piece,
            captured,
        }
    }

    /// Convert move to algebraic notation (e.g., "e2e4", "e7e8q" for promotion)
    pub fn to_algebraic(&self) -> String {
        let base = format!("{}{}", self.from.to_algebraic(), self.to.to_algebraic());
        match self.move_type {
            MoveType::Promotion(piece_type) => {
                let piece_char = match piece_type {
                    PieceType::Queen => 'q',
                    PieceType::Rook => 'r',
                    PieceType::Bishop => 'b',
                    PieceType::Knight => 'n',
                    _ => unreachable!(),
                };
                format!("{}{}", base, piece_char)
            }
            _ => base,
        }
    }

    /// Parse algebraic notation (e.g., "e2e4", "e7e8q")
    pub fn from_algebraic(s: &str, piece: Piece, captured: Option<Piece>) -> Option<Self> {
        if s.len() < 4 {
            return None;
        }

        let from = Position::from_algebraic(&s[0..2])?;
        let to = Position::from_algebraic(&s[2..4])?;

        if s.len() == 5 {
            // Promotion
            let promote_to = match s.chars().nth(4)? {
                'q' | 'Q' => PieceType::Queen,
                'r' | 'R' => PieceType::Rook,
                'b' | 'B' => PieceType::Bishop,
                'n' | 'N' => PieceType::Knight,
                _ => return None,
            };
            Some(Move::promotion(from, to, piece, promote_to, captured))
        } else {
            let mut mov = Move::new(from, to, piece);
            if let Some(cap) = captured {
                mov.captured = Some(cap);
                mov.move_type = MoveType::Capture;
            }
            Some(mov)
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::Color;

    #[test]
    fn test_move_creation() {
        let from = Position::new(1, 4).unwrap();
        let to = Position::new(3, 4).unwrap();
        let piece = Piece::new(PieceType::Pawn, Color::White);

        let mov = Move::new(from, to, piece);
        assert_eq!(mov.from, from);
        assert_eq!(mov.to, to);
        assert_eq!(mov.move_type, MoveType::Normal);
    }

    #[test]
    fn test_algebraic_notation() {
        let from = Position::from_algebraic("e2").unwrap();
        let to = Position::from_algebraic("e4").unwrap();
        let piece = Piece::new(PieceType::Pawn, Color::White);

        let mov = Move::new(from, to, piece);
        assert_eq!(mov.to_algebraic(), "e2e4");
    }

    #[test]
    fn test_promotion_notation() {
        let from = Position::from_algebraic("e7").unwrap();
        let to = Position::from_algebraic("e8").unwrap();
        let piece = Piece::new(PieceType::Pawn, Color::White);

        let mov = Move::promotion(from, to, piece, PieceType::Queen, None);
        assert_eq!(mov.to_algebraic(), "e7e8q");
    }
}
