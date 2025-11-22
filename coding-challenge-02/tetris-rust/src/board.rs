/// Game board logic - collision detection, line clearing, scoring
use macroquad::prelude::*;
use crate::pieces::Piece;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub filled: bool,
    pub color: Color,
}

impl Cell {
    pub fn empty() -> Self {
        Cell {
            filled: false,
            color: BLACK,
        }
    }

    pub fn filled(color: Color) -> Self {
        Cell {
            filled: true,
            color,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub grid: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
    pub clearing_lines: Vec<usize>,
    pub clear_animation_timer: f32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[Cell::empty(); BOARD_WIDTH]; BOARD_HEIGHT],
            clearing_lines: Vec::new(),
            clear_animation_timer: 0.0,
        }
    }

    pub fn is_valid_position(&self, piece: &Piece) -> bool {
        for (x, y) in piece.blocks() {
            // Check bounds
            if x < 0 || x >= BOARD_WIDTH as i32 || y < 0 || y >= BOARD_HEIGHT as i32 {
                return false;
            }
            // Check collision with existing blocks
            if self.grid[y as usize][x as usize].filled {
                return false;
            }
        }
        true
    }

    pub fn lock_piece(&mut self, piece: &Piece) {
        let color = piece.color();
        for (x, y) in piece.blocks() {
            if y >= 0 && y < BOARD_HEIGHT as i32 && x >= 0 && x < BOARD_WIDTH as i32 {
                self.grid[y as usize][x as usize] = Cell::filled(color);
            }
        }
    }

    /// Check for completed lines and mark them for clearing
    pub fn check_lines(&mut self) -> usize {
        self.clearing_lines.clear();
        for y in 0..BOARD_HEIGHT {
            if self.grid[y].iter().all(|cell| cell.filled) {
                self.clearing_lines.push(y);
            }
        }

        if !self.clearing_lines.is_empty() {
            self.clear_animation_timer = 0.0;
        }

        self.clearing_lines.len()
    }

    /// Update line clearing animation
    pub fn update_clear_animation(&mut self, dt: f32) -> bool {
        if self.clearing_lines.is_empty() {
            return false;
        }

        self.clear_animation_timer += dt;

        // Animation duration
        const CLEAR_DURATION: f32 = 0.3;

        if self.clear_animation_timer >= CLEAR_DURATION {
            self.clear_lines();
            return true;
        }

        false
    }

    /// Actually remove cleared lines and drop blocks above
    fn clear_lines(&mut self) {
        if self.clearing_lines.is_empty() {
            return;
        }

        // Sort in descending order to clear from bottom to top
        self.clearing_lines.sort_by(|a, b| b.cmp(a));

        for &line in &self.clearing_lines {
            // Remove the line
            for y in (1..=line).rev() {
                self.grid[y] = self.grid[y - 1];
            }
            // Clear top line
            self.grid[0] = [Cell::empty(); BOARD_WIDTH];
        }

        self.clearing_lines.clear();
        self.clear_animation_timer = 0.0;
    }

    pub fn is_clearing(&self) -> bool {
        !self.clearing_lines.is_empty()
    }

    /// Calculate how far down the piece can fall (for ghost piece)
    pub fn ghost_y(&self, piece: &Piece) -> i32 {
        let mut ghost = piece.clone();
        while self.is_valid_position(&ghost) {
            ghost.y += 1;
        }
        ghost.y - 1
    }

    pub fn reset(&mut self) {
        self.grid = [[Cell::empty(); BOARD_WIDTH]; BOARD_HEIGHT];
        self.clearing_lines.clear();
        self.clear_animation_timer = 0.0;
    }
}

/// Scoring system following standard Tetris guidelines
#[derive(Debug)]
pub struct Score {
    pub points: u32,
    pub lines_cleared: u32,
    pub level: u32,
    pub combo: u32,
}

impl Score {
    pub fn new() -> Self {
        Score {
            points: 0,
            lines_cleared: 0,
            level: 1,
            combo: 0,
        }
    }

