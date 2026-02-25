# Maze Generator CLI - High Level Design

## 1. Overview
A command-line Maze Generator and solver supporting multiple generation algorithms, interactive gameplay, and comprehensive statistics. Built in Rust for performance and memory efficiency.

## 2. Maze Generation Algorithms

### 2.1 Recursive Backtracker
- **Type**: Depth-First Search (DFS) based
- **Characteristics**:
  - Creates long, winding passages
  - High loop avoidance, low density mazes
  - Biased toward long corridors
- **Time Complexity**: O(n*m) where n,m are dimensions
- **Space Complexity**: O(n*m) for recursion stack
- **Generation Speed**: ~50-100μs per cell
- **Solvability**: Guaranteed single solution tree structure

### 2.2 Kruskal's Algorithm
- **Type**: Randomized Minimum Spanning Tree (MST)
- **Characteristics**:
  - Uniform random maze generation
  - Balanced passage distribution
  - Uses Union-Find data structure
- **Time Complexity**: O(n*m * α(n*m)) with path compression
- **Space Complexity**: O(n*m)
- **Generation Speed**: ~30-60μs per cell
- **Solvability**: Guaranteed spanning tree

### 2.3 Prim's Algorithm
- **Type**: Randomized MST with frontier expansion
- **Characteristics**:
  - Clustered passages
  - Natural-looking maze patterns
  - Fewer long corridors than Backtracker
- **Time Complexity**: O(n*m * log(n*m))
- **Space Complexity**: O(n*m)
- **Generation Speed**: ~40-80μs per cell
- **Solvability**: Guaranteed spanning tree

### 2.4 Binary Tree
- **Type**: Directional bias algorithm
- **Characteristics**:
  - Very fast generation
  - Diagonal bias in solution paths
  - Simplified algorithm, easy to implement
- **Time Complexity**: O(n*m)
- **Space Complexity**: O(1) (per-cell computation)
- **Generation Speed**: ~10-20μs per cell (fastest)
- **Solvability**: Guaranteed tree structure

### 2.5 Aldous-Broder
- **Type**: Random walk based
- **Characteristics**:
  - Unbiased uniform generation
  - Slow but fair maze distribution
  - Explores all cells randomly
- **Time Complexity**: O(n^2*m^2) worst case
- **Space Complexity**: O(n*m)
- **Generation Speed**: ~100-200μs per cell (slowest)
- **Solvability**: Guaranteed spanning tree

### 2.6 Wilson's Algorithm
- **Type**: Loop-erased random walk
- **Characteristics**:
  - Unbiased uniform generation
  - Better performance than Aldous-Broder
  - Self-avoiding walks
- **Time Complexity**: O(n*m * log(n*m)) expected
- **Space Complexity**: O(n*m)
- **Generation Speed**: ~50-100μs per cell
- **Solvability**: Guaranteed spanning tree

## 3. Data Structures

### 3.1 Cell Representation
```
struct Cell {
    x: usize,
    y: usize,
    visited: bool,
    walls: [bool; 4],  // N, E, S, W
}
```

### 3.2 Maze Representation
```
struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    start: (usize, usize),
    end: (usize, usize),
    seed: u64,
    algorithm: Algorithm,
}
```

### 3.3 Game State
```
struct GameState {
    maze: Maze,
    player_pos: (usize, usize),
    visited_cells: HashSet<(usize, usize)>,
    start_time: Instant,
    solution: Option<Vec<(usize, usize)>>,
    solution_revealed: bool,
    breadcrumb_trail: VecDeque<(usize, usize)>,
}
```

## 4. Pathfinding Algorithms

### 4.1 Breadth-First Search (BFS)
- **Use**: Finding optimal solution path
- **Characteristics**: Guarantees shortest path
- **Time Complexity**: O(n*m)
- **Space Complexity**: O(n*m) for queue
- **Speed**: <10ms for 100x100 maze

### 4.2 Depth-First Search (DFS)
- **Use**: Quick solution finding
- **Characteristics**: Finds a solution, not necessarily shortest
- **Time Complexity**: O(n*m)
- **Space Complexity**: O(n*m) for stack
- **Speed**: <5ms for 100x100 maze

