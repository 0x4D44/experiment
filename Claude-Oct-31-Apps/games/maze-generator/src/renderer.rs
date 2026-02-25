use crate::maze::{Maze, Direction};
use std::collections::HashSet;

/// Renders maze to ASCII art
pub struct Renderer;

impl Renderer {
    /// Render maze with all features
    pub fn render(
        maze: &Maze,
        player_pos: (usize, usize),
        visited_cells: &HashSet<(usize, usize)>,
        solution: Option<&Vec<(usize, usize)>>,
        show_breadcrumbs: bool,
    ) -> String {
        let mut output = String::new();

        // Top border
        output.push_str("┌");
        for x in 0..maze.width {
            if x < maze.width - 1 {
                output.push_str("───┬");
            } else {
                output.push_str("───┐\n");
            }
        }

        // Cells and walls
        for y in 0..maze.height {
            // Vertical walls and cell contents
            output.push('│');

            for x in 0..maze.width {
                let cell = &maze.cells[y * maze.width + x];

                // Cell content - priority: player > solution > start/end > visited > empty
                if (x, y) == player_pos {
                    output.push('@');
                } else if let Some(sol) = solution {
                    if sol.contains(&(x, y)) {
                        if (x, y) == maze.start {
                            output.push('S'); // Show start marker even in solution
                        } else if (x, y) == maze.end {
                            output.push('E'); // Show end marker even in solution
                        } else {
                            output.push('*');
                        }
                    } else if (x, y) == maze.start {
                        output.push('S');
                    } else if (x, y) == maze.end {
                        output.push('E');
                    } else if show_breadcrumbs && visited_cells.contains(&(x, y)) {
                        output.push('·');
                    } else {
                        output.push(' ');
                    }
                } else if (x, y) == maze.start {
                    output.push('S');
                } else if (x, y) == maze.end {
                    output.push('E');
                } else if show_breadcrumbs && visited_cells.contains(&(x, y)) {
                    output.push('·');
                } else {
                    output.push(' ');
                }

                // Right wall
                if x < maze.width - 1 {
                    if cell.walls[Direction::East as usize] {
                        output.push('│');
                    } else {
                        output.push(' ');
                    }
                } else {
                    output.push('│');
                }
            }

            output.push('\n');

            // Horizontal walls and intersections
            if y < maze.height - 1 {
                output.push('├');
                for x in 0..maze.width {
                    let cell = &maze.cells[y * maze.width + x];

                    // Bottom wall
                    if cell.walls[Direction::South as usize] {
                        output.push_str("───");
                    } else {
                        output.push_str("   ");
                    }

                    if x < maze.width - 1 {
                        output.push('┼');
                    } else {
                        output.push('┤');
                    }
                }
                output.push('\n');
            }
        }

        // Bottom border
        output.push_str("└");
        for x in 0..maze.width {
            if x < maze.width - 1 {
                output.push_str("───┴");
            } else {
                output.push_str("───┘\n");
            }
        }

        output
    }

    /// Render simple maze without overlays
    pub fn render_simple(maze: &Maze) -> String {
        let empty_set = HashSet::new();
        Self::render(maze, maze.start, &empty_set, None, false)
    }

    /// Render maze with solution
    pub fn render_with_solution(maze: &Maze, solution: &Vec<(usize, usize)>) -> String {
        let empty_set = HashSet::new();
        Self::render(maze, maze.start, &empty_set, Some(solution), false)
    }

    /// Render maze during gameplay
    pub fn render_gameplay(
        maze: &Maze,
        player_pos: (usize, usize),
        visited_cells: &HashSet<(usize, usize)>,
    ) -> String {
        Self::render(maze, player_pos, visited_cells, None, true)
    }

    /// Create status line
    pub fn status_line(
        width: usize,
        height: usize,
        steps: usize,
        elapsed_secs: u64,
        hints_used: usize,
    ) -> String {
        let minutes = elapsed_secs / 60;
        let seconds = elapsed_secs % 60;
        format!(
            "Size: {}x{}  Steps: {}  Time: {:02}:{:02}  Hints: {}",
            width, height, steps, minutes, seconds, hints_used
        )
    }

    /// Create header line
    pub fn header_line(maze_name: &str) -> String {
        format!("╔═══════════════════════════╗\n║ {} ║\n╚═══════════════════════════╝\n", maze_name)
    }

    /// Create control hints
    pub fn controls_line() -> String {
        "Controls: ↑↓←→/WASD Move | H Hint | S Show Solution | R Reset | Q Quit".to_string()
    }

