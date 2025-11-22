use crate::maze::Maze;
use crate::visualization::renderer::MazeRenderer;
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

pub struct MazeAnimator {
    renderer: MazeRenderer,
    delay_ms: u64,
}

impl MazeAnimator {
    pub fn new(delay_ms: u64) -> Self {
        Self {
            renderer: MazeRenderer::new(),
            delay_ms,
        }
    }

    pub fn animate_solution(
        &self,
        maze: &Maze,
        path: &[(usize, usize)],
        visited: &HashSet<(usize, usize)>,
    ) -> std::io::Result<()> {
        // Clear screen and hide cursor
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(ClearType::All))?;
        stdout.execute(cursor::Hide)?;

        // Animate visited cells
        let mut current_visited = HashSet::new();
        for &cell in visited.iter() {
            current_visited.insert(cell);

            // Clear and redraw
            stdout.execute(cursor::MoveTo(0, 0))?;
            let frame = self.renderer.render_with_highlights(
                maze,
                &current_visited,
                &HashSet::new(),
            );
            print!("{}", frame);
            stdout.flush()?;

            thread::sleep(Duration::from_millis(self.delay_ms));
        }

        // Animate solution path
        let mut current_solution = HashSet::new();
        for &cell in path.iter() {
            current_solution.insert(cell);

            stdout.execute(cursor::MoveTo(0, 0))?;
            let frame = self.renderer.render_with_highlights(
                maze,
                visited,
                &current_solution,
            );
            print!("{}", frame);
            stdout.flush()?;

            thread::sleep(Duration::from_millis(self.delay_ms * 2));
        }

        // Show cursor again
        stdout.execute(cursor::Show)?;
        stdout.flush()?;

        Ok(())
    }

    pub fn animate_generation(
        &self,
        maze: &Maze,
        steps: &[HashSet<(usize, usize)>],
    ) -> std::io::Result<()> {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(ClearType::All))?;
        stdout.execute(cursor::Hide)?;

        for step in steps {
            stdout.execute(cursor::MoveTo(0, 0))?;
            let frame = self.renderer.render_with_highlights(
                maze,
                step,
                &HashSet::new(),
            );
            print!("{}", frame);
            stdout.flush()?;

            thread::sleep(Duration::from_millis(self.delay_ms));
        }

        stdout.execute(cursor::Show)?;
        stdout.flush()?;

        Ok(())
    }

    pub fn show_static(&self, maze: &Maze) -> std::io::Result<()> {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(ClearType::All))?;

        let output = self.renderer.render(maze);
        println!("{}", output);

        stdout.flush()?;
        Ok(())
    }
}

impl Default for MazeAnimator {
    fn default() -> Self {
        Self::new(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animator_creation() {
        let animator = MazeAnimator::new(100);
        assert_eq!(animator.delay_ms, 100);
    }

    #[test]
    fn test_default_animator() {
        let animator = MazeAnimator::default();
        assert_eq!(animator.delay_ms, 50);
    }
}
