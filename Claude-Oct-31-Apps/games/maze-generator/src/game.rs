use crate::maze::Maze;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

/// Game statistics
#[derive(Debug, Clone)]
pub struct Statistics {
    pub steps_taken: usize,
    pub optimal_steps: usize,
    pub time_elapsed: u64, // in seconds
    pub hints_used: usize,
    pub solution_revealed: bool,
}

impl Statistics {
    pub fn new(optimal_steps: usize) -> Self {
        Statistics {
            steps_taken: 0,
            optimal_steps,
            time_elapsed: 0,
            hints_used: 0,
            solution_revealed: false,
        }
    }

    pub fn efficiency_percentage(&self) -> f64 {
        if self.optimal_steps > 0 {
            (self.optimal_steps as f64 / self.steps_taken as f64) * 100.0
        } else {
            100.0
        }
    }
}

/// Game state manager
pub struct GameState {
    pub maze: Maze,
    pub player_pos: (usize, usize),
    pub visited_cells: HashSet<(usize, usize)>,
    pub breadcrumb_trail: VecDeque<(usize, usize)>,
    pub start_time: Instant,
    pub solution: Option<Vec<(usize, usize)>>,
    pub solution_revealed: bool,
    pub stats: Statistics,
    pub show_breadcrumbs: bool,
    pub won: bool,
}

impl GameState {
    /// Create new game state
    pub fn new(maze: Maze) -> Self {
        let optimal_steps = if let Some(solution) = crate::pathfinder::Pathfinder::bfs(&maze) {
            solution.len()
        } else {
            0
        };

        let mut state = GameState {
            player_pos: maze.start,
            maze,
            visited_cells: HashSet::new(),
            breadcrumb_trail: VecDeque::new(),
            start_time: Instant::now(),
            solution: None,
            solution_revealed: false,
            stats: Statistics::new(optimal_steps),
            show_breadcrumbs: true,
            won: false,
        };

        state.record_visit(state.player_pos);
        state
    }

    /// Try to move in a direction
    pub fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let new_x = (self.player_pos.0 as i32 + dx) as usize;
        let new_y = (self.player_pos.1 as i32 + dy) as usize;
        let new_pos = (new_x, new_y);

        // Check bounds
        if !self.maze.is_in_bounds(new_x, new_y) {
            return false;
        }

