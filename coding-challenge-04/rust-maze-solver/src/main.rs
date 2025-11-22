use clap::{Parser, Subcommand, ValueEnum};
use rust_maze_solver::{
    algorithms::{GeneratorAlgorithm, MazeGenerator, MazeSolver, PathfindingAlgorithm},
    io,
    visualization::{
        renderer::{MazeRenderer, SolutionStats},
        MazeAnimator,
    },
};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "rust-maze-solver")]
#[command(author = "Maze Challenge Team")]
#[command(version = "1.0")]
#[command(about = "An amazing maze generator and solver with multiple algorithms", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new maze
    Generate {
        /// Width of the maze
        #[arg(short, long, default_value = "25")]
        width: usize,

        /// Height of the maze
        #[arg(short = 'H', long, default_value = "25")]
        height: usize,

        /// Generation algorithm to use
        #[arg(short, long, value_enum, default_value = "recursive-backtracker")]
        algorithm: GenAlgo,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Don't display the maze
        #[arg(long)]
        no_display: bool,
    },

    /// Solve an existing maze
    Solve {
        /// Input maze file
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Width of the maze (if not loading from file)
        #[arg(short, long, default_value = "25")]
        width: usize,

        /// Height of the maze (if not loading from file)
        #[arg(short = 'H', long, default_value = "25")]
        height: usize,

        /// Generation algorithm (if not loading from file)
        #[arg(short = 'g', long, value_enum, default_value = "recursive-backtracker")]
        gen_algorithm: GenAlgo,

        /// Solving algorithm to use
        #[arg(short, long, value_enum, default_value = "a-star")]
        algorithm: SolveAlgo,

        /// Animate the solution
        #[arg(short = 'A', long)]
        animate: bool,

        /// Animation delay in milliseconds
        #[arg(short, long, default_value = "50")]
        delay: u64,

        /// Don't display statistics
        #[arg(long)]
        no_stats: bool,
    },

    /// Generate and solve a maze in one command
    Auto {
        /// Width of the maze
        #[arg(short, long, default_value = "25")]
        width: usize,

        /// Height of the maze
        #[arg(short = 'H', long, default_value = "25")]
        height: usize,

        /// Generation algorithm to use
        #[arg(short = 'g', long, value_enum, default_value = "recursive-backtracker")]
        gen_algorithm: GenAlgo,

        /// Solving algorithm to use
        #[arg(short, long, value_enum, default_value = "a-star")]
        solve_algorithm: SolveAlgo,

        /// Animate the solution
        #[arg(short = 'A', long)]
        animate: bool,

        /// Animation delay in milliseconds
        #[arg(short, long, default_value = "30")]
        delay: u64,

        /// Save the maze to a file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Export a maze to text file
    Export {
        /// Input maze file
        #[arg(short, long)]
        input: PathBuf,

        /// Output text file
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum GenAlgo {
    /// Recursive Backtracker (DFS-based) - Long corridors
    RecursiveBacktracker,
    /// Prim's Algorithm - Many short dead ends
    Prims,
    /// Kruskal's Algorithm - Uniform mazes
    Kruskals,
    /// Aldous-Broder - Random walk algorithm
    AldousBroder,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SolveAlgo {
    /// A* - Optimal pathfinding with heuristic
    AStar,
    /// Breadth-First Search - Guarantees shortest path
    Bfs,
    /// Depth-First Search - Fast but may not find shortest path
    Dfs,
    /// Dijkstra's Algorithm - Optimal without heuristic
    Dijkstra,
}

impl From<GenAlgo> for GeneratorAlgorithm {
    fn from(algo: GenAlgo) -> Self {
        match algo {
            GenAlgo::RecursiveBacktracker => GeneratorAlgorithm::RecursiveBacktracker,
            GenAlgo::Prims => GeneratorAlgorithm::Prims,
            GenAlgo::Kruskals => GeneratorAlgorithm::Kruskals,
            GenAlgo::AldousBroder => GeneratorAlgorithm::AldousBroder,
        }
    }
}

impl From<SolveAlgo> for PathfindingAlgorithm {
    fn from(algo: SolveAlgo) -> Self {
        match algo {
            SolveAlgo::AStar => PathfindingAlgorithm::AStar,
            SolveAlgo::Bfs => PathfindingAlgorithm::BFS,
            SolveAlgo::Dfs => PathfindingAlgorithm::DFS,
            SolveAlgo::Dijkstra => PathfindingAlgorithm::Dijkstra,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            width,
            height,
            algorithm,
            output,
            no_display,
        } => {
            println!("Generating {}x{} maze using {:?}...", width, height, algorithm);
            let maze = MazeGenerator::generate(width, height, algorithm.into());

            if !no_display {
                let renderer = MazeRenderer::new();
                println!("\n{}", renderer.render(&maze));
            }

            if let Some(path) = output {
                match io::save_maze(&maze, &path, Some(&format!("{:?}", algorithm))) {
                    Ok(_) => println!("Maze saved to {}", path.display()),
                    Err(e) => eprintln!("Error saving maze: {}", e),
                }
            }

            println!("Generation complete!");
        }

        Commands::Solve {
            input,
            width,
            height,
            gen_algorithm,
            algorithm,
            animate,
            delay,
            no_stats,
        } => {
            let maze = if let Some(path) = input {
                println!("Loading maze from {}...", path.display());
                match io::load_maze(&path) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("Error loading maze: {}", e);
                        return;
                    }
                }
            } else {
                println!("Generating {}x{} maze...", width, height);
                MazeGenerator::generate(width, height, gen_algorithm.into())
            };

            println!("Solving maze using {:?}...", algorithm);
            let start_time = Instant::now();
            let result = match MazeSolver::solve(&maze, algorithm.into()) {
                Some(r) => r,
                None => {
                    eprintln!("No solution found!");
                    return;
                }
            };
            let solve_time = start_time.elapsed().as_secs_f64() * 1000.0;

            if animate {
                let animator = MazeAnimator::new(delay);
                if let Err(e) = animator.animate_solution(&maze, &result.path, &result.visited) {
                    eprintln!("Animation error: {}", e);
                }
            } else {
                let renderer = MazeRenderer::new();
                let solution_set = result.path.iter().copied().collect();
                println!("\n{}", renderer.render_with_highlights(&maze, &result.visited, &solution_set));
            }

            if !no_stats {
                let renderer = MazeRenderer::new();
                let stats = SolutionStats {
                    algorithm: format!("{:?}", algorithm),
                    path_length: result.path_length,
                    nodes_explored: result.nodes_explored,
                    solve_time: Some(solve_time),
                };
                println!("{}", renderer.render_statistics(&stats));
            }
        }

        Commands::Auto {
            width,
            height,
            gen_algorithm,
            solve_algorithm,
            animate,
            delay,
            output,
        } => {
            println!("Generating {}x{} maze using {:?}...", width, height, gen_algorithm);
            let maze = MazeGenerator::generate(width, height, gen_algorithm.into());

            if let Some(path) = &output {
                if let Err(e) = io::save_maze(&maze, path, Some(&format!("{:?}", gen_algorithm))) {
                    eprintln!("Error saving maze: {}", e);
                }
            }

            println!("Solving maze using {:?}...", solve_algorithm);
            let start_time = Instant::now();
            let result = match MazeSolver::solve(&maze, solve_algorithm.into()) {
                Some(r) => r,
                None => {
                    eprintln!("No solution found!");
                    return;
                }
            };
            let solve_time = start_time.elapsed().as_secs_f64() * 1000.0;

            if animate {
                let animator = MazeAnimator::new(delay);
                if let Err(e) = animator.animate_solution(&maze, &result.path, &result.visited) {
                    eprintln!("Animation error: {}", e);
                }
            } else {
                let renderer = MazeRenderer::new();
                let solution_set = result.path.iter().copied().collect();
                println!("\n{}", renderer.render_with_highlights(&maze, &result.visited, &solution_set));
            }

            let renderer = MazeRenderer::new();
            let stats = SolutionStats {
                algorithm: format!("{:?}", solve_algorithm),
                path_length: result.path_length,
                nodes_explored: result.nodes_explored,
                solve_time: Some(solve_time),
            };
            println!("{}", renderer.render_statistics(&stats));

            println!("\nMaze generated with {:?}", gen_algorithm);
            if let Some(path) = output {
                println!("Saved to {}", path.display());
            }
        }

        Commands::Export { input, output } => {
            println!("Loading maze from {}...", input.display());
            let maze = match io::load_maze(&input) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error loading maze: {}", e);
                    return;
                }
            };

            println!("Exporting to {}...", output.display());
            match io::export_maze_as_text(&maze, &output) {
                Ok(_) => println!("Export complete!"),
                Err(e) => eprintln!("Error exporting maze: {}", e),
            }
        }
    }
}
