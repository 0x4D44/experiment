# Rust Maze Solver

An amazing maze generator and solver CLI application featuring multiple algorithms, beautiful terminal visualizations, and animated solving processes. Built in Rust for maximum performance and reliability.

## Features

### Maze Generation Algorithms
- **Recursive Backtracker (DFS-based)**: Creates perfect mazes with long, winding corridors
- **Prim's Algorithm**: Generates mazes with many short dead ends and branching paths
- **Kruskal's Algorithm**: Produces uniform mazes with consistent complexity
- **Aldous-Broder**: Random walk algorithm that creates uniform spanning trees

### Pathfinding Algorithms
- **A\***: Optimal pathfinding with heuristic guidance (fastest optimal solver)
- **Breadth-First Search (BFS)**: Guarantees shortest path through layer-by-layer exploration
- **Depth-First Search (DFS)**: Fast exploration but may not find the shortest path
- **Dijkstra's Algorithm**: Optimal pathfinding without heuristics

### Visualization
- Beautiful Unicode or ASCII terminal rendering
- Colored output for easy visualization
- Animated solving process
- Real-time statistics and metrics

### Additional Features
- Save and load mazes to/from JSON files
- Export mazes as text files
- Configurable maze sizes
- Performance benchmarks
- Comprehensive test suite

## Installation

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
cd rust-maze-solver

# Build the project
cargo build --release

# The binary will be available at target/release/rust-maze-solver
```

## Usage

### Quick Start

Generate and solve a maze in one command:

```bash
cargo run --release -- auto -w 25 -H 25
```

### Commands

#### Generate a Maze

```bash
# Generate a 30x30 maze using Prim's algorithm
cargo run --release -- generate -w 30 -H 30 -a prims

# Generate and save to file
cargo run --release -- generate -w 20 -H 20 -a recursive-backtracker -o maze.json

# Generate without displaying
cargo run --release -- generate -w 15 -H 15 --no-display -o maze.json
```

#### Solve a Maze

```bash
# Solve a saved maze using A*
cargo run --release -- solve -i maze.json -a a-star

# Generate and solve on-the-fly with BFS
cargo run --release -- solve -w 20 -H 20 -a bfs

# Solve with animation
cargo run --release -- solve -i maze.json -a a-star -A --delay 30

# Solve without statistics
cargo run --release -- solve -i maze.json -a dijkstra --no-stats
```

#### Auto Mode (Generate + Solve)

```bash
# Generate and solve in one command
cargo run --release -- auto -w 25 -H 25

# Specify algorithms
cargo run --release -- auto -w 30 -H 30 -g kruskals -s a-star

# With animation
cargo run --release -- auto -w 20 -H 20 -A --delay 50

# Save the generated maze
cargo run --release -- auto -w 25 -H 25 -o my_maze.json
```

#### Export a Maze

```bash
# Export saved maze to text file
cargo run --release -- export -i maze.json -o maze.txt
```

### Command Line Options

#### Generation Algorithms (`-a, --algorithm`)
- `recursive-backtracker`: DFS-based, creates long corridors (default)
- `prims`: Creates many short dead ends
- `kruskals`: Uniform maze generation
- `aldous-broder`: Random walk algorithm

#### Solving Algorithms (`-a, --algorithm`)
- `a-star`: A* pathfinding (default, optimal and fast)
- `bfs`: Breadth-First Search (optimal)
- `dfs`: Depth-First Search (fast but not optimal)
- `dijkstra`: Dijkstra's algorithm (optimal)

#### Other Options
- `-w, --width <WIDTH>`: Maze width (default: 25)
- `-H, --height <HEIGHT>`: Maze height (default: 25)
- `-o, --output <FILE>`: Save maze to file
- `-i, --input <FILE>`: Load maze from file
- `-A, --animate`: Enable animation
- `-d, --delay <MS>`: Animation delay in milliseconds (default: 50)
- `--no-display`: Don't display the maze
- `--no-stats`: Don't display statistics

## Examples

### Example 1: Small Maze with Different Algorithms

```bash
# Recursive Backtracker - Long corridors
cargo run --release -- auto -w 15 -H 15 -g recursive-backtracker

# Prim's - Many dead ends
cargo run --release -- auto -w 15 -H 15 -g prims

# Kruskal's - Uniform complexity
cargo run --release -- auto -w 15 -H 15 -g kruskals
```

### Example 2: Large Maze with Performance Testing

```bash
# Generate a large maze and measure solving performance
cargo run --release -- auto -w 50 -H 50 -g recursive-backtracker -s a-star

# Compare different solving algorithms
cargo run --release -- solve -w 50 -H 50 -a a-star
cargo run --release -- solve -w 50 -H 50 -a bfs
cargo run --release -- solve -w 50 -H 50 -a dfs
cargo run --release -- solve -w 50 -H 50 -a dijkstra
```

### Example 3: Animated Solving

```bash
# Watch the algorithm explore the maze
cargo run --release -- auto -w 20 -H 20 -A --delay 30

# Slower animation for better visualization
cargo run --release -- solve -w 15 -H 15 -a bfs -A --delay 100
```

### Example 4: Save and Load Workflow

```bash
# Generate a complex maze and save it
cargo run --release -- generate -w 30 -H 30 -a kruskals -o challenge.json

# Solve it later with different algorithms
cargo run --release -- solve -i challenge.json -a a-star
cargo run --release -- solve -i challenge.json -a bfs
cargo run --release -- solve -i challenge.json -a dfs

