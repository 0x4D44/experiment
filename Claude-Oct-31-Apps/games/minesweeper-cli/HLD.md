# Minesweeper CLI - High Level Design

## 1. Game Architecture and Module Structure

The Minesweeper CLI implementation will follow a modular Rust architecture with the following components:

### Module Organization
```
src/
├── main.rs                 # Application entry point, game loop
├── lib.rs                  # Public API and module exports
├── board/
│   ├── mod.rs              # Board definition and core operations
│   ├── generator.rs        # Mine placement algorithm
│   └── reveal.rs           # Recursive reveal logic
├── cell.rs                 # Cell state enum and operations
├── game.rs                 # Game state management
├── ui/
│   ├── mod.rs              # UI rendering
│   ├── renderer.rs         # Board rendering with colors
│   └── colors.rs           # Color definitions
├── input.rs                # Input handling (keyboard)
├── timer.rs                # Game timer with millisecond precision
├── difficulty.rs           # Difficulty presets
├── statistics.rs           # Game statistics tracking
└── util.rs                 # Utility functions
```

### Core Data Structures

**Cell State:**
```rust
pub enum Cell {
    Unrevealed,
    Revealed(u8),           // 0-8 mines nearby
    Flagged,
    QuestionMarked,
}

pub struct CellData {
    cell: Cell,
    is_mine: bool,
}
```

**Board:**
```rust
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellData>>,
    mines_count: usize,
}
```

**Game State:**
```rust
pub struct Game {
    board: Board,
    game_status: GameStatus,  // Playing, Won, Lost
    first_click: bool,
    timer: GameTimer,
    flagged_count: usize,
    move_history: Vec<Move>,  // For undo/redo
}

pub enum GameStatus {
    Playing,
    Won,
    Lost { revealed_mine: (usize, usize) },
}
```

## 2. Board Generation Algorithm (Mine Placement)

### Fair Distribution Strategy

**Algorithm: Constrained Random Placement with First-Click Guarantee**

1. **First-Click Safety:**
   - Accept first click coordinates before placing any mines
   - Guarantee no mine will be placed at first click or adjacent cells
   - Reserve a 3x3 safe zone around first click

2. **Mine Placement Process:**
   - Use `rand` crate for cryptographically-secure random number generation
   - Generate candidate positions uniformly across available cells
   - Reject positions in reserved safe zone
   - Use rejection sampling to ensure fair distribution
   - Time complexity: O(mines * attempts), typically <2 attempts per mine

3. **Fairness Metrics:**
   - Verify uniform distribution across all cells
   - Check for clustering (no adjacent mine concentration)
   - Ensure average adjacent mine count is reasonable

### Generation Algorithm Pseudocode
```
function generate_board(width, height, num_mines, safe_zone):
    board = initialize_empty_board(width, height)
    placed_mines = 0

    while placed_mines < num_mines:
        x = random(0, width)
        y = random(0, height)

        if (x, y) not in safe_zone and not board[y][x].is_mine:
            board[y][x].is_mine = true
            placed_mines += 1

    calculate_adjacent_counts(board)
    return board
```

## 3. Recursive Reveal Algorithm

### Flood Fill with Adjacent Mine Checking

**Algorithm: Breadth-First Search (BFS) Flood Fill**

1. **Reveal Logic:**
   - If cell is mine: game lost (unless flagged)
   - If cell has 0 adjacent mines: recursively reveal all adjacent cells
   - If cell has 1-8 adjacent mines: only reveal this cell
   - Use a queue to avoid stack overflow on large boards

2. **Implementation Details:**
   - Track visited cells to prevent redundant processing
   - Use a VecDeque for efficient queue operations
   - Each cell processed exactly once: O(width * height) complexity
   - Memory: O(width * height) for visited set

3. **Edge Cases:**
   - Boundary checking for board edges
   - Already revealed/flagged cells (skip processing)
   - Multiple reveals in one turn (chain reactions)

