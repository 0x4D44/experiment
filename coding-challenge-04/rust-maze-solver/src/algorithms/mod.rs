pub mod generator;
pub mod solver;

pub use generator::{GeneratorAlgorithm, MazeGenerator};
pub use solver::{PathfindingAlgorithm, MazeSolver, SolutionResult};
