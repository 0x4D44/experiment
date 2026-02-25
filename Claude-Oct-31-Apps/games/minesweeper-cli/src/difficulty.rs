//! Difficulty presets and custom game configuration

use std::fmt;

/// Difficulty configuration
#[derive(Clone, Debug)]
pub struct Difficulty {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub mines: usize,
    pub allow_question_marks: bool,
}

impl Difficulty {
    /// Beginner difficulty (9×9, 10 mines)
    pub fn beginner() -> Self {
        Difficulty {
            name: "Beginner".to_string(),
            width: 9,
            height: 9,
            mines: 10,
            allow_question_marks: true,
        }
    }

    /// Intermediate difficulty (16×16, 40 mines)
    pub fn intermediate() -> Self {
        Difficulty {
            name: "Intermediate".to_string(),
            width: 16,
            height: 16,
            mines: 40,
            allow_question_marks: true,
        }
    }

    /// Expert difficulty (30×16, 99 mines)
    pub fn expert() -> Self {
        Difficulty {
            name: "Expert".to_string(),
            width: 30,
            height: 16,
            mines: 99,
            allow_question_marks: true,
        }
    }

    /// Custom difficulty with specified dimensions
    pub fn custom(width: usize, height: usize, mines: usize) -> Result<Self, String> {
        let difficulty = Difficulty {
            name: "Custom".to_string(),
            width,
            height,
            mines,
            allow_question_marks: true,
        };

        difficulty.validate()?;
        Ok(difficulty)
    }

    /// Validate difficulty configuration
    pub fn validate(&self) -> Result<(), String> {
        // Check minimum board size (4×4 = 16 cells)
        if self.width < 4 || self.height < 4 {
            return Err(format!(
                "Board too small: {}×{} (minimum 4×4)",
                self.width, self.height
            ));
        }

        // Check maximum board size (100×100)
        if self.width > 100 || self.height > 100 {
            return Err(format!(
                "Board too large: {}×{} (maximum 100×100)",
                self.width, self.height
            ));
        }

        let total_cells = self.width * self.height;

        // Mines must be at least 1
        if self.mines == 0 {
            return Err("Must have at least 1 mine".to_string());
        }

        // Mines cannot exceed 90% of cells
        let max_mines = (total_cells * 90) / 100;
        if self.mines > max_mines {
            return Err(format!(
                "Too many mines: {} (maximum {} for {}×{} board)",
                self.mines, max_mines, self.width, self.height
            ));
        }

        // Mines must fit on board
        if self.mines >= total_cells {
            return Err(format!(
                "Mines exceed board size: {} (available cells: {})",
                self.mines, total_cells
            ));
        }

        Ok(())
    }

    /// Get the 3BV (Bechtel Board Versatility) estimate for this difficulty
    /// This is used for measuring puzzle complexity and scoring
    pub fn estimated_3bv(&self) -> usize {
        let total_cells = self.width * self.height;
        let safe_cells = total_cells - self.mines;

        // Approximate 3BV as roughly 30% of safe cells
        // (First click + opening + flagging)
        (safe_cells * 30) / 100
    }

    /// Get difficulty name
    pub fn difficulty_name(&self) -> &str {
        &self.name
    }

    /// Get total cell count
    pub fn total_cells(&self) -> usize {
        self.width * self.height
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}×{}, {} mines)",
            self.name, self.width, self.height, self.mines
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beginner_preset() {
        let diff = Difficulty::beginner();
        assert_eq!(diff.width, 9);
        assert_eq!(diff.height, 9);
        assert_eq!(diff.mines, 10);
    }

    #[test]
    fn test_intermediate_preset() {
        let diff = Difficulty::intermediate();
        assert_eq!(diff.width, 16);
        assert_eq!(diff.height, 16);
        assert_eq!(diff.mines, 40);
    }

    #[test]
    fn test_expert_preset() {
        let diff = Difficulty::expert();
        assert_eq!(diff.width, 30);
        assert_eq!(diff.height, 16);
        assert_eq!(diff.mines, 99);
    }

    #[test]
    fn test_custom_valid() {
        let diff = Difficulty::custom(10, 10, 20);
        assert!(diff.is_ok());
    }

    #[test]
    fn test_custom_board_too_small() {
        let diff = Difficulty::custom(3, 3, 1);
        assert!(diff.is_err());
    }

    #[test]
    fn test_custom_board_too_large() {
        let diff = Difficulty::custom(101, 100, 100);
        assert!(diff.is_err());
    }

    #[test]
    fn test_custom_no_mines() {
        let diff = Difficulty::custom(10, 10, 0);
        assert!(diff.is_err());
    }

    #[test]
    fn test_custom_too_many_mines() {
        let diff = Difficulty::custom(10, 10, 100); // 100% of cells
        assert!(diff.is_err());
    }

    #[test]
    fn test_estimated_3bv() {
        let diff = Difficulty::beginner(); // 9×9 = 81, 10 mines, 71 safe
        let estimated = diff.estimated_3bv();
        assert!(estimated > 0);
        assert!(estimated <= 71);
    }

    #[test]
    fn test_total_cells() {
        let diff = Difficulty::beginner();
        assert_eq!(diff.total_cells(), 81);
    }

    #[test]
    fn test_display() {
        let diff = Difficulty::beginner();
        let display = format!("{}", diff);
        assert!(display.contains("Beginner"));
        assert!(display.contains("9×9"));
    }
}