### Reveal Algorithm Pseudocode
```
function reveal_recursive(board, x, y, visited):
    if (x, y) outside bounds or visited[(x, y)]:
        return

    visited.insert((x, y))
    board[y][x].state = Revealed

    if board[y][x].is_mine:
        return GameStatus::Lost

    adjacent_mines = count_adjacent_mines(board, x, y)

    if adjacent_mines == 0:
        for (nx, ny) in get_adjacent_cells(x, y):
            reveal_recursive(board, nx, ny, visited)

    return GameStatus::Playing
```

## 4. Flagging System Design

### Flag Management

**Features:**
1. **Toggle Flags:**
   - Right-click to cycle through: Unflagged → Flagged → QuestionMarked → Unflagged
   - Question marks: optional flags for uncertain cells
   - Cannot flag revealed cells
   - Can flag/unflag unrevealed cells

2. **Flag Counter:**
   - Display: `Mines: [total - flagged]`
   - Updates in real-time
   - Can be negative (over-flagged)

3. **Chord Operation (Middle Click):**
   - If cell is revealed with N adjacent mines
   - And exactly N adjacent cells are flagged
   - Automatically reveal remaining adjacent cells
   - Useful for mass revealing in safe areas

### Flag Data
```rust
pub enum Flag {
    None,
    Flagged,
    QuestionMarked,
}
```

## 5. Timer and Scoring System

### Millisecond Precision Timer

**Features:**
1. **Timing:**
   - Starts on first click
   - Pauses on game end
   - Displays as MM:SS.d (minutes:seconds.deciseconds)
   - Maximum displayable: 99:59.9 (9999 seconds)

2. **Implementation:**
   - Use `std::time::Instant` for high-precision timing
   - Calculate elapsed duration on each frame
   - Thread-safe for future multiplayer features

3. **Statistics Tracking:**
   - Best time per difficulty
   - Average time per game
   - Fastest 10 games
   - Time distribution analysis

### Timer Data
```rust
pub struct GameTimer {
    start_time: Option<Instant>,
    elapsed_ms: u64,
    is_paused: bool,
}
```

## 6. Difficulty Presets and Custom Games

### Predefined Difficulties

| Difficulty | Width | Height | Mines | 3BV | Flags |
|-----------|-------|--------|-------|-----|-------|
| Beginner  | 9     | 9      | 10    | ~40 | Standard |
| Intermediate | 16 | 16     | 40    | ~100 | Standard |
| Expert    | 30    | 16     | 99    | ~240 | Standard |
| Easy (Custom) | 10 | 10    | 5     | ~20 | Standard |

### Custom Game Creation
```rust
pub struct Difficulty {
    name: String,
    width: usize,
    height: usize,
    mines: usize,
    allow_question_marks: bool,
}

impl Difficulty {
    pub fn validate(&self) -> Result<(), String> {
        // Check mines < cells * 0.9
        // Check reasonable board dimensions
        // Return validation errors
    }
}
```

### Validation Rules
- Minimum board: 4x4 (at least 1 mine)
- Maximum board: 100x100 (memory/performance)
- Mines must be < 90% of total cells
- Mines must be > 0 and < total cells

## 7. Board Rendering Approach

### Visual Elements and Colors

**Rendering Symbols:**
```
Unrevealed:  ▢ (U+25A2)
Revealed 0:  · (U+00B7)
Revealed 1:  1 (Blue)
Revealed 2:  2 (Green)
Revealed 3:  3 (Red)
Revealed 4:  4 (Dark Blue)
Revealed 5:  5 (Brown/Magenta)
Revealed 6:  6 (Cyan)
Revealed 7:  7 (Black/White)
Revealed 8:  8 (Gray)
Flagged:     ⚑ (U+26D1, Red)
QuestionMark: ? (Yellow)
Mine (lost): ✸ (U+2738, Red background)
```

### Color Implementation (ANSI)
```rust
pub enum Color {
    Blue = 34,
    Green = 32,
    Red = 31,
    DarkBlue = 44,
    Brown = 35,
    Cyan = 36,
    Black = 30,
    Gray = 90,
    Yellow = 33,
}
```

### Rendering Features