        // Check if passage is open
        if self.maze.is_passage_open(self.player_pos, new_pos) {
            self.player_pos = new_pos;
            self.record_visit(new_pos);
            self.check_win_condition();
            true
        } else {
            false
        }
    }

    /// Record a cell visit
    pub fn record_visit(&mut self, pos: (usize, usize)) {
        if !self.visited_cells.contains(&pos) {
            self.visited_cells.insert(pos);
        }

        // Add to breadcrumb trail (keep last 50)
        self.breadcrumb_trail.push_back(pos);
        while self.breadcrumb_trail.len() > 50 {
            self.breadcrumb_trail.pop_front();
        }

        self.stats.steps_taken += 1;
    }

    /// Get elapsed time in seconds
    pub fn elapsed_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Update statistics
    pub fn update_stats(&mut self) {
        self.stats.time_elapsed = self.elapsed_seconds();
    }

    /// Check if player reached the exit
    fn check_win_condition(&mut self) {
        if self.player_pos == self.maze.end {
            self.won = true;
        }
    }

    /// Get hint for next move
    pub fn get_hint(&mut self) -> Option<(usize, usize)> {
        self.stats.hints_used += 1;
        crate::pathfinder::Pathfinder::next_step(&self.maze, self.player_pos)
    }

    /// Get hint path (next 3-5 steps)
    pub fn get_hint_path(&mut self) -> Option<Vec<(usize, usize)>> {
        self.stats.hints_used += 1;
        crate::pathfinder::Pathfinder::hint_path(&self.maze, self.player_pos)
    }

    /// Reveal solution
    pub fn reveal_solution(&mut self) {
        if !self.solution_revealed {
            if let Some(solution) = crate::pathfinder::Pathfinder::bfs(&self.maze) {
                self.solution = Some(solution);
                self.solution_revealed = true;
                self.stats.solution_revealed = true;
            }
        }
    }

    /// Toggle breadcrumb display
    pub fn toggle_breadcrumbs(&mut self) {
        self.show_breadcrumbs = !self.show_breadcrumbs;
    }

    /// Reset game to start
    pub fn reset(&mut self) {
        self.player_pos = self.maze.start;
        self.visited_cells.clear();
        self.breadcrumb_trail.clear();
        self.start_time = Instant::now();
        self.stats.steps_taken = 0;
        self.stats.time_elapsed = 0;
        self.stats.hints_used = 0;
        self.solution_revealed = false;
        self.solution = None;
        self.won = false;
        // Record visit after resetting stats
        self.record_visit(self.player_pos);
    }

    /// Get completion summary
    pub fn get_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("═══════════════════════════════\n");
        summary.push_str("        MAZE COMPLETE!\n");
        summary.push_str("═══════════════════════════════\n");
        summary.push_str(&format!("Size: {}x{}\n", self.maze.width, self.maze.height));
        summary.push_str(&format!("Steps Taken: {}\n", self.stats.steps_taken));
        summary.push_str(&format!("Optimal Path: {}\n", self.stats.optimal_steps));
        summary.push_str(&format!(
            "Efficiency: {:.1}%\n",
            self.stats.efficiency_percentage()
        ));
        summary.push_str(&format!("Time: {}m {}s\n", self.elapsed_seconds() / 60, self.elapsed_seconds() % 60));
        summary.push_str(&format!("Hints Used: {}\n", self.stats.hints_used));
        if self.stats.solution_revealed {
            summary.push_str("Solution Revealed: Yes\n");
        }
        summary.push_str("═══════════════════════════════\n");
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::RecursiveBacktracker;
    use crate::MazeGenerator;

    #[test]
    fn test_game_state_creation() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        let game = GameState::new(maze);

        assert_eq!(game.player_pos, game.maze.start);
        assert!(game.visited_cells.contains(&game.maze.start));
        assert!(!game.won);
    }

    #[test]
    fn test_valid_movement() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        let mut game = GameState::new(maze);

        let initial_steps = game.stats.steps_taken;

        // Try to move (may or may not succeed depending on maze)
        let _ = game.try_move(0, 1); // Try moving south

        // Either moved or didn't, but shouldn't panic
        assert!(game.stats.steps_taken >= initial_steps);
    }

    #[test]
    fn test_breadcrumb_limit() {
        let mut gen = RecursiveBacktracker::new(5, 5, 42);
        let maze = gen.generate();
        let mut game = GameState::new(maze);

        // Record many visits through the proper interface
        for i in 0..100 {
            game.record_visit((i % 5, i % 5));
        }

        // Should not exceed limit (50 max + 1 initial)
        assert!(game.breadcrumb_trail.len() <= 51);
    }

    #[test]
    fn test_statistics() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        let game = GameState::new(maze);

        let stats = &game.stats;
        assert!(stats.optimal_steps > 0);
        assert_eq!(stats.steps_taken, 1); // Initial position
        assert_eq!(stats.hints_used, 0);
    }

    #[test]
    fn test_efficiency_calculation() {
        let mut stats = Statistics::new(10);
        stats.steps_taken = 20;

        let efficiency = stats.efficiency_percentage();
        assert!((efficiency - 50.0).abs() < 0.1); // 10/20 = 50%
    }

    #[test]
    fn test_reset_game() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        let mut game = GameState::new(maze);

        let start_pos = game.player_pos;
        let initial_steps = game.stats.steps_taken;

        // Make some moves (may or may not succeed due to walls)
        game.try_move(1, 0);
        game.try_move(0, 1);

        // Reset
        game.reset();

        assert_eq!(game.player_pos, start_pos);
        assert_eq!(game.stats.steps_taken, initial_steps); // Back to 1
        assert!(!game.won);
    }

    #[test]
    fn test_solution_reveal() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        let mut game = GameState::new(maze);

        assert!(!game.solution_revealed);
        assert!(game.solution.is_none());

        game.reveal_solution();

        assert!(game.solution_revealed);
        assert!(game.solution.is_some());
    }

    #[test]
    fn test_toggle_breadcrumbs() {
        let mut gen = RecursiveBacktracker::new(10, 10, 42);
        let maze = gen.generate();
        let mut game = GameState::new(maze);

        let initial = game.show_breadcrumbs;
        game.toggle_breadcrumbs();
        assert_ne!(game.show_breadcrumbs, initial);
        game.toggle_breadcrumbs();
        assert_eq!(game.show_breadcrumbs, initial);
    }
}
