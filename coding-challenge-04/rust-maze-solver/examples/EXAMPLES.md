# Example Commands and Outputs

This file contains example commands you can run to showcase the maze solver's capabilities.

## Basic Examples

### 1. Quick Demo
```bash
cargo run --release -- auto -w 15 -H 15
```
Generates and solves a 15x15 maze using default algorithms (Recursive Backtracker + A*).

### 2. Small Animated Maze
```bash
cargo run --release -- auto -w 12 -H 12 -A --delay 40
```
Watch the pathfinding algorithm explore the maze in real-time!

### 3. Large Challenge Maze
```bash
cargo run --release -- auto -w 40 -H 40 -g kruskals -s a-star
```
Creates a large, complex maze and solves it efficiently.

## Algorithm Comparison

### Compare Generation Algorithms

```bash
# Recursive Backtracker - Long winding paths
cargo run --release -- generate -w 20 -H 20 -a recursive-backtracker

# Prim's - Many branches and dead ends
cargo run --release -- generate -w 20 -H 20 -a prims

# Kruskal's - Uniform complexity
cargo run --release -- generate -w 20 -H 20 -a kruskals

# Aldous-Broder - Random uniform trees
cargo run --release -- generate -w 20 -H 20 -a aldous-broder
```

### Compare Solving Algorithms

Generate a maze once and solve it with different algorithms:

```bash
# Generate and save
cargo run --release -- generate -w 25 -H 25 -a recursive-backtracker -o test.json

# Solve with A*
cargo run --release -- solve -i test.json -a a-star

# Solve with BFS
cargo run --release -- solve -i test.json -a bfs

# Solve with DFS
cargo run --release -- solve -i test.json -a dfs

# Solve with Dijkstra
cargo run --release -- solve -i test.json -a dijkstra
```

## Advanced Examples

### 1. Benchmarking Different Sizes

```bash
# Small
cargo run --release -- auto -w 10 -H 10

# Medium
cargo run --release -- auto -w 25 -H 25

# Large
cargo run --release -- auto -w 50 -H 50

# Very Large
cargo run --release -- auto -w 100 -H 100
```

### 2. Animation Speed Comparison

```bash
# Fast animation
cargo run --release -- auto -w 15 -H 15 -A --delay 10

# Medium animation
cargo run --release -- auto -w 15 -H 15 -A --delay 50

# Slow animation (educational)
cargo run --release -- auto -w 15 -H 15 -A --delay 150
```

### 3. Generate Multiple Mazes

```bash
# Create a collection of mazes
cargo run --release -- generate -w 20 -H 20 -a recursive-backtracker -o maze1.json
cargo run --release -- generate -w 20 -H 20 -a prims -o maze2.json
cargo run --release -- generate -w 20 -H 20 -a kruskals -o maze3.json
cargo run --release -- generate -w 20 -H 20 -a aldous-broder -o maze4.json

# Solve them all with the same algorithm
cargo run --release -- solve -i maze1.json -a a-star
cargo run --release -- solve -i maze2.json -a a-star
cargo run --release -- solve -i maze3.json -a a-star
cargo run --release -- solve -i maze4.json -a a-star
```

### 4. Perfect for Screenshots

```bash
# Generate visually interesting mazes
cargo run --release -- auto -w 30 -H 20 -g recursive-backtracker -s a-star
cargo run --release -- auto -w 25 -H 25 -g prims -s bfs
cargo run --release -- auto -w 35 -H 15 -g kruskals -s dijkstra
```

## Performance Testing

### Test Generation Speed

```bash
# Time the generation
time cargo run --release -- generate -w 100 -H 100 --no-display
```

### Test Solving Speed

```bash
# Generate a large maze
cargo run --release -- generate -w 100 -H 100 -o large.json --no-display

# Time different solving algorithms
time cargo run --release -- solve -i large.json -a a-star --no-stats
time cargo run --release -- solve -i large.json -a bfs --no-stats
time cargo run --release -- solve -i large.json -a dfs --no-stats
time cargo run --release -- solve -i large.json -a dijkstra --no-stats
```

## Export Examples

### Create Text File Exports

```bash
# Generate and export
cargo run --release -- generate -w 20 -H 20 -a prims -o maze.json
cargo run --release -- export -i maze.json -o maze_visual.txt

# View the text file
cat maze_visual.txt
```

## Fun Challenges

### Challenge 1: The Labyrinth
```bash
# Create a massive maze
cargo run --release -- auto -w 50 -H 50 -g recursive-backtracker -s a-star -o labyrinth.json
```

### Challenge 2: Speed Run
```bash
# Find the fastest algorithm combination
cargo run --release -- auto -w 30 -H 30 -g kruskals -s a-star
```

### Challenge 3: The Visualizer
```bash
# Create the most visually appealing animation
cargo run --release -- auto -w 25 -H 25 -g prims -s bfs -A --delay 30
```

### Challenge 4: Algorithm Tournament
```bash
# Generate one maze, solve with all algorithms
cargo run --release -- generate -w 30 -H 30 -a recursive-backtracker -o tournament.json

echo "A* Algorithm:"
cargo run --release -- solve -i tournament.json -a a-star

echo "BFS Algorithm:"
cargo run --release -- solve -i tournament.json -a bfs

echo "DFS Algorithm:"
cargo run --release -- solve -i tournament.json -a dfs

echo "Dijkstra Algorithm:"
cargo run --release -- solve -i tournament.json -a dijkstra
```

## Tips

1. **For Presentations**: Use medium-sized mazes (20-30) with animation enabled
2. **For Performance**: Use release builds and larger mazes (50+)
3. **For Learning**: Use small mazes (10-15) with slow animation (100ms+)
4. **For Testing**: Generate mazes once and solve multiple times
5. **For Screenshots**: Use rectangular mazes for interesting layouts (e.g., 35x20)
