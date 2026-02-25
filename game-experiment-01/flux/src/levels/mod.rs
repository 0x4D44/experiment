use crate::core::{CellState, Grid, Topology};

/// A puzzle level
#[derive(Debug, Clone)]
pub struct Level {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub width: usize,
    pub height: usize,
    pub initial_state: Vec<CellState>,
    pub target_state: Vec<CellState>,
    pub topology: Topology,
}

impl Level {
    /// Create a level where goal is to get all cells to 0
    pub fn new_all_zeros(
        id: usize,
        name: impl Into<String>,
        description: impl Into<String>,
        width: usize,
        height: usize,
        initial_state: Vec<CellState>,
        topology: Topology,
    ) -> Self {
        let target_state = vec![CellState::new(0); width * height];
        Level {
            id,
            name: name.into(),
            description: description.into(),
            width,
            height,
            initial_state,
            target_state,
            topology,
        }
    }

    /// Create a grid from this level
    pub fn create_grid(&self) -> Grid {
        Grid::from_state(
            self.width,
            self.height,
            self.initial_state.clone(),
            self.topology.clone(),
        )
    }
}

/// Collection of levels
pub struct LevelPack {
    levels: Vec<Level>,
}

impl LevelPack {
    pub fn new(levels: Vec<Level>) -> Self {
        LevelPack { levels }
    }

    pub fn get_level(&self, id: usize) -> Option<&Level> {
        self.levels.iter().find(|level| level.id == id)
    }

    pub fn len(&self) -> usize {
        self.levels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }

