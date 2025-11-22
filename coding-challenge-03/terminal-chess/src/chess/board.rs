use super::{Color, Move, MoveType, Piece, PieceType, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    pub en_passant_target: Option<Position>,
    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Board {
    /// Create a new board with the standard starting position
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[None; 8]; 8],
            en_passant_target: None,
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        // Place pawns
        for col in 0..8 {
            board.set_piece(Position::new(1, col).unwrap(), Some(Piece::new(PieceType::Pawn, Color::White)));
            board.set_piece(Position::new(6, col).unwrap(), Some(Piece::new(PieceType::Pawn, Color::Black)));
        }

        // Place white pieces
        board.set_piece(Position::new(0, 0).unwrap(), Some(Piece::new(PieceType::Rook, Color::White)));
        board.set_piece(Position::new(0, 7).unwrap(), Some(Piece::new(PieceType::Rook, Color::White)));
        board.set_piece(Position::new(0, 1).unwrap(), Some(Piece::new(PieceType::Knight, Color::White)));
        board.set_piece(Position::new(0, 6).unwrap(), Some(Piece::new(PieceType::Knight, Color::White)));
        board.set_piece(Position::new(0, 2).unwrap(), Some(Piece::new(PieceType::Bishop, Color::White)));
        board.set_piece(Position::new(0, 5).unwrap(), Some(Piece::new(PieceType::Bishop, Color::White)));
        board.set_piece(Position::new(0, 3).unwrap(), Some(Piece::new(PieceType::Queen, Color::White)));
        board.set_piece(Position::new(0, 4).unwrap(), Some(Piece::new(PieceType::King, Color::White)));

        // Place black pieces
        board.set_piece(Position::new(7, 0).unwrap(), Some(Piece::new(PieceType::Rook, Color::Black)));
        board.set_piece(Position::new(7, 7).unwrap(), Some(Piece::new(PieceType::Rook, Color::Black)));
        board.set_piece(Position::new(7, 1).unwrap(), Some(Piece::new(PieceType::Knight, Color::Black)));
        board.set_piece(Position::new(7, 6).unwrap(), Some(Piece::new(PieceType::Knight, Color::Black)));
        board.set_piece(Position::new(7, 2).unwrap(), Some(Piece::new(PieceType::Bishop, Color::Black)));
        board.set_piece(Position::new(7, 5).unwrap(), Some(Piece::new(PieceType::Bishop, Color::Black)));
        board.set_piece(Position::new(7, 3).unwrap(), Some(Piece::new(PieceType::Queen, Color::Black)));
        board.set_piece(Position::new(7, 4).unwrap(), Some(Piece::new(PieceType::King, Color::Black)));

        board
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        self.squares[pos.row as usize][pos.col as usize]
    }

    pub fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
        self.squares[pos.row as usize][pos.col as usize] = piece;
    }

    /// Make a move on the board
    pub fn make_move(&mut self, mov: &Move) {
        match mov.move_type {
            MoveType::Castle => {
                // Move king
                self.set_piece(mov.from, None);
                self.set_piece(mov.to, Some(mov.piece));

                // Move rook
                let (rook_from, rook_to) = if mov.to.col > mov.from.col {
                    // Kingside
                    (Position::new(mov.from.row, 7).unwrap(), Position::new(mov.from.row, 5).unwrap())
                } else {
                    // Queenside
                    (Position::new(mov.from.row, 0).unwrap(), Position::new(mov.from.row, 3).unwrap())
                };
                let rook = self.get_piece(rook_from).unwrap();
                self.set_piece(rook_from, None);
                self.set_piece(rook_to, Some(rook));
            }
            MoveType::EnPassant => {
                // Move pawn
                self.set_piece(mov.from, None);
                self.set_piece(mov.to, Some(mov.piece));
                // Remove captured pawn
                let captured_pos = Position::new(mov.from.row, mov.to.col).unwrap();
                self.set_piece(captured_pos, None);
            }
            MoveType::Promotion(piece_type) => {
                self.set_piece(mov.from, None);
                self.set_piece(mov.to, Some(Piece::new(piece_type, mov.piece.color)));
            }
            _ => {
                // Normal move or capture
                self.set_piece(mov.from, None);
                self.set_piece(mov.to, Some(mov.piece));
            }
        }

        // Update castling rights
        match mov.piece.piece_type {
            PieceType::King => {
                match mov.piece.color {
                    Color::White => {
                        self.white_can_castle_kingside = false;
                        self.white_can_castle_queenside = false;
                    }
                    Color::Black => {
                        self.black_can_castle_kingside = false;
                        self.black_can_castle_queenside = false;
                    }
                }
            }
            PieceType::Rook => {
                if mov.from.col == 0 {
                    if mov.piece.color == Color::White {
                        self.white_can_castle_queenside = false;
                    } else {
                        self.black_can_castle_queenside = false;
                    }
                } else if mov.from.col == 7 {
                    if mov.piece.color == Color::White {
                        self.white_can_castle_kingside = false;
                    } else {
                        self.black_can_castle_kingside = false;
                    }
                }
            }
            _ => {}
        }

        // Update en passant target
        if mov.piece.piece_type == PieceType::Pawn && (mov.to.row - mov.from.row).abs() == 2 {
            let ep_row = (mov.from.row + mov.to.row) / 2;
            self.en_passant_target = Position::new(ep_row, mov.from.col);
        } else {
            self.en_passant_target = None;
        }

        // Update halfmove clock
        if mov.piece.piece_type == PieceType::Pawn || mov.captured.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
    }

    /// Undo a move
    pub fn unmake_move(&mut self, mov: &Move, prev_en_passant: Option<Position>, prev_castling: (bool, bool, bool, bool), prev_halfmove: u32) {
        match mov.move_type {
            MoveType::Castle => {
                // Move king back
                self.set_piece(mov.to, None);
                self.set_piece(mov.from, Some(mov.piece));

                // Move rook back
                let (rook_from, rook_to) = if mov.to.col > mov.from.col {
                    (Position::new(mov.from.row, 7).unwrap(), Position::new(mov.from.row, 5).unwrap())
                } else {
                    (Position::new(mov.from.row, 0).unwrap(), Position::new(mov.from.row, 3).unwrap())
                };
                let rook = self.get_piece(rook_to).unwrap();
                self.set_piece(rook_to, None);
                self.set_piece(rook_from, Some(rook));
            }
            MoveType::EnPassant => {
                // Move pawn back
                self.set_piece(mov.to, None);
                self.set_piece(mov.from, Some(mov.piece));
                // Restore captured pawn
                let captured_pos = Position::new(mov.from.row, mov.to.col).unwrap();
                self.set_piece(captured_pos, mov.captured);
            }
            MoveType::Promotion(_) => {
                self.set_piece(mov.to, mov.captured);
                self.set_piece(mov.from, Some(mov.piece));
            }
            _ => {
                self.set_piece(mov.to, mov.captured);
                self.set_piece(mov.from, Some(mov.piece));
            }
        }

        self.en_passant_target = prev_en_passant;
        self.white_can_castle_kingside = prev_castling.0;
        self.white_can_castle_queenside = prev_castling.1;
        self.black_can_castle_kingside = prev_castling.2;
        self.black_can_castle_queenside = prev_castling.3;
        self.halfmove_clock = prev_halfmove;
    }

    /// Find the king position for a given color
    pub fn find_king(&self, color: Color) -> Option<Position> {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.get_piece(Position::new(row, col).unwrap()) {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Position::new(row, col);
                    }
                }
            }
        }
        None
    }

    /// Check if a position is under attack by the given color
    pub fn is_square_attacked(&self, pos: Position, by_color: Color) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                let from = Position::new(row, col).unwrap();
                if let Some(piece) = self.get_piece(from) {
                    if piece.color == by_color {
                        let pseudo_moves = self.generate_pseudo_legal_moves(from, piece);
                        for mov in pseudo_moves {
                            if mov.to == pos {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Check if the given color is in check
    pub fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_pos) = self.find_king(color) {
            self.is_square_attacked(king_pos, color.opposite())
        } else {
            false
        }
    }

    /// Generate all pseudo-legal moves (may leave king in check)
    /// If include_castling is false, castling moves are excluded (to avoid infinite recursion)
    fn generate_pseudo_legal_moves(&self, from: Position, piece: Piece) -> Vec<Move> {
        self.generate_pseudo_legal_moves_internal(from, piece, false)
    }

    fn generate_pseudo_legal_moves_internal(&self, from: Position, piece: Piece, include_castling: bool) -> Vec<Move> {
        match piece.piece_type {
            PieceType::Pawn => self.generate_pawn_moves(from, piece),
            PieceType::Knight => self.generate_knight_moves(from, piece),
            PieceType::Bishop => self.generate_bishop_moves(from, piece),
            PieceType::Rook => self.generate_rook_moves(from, piece),
            PieceType::Queen => self.generate_queen_moves(from, piece),
            PieceType::King => self.generate_king_moves_internal(from, piece, include_castling),
        }
    }

    fn generate_pawn_moves(&self, from: Position, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let direction = if piece.color == Color::White { 1 } else { -1 };
        let start_row = if piece.color == Color::White { 1 } else { 6 };
        let promotion_row = if piece.color == Color::White { 7 } else { 0 };

        // Forward move
        if let Some(to) = from.offset(direction, 0) {
            if self.get_piece(to).is_none() {
                if to.row == promotion_row {
                    // Promotion
                    for piece_type in &[PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                        moves.push(Move::promotion(from, to, piece, *piece_type, None));
                    }
                } else {
                    moves.push(Move::new(from, to, piece));
                }

                // Double forward move from starting position
                if from.row == start_row {
                    if let Some(double_to) = from.offset(direction * 2, 0) {
                        if self.get_piece(double_to).is_none() {
                            moves.push(Move::new(from, double_to, piece));
                        }
                    }
                }
            }
        }

        // Captures
        for col_offset in [-1, 1] {
            if let Some(to) = from.offset(direction, col_offset) {
                if let Some(captured) = self.get_piece(to) {
                    if captured.color != piece.color {
                        if to.row == promotion_row {
                            for piece_type in &[PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                                moves.push(Move::promotion(from, to, piece, *piece_type, Some(captured)));
                            }
                        } else {
                            moves.push(Move::with_capture(from, to, piece, captured));
                        }
                    }
                }

                // En passant
                if Some(to) == self.en_passant_target {
                    if let Some(captured_pos) = Position::new(from.row, to.col) {
                        if let Some(captured) = self.get_piece(captured_pos) {
                            moves.push(Move::en_passant(from, to, piece, captured));
                        }
                    }
                }
            }
        }

        moves
    }

    fn generate_knight_moves(&self, from: Position, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let offsets = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2),
        ];

        for (row_offset, col_offset) in offsets {
            if let Some(to) = from.offset(row_offset, col_offset) {
                match self.get_piece(to) {
                    None => moves.push(Move::new(from, to, piece)),
                    Some(captured) if captured.color != piece.color => {
                        moves.push(Move::with_capture(from, to, piece, captured));
                    }
                    _ => {}
                }
            }
        }

        moves
    }

    fn generate_sliding_moves(&self, from: Position, piece: Piece, directions: &[(i8, i8)]) -> Vec<Move> {
        let mut moves = Vec::new();

        for &(row_dir, col_dir) in directions {
            let mut current = from;
            while let Some(next) = current.offset(row_dir, col_dir) {
                match self.get_piece(next) {
                    None => {
                        moves.push(Move::new(from, next, piece));
                        current = next;
                    }
                    Some(captured) => {
                        if captured.color != piece.color {
                            moves.push(Move::with_capture(from, next, piece, captured));
                        }
                        break;
                    }
                }
            }
        }

        moves
    }

    fn generate_bishop_moves(&self, from: Position, piece: Piece) -> Vec<Move> {
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.generate_sliding_moves(from, piece, &directions)
    }

    fn generate_rook_moves(&self, from: Position, piece: Piece) -> Vec<Move> {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        self.generate_sliding_moves(from, piece, &directions)
    }

    fn generate_queen_moves(&self, from: Position, piece: Piece) -> Vec<Move> {
        let directions = [
            (1, 0), (-1, 0), (0, 1), (0, -1),
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        self.generate_sliding_moves(from, piece, &directions)
    }


    fn generate_king_moves_internal(&self, from: Position, piece: Piece, include_castling: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let directions = [
            (1, 0), (-1, 0), (0, 1), (0, -1),
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];

        for (row_offset, col_offset) in directions {
            if let Some(to) = from.offset(row_offset, col_offset) {
                match self.get_piece(to) {
                    None => moves.push(Move::new(from, to, piece)),
                    Some(captured) if captured.color != piece.color => {
                        moves.push(Move::with_capture(from, to, piece, captured));
                    }
                    _ => {}
                }
            }
        }

        // Only include castling if requested (to avoid infinite recursion in is_square_attacked)
        if include_castling {
            // Castling
            let (can_kingside, can_queenside) = match piece.color {
                Color::White => (self.white_can_castle_kingside, self.white_can_castle_queenside),
                Color::Black => (self.black_can_castle_kingside, self.black_can_castle_queenside),
            };

            if can_kingside && !self.is_in_check(piece.color) {
                let squares_empty = self.get_piece(Position::new(from.row, 5).unwrap()).is_none()
                    && self.get_piece(Position::new(from.row, 6).unwrap()).is_none();
                let squares_safe = !self.is_square_attacked(Position::new(from.row, 5).unwrap(), piece.color.opposite())
                    && !self.is_square_attacked(Position::new(from.row, 6).unwrap(), piece.color.opposite());

                if squares_empty && squares_safe {
                    moves.push(Move::castle(from, Position::new(from.row, 6).unwrap(), piece));
                }
            }

            if can_queenside && !self.is_in_check(piece.color) {
                let squares_empty = self.get_piece(Position::new(from.row, 1).unwrap()).is_none()
                    && self.get_piece(Position::new(from.row, 2).unwrap()).is_none()
                    && self.get_piece(Position::new(from.row, 3).unwrap()).is_none();
                let squares_safe = !self.is_square_attacked(Position::new(from.row, 2).unwrap(), piece.color.opposite())
                    && !self.is_square_attacked(Position::new(from.row, 3).unwrap(), piece.color.opposite());

                if squares_empty && squares_safe {
                    moves.push(Move::castle(from, Position::new(from.row, 2).unwrap(), piece));
                }
            }
        }

        moves
    }

    /// Generate all legal moves for a given color
    pub fn generate_legal_moves(&self, color: Color) -> Vec<Move> {
        let mut legal_moves = Vec::new();

        for row in 0..8 {
            for col in 0..8 {
                let from = Position::new(row, col).unwrap();
                if let Some(piece) = self.get_piece(from) {
                    if piece.color == color {
                        // Include castling moves in legal move generation
                        let pseudo_moves = self.generate_pseudo_legal_moves_internal(from, piece, true);

                        for mov in pseudo_moves {
                            // Test if move leaves king in check
                            let mut test_board = self.clone();
                            test_board.make_move(&mov);
                            if !test_board.is_in_check(color) {
                                legal_moves.push(mov);
                            }
                        }
                    }
                }
            }
        }

        legal_moves
    }

    /// Evaluate the board position (positive = white advantage)
    pub fn evaluate(&self) -> i32 {
        let mut score = 0;

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.get_piece(Position::new(row, col).unwrap()) {
                    let value = piece.value();
                    let position_bonus = self.get_position_bonus(piece, row, col);

                    match piece.color {
                        Color::White => score += value + position_bonus,
                        Color::Black => score -= value + position_bonus,
                    }
                }
            }
        }

        score
    }

    /// Get position-based bonus for piece-square tables
    fn get_position_bonus(&self, piece: Piece, row: i8, col: i8) -> i32 {
        let actual_row = if piece.color == Color::White { row } else { 7 - row };

        match piece.piece_type {
            PieceType::Pawn => {
                let pawn_table = [
                    0,  0,  0,  0,  0,  0,  0,  0,
                    50, 50, 50, 50, 50, 50, 50, 50,
                    10, 10, 20, 30, 30, 20, 10, 10,
                    5,  5, 10, 25, 25, 10,  5,  5,
                    0,  0,  0, 20, 20,  0,  0,  0,
                    5, -5,-10,  0,  0,-10, -5,  5,
                    5, 10, 10,-20,-20, 10, 10,  5,
                    0,  0,  0,  0,  0,  0,  0,  0
                ];
                pawn_table[(actual_row * 8 + col) as usize]
            }
            PieceType::Knight => {
                let knight_table = [
                    -50,-40,-30,-30,-30,-30,-40,-50,
                    -40,-20,  0,  0,  0,  0,-20,-40,
                    -30,  0, 10, 15, 15, 10,  0,-30,
                    -30,  5, 15, 20, 20, 15,  5,-30,
                    -30,  0, 15, 20, 20, 15,  0,-30,
                    -30,  5, 10, 15, 15, 10,  5,-30,
                    -40,-20,  0,  5,  5,  0,-20,-40,
                    -50,-40,-30,-30,-30,-30,-40,-50,
                ];
                knight_table[(actual_row * 8 + col) as usize]
            }
            _ => 0,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_board() {
        let board = Board::new();

        // Check white pawns
        for col in 0..8 {
            let piece = board.get_piece(Position::new(1, col).unwrap()).unwrap();
            assert_eq!(piece.piece_type, PieceType::Pawn);
            assert_eq!(piece.color, Color::White);
        }

        // Check black pawns
        for col in 0..8 {
            let piece = board.get_piece(Position::new(6, col).unwrap()).unwrap();
            assert_eq!(piece.piece_type, PieceType::Pawn);
            assert_eq!(piece.color, Color::Black);
        }

        // Check white king
        let white_king = board.get_piece(Position::new(0, 4).unwrap()).unwrap();
        assert_eq!(white_king.piece_type, PieceType::King);
        assert_eq!(white_king.color, Color::White);
    }

    #[test]
    fn test_move_generation() {
        let board = Board::new();
        let white_moves = board.generate_legal_moves(Color::White);
        assert_eq!(white_moves.len(), 20); // 16 pawn moves + 4 knight moves
    }

    #[test]
    fn test_make_move() {
        let mut board = Board::new();
        let from = Position::from_algebraic("e2").unwrap();
        let to = Position::from_algebraic("e4").unwrap();
        let piece = board.get_piece(from).unwrap();
        let mov = Move::new(from, to, piece);

        board.make_move(&mov);

        assert!(board.get_piece(from).is_none());
        assert_eq!(board.get_piece(to).unwrap().piece_type, PieceType::Pawn);
    }

    #[test]
    fn test_check_detection() {
        let mut board = Board::new();

        // Clear some pieces to create a check scenario
        board.set_piece(Position::from_algebraic("e2").unwrap(), None);
        board.set_piece(Position::from_algebraic("d2").unwrap(), None);
        board.set_piece(Position::from_algebraic("d1").unwrap(), None);

        // Move black queen to check white king
        let black_queen = Piece::new(PieceType::Queen, Color::Black);
        board.set_piece(Position::from_algebraic("d7").unwrap(), None);
        board.set_piece(Position::from_algebraic("d2").unwrap(), Some(black_queen));

        assert!(board.is_in_check(Color::White));
        assert!(!board.is_in_check(Color::Black));
    }
}