    /// Add points for cleared lines
    pub fn add_line_clear(&mut self, lines: usize) {
        if lines == 0 {
            self.combo = 0;
            return;
        }

        self.lines_cleared += lines as u32;
        self.combo += 1;

        // Base points for line clears
        let base_points = match lines {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800, // Tetris!
            _ => 0,
        };

        // Apply level multiplier and combo bonus
        let level_multiplier = self.level;
        let combo_bonus = if self.combo > 1 {
            50 * (self.combo - 1)
        } else {
            0
        };

        self.points += (base_points * level_multiplier) + combo_bonus;

        // Level up every 10 lines
        self.level = 1 + (self.lines_cleared / 10);
    }

    /// Add points for soft drop (manual down movement)
    pub fn add_soft_drop(&mut self, cells: u32) {
        self.points += cells;
    }

    /// Add points for hard drop
    pub fn add_hard_drop(&mut self, cells: u32) {
        self.points += cells * 2;
    }

    /// Get the fall speed based on level (in seconds per row)
    pub fn fall_speed(&self) -> f32 {
        // Speed increases with level
        let base_speed = 1.0;
        let speed = base_speed * 0.9_f32.powi(self.level as i32 - 1);
        speed.max(0.05) // Minimum speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::PieceType;

    #[test]
    fn test_board_creation() {
        let board = Board::new();
        assert_eq!(board.grid.len(), BOARD_HEIGHT);
        assert_eq!(board.grid[0].len(), BOARD_WIDTH);
    }

    #[test]
    fn test_valid_position() {
        let board = Board::new();
        let piece = Piece::new(PieceType::I);
        assert!(board.is_valid_position(&piece));
    }

    #[test]
    fn test_invalid_position_bounds() {
        let board = Board::new();
        let mut piece = Piece::new(PieceType::I);
        piece.x = -1;
        assert!(!board.is_valid_position(&piece));

        piece.x = BOARD_WIDTH as i32;
        assert!(!board.is_valid_position(&piece));
    }

    #[test]
    fn test_lock_piece() {
        let mut board = Board::new();
        let mut piece = Piece::new(PieceType::O);
        piece.y = BOARD_HEIGHT as i32 - 2;

        board.lock_piece(&piece);

        let blocks = piece.blocks();
        for (x, y) in blocks {
            assert!(board.grid[y as usize][x as usize].filled);
        }
    }

    #[test]
    fn test_line_clearing() {
        let mut board = Board::new();

        // Fill bottom line
        let bottom = BOARD_HEIGHT - 1;
        for x in 0..BOARD_WIDTH {
            board.grid[bottom][x] = Cell::filled(RED);
        }

        let cleared = board.check_lines();
        assert_eq!(cleared, 1);
        assert_eq!(board.clearing_lines.len(), 1);
        assert_eq!(board.clearing_lines[0], bottom);
    }

    #[test]
    fn test_scoring_single_line() {
        let mut score = Score::new();
        score.add_line_clear(1);
        assert_eq!(score.points, 100);
        assert_eq!(score.lines_cleared, 1);
    }

    #[test]
    fn test_scoring_tetris() {
        let mut score = Score::new();
        score.add_line_clear(4);
        assert_eq!(score.points, 800);
        assert_eq!(score.lines_cleared, 4);
    }

    #[test]
    fn test_level_progression() {
        let mut score = Score::new();
        assert_eq!(score.level, 1);

        score.add_line_clear(10);
        assert_eq!(score.level, 2);

        score.add_line_clear(10);
        assert_eq!(score.level, 3);
    }

    #[test]
    fn test_combo_system() {
        let mut score = Score::new();
        score.add_line_clear(1);
        assert_eq!(score.combo, 1);

        let points_before = score.points;
        score.add_line_clear(1);
        assert_eq!(score.combo, 2);
        // Second clear should give bonus
        assert!(score.points > points_before + 100);
    }

    #[test]
    fn test_ghost_piece() {
        let board = Board::new();
        let piece = Piece::new(PieceType::I);
        let ghost_y = board.ghost_y(&piece);
        assert_eq!(ghost_y, BOARD_HEIGHT as i32 - 2);
    }
}
