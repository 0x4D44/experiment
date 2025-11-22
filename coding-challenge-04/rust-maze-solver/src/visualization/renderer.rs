use crate::maze::{CellType, Maze};
use colored::Colorize;
use std::collections::HashSet;

pub struct MazeRenderer {
    pub use_unicode: bool,
    pub show_grid: bool,
}

impl MazeRenderer {
    pub fn new() -> Self {
        Self {
            use_unicode: true,
            show_grid: true,
        }
    }

    pub fn render(&self, maze: &Maze) -> String {
        self.render_with_highlights(maze, &HashSet::new(), &HashSet::new())
    }

    pub fn render_with_highlights(
        &self,
        maze: &Maze,
        visited: &HashSet<(usize, usize)>,
        solution: &HashSet<(usize, usize)>,
    ) -> String {
        let mut output = String::new();

        // Top border
        output.push_str(&self.render_top_border(maze.width));
        output.push('\n');

        // Render maze rows
        for row in 0..maze.height {
            output.push_str(&self.render_row(maze, row, visited, solution));
            output.push('\n');
        }

        // Bottom border
        output.push_str(&self.render_bottom_border(maze.width));
        output.push('\n');

        output
    }

    fn render_row(
        &self,
        maze: &Maze,
        row: usize,
        visited: &HashSet<(usize, usize)>,
        solution: &HashSet<(usize, usize)>,
    ) -> String {
        let mut output = String::new();

        // Left border
        if self.use_unicode {
            output.push('│');
        } else {
            output.push('|');
        }

        // Cells
        for col in 0..maze.width {
            let cell = maze.get(row, col).unwrap();
            let symbol = self.get_cell_symbol(
                cell.cell_type,
                (row, col),
                visited,
                solution,
            );
            output.push_str(&symbol);

            // Cell spacing
            if self.show_grid && col < maze.width - 1 {
                output.push(' ');
            }
        }

        // Right border
        if self.use_unicode {
            output.push('│');
        } else {
            output.push('|');
        }

        output
    }

    fn get_cell_symbol(
        &self,
        cell_type: CellType,
        position: (usize, usize),
        visited: &HashSet<(usize, usize)>,
        solution: &HashSet<(usize, usize)>,
    ) -> String {
        if solution.contains(&position) {
            return if self.use_unicode {
                "●".bright_yellow().to_string()
            } else {
                "o".bright_yellow().to_string()
            };
        }

        if visited.contains(&position) {
            return if self.use_unicode {
                "·".bright_blue().to_string()
            } else {
                ".".bright_blue().to_string()
            };
        }

        match cell_type {
            CellType::Wall => {
                if self.use_unicode {
                    "█".red().to_string()
                } else {
                    "#".red().to_string()
                }
            }
            CellType::Path => " ".to_string(),
            CellType::Start => "S".bright_green().bold().to_string(),
            CellType::End => "E".bright_red().bold().to_string(),
            CellType::Visited => {
                if self.use_unicode {
                    "·".bright_blue().to_string()
                } else {
                    ".".bright_blue().to_string()
                }
            }
            CellType::Solution => {
                if self.use_unicode {
                    "●".bright_yellow().to_string()
                } else {
                    "o".bright_yellow().to_string()
                }
            }
        }
    }

    fn render_top_border(&self, width: usize) -> String {
        if self.use_unicode {
            format!("┌{}┐", "─".repeat(if self.show_grid { width * 2 - 1 } else { width }))
        } else {
            format!("+{}+", "-".repeat(if self.show_grid { width * 2 - 1 } else { width }))
        }
    }

    fn render_bottom_border(&self, width: usize) -> String {
        if self.use_unicode {
            format!("└{}┘", "─".repeat(if self.show_grid { width * 2 - 1 } else { width }))
        } else {
            format!("+{}+", "-".repeat(if self.show_grid { width * 2 - 1 } else { width }))
        }
    }

    pub fn render_statistics(&self, stats: &SolutionStats) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{}\n", "=== Solution Statistics ===".bright_cyan().bold()));
        output.push_str(&format!("  Algorithm: {}\n", stats.algorithm.bright_white()));
        output.push_str(&format!("  Path Length: {}\n", stats.path_length.to_string().bright_yellow()));
        output.push_str(&format!("  Nodes Explored: {}\n", stats.nodes_explored.to_string().bright_blue()));

        if let Some(time) = stats.solve_time {
            output.push_str(&format!("  Time Taken: {}\n", format!("{:.2}ms", time).bright_magenta()));
        }

        let efficiency = if stats.nodes_explored > 0 {
            (stats.path_length as f64 / stats.nodes_explored as f64) * 100.0
        } else {
            0.0
        };
        output.push_str(&format!("  Efficiency: {:.2}%\n", efficiency.to_string().bright_green()));

        output
    }
}

impl Default for MazeRenderer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SolutionStats {
    pub algorithm: String,
    pub path_length: usize,
    pub nodes_explored: usize,
    pub solve_time: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = MazeRenderer::new();
        assert!(renderer.use_unicode);
        assert!(renderer.show_grid);
    }

    #[test]
    fn test_render_small_maze() {
        let maze = Maze::new(5, 5);
        let renderer = MazeRenderer::new();
        let output = renderer.render(&maze);

        assert!(!output.is_empty());
        assert!(output.contains('│') || output.contains('|'));
    }

    #[test]
    fn test_statistics_rendering() {
        let renderer = MazeRenderer::new();
        let stats = SolutionStats {
            algorithm: "A*".to_string(),
            path_length: 42,
            nodes_explored: 100,
            solve_time: Some(15.5),
        };

        let output = renderer.render_statistics(&stats);
        assert!(output.contains("A*"));
        assert!(output.contains("42"));
        assert!(output.contains("100"));
    }
}
