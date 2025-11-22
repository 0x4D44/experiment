use rust_maze_solver::{
    algorithms::{GeneratorAlgorithm, MazeGenerator, MazeSolver, PathfindingAlgorithm},
    io,
};
use std::fs;

#[test]
fn test_full_workflow_generate_and_solve() {
    // Generate a maze
    let maze = MazeGenerator::generate(20, 20, GeneratorAlgorithm::RecursiveBacktracker);

    assert_eq!(maze.width, 20);
    assert_eq!(maze.height, 20);

    // Solve the maze
    let result = MazeSolver::solve(&maze, PathfindingAlgorithm::AStar);
    assert!(result.is_some());

    let solution = result.unwrap();
    assert!(solution.path_length > 0);
    assert!(solution.nodes_explored > 0);
    assert_eq!(solution.path.first().unwrap(), &maze.start);
    assert_eq!(solution.path.last().unwrap(), &maze.end);
}

#[test]
fn test_all_generation_algorithms() {
    let algorithms = vec![
        GeneratorAlgorithm::RecursiveBacktracker,
        GeneratorAlgorithm::Prims,
        GeneratorAlgorithm::Kruskals,
        GeneratorAlgorithm::AldousBroder,
    ];

    for algo in algorithms {
        let maze = MazeGenerator::generate(15, 15, algo);
        assert_eq!(maze.width, 15);
        assert_eq!(maze.height, 15);

        // Verify maze can be solved
        let result = MazeSolver::solve(&maze, PathfindingAlgorithm::AStar);
        assert!(result.is_some(), "Maze generated with {:?} should be solvable", algo);
    }
}

#[test]
fn test_all_solving_algorithms() {
    let maze = MazeGenerator::generate(15, 15, GeneratorAlgorithm::RecursiveBacktracker);

    let algorithms = vec![
        PathfindingAlgorithm::AStar,
        PathfindingAlgorithm::BFS,
        PathfindingAlgorithm::DFS,
        PathfindingAlgorithm::Dijkstra,
    ];

    for algo in algorithms {
        let result = MazeSolver::solve(&maze, algo);
        assert!(result.is_some(), "{:?} should find a solution", algo);

        let solution = result.unwrap();
        assert!(solution.path_length > 0);
        assert_eq!(solution.path.first().unwrap(), &maze.start);
        assert_eq!(solution.path.last().unwrap(), &maze.end);
    }
}

#[test]
fn test_save_and_load_maze() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_maze_integration.json");

    // Generate and save
    let original_maze = MazeGenerator::generate(10, 10, GeneratorAlgorithm::Prims);
    io::save_maze(&original_maze, &file_path, Some("Prims")).unwrap();

    // Load and verify
    let loaded_maze = io::load_maze(&file_path).unwrap();
    assert_eq!(loaded_maze.width, original_maze.width);
    assert_eq!(loaded_maze.height, original_maze.height);
    assert_eq!(loaded_maze.start, original_maze.start);
    assert_eq!(loaded_maze.end, original_maze.end);

    // Cleanup
    fs::remove_file(file_path).ok();
}

#[test]
fn test_optimal_algorithms_find_shortest_path() {
    let maze = MazeGenerator::generate(20, 20, GeneratorAlgorithm::RecursiveBacktracker);

    // A* and BFS should find the same path length (both optimal)
    let astar_result = MazeSolver::solve(&maze, PathfindingAlgorithm::AStar).unwrap();
    let bfs_result = MazeSolver::solve(&maze, PathfindingAlgorithm::BFS).unwrap();

    assert_eq!(
        astar_result.path_length, bfs_result.path_length,
        "A* and BFS should find paths of equal length"
    );
}

#[test]
fn test_large_maze_generation_and_solving() {
    let maze = MazeGenerator::generate(50, 50, GeneratorAlgorithm::Kruskals);

    assert_eq!(maze.width, 50);
    assert_eq!(maze.height, 50);

    // Should still be solvable
    let result = MazeSolver::solve(&maze, PathfindingAlgorithm::AStar);
    assert!(result.is_some());

    let solution = result.unwrap();
    assert!(solution.path_length > 0);
}

#[test]
fn test_small_maze_edge_case() {
    let maze = MazeGenerator::generate(3, 3, GeneratorAlgorithm::RecursiveBacktracker);

    assert_eq!(maze.width, 3);
    assert_eq!(maze.height, 3);

    let result = MazeSolver::solve(&maze, PathfindingAlgorithm::BFS);
    assert!(result.is_some());
}

#[test]
fn test_export_maze_as_text() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_maze_export.txt");

    let maze = MazeGenerator::generate(8, 8, GeneratorAlgorithm::Prims);
    io::export_maze_as_text(&maze, &file_path).unwrap();

    assert!(file_path.exists());

    let contents = fs::read_to_string(&file_path).unwrap();
    assert!(!contents.is_empty());
    assert!(contents.contains('│') || contents.contains('|')); // Should contain borders

    // Cleanup
    fs::remove_file(file_path).ok();
}

#[test]
fn test_maze_start_and_end_different() {
    let maze = MazeGenerator::generate(10, 10, GeneratorAlgorithm::RecursiveBacktracker);
    assert_ne!(maze.start, maze.end, "Start and end should be different positions");
}

#[test]
fn test_solution_path_is_continuous() {
    let maze = MazeGenerator::generate(15, 15, GeneratorAlgorithm::Prims);
    let result = MazeSolver::solve(&maze, PathfindingAlgorithm::AStar).unwrap();

    // Check that each step in the path is adjacent to the next
    for i in 0..result.path.len() - 1 {
        let (r1, c1) = result.path[i];
        let (r2, c2) = result.path[i + 1];

        let row_diff = (r1 as i32 - r2 as i32).abs();
        let col_diff = (c1 as i32 - c2 as i32).abs();

        assert!(
            (row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1),
            "Path should be continuous with adjacent cells"
        );
    }
}