1. **Board Display:**
   - Unicode box drawing for borders
   - Center board on screen
   - Show coordinates (x, y) below board
   - Display cursor highlight on current cell
   - Show mine counter and timer in header

2. **Layout:**
```
╔═══════════════════════════╗
║  MINESWEEPER INTERMEDIATE ║
║  Mines: 040  Time: 00:12.3 ║
╠═══════════════════════════╣
║ ▢ 1 · · 1 ▢ ▢ ▢ ▢ ▢ ... ║
║ 1 1 · · 2 ▢ ▢ ▢ ▢ ▢ ... ║
║ ⚑ 2 · 1 ⚑ ▢ ▢ ▢ ▢ ▢ ... ║
║ 1 ▢ · 1 2 ▢ ▢ ▢ ▢ ▢ ... ║
║ 1 1 · 1 ▢ ▢ ▢ ▢ ▢ ▢ ... ║
║ ...                       ║
╚═══════════════════════════╝
  Position: (5, 3)  Space: Reveal | F: Flag | H: Hint | R: Restart
```

3. **Cursor Highlighting:**
   - Inverse video (white on black) or dim background
   - Clear visual indicator of selected cell
   - Updates in real-time as user moves

4. **Animation on Game End:**
   - Victory: Flash board green, show all safe cells
   - Loss: Flash board red, reveal all mines
   - Display message for 1 second
   - Prompt to play again

## 8. Game State Management

### State Machine

```
       Initialize
            |
            v
       New Game Dialog
            |
            v
       Playing <- Click -> First Click -> Reveal Board -> Playing
            |
            +---> Win (All non-mines revealed)
            |
            +---> Loss (Mine revealed)

Win/Loss -> Show Results -> Play Again? -> Yes: New Game / No: Exit
```

### Game State Transitions

1. **Initialize:**
   - Load configuration
   - Initialize statistics
   - Show main menu

2. **Setup:**
   - Select difficulty or create custom
   - Validate parameters
   - Create empty board

3. **First Click:**
   - Accept player input
   - Generate board with safe zone
   - Start timer
   - Perform initial reveal

4. **Playing:**
   - Accept input: move, reveal, flag, chord, hint, undo/redo
   - Update board state
   - Check win/loss conditions
   - Render updated board

5. **Game End:**
   - Stop timer
   - Reveal all mines or safe cells
   - Display result message
   - Save statistics
   - Prompt for new game

### Move History (for Undo/Redo)

```rust
pub enum Move {
    Reveal { x: usize, y: usize },
    Flag { x: usize, y: usize, flag: Flag },
    Chord { x: usize, y: usize },
}

pub struct MoveHistory {
    moves: Vec<Move>,
    current_index: usize,
}
```

## 9. Input Handling and Controls

### Keyboard Controls

| Input | Action |
|-------|--------|
| Arrow Keys / WASD | Move cursor |
| Space | Reveal cell |
| F | Toggle flag (cycle: None → Flag → Question → None) |
| C | Chord (reveal adjacent if flags match) |
| Z | Undo move |
| Y | Redo move |
| H | Get hint (probability calculation) |
| R | Restart game |
| S | Save game state |
| L | Load game state |
| Q | Quit game |

### Input Processing
```rust
pub enum InputEvent {
    Move(Direction),
    Reveal,
    Flag,
    Chord,
    Undo,
    Redo,
    Hint,
    Restart,
    Save,
    Load,
    Quit,
}

pub enum Direction {
    Up, Down, Left, Right,
}
```

## 10. Advanced Features

### No-Guessing Algorithm

**Purpose:** Guarantee all safe boards are solvable by logic alone

**Algorithm:**
1. After mine placement, run constraint propagation
2. Identify cells that can be determined logically
3. If any cell cannot be determined:
   - Regenerate the board
   - Retry up to 10 times

**Constraint Propagation:**
- If flagged = adjacent_mines: reveal all unrevealed adjacent
- If unrevealed = mine_count - flagged: flag all unrevealed
- Pattern matching for advanced deductions

### Hint System