### 4.3 A* Search
- **Use**: Hint system for optimal next move
- **Heuristic**: Manhattan distance to exit
- **Characteristics**: Optimal and efficient
- **Time Complexity**: O(n*m * log(n*m))
- **Space Complexity**: O(n*m)
- **Speed**: <5ms for 100x100 maze

## 5. Rendering System

### 5.1 Box-Drawing Characters
- Corners: ┌ ┐ └ ┘ ┼
- Horizontal: ─
- Vertical: │
- T-junctions: ├ ┤ ┬ ┴
- Crosses: ┼
- Rounded: ╭ ╮ ╰ ╯

### 5.2 Cell States
- Empty: Space
- Start: 'S' or 'A'
- End/Goal: 'E' or 'Z'
- Player: 'P' or '@'
- Solution: '*' or '.'
- Visited: '#' or '·'

### 5.3 Rendering Algorithm
```
For each cell:
  1. Render walls based on adjacent cells
  2. Overlay player position if current cell
  3. Overlay start/end markers
  4. Overlay solution path if revealed
  5. Overlay visited cells if breadcrumbs enabled
```

### 5.4 Terminal Optimization
- Use ANSI escape codes for cursor positioning
- Buffer entire frame before rendering
- Clear only changed regions for efficiency
- Support 80x24 minimum terminal size

## 6. Player Movement and Navigation

### 6.1 Input Handling
- Arrow keys (↑ ↓ ← →) or WASD
- 'H': Request hint (show next optimal move)
- 'S': Reveal full solution
- 'R': Reset maze with same seed
- 'N': Generate new maze
- 'Q': Quit

### 6.2 Movement Logic
- Validate movement against walls
- Update player position
- Record movement in breadcrumb trail
- Update visited cells set
- Check win condition (reached exit)

### 6.3 Collision Detection
- Check if adjacent cell is passable
- Account for wall configuration
- Prevent out-of-bounds movement

## 7. Maze Solving Hints

### 7.1 Hint System
- Calculate optimal path using A* with Manhattan distance
- Suggest next step toward goal
- Show highlighted path section (3-5 steps)
- Increment hint counter

### 7.2 Solution Display
- Full solution path: BFS from start to end
- Overlay solution with special character
- Toggle solution display on/off
- Show path statistics

## 8. Difficulty Levels

### 8.1 Preset Sizes
- **Tiny**: 10x10 (100 cells) - <1ms generation
- **Small**: 25x25 (625 cells) - <5ms generation
- **Medium**: 50x50 (2,500 cells) - <20ms generation
- **Large**: 100x100 (10,000 cells) - <100ms generation
- **Huge**: 250x250 (62,500 cells) - <500ms generation
- **Massive**: 500x500 (250,000 cells) - <2s generation

### 8.2 Algorithm Selection Per Difficulty
- Tiny: All algorithms equally viable
- Small: Recommend Recursive Backtracker or Kruskal's
- Medium: Recommend Kruskal's or Prim's for better distribution
- Large: Recommend any algorithm
- Huge: Use Binary Tree for speed or Kruskal's for quality
- Massive: Binary Tree recommended for speed

## 9. Performance Optimization

### 9.1 Memory Optimization
- Use bit flags for wall representation (4 bits per cell)
- Store only visited set during generation, not full history
- Use compact coordinate tuples instead of objects
- Lazy evaluation for pathfinding

### 9.2 Computation Optimization
- Pre-compute valid moves during setup
- Cache pathfinding results
- Use iterative algorithms instead of recursive for large mazes
- Batch wall computations

### 9.3 Rendering Optimization
- Buffer output to minimize terminal I/O
- Use ANSI clear codes selectively
- Cache character calculations
- Parallel processing for very large mazes (optional)

### 9.4 Target Performance
| Operation | Target | Maze Size |
|-----------|--------|-----------|
| Generate | <100ms | 100x100 |
| Generate | <500ms | 250x250 |
| Generate | <2s | 500x500 |
| Render | <100ms | 100x100 |
| Solve (BFS) | <10ms | 100x100 |
| Hint (A*) | <5ms | 100x100 |
| Move | <1ms | Any |

## 10. Verification and Correctness

### 10.1 Maze Properties to Verify
1. **Connectivity**: All cells reachable from start
2. **Tree Structure**: No cycles (n*m cells, n*m-1 passages)
3. **Solvability**: Path exists from start to exit
4. **Solution Uniqueness**: Single optimal path (tree structure guarantees this)

