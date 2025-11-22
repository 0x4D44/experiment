# Quick Start Guide

Get up and running with Rust Maze Solver in 60 seconds!

## Installation

```bash
cd rust-maze-solver
cargo build --release
```

## Try It Now!

### 1. Generate and Solve (Auto Mode)
The fastest way to see it in action:

```bash
cargo run --release -- auto -w 20 -H 20
```

This generates a 20x20 maze and solves it automatically.

### 2. With Animation
Watch the algorithm work:

```bash
cargo run --release -- auto -w 15 -H 15 -A --delay 40
```

### 3. Different Algorithms

Try different generation algorithms:

```bash
# Long winding corridors
cargo run --release -- auto -w 20 -H 20 -g recursive-backtracker

# Many branches
cargo run --release -- auto -w 20 -H 20 -g prims

# Uniform structure
cargo run --release -- auto -w 20 -H 20 -g kruskals
```

Try different solving algorithms:

```bash
# Fast optimal solver
cargo run --release -- auto -w 20 -H 20 -s a-star

# Shortest path guaranteed
cargo run --release -- auto -w 20 -H 20 -s bfs

# Fast exploration
cargo run --release -- auto -w 20 -H 20 -s dfs
```

### 4. Save and Load

Generate a maze:
```bash
cargo run --release -- generate -w 25 -H 25 -o my_maze.json
```

Solve it later:
```bash
cargo run --release -- solve -i my_maze.json -a a-star
```

### 5. Large Maze Challenge

Create a complex maze:
```bash
cargo run --release -- auto -w 50 -H 50
```

## All Commands

```
auto       - Generate and solve in one command
generate   - Generate a maze
solve      - Solve a maze
export     - Export to text file
```

Get help for any command:
```bash
cargo run --release -- auto --help
cargo run --release -- generate --help
cargo run --release -- solve --help
```

## Common Options

- `-w, --width`: Maze width (default: 25)
- `-H, --height`: Maze height (default: 25)
- `-g, --gen-algorithm`: Generation algorithm
- `-s, --solve-algorithm`: Solving algorithm
- `-A, --animate`: Enable animation
- `-d, --delay`: Animation delay in ms
- `-o, --output`: Save to file
- `-i, --input`: Load from file

## Generation Algorithms

- `recursive-backtracker` - Long corridors (default)
- `prims` - Many dead ends
- `kruskals` - Uniform mazes
- `aldous-broder` - Random walk

## Solving Algorithms

- `a-star` - Optimal + fast (default)
- `bfs` - Shortest path
- `dfs` - Fast exploration
- `dijkstra` - Optimal

## Tests

Run the test suite:
```bash
cargo test
```

## Benchmark

Run performance benchmarks:
```bash
cargo bench
```

## Tips

1. Start with small mazes (15x15) to understand the algorithms
2. Use animation (`-A`) to visualize how algorithms work
3. Try different algorithm combinations to see performance differences
4. Save interesting mazes to solve them multiple times
5. Use release builds (`--release`) for best performance

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Check [EXAMPLES.md](examples/EXAMPLES.md) for more examples
- See [FEATURES.md](FEATURES.md) to learn about all features

Happy maze solving!
