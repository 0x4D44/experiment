/// Tetris piece definitions and rotation logic
use macroquad::prelude::*;
use ::rand::{Rng, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceType {
    pub fn all() -> [PieceType; 7] {
        [
            PieceType::I,
            PieceType::O,
            PieceType::T,
            PieceType::S,
            PieceType::Z,
            PieceType::J,
            PieceType::L,
        ]
    }

    pub fn random() -> PieceType {
        let mut rng = thread_rng();
        let pieces = Self::all();
        pieces[rng.gen_range(0..pieces.len())]
    }

    pub fn color(&self) -> Color {
        match self {
            PieceType::I => Color::from_rgba(0, 240, 240, 255),  // Cyan
            PieceType::O => Color::from_rgba(240, 240, 0, 255),  // Yellow
            PieceType::T => Color::from_rgba(160, 0, 240, 255),  // Purple
            PieceType::S => Color::from_rgba(0, 240, 0, 255),    // Green
            PieceType::Z => Color::from_rgba(240, 0, 0, 255),    // Red
            PieceType::J => Color::from_rgba(0, 0, 240, 255),    // Blue
            PieceType::L => Color::from_rgba(240, 160, 0, 255),  // Orange
        }
    }

    /// Get the shape matrix for this piece at the given rotation
    /// Rotation: 0 = 0°, 1 = 90°, 2 = 180°, 3 = 270°
    pub fn shape(&self, rotation: u8) -> Vec<Vec<bool>> {
        let rotation = rotation % 4;
        match self {
            PieceType::I => match rotation {
                0 => vec![
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                1 => vec![
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                ],
                2 => vec![
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                ],
                _ => vec![
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                ],
            },
            PieceType::O => vec![
                vec![false, true, true, false],
                vec![false, true, true, false],
                vec![false, false, false, false],
            ],
            PieceType::T => match rotation {
                0 => vec![
                    vec![false, true, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                1 => vec![
                    vec![false, true, false],
                    vec![false, true, true],
                    vec![false, true, false],
                ],
                2 => vec![
                    vec![false, false, false],
                    vec![true, true, true],
                    vec![false, true, false],
                ],
                _ => vec![
                    vec![false, true, false],
                    vec![true, true, false],
                    vec![false, true, false],
                ],
            },
            PieceType::S => match rotation {
                0 | 2 => vec![
                    vec![false, true, true],
                    vec![true, true, false],
                    vec![false, false, false],
                ],
                _ => vec![
                    vec![false, true, false],
                    vec![false, true, true],
                    vec![false, false, true],
                ],
            },
            PieceType::Z => match rotation {
                0 | 2 => vec![
                    vec![true, true, false],
                    vec![false, true, true],
                    vec![false, false, false],
                ],
                _ => vec![
                    vec![false, false, true],
                    vec![false, true, true],
                    vec![false, true, false],
                ],
            },
            PieceType::J => match rotation {
                0 => vec![
                    vec![true, false, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                1 => vec![
                    vec![false, true, true],
                    vec![false, true, false],
                    vec![false, true, false],
                ],
                2 => vec![
                    vec![false, false, false],
                    vec![true, true, true],
                    vec![false, false, true],
                ],
                _ => vec![
                    vec![false, true, false],
                    vec![false, true, false],
                    vec![true, true, false],
                ],
            },
            PieceType::L => match rotation {
                0 => vec![
                    vec![false, false, true],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                1 => vec![
                    vec![false, true, false],
                    vec![false, true, false],
                    vec![false, true, true],
                ],
                2 => vec![
                    vec![false, false, false],
                    vec![true, true, true],
                    vec![true, false, false],
                ],
                _ => vec![
                    vec![true, true, false],
                    vec![false, true, false],
                    vec![false, true, false],
                ],
            },
        }
    }

    /// Get wall kick offsets for SRS (Super Rotation System)
    pub fn get_wall_kicks(&self, from_rotation: u8, to_rotation: u8) -> Vec<(i32, i32)> {
        let from = from_rotation % 4;
        let to = to_rotation % 4;

        if *self == PieceType::I {
            // I piece has special wall kicks
            match (from, to) {
                (0, 1) => vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (1, 0) => vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (1, 2) => vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                (2, 1) => vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (2, 3) => vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (3, 2) => vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (3, 0) => vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (0, 3) => vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                _ => vec![(0, 0)],
            }
        } else if *self == PieceType::O {
            // O piece doesn't rotate
            vec![(0, 0)]
        } else {
            // Standard wall kicks for J, L, S, T, Z
            match (from, to) {
                (0, 1) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (1, 0) => vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (1, 2) => vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (2, 1) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (2, 3) => vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                (3, 2) => vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                (3, 0) => vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                (0, 3) => vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                _ => vec![(0, 0)],
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        Piece {
            piece_type,
            x: 3,
            y: 0,
            rotation: 0,
        }
    }

    pub fn shape(&self) -> Vec<Vec<bool>> {
        self.piece_type.shape(self.rotation)
    }

    pub fn color(&self) -> Color {
        self.piece_type.color()
    }

    /// Get all filled block positions relative to piece position
    pub fn blocks(&self) -> Vec<(i32, i32)> {
        let shape = self.shape();
        let mut blocks = Vec::new();
        for (y, row) in shape.iter().enumerate() {
            for (x, &filled) in row.iter().enumerate() {
                if filled {
                    blocks.push((self.x + x as i32, self.y + y as i32));
                }
            }
        }
        blocks
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = if self.rotation == 0 { 3 } else { self.rotation - 1 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_shapes() {
        // Test that all pieces have valid shapes
        for piece_type in PieceType::all() {
            for rotation in 0..4 {
                let shape = piece_type.shape(rotation);
                assert!(!shape.is_empty(), "Shape should not be empty");
                assert!(!shape[0].is_empty(), "Shape rows should not be empty");
            }
        }
    }

    #[test]
    fn test_o_piece_no_rotation() {
        let shape0 = PieceType::O.shape(0);
        let shape1 = PieceType::O.shape(1);
        assert_eq!(shape0, shape1, "O piece should not change with rotation");
    }

    #[test]
    fn test_piece_blocks() {
        let piece = Piece::new(PieceType::I);
        let blocks = piece.blocks();
        assert_eq!(blocks.len(), 4, "I piece should have 4 blocks");
    }

    #[test]
    fn test_rotation() {
        let mut piece = Piece::new(PieceType::T);
        assert_eq!(piece.rotation, 0);
        piece.rotate_cw();
        assert_eq!(piece.rotation, 1);
        piece.rotate_cw();
        assert_eq!(piece.rotation, 2);
        piece.rotate_ccw();
        assert_eq!(piece.rotation, 1);
    }
}