    /// Create the main campaign with all levels
    pub fn main_campaign() -> Self {
        let mut levels = Vec::new();

        // Level 1: Tutorial - 2x2 single click solution
        levels.push(Level::new_all_zeros(
            0,
            "First Steps",
            "Click the lit cell to turn it off. Simple!",
            2,
            2,
            vec![
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(0),
            ],
            Topology::Rectangular,
        ));

        // Level 2: Understanding neighbors - 2x2
        levels.push(Level::new_all_zeros(
            1,
            "Ripple Effect",
            "Clicking affects neighbors too!",
            2,
            2,
            vec![
                CellState::new(0),
                CellState::new(1),
                CellState::new(1),
                CellState::new(0),
            ],
            Topology::Rectangular,
        ));

        // Level 3: 2x2 requires thinking
        levels.push(Level::new_all_zeros(
            2,
            "Think Ahead",
            "Two clicks needed. Which ones?",
            2,
            2,
            vec![
                CellState::new(2),
                CellState::new(1),
                CellState::new(1),
                CellState::new(2),
            ],
            Topology::Rectangular,
        ));

        // Level 4: First 3x3
        levels.push(Level::new_all_zeros(
            3,
            "Bigger Picture",
            "Welcome to 3x3. Corner pieces are key.",
            3,
            3,
            vec![
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
            ],
            Topology::Rectangular,
        ));

        // Level 5: 3x3 cross pattern
        levels.push(Level::new_all_zeros(
            4,
            "The Cross",
            "A classic pattern emerges.",
            3,
            3,
            vec![
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(1),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
            ],
            Topology::Rectangular,
        ));

        // Level 6: 3x3 all lit
        levels.push(Level::new_all_zeros(
            5,
            "Full House",
            "Everything is lit. Find the pattern.",
            3,
            3,
            vec![
                CellState::new(1),
                CellState::new(1),
                CellState::new(1),
                CellState::new(1),
                CellState::new(1),
                CellState::new(1),
                CellState::new(1),
                CellState::new(1),
                CellState::new(1),
            ],
            Topology::Rectangular,
        ));

        // Level 7: 3x3 with 2s and 3s
        levels.push(Level::new_all_zeros(
            6,
            "Higher Numbers",
            "States go beyond 1. Count carefully!",
            3,
            3,
            vec![
                CellState::new(2),
                CellState::new(0),
                CellState::new(2),
                CellState::new(0),
                CellState::new(3),
                CellState::new(0),
                CellState::new(2),
                CellState::new(0),
                CellState::new(2),
            ],
            Topology::Rectangular,
        ));

        // Level 8: 4x4 corners
        levels.push(Level::new_all_zeros(
            7,
            "Four Corners",
            "4x4 grid. Corner strategy still works.",
            4,
            4,
            vec![
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(0),
                CellState::new(0),
                CellState::new(0),
                CellState::new(0),
                CellState::new(0),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(1),
            ],
            Topology::Rectangular,
        ));

        // Level 9: 4x4 checkerboard
        levels.push(Level::new_all_zeros(
            8,
            "Checkerboard",
            "An alternating pattern to solve.",
            4,
            4,
            vec![
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
            ],
            Topology::Rectangular,
        ));

        // Level 10: 4x4 complex
        levels.push(Level::new_all_zeros(
            9,
            "The Challenge",
            "A puzzle that tests everything you've learned.",
            4,
            4,
            vec![
                CellState::new(2),
                CellState::new(1),
                CellState::new(1),
                CellState::new(2),
                CellState::new(1),
                CellState::new(2),
                CellState::new(2),
                CellState::new(1),
                CellState::new(1),
                CellState::new(2),
                CellState::new(2),
                CellState::new(1),
                CellState::new(2),
                CellState::new(1),
                CellState::new(1),
                CellState::new(2),
            ],
            Topology::Rectangular,
        ));

        // Level 11: 3x3 with diagonals
        levels.push(Level::new_all_zeros(
            10,
            "Diagonal Thinking",
            "Now diagonals count as neighbors too!",
            3,
            3,
            vec![
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
            ],
            Topology::RectangularWithDiagonals,
        ));

        // Level 12: 4x4 with diagonals
        levels.push(Level::new_all_zeros(
            11,
            "Diagonal Spread",
            "Diagonals make everything more complex.",
            4,
            4,
            vec![
                CellState::new(0),
                CellState::new(1),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(1),
                CellState::new(1),
                CellState::new(0),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(1),
                CellState::new(0),
            ],
            Topology::RectangularWithDiagonals,
        ));

        // Level 13: 5x5 moderate
        levels.push(Level::new_all_zeros(
            12,
            "Expanding Mind",
            "5x5 grid. Time to think bigger.",
            5,
            5,
            vec![
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(2),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
                CellState::new(0),
                CellState::new(1),
            ],
            Topology::Rectangular,
        ));

        // Level 14: 5x5 with higher numbers
        levels.push(Level::new_all_zeros(
            13,
            "Maximum Flux",
            "All four states in play.",
            5,
            5,
            vec![
                CellState::new(3),
                CellState::new(1),
                CellState::new(2),
                CellState::new(1),
                CellState::new(3),
                CellState::new(1),
                CellState::new(2),
                CellState::new(3),
                CellState::new(2),
                CellState::new(1),
                CellState::new(2),
                CellState::new(3),
                CellState::new(1),
                CellState::new(3),
                CellState::new(2),
                CellState::new(1),
                CellState::new(2),
                CellState::new(3),
                CellState::new(2),
                CellState::new(1),
                CellState::new(3),
                CellState::new(1),
                CellState::new(2),
                CellState::new(1),
                CellState::new(3),
            ],
            Topology::Rectangular,
        ));

        // Level 15: Final challenge - 5x5 with diagonals
        levels.push(Level::new_all_zeros(
            14,
            "The Gauntlet",
            "The ultimate test. Good luck!",
            5,
            5,
            vec![
                CellState::new(2),
                CellState::new(1),
                CellState::new(3),
                CellState::new(1),
                CellState::new(2),
                CellState::new(1),
                CellState::new(3),
                CellState::new(2),
                CellState::new(3),
                CellState::new(1),
                CellState::new(3),
                CellState::new(2),
                CellState::new(1),
                CellState::new(2),
                CellState::new(3),
                CellState::new(1),
                CellState::new(3),
                CellState::new(2),
                CellState::new(3),
                CellState::new(1),
                CellState::new(2),
                CellState::new(1),
                CellState::new(3),
                CellState::new(1),
                CellState::new(2),
            ],
            Topology::RectangularWithDiagonals,
        ));

        LevelPack::new(levels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_pack_creation() {
        let pack = LevelPack::main_campaign();
        assert_eq!(pack.len(), 15);
    }

    #[test]
    fn test_get_level() {
        let pack = LevelPack::main_campaign();
        let level = pack.get_level(0).unwrap();
        assert_eq!(level.name, "First Steps");
        assert_eq!(level.width, 2);
        assert_eq!(level.height, 2);
    }

    #[test]
    fn test_create_grid_from_level() {
        let pack = LevelPack::main_campaign();
        let level = pack.get_level(0).unwrap();
        let grid = level.create_grid();
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 2);
    }
}
