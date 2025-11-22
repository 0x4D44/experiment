use crate::maze::Maze;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct MazeFile {
    version: String,
    maze: Maze,
    metadata: MazeMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct MazeMetadata {
    generated_algorithm: Option<String>,
    created_at: String,
}

pub fn save_maze(maze: &Maze, path: &Path, algorithm: Option<&str>) -> io::Result<()> {
    let maze_file = MazeFile {
        version: "1.0".to_string(),
        maze: maze.clone(),
        metadata: MazeMetadata {
            generated_algorithm: algorithm.map(|s| s.to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
        },
    };

    let json = serde_json::to_string_pretty(&maze_file)?;
    fs::write(path, json)?;

    Ok(())
}

pub fn load_maze(path: &Path) -> io::Result<Maze> {
    let contents = fs::read_to_string(path)?;
    let maze_file: MazeFile = serde_json::from_str(&contents)?;

    Ok(maze_file.maze)
}

pub fn export_maze_as_text(maze: &Maze, path: &Path) -> io::Result<()> {
    use crate::visualization::MazeRenderer;

    let renderer = MazeRenderer::new();
    let output = renderer.render(maze);

    fs::write(path, output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::{GeneratorAlgorithm, MazeGenerator};
    use std::fs;

    #[test]
    fn test_save_and_load_maze() {
        let maze = MazeGenerator::generate(10, 10, GeneratorAlgorithm::RecursiveBacktracker);
        let temp_path = std::env::temp_dir().join("test_maze.json");

        save_maze(&maze, &temp_path, Some("RecursiveBacktracker")).unwrap();
        assert!(temp_path.exists());

        let loaded_maze = load_maze(&temp_path).unwrap();
        assert_eq!(loaded_maze.width, maze.width);
        assert_eq!(loaded_maze.height, maze.height);

        fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_export_as_text() {
        let maze = Maze::new(5, 5);
        let temp_path = std::env::temp_dir().join("test_maze.txt");

        export_maze_as_text(&maze, &temp_path).unwrap();
        assert!(temp_path.exists());

        let contents = fs::read_to_string(&temp_path).unwrap();
        assert!(!contents.is_empty());

        fs::remove_file(temp_path).ok();
    }
}