# Export to text file
cargo run --release -- export -i challenge.json -o challenge.txt
```

## Output Symbols

- `█` or `#`: Wall
- ` ` (space): Path
- `S`: Start position (green)
- `E`: End position (red)
- `·` or `.`: Visited cells during search (blue)
- `●` or `o`: Solution path (yellow)

## Performance

The application is optimized for performance:

- Efficient data structures for maze representation
- Optimized pathfinding algorithms
- Fast maze generation
- Minimal memory overhead

Run benchmarks:

```bash
cargo bench
```

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_a_star_finds_solution
```

### Test Coverage

- Unit tests for all algorithms
- Integration tests for complete workflows
- Property-based tests for algorithm correctness
- Performance benchmarks

## Algorithm Details

### Generation Algorithms

**Recursive Backtracker**
- Time Complexity: O(n)
- Space Complexity: O(n)
- Characteristics: Creates perfect mazes with long corridors, low branching factor

**Prim's Algorithm**
- Time Complexity: O(n log n)
- Space Complexity: O(n)
- Characteristics: Many short dead ends, medium branching factor

**Kruskal's Algorithm**
- Time Complexity: O(n log n)
- Space Complexity: O(n)
- Characteristics: Uniform maze structure, consistent complexity

**Aldous-Broder**
- Time Complexity: O(n²) expected
- Space Complexity: O(n)
- Characteristics: Random walk, creates uniform spanning trees

### Pathfinding Algorithms

**A\***
- Time Complexity: O(b^d) where b is branching factor, d is depth
- Space Complexity: O(b^d)
- Optimality: Guaranteed optimal path
- Best for: Most use cases, excellent balance of speed and optimality

**BFS**
- Time Complexity: O(V + E)
- Space Complexity: O(V)
- Optimality: Guaranteed optimal path
- Best for: Unweighted graphs, guaranteed shortest path

**DFS**
- Time Complexity: O(V + E)
- Space Complexity: O(V)
- Optimality: Not guaranteed
- Best for: Fast exploration, memory-constrained environments

**Dijkstra**
- Time Complexity: O((V + E) log V)
- Space Complexity: O(V)
- Optimality: Guaranteed optimal path
- Best for: Weighted graphs, when heuristic is not available

## Project Structure

```
rust-maze-solver/
├── src/
│   ├── main.rs              # CLI interface
│   ├── lib.rs               # Library exports
│   ├── algorithms/
│   │   ├── generator.rs     # Maze generation algorithms
│   │   └── solver.rs        # Pathfinding algorithms
│   ├── maze/
│   │   ├── cell.rs          # Cell and direction definitions
│   │   └── grid.rs          # Maze grid structure
│   ├── visualization/
│   │   ├── renderer.rs      # Terminal rendering
│   │   └── animator.rs      # Animation support
│   └── io/
│       └── mod.rs           # File I/O operations
├── tests/
│   └── integration_test.rs  # Integration tests
├── benches/
│   └── maze_benchmark.rs    # Performance benchmarks
├── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome! Areas for improvement:

- Additional maze generation algorithms (e.g., Wilson's, Eller's)
- More pathfinding algorithms (e.g., Jump Point Search, Theta*)
- 3D maze support
- GUI interface
- More export formats
- Additional visualization options

## License

This project is created for educational and demonstration purposes.

## Acknowledgments

- Built with Rust for performance and safety
- Uses `clap` for CLI parsing
- Uses `crossterm` for terminal control
- Uses `colored` for colorful output
- Implements classic maze generation and solving algorithms

## Performance Benchmarks

Typical performance on modern hardware:

| Maze Size | Generation Time | Solving Time (A*) |
|-----------|----------------|-------------------|
| 10x10     | < 1ms          | < 1ms             |
| 25x25     | < 5ms          | < 2ms             |
| 50x50     | < 20ms         | < 10ms            |
| 100x100   | < 100ms        | < 50ms            |

## FAQ

**Q: Which generation algorithm should I use?**
A: For general use, Recursive Backtracker creates interesting mazes with long paths. Prim's creates more complex mazes with branches. Try different algorithms to see which you prefer!

**Q: Which solving algorithm is fastest?**
A: A* is typically fastest while still guaranteeing optimal paths. DFS is faster but doesn't guarantee the shortest path.

**Q: Can I use this as a library?**
A: Yes! The core functionality is exposed as a library. Add it to your Cargo.toml and use the maze generation and solving algorithms in your own projects.

**Q: Why do some mazes look all paths?**
A: The generation algorithms create "perfect mazes" where all cells are paths and walls are removed between them. This creates solvable mazes with exactly one path between any two points.

**Q: Can I customize the visualization?**
A: Currently, the visualization uses predefined colors and symbols. You can modify the `renderer.rs` file to customize the appearance.

## Showcase Examples

```bash
# Challenge yourself with a large maze
cargo run --release -- auto -w 50 -H 50 -g recursive-backtracker -s a-star

# Watch the algorithm work
cargo run --release -- auto -w 25 -H 25 -g prims -s bfs -A --delay 20

# Perfect for screenshots
cargo run --release -- auto -w 30 -H 30 -g kruskals -s a-star

# Compare algorithms side-by-side (run multiple times)
cargo run --release -- auto -w 20 -H 20 -g recursive-backtracker -s a-star
cargo run --release -- auto -w 20 -H 20 -g prims -s bfs
cargo run --release -- auto -w 20 -H 20 -g kruskals -s dijkstra
```

---

Built with love and Rust. Happy maze solving!