    /// Estimate rendered size
    pub fn estimate_render_size(width: usize, height: usize) -> (usize, usize) {
        let render_width = width * 4 + 1;
        let render_height = height * 2 + 3;
        (render_width, render_height)
    }

    /// Check if maze fits in terminal
    pub fn fits_in_terminal(width: usize, height: usize, term_width: usize, term_height: usize) -> bool {
        let (rw, rh) = Self::estimate_render_size(width, height);
        rw <= term_width && rh <= term_height
    }

    /// Render compact maze for large mazes (viewport-based)
    pub fn render_viewport(
        maze: &Maze,
        player_pos: (usize, usize),
        viewport_width: usize,
        viewport_height: usize,
    ) -> String {
        let mut output = String::new();

        // Calculate viewport bounds centered on player
        let start_x = player_pos.0.saturating_sub(viewport_width / 2);
        let start_y = player_pos.1.saturating_sub(viewport_height / 2);
        let end_x = std::cmp::min(start_x + viewport_width, maze.width);
        let end_y = std::cmp::min(start_y + viewport_height, maze.height);

        // Top border
        output.push_str("┌");
        for _ in start_x..end_x {
            output.push_str("───┬");
        }
        output.push_str("┐\n");

        // Cells
        for y in start_y..end_y {
            output.push('│');

            for x in start_x..end_x {
                let cell = &maze.cells[y * maze.width + x];

                // Cell content
                if (x, y) == player_pos {
                    output.push('@');
                } else if (x, y) == maze.start {
                    output.push('S');
                } else if (x, y) == maze.end {
                    output.push('E');
                } else {
                    output.push(' ');
                }

                // Right wall
                if x < end_x - 1 {
                    if cell.walls[Direction::East as usize] {
                        output.push('│');
                    } else {
                        output.push(' ');
                    }
                } else {
                    output.push('│');
                }
            }

            output.push('\n');

            // Horizontal walls
            if y < end_y - 1 {
                output.push('├');
                for x in start_x..end_x {
                    let cell = &maze.cells[y * maze.width + x];

                    if cell.walls[Direction::South as usize] {
                        output.push_str("───");
                    } else {
                        output.push_str("   ");
                    }

                    if x < end_x - 1 {
                        output.push('┼');
                    } else {
                        output.push('┤');
                    }
                }
                output.push('\n');
            }
        }

        // Bottom border
        output.push_str("└");
        for _ in start_x..end_x {
            output.push_str("───┴");
        }
        output.push_str("┘\n");

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::RecursiveBacktracker;
    use crate::MazeGenerator;

    #[test]
    fn test_render_simple() {
        let mut gen = RecursiveBacktracker::new(5, 5, 42);
        let maze = gen.generate();

        let rendered = Renderer::render_simple(&maze);
        assert!(!rendered.is_empty());
        assert!(rendered.contains("│"));
        assert!(rendered.contains("─"));
    }

    #[test]
    fn test_render_with_solution() {
        let mut gen = RecursiveBacktracker::new(5, 5, 42);
        let maze = gen.generate();

        // Get an actual solution path
        if let Some(solution) = crate::pathfinder::Pathfinder::bfs(&maze) {
            let rendered = Renderer::render_with_solution(&maze, &solution);
            assert!(!rendered.is_empty());
            // Solution should have at least one non-start/end cell marked with *
            // Or start/end should be present
            assert!(rendered.contains("S") || rendered.contains("E") || rendered.contains("*"));
        }
    }

    #[test]
    fn test_render_gameplay() {
        let mut gen = RecursiveBacktracker::new(5, 5, 42);
        let maze = gen.generate();

        let visited = HashSet::new();
        let rendered = Renderer::render_gameplay(&maze, maze.start, &visited);
        assert!(!rendered.is_empty());
        assert!(rendered.contains("@"));
    }

    #[test]
    fn test_status_line() {
        let status = Renderer::status_line(10, 10, 50, 125, 2);
        assert!(status.contains("10x10"));
        assert!(status.contains("50"));
        assert!(status.contains("02:05"));
        assert!(status.contains("2"));
    }

    #[test]
    fn test_estimate_render_size() {
        let (w, h) = Renderer::estimate_render_size(5, 5);
        assert!(w > 0);
        assert!(h > 0);
    }

    #[test]
    fn test_fits_in_terminal() {
        assert!(Renderer::fits_in_terminal(5, 5, 100, 50));
        assert!(!Renderer::fits_in_terminal(100, 100, 50, 30));
    }
}