### 10.2 Testing Strategy
- Unit tests for each algorithm
- Property-based tests (all generated mazes valid)
- Pathfinding correctness tests
- Performance benchmarks
- Edge case tests (1x1, very large, narrow passages)

## 11. Game Modes

### 11.1 Play Mode
- Interactive maze solving
- Real-time timer
- Step counter
- Hint system
- Breadcrumb trail visualization

### 11.2 Race Mode
- Timed maze solving (5-10 minute challenge)
- Leaderboard tracking
- Statistics (efficiency percentage)

### 11.3 Generate Only
- Create maze without playing
- Display and optionally save
- Perfect for screensaver or demonstration

### 11.4 Screensaver Mode
- Auto-generate new mazes
- Continuous display rotation
- User-defined generation interval

## 12. Export and Persistence

### 12.1 Save Formats
- **Text**: Plain text maze file with solution
- **State**: Serialized game state (position, time, etc.)
- **Seed**: Reproducible maze via seed + algorithm

### 12.2 Metadata
- Generation algorithm used
- Maze dimensions
- Generation timestamp
- Random seed for reproduction
- Solution path coordinates

## 13. Statistics Tracking

### 13.1 Per-Game Statistics
- Total steps taken
- Optimal path length
- Efficiency percentage: (optimal / actual) * 100
- Time elapsed
- Hints used
- Solution revealed flag

### 13.2 Global Statistics
- Total mazes solved
- Total time played
- Average efficiency
- Preferred algorithm
- Difficulty distribution

## 14. Architecture Overview

```
┌─────────────────────────────────┐
│        Main Game Loop            │
├─────────────────────────────────┤
│  ┌──────────────────────────┐   │
│  │ Input Handler            │   │
│  │ - Parse keyboard events  │   │
│  └──────────────────────────┘   │
│  ┌──────────────────────────┐   │
│  │ Game State Manager       │   │
│  │ - Update positions       │   │
│  │ - Check win condition    │   │
│  └──────────────────────────┘   │
│  ┌──────────────────────────┐   │
│  │ Maze Generator           │   │
│  │ - Multiple algorithms    │   │
│  └──────────────────────────┘   │
│  ┌──────────────────────────┐   │
│  │ Pathfinder               │   │
│  │ - BFS, DFS, A*           │   │
│  └──────────────────────────┘   │
│  ┌──────────────────────────┐   │
│  │ Renderer                 │   │
│  │ - ASCII terminal output  │   │
│  └──────────────────────────┘   │
└─────────────────────────────────┘
```

## 15. Error Handling

### 15.1 Invalid Operations
- Movement outside bounds: Reject silently or with feedback
- Unsupported terminal size: Display minimum size requirement
- Invalid maze dimensions: Clamp or request new input
- File I/O errors: Graceful error messages

### 15.2 Performance Degradation
- Large mazes on slow systems: Estimate generation time
- Viewport limitations: Implement scrollable maze view
- Memory limits: Stream rendering for very large mazes

## 16. Testing Strategy (Phase 3)

### 16.1 Unit Tests
- Algorithm correctness for all 6 algorithms
- Maze solvability verification
- Pathfinding accuracy (BFS/DFS/A*)
- Edge cases (1x1, 2x2, long corridors)

### 16.2 Integration Tests
- Full game flow (generate, play, solve)
- State persistence and loading
- Export/import functionality

### 16.3 Performance Tests
- Benchmark all algorithms at standard sizes
- Measure rendering time vs maze size
- Profile memory usage

### 16.4 Property-Based Tests
- All generated mazes are valid spanning trees
- All solvable mazes have solutions
- Pathfinding always finds optimal solution

## Summary

This design provides a comprehensive, performant Maze Generator with:
- Multiple generation algorithms with different characteristics
- Guaranteed solvable mazes with proven mathematical properties
- Interactive gameplay with hints and solution reveal
- Efficient data structures and rendering optimizations
- Comprehensive testing and verification
- Extensible architecture for future features

The implementation prioritizes correctness (all mazes are valid spanning trees), performance (O(n*m) generation and rendering), and user experience (responsive controls, clear visuals).