**Hint Strategy:**
1. Use Bayesian probability calculation
2. Compute probability for each unrevealed cell
3. For each cell with adjacent revealed numbers:
   - Calculate odds of being a mine
   - Suggest safest cell (lowest probability)
   - Or suggest guaranteed mine (probability 1.0)

**Hint Output:**
```
Hint: Cell (12, 5) is 95% safe
      Cell (8, 3) is definitely a mine
```

### Save/Load Game State

**Persisted Data:**
```rust
pub struct SavedGame {
    board: Board,
    game_status: GameStatus,
    elapsed_ms: u64,
    difficulty: Difficulty,
    timestamp: SystemTime,
}
```

**Storage:** JSON file in `~/.local/share/minesweeper/`

### Statistics System

**Tracked Metrics:**
```rust
pub struct Statistics {
    games_played: usize,
    games_won: usize,
    games_lost: usize,
    total_time_played: Duration,
    best_times: HashMap<String, Duration>,
    avg_time: HashMap<String, Duration>,
    longest_streak: usize,
    current_streak: usize,
    mines_cleared: usize,
}
```

**Display Statistics Command:**
- Show all-time stats
- Per-difficulty breakdown
- Best times leaderboard
- Win rate percentage

## 11. Performance Considerations

### Complexity Analysis

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Board Generation | O(mines) | Typical case, rejection sampling |
| Mine Placement | O(mines * log(cells)) | Expected with good RNG |
| Reveal (single) | O(cells) | Worst case: flood fill entire board |
| Rendering | O(width * height) | Every frame |
| Chord | O(8) | Max 8 adjacent cells |

### Memory Optimization

- Board: `Vec<Vec<CellData>>` = 2 bytes per cell (Cell enum + bool)
  - Worst case (100x100): ~20KB
- Move history: Store only moves, not board snapshots
- Statistics: Fixed-size structures
- Streaming rendering (only visible cells)

### UI Responsiveness

- Render on demand (no polling)
- Input handling before render
- Non-blocking game loop
- 60 FPS target (16ms per frame)

## 12. Error Handling

### Recoverable Errors

```rust
pub enum GameError {
    InvalidBoardSize { width: usize, height: usize },
    TooManyMines { mines: usize, cells: usize },
    InvalidMove { reason: String },
    SaveError { path: String, reason: String },
    LoadError { path: String, reason: String },
}
```

### Error Recovery

- Log errors to stderr
- Display user-friendly messages
- Maintain game state integrity
- Allow retry/continue

## 13. Testing Strategy

### Unit Tests

1. **Board Generation (30% coverage):**
   - Fair distribution of mines
   - First-click safe zone enforcement
   - Adjacent mine counting
   - Board size validation

2. **Game Logic (40% coverage):**
   - Reveal algorithm correctness
   - Win/loss detection
   - Flag management
   - Chord operations
   - Undo/redo functionality

3. **Algorithms (20% coverage):**
   - Hint system accuracy
   - No-guess verification
   - Constraint propagation

4. **Statistics (10% coverage):**
   - Time tracking
   - Streak counting
   - Best time updates

### Integration Tests

- Full game playthrough scenarios
- Save/load functionality
- Multiple games in sequence

### Property-Based Tests

- Mine placement uniformity
- Recursive reveal termination
- Board validity invariants

## 14. Build and Deployment

### Cargo Configuration

```toml
[package]
name = "minesweeper-cli"
version = "1.0.0"
edition = "2021"

[dependencies]
rand = "0.8"
crossterm = "0.27"  # Terminal handling
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Optimization Profile

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### Platform Support

- Linux: Primary target
- macOS: Tested
- Windows: ANSI support via crossterm

## 15. Future Enhancements

- Daily challenges with fixed seed
- Multiplayer competitive mode (side-by-side)
- Board solver with complexity rating
- Speed-run leaderboard
- Custom themes and symbols
- AI opponent for competitive play
- Web version (WASM)
- Mobile companion app
- Achievement system (unlock badges)
- Replay system with move playback

---

This HLD provides a complete blueprint for implementing a professional-grade Minesweeper CLI game with modern features, solid architecture, and excellent performance characteristics.
