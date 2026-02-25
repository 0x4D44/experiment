# Tic-Tac-Toe with AI - High Level Design

## 1. System Overview

This document describes the architecture for a feature-rich Tic-Tac-Toe game with multiple AI difficulty levels, comprehensive analysis tools, and support for game variants.

### Project Goals
- Implement perfect AI play using minimax algorithm
- Support 5 difficulty levels from random to impossible
- Provide educational insights into game tree exploration
- Maintain <100ms response time for all AI levels
- Support multiple game variants (3x3, 4x4, 5x5, 3D, Ultimate)

---

## 2. Architecture Overview

### 2.1 Class Hierarchy

```
System Architecture:
├── Game Engine
│   ├── Board
│   ├── Player
│   └── GameState
├── AI Engine
│   ├── MiniMaxSolver
│   │   ├── AlphaBetaPruning
│   │   ├── TranspositionTable
│   │   └── MoveOrdering
│   ├── DifficultyScaler
│   └── EvaluationEngine
├── UI Layer
│   ├── BoardDisplay
│   ├── InputHandler
│   └── GameController
└── Utilities
    ├── PerformanceMonitor
    ├── GameHistory
    └── Statistics
```

### 2.2 Core Components

**Board**
- Represents game state
- Supports 3x3, 4x4, 5x5, and 3D variants
- Bitboard representation for 3x3 (8 ints: 3x3 board = 18 bits)
- Dynamic 2D/3D array for larger variants
- O(1) win detection with lookup tables

**GameState**
- Tracks whose turn it is
- Move history (stack-based)
- Board snapshot at each move
- Draw detection
- Win detection with line checking

**Player**
- Abstract base class
- Human and AI implementations
- AI: DifficultyLevel parameter
- Response time tracking

**MiniMaxSolver**
- Recursive minimax algorithm
- Alpha-beta pruning
- Transposition table with Zobrist hashing
- Move ordering heuristics
- Iterative deepening for time-limited searches

**DifficultyScaler**
- Maps difficulty to algorithm parameters
- Random move probability
- Search depth limits
- Opening book selection
- Endgame database usage

**EvaluationEngine**
- Game state scoring (+10 win, -10 loss, 0 draw)
- Position evaluation for depth penalties
- Win probability estimation
- Move quality assessment

---

## 3. Board Representation

### 3.1 3x3 Standard Board

**Bitboard Approach (Optimal)**
```cpp
struct Board3x3 {
    uint32_t player_x;    // 9 bits for X positions
    uint32_t player_o;    // 9 bits for O positions
    bool is_x_turn;       // Whose turn is it
};
```

**Advantages**:
- 32 bits per board state (can compress to 18 bits)
- O(1) copy and undo operations
- Perfect for transposition table keys
- Zobrist hashing very efficient
- Bitwise operations for move generation

**Win Detection Lookup Table**:
- 8 winning lines (3 rows, 3 cols, 2 diagonals)
- Pre-computed masks: `0b111000000`, `0b111`, `0b100100100`, etc.
- O(1) check: `(board & winning_mask) == winning_mask`

### 3.2 4x4 and 5x5 Variants

**Dynamic 2D Array**:
```cpp
template<int Size>
struct Board {
    std::array<std::array<Cell, Size>, Size> grid;
    int move_count;
    std::vector<int> move_history;
};

enum class Cell { Empty = 0, X = 1, O = 2 };
```

**Win Detection**:
- Check rows, columns, diagonals
- O(n) operation where n is grid size
- Early termination when win found

### 3.3 3D Tic-Tac-Toe (3x3x3 Cube)

```cpp
struct Board3D {
    std::array<std::array<std::array<Cell, 3>, 3>, 3> cube;

    // Win conditions:
    // - Lines along any axis (27 lines)
    // - Face diagonals (36 lines)
    // - Body diagonals (4 lines)
    // Total: 49 possible winning lines
};
```

### 3.4 Ultimate Tic-Tac-Toe

```cpp
struct UltimateBoard {
    std::array<Board3x3, 9> mini_boards;  // 9 3x3 boards
    Board3x3 meta_board;                  // Board tracking which mini-boards are won
    int active_mini_board;                 // Current playable area
};
```

---

## 4. Game Flow and State Management

### 4.1 Game State Diagram

```
START
  ↓
SELECT GAME MODE
  ├─ Human vs Human
  ├─ Human vs AI (choose difficulty)
  ├─ AI vs AI
  └─ Training Mode
  ↓
INITIALIZE BOARD
  ↓
GAME LOOP:
  ├─ Display board
  ├─ Get player move
  │  ├─ If human: input validation
  │  └─ If AI: minimax calculation
  ├─ Validate move
  ├─ Update board
  ├─ Check win/draw
  ├─ If game over: ask replay
  └─ Next player turn
  ↓
RECORD STATS & SAVE HISTORY
  ↓
END
```

### 4.2 Move Representation

**Numbered System (1-9 for 3x3)**:
```
1 | 2 | 3
---------
4 | 5 | 6
---------
7 | 8 | 9
```

**Coordinate System (A1-C3)**:
```
  A | B | C
-----------
1 A1| B1| C1
-----------
2 A2| B2| C2
-----------
3 A3| B3| C3
```

**Internal Representation**:
- Index: 0-8 for 3x3 (row * size + col)
- Conversion: move_number = index + 1

### 4.3 Move Validation

```cpp
bool is_valid_move(const Board& board, int move) {
    if (move < 1 || move > board.size * board.size) return false;
    return board[move - 1] == Cell::Empty;
}
```

---

## 5. Minimax Algorithm with Alpha-Beta Pruning

### 5.1 Core Algorithm

```
function minimax(board, depth, is_maximizing, alpha, beta):
    // Terminal states
    if board is terminal:
        return evaluate(board, depth)

    if is_maximizing:  // AI's turn (trying to maximize)
        max_eval = -infinity
        for each legal move in board:
            child = make_move(board, move)
            eval = minimax(child, depth+1, false, alpha, beta)
            max_eval = max(max_eval, eval)
            alpha = max(alpha, eval)
            if beta <= alpha:
                break  // Beta cutoff
        return max_eval
    else:  // Opponent's turn (trying to minimize)
        min_eval = +infinity
        for each legal move in board:
            child = make_move(board, move)
            eval = minimax(child, depth+1, true, alpha, beta)
            min_eval = min(min_eval, eval)
            beta = min(beta, eval)
            if beta <= alpha:
                break  // Alpha cutoff
        return min_eval
```

### 5.2 Evaluation Function

```
evaluate(board, depth):
    if board.is_won_by(AI):
        return +10 - depth  // Prefer faster wins
    if board.is_won_by(PLAYER):
        return -10 + depth  // Prefer delayed losses
    if board.is_draw():
        return 0
    return heuristic_evaluation(board)  // For depth limits
```

**Scoring**:
- Win: +10 to +1 (depending on depth)
- Loss: -10 to -1
- Draw: 0
- Heuristic for incomplete games at depth limit

### 5.3 Alpha-Beta Pruning Efficiency

```
Expected reduction:
- Best case: O(b^(d/2)) instead of O(b^d)
- Average case: ~35% node reduction
- Worst case: No pruning if move ordering is poor

With good move ordering:
- 70% of branches are cut off
- Search 50-70% fewer nodes
```

### 5.4 Move Ordering Strategy

**Priority Order**:
1. Center squares (5 for 3x3) - usually stronger
2. Corner squares (1,3,7,9) - defensive value
3. Edge squares (2,4,6,8) - weaker
4. Killer moves from sibling nodes
5. Transposition table hits
6. Random order otherwise

**Implementation**:
```cpp
std::vector<int> order_moves(const Board& board, int depth) {
    auto moves = get_legal_moves(board);
    std::sort(moves.begin(), moves.end(),
        [](int a, int b) {
            return move_priority(a) > move_priority(b);
        });
    return moves;
}
```

---

## 6. Optimizations

### 6.1 Transposition Table

**Purpose**: Cache evaluation results for previously seen positions

**Implementation**:
```cpp
struct TranspositionEntry {
    uint64_t zobrist_hash;      // Position hash
    int depth;                  // Search depth
    int eval;                   // Evaluation score
    EntryFlag flag;             // EXACT, LOWER, UPPER
    uint32_t timestamp;         // For LRU eviction
};

std::unordered_map<uint64_t, TranspositionEntry> tt;
```

**Zobrist Hashing**:
```cpp
uint64_t compute_hash(const Board& board) {
    uint64_t hash = 0;
    for (int i = 0; i < board.size; i++) {
        if (board[i] == Cell::X)
            hash ^= zobrist_keys[i][0];
        else if (board[i] == Cell::O)
            hash ^= zobrist_keys[i][1];
    }
    if (is_o_turn)
        hash ^= zobrist_turn_key;
    return hash;
}
```

**Benefits**:
- Avoid re-computing positions seen before
- Typical 20-40% speed improvement
- 16MB hash table for 3x3 (262K entries)

### 6.2 Iterative Deepening

**Strategy**: Search with increasing depth limits until time limit

```cpp
int best_move = -1;
for (int depth = 1; depth <= max_depth; ++depth) {
    auto [move, score] = minimax_depth_limited(board, depth);
    if (time_expired()) break;
    best_move = move;  // Keep last completed search
}
return best_move;
```

**Advantages**:
- Anytime algorithm (can stop at any point)
- Benefits from better move ordering at each level
- Asymptotically faster than fixed depth search
- ~10% time overhead but 50% more reliable

### 6.3 Bitboard Operations (3x3 Only)

```cpp
// Check win with single operation
bool is_winning(uint32_t board, uint32_t winning_mask) {
    return (board & winning_mask) == winning_mask;
}

// Get legal moves as bitmask
uint32_t legal_moves_mask(uint32_t occupied) {
    return ~occupied & 0x1FF;  // 0x1FF = 0b111111111
}

// Count bits for quick draw detection
int empty_count = __builtin_popcount(~(x_board | o_board) & 0x1FF);
```

### 6.4 Symmetry Reduction (3x3)

```
Standard board has 8-fold symmetry:
- 4 rotations
- 4 reflections

Can reduce transposition table by 8x:
canonical_board = apply_canonical_transform(board);
```

### 6.5 Opening Book

**Pre-computed optimal first moves**:
```cpp
std::unordered_map<uint64_t, int> opening_book = {
    // Opening positions with known best responses
    // Reduces first-move computation to O(1) lookup
};
```

### 6.6 Endgame Database

**For positions with <=4 pieces**: Pre-computed outcomes
```cpp
// Retrograde analysis: compute all terminal positions
// Work backward to determine if position is W/L/D
std::unordered_map<uint64_t, Outcome> endgame_db;
```

---

## 7. AI Difficulty Levels

### 7.1 Difficulty Configuration

| Level | Algorithm | Search Depth | Random % | Response Time |
|-------|-----------|--------------|----------|----------------|
| 1: Random | Random selection | 0 | 100% | <1ms |
| 2: Easy | Random + basic | 1 | 70% | <10ms |
| 3: Medium | Minimax + limit | 3 | 30% | <20ms |
| 4: Hard | Minimax + TT | 5 | 10% | <50ms |
| 5: Impossible | Full minimax | Full | 0% | <100ms |

### 7.2 Difficulty Scaling Implementation

```cpp
class DifficultyScaler {
public:
    struct Config {
        int max_depth;
        float random_probability;
        bool use_transposition_table;
        bool use_opening_book;
        bool use_endgame_db;
        int time_limit_ms;
    };

    static Config get_config(Difficulty level) {
        switch(level) {
            case Difficulty::Random:
                return {0, 1.0f, false, false, false, 1};
            case Difficulty::Easy:
                return {1, 0.7f, false, false, false, 10};
            case Difficulty::Medium:
                return {3, 0.3f, true, true, false, 20};
            case Difficulty::Hard:
                return {5, 0.1f, true, true, false, 50};
            case Difficulty::Impossible:
                return {9, 0.0f, true, true, true, 100};
        }
    }
};
```

### 7.3 Easy Mode Implementation

```cpp
int get_easy_move(const Board& board) {
    if (random() < 0.7) {
        return get_random_move(board);  // 70% random
    } else {
        return minimax_depth_1(board);  // 30% smart
    }
}
```

### 7.4 Medium Mode Implementation

```cpp
int get_medium_move(const Board& board) {
    auto config = DifficultyScaler::get_config(Difficulty::Medium);
    return minimax(board, 0, true, -INF, +INF,
                   config.max_depth,
                   config.time_limit_ms);
}
```

---

## 8. Win Detection Optimization

### 8.1 Win Detection Table (3x3)

Pre-computed winning combinations:
```cpp
const std::array<uint32_t, 8> WIN_MASKS = {
    0b111000000,  // Row 1
    0b000111000,  // Row 2
    0b000000111,  // Row 3
    0b100100100,  // Col 1
    0b010010010,  // Col 2
    0b001001001,  // Col 3
    0b100010001,  // Diagonal /
    0b001010100   // Diagonal \
};

bool is_won(uint32_t board) {
    for (auto mask : WIN_MASKS) {
        if ((board & mask) == mask)
            return true;
    }
    return false;
}
```

### 8.2 Incremental Win Detection

**Track potential wins as moves are made**:
```cpp
struct WinTracker {
    int row_count[3][2];     // [row][player]
    int col_count[3][2];     // [col][player]
    int diag_count[2][2];    // [diag][player]

    bool check_win(int row, int col, Player p) {
        return row_count[row][p] == 3 ||
               col_count[col][p] == 3 ||
               (is_main_diag(row,col) && diag_count[0][p] == 3) ||
               (is_anti_diag(row,col) && diag_count[1][p] == 3);
    }
};
```

---

## 9. Move History and Undo

### 9.1 Move Stack

```cpp
struct Move {
    int position;
    Player player;
    uint64_t board_hash;
    int move_number;
};

class GameHistory {
    std::vector<Move> move_stack;
    std::vector<Board> board_states;  // Snapshots for fast undo

public:
    void push_move(const Move& move, const Board& board);
    Move pop_move();
    bool can_undo() const { return !move_stack.empty(); }
};
```

### 9.2 Undo Implementation

```cpp
void undo_move() {
    if (game_history.can_undo()) {
        game_history.pop_move();
        board = game_history.get_last_board();
        current_player = opposite(current_player);
    }
}
```

---

## 10. User Interface Design

### 10.1 Board Display Format

```
Standard 3x3 Board:
     |     |
  1  |  2  |  3
_____|_____|_____
     |     |
  4  |  5  |  6
_____|_____|_____
     |     |
  7  |  8  |  9
     |     |

With moves:
     |     |
  X  |  O  |  X
_____|_____|_____
     |     |
  O  |  X  |  6
_____|_____|_____
     |     |
  7  |  8  |  O
     |     |
```

### 10.2 Input Validation

**Supported Input Formats**:
- Numeric: "5" (position 1-9)
- Coordinate: "B2" (chess notation)
- Verbose: "center", "top-left"

**Validation Logic**:
```cpp
struct InputValidator {
    static bool is_numeric(const std::string& input) {
        return input.size() == 1 && input[0] >= '1' && input[0] <= '9';
    }

    static bool is_coordinate(const std::string& input) {
        return input.size() == 2 &&
               input[0] >= 'A' && input[0] <= 'C' &&
               input[1] >= '1' && input[1] <= '3';
    }

    static int parse_input(const std::string& input) {
        if (is_numeric(input))
            return std::stoi(input) - 1;
        if (is_coordinate(input))
            return (input[0] - 'A') * 3 + (input[1] - '1');
        throw std::invalid_argument("Invalid input format");
    }
};
```

### 10.3 Game Messages

```
Game Start:
"Welcome to Tic-Tac-Toe AI!"
"Select game mode: (1) Human vs Human (2) Human vs AI (3) AI vs AI"

During Game:
"Your turn. Enter move (1-9 or A1-C3): "
"AI is thinking..."

AI Move:
"AI chose position 5 (center)"

Game End:
"You won! Congratulations!"
"It's a draw."
"AI won. Better luck next time!"

Analysis:
"Best move was position 5 (score: +8)"
"You made an excellent move!"
```

### 10.4 Statistics Display

```
Game Statistics:
- Total games: 12
- Human wins: 4
- AI wins: 6
- Draws: 2
- Win rate: 33%

Latest Game:
- Duration: 2m 34s
- Moves: 8
- AI response time: avg 45ms, max 98ms
```

---

## 11. Game Modes

### 11.1 Human vs Human
- Two players alternate turns
- Simple gameplay, no AI
- Good for learning rules

### 11.2 Human vs AI
- Choose AI difficulty (1-5)
- Human plays as X (first)
- AI plays as O
- Option to swap colors

### 11.3 AI vs AI
- Choose two AI difficulties
- Watch full game play
- Useful for testing and learning

### 11.4 Training Mode
- Human vs Medium AI
- AI suggests moves before human plays
- Shows why suggested move is good
- Provides hints for bad moves

### 11.5 Tournament Mode
- Play multiple games
- Rotating who goes first
- Track cumulative score
- Best of N games

---

## 12. Analysis and Learning Features

### 12.1 Move Evaluation

After each AI move, show:
```
Move Analysis:
- Chosen move: 5
- Score: +7.5 (winning)
- Alternatives:
  * Move 1: +2 (okay)
  * Move 3: -0.5 (risky)
  * Move 7: -3 (bad)
- Reasoning: Secures center, blocks opponent threats
```

### 12.2 Win Probability

```cpp
float calculate_win_probability(const Board& board, Player player) {
    // Use opening book statistics
    // Or estimate from position evaluation
    int score = evaluate_position(board);
    return sigmoid(score / 10.0f);  // Convert to probability
}
```

Display:
```
Position Analysis:
- Current score: +4.2
- Win probability (AI): 78%
- Win probability (You): 15%
- Draw probability: 7%
```

### 12.3 Best Move Suggestion

```cpp
int suggest_best_move(const Board& board) {
    auto moves = get_legal_moves(board);
    int best_move = -1;
    int best_score = -INF;

    for (auto move : moves) {
        auto next_board = make_move(board, move);
        int score = -minimax(next_board, ...);
        if (score > best_score) {
            best_score = score;
            best_move = move;
        }
    }
    return best_move;
}
```

Display:
```
Best move suggestion: 5 (center)
Confidence: Very high (forced winning line)
```

### 12.4 Game Tree Visualization

```
Root: Current Position (Score: +3)
├─ Move 1 (Score: +1)
│  ├─ Opponent move 4 (Score: 0)
│  └─ Opponent move 7 (Score: -2)
├─ Move 5 (Score: +7) *BEST*
│  ├─ Opponent move 1 (Score: +5)
│  └─ Opponent move 3 (Score: +8)
└─ Move 9 (Score: -1)
   ├─ Opponent move 2 (Score: -3)
   └─ Opponent move 4 (Score: +1)
```

### 12.5 Search Statistics

Display after each AI move:
```
Search Statistics:
- Nodes explored: 1,524
- Pruning efficiency: 64%
- Transposition table hits: 342 (22%)
- Average branching factor: 5.2
- Search depth reached: 7
- Time taken: 47ms
- Moves per second: 32,426
```

---

## 13. Game Variants Implementation

### 13.1 4x4 Board (First to 4)

**Adaptations**:
- 4x4 grid instead of 3x3
- Need 4 in a row/column/diagonal to win
- 16 possible moves (more complex)
- More draw states
- Same minimax algorithm applies

```cpp
template<int Size>
class BoardTemplate {
    std::array<std::array<Cell, Size>, Size> grid;

    bool check_line_win(int row, int col) {
        // Check 4 in a row horizontally/vertically/diagonally
        // from position (row, col)
    }
};
```

### 13.2 5x5 Board (First to 5)

Similar to 4x4 but with 5x5 grid.

### 13.3 Ultimate Tic-Tac-Toe

**Rules**:
- 9 3x3 boards arranged in a 3x3 meta-grid
- Winning a mini-board marks it on meta-board
- Winning meta-board wins the game
- Your move in a mini-board sends opponent to corresponding position

**Implementation**: See section 3.4

### 13.4 3D Tic-Tac-Toe (3x3x3)

**Rules**:
- 3x3x3 cube (27 cells)
- Win by getting 3 in a row in any direction:
  - Axes (3): 9 lines along x, y, z
  - Face diagonals (36): diagonals within planes
  - Body diagonals (4): major diagonals through space
- Total: 49 possible winning lines

**Win Detection**:
```cpp
bool is_3d_win(const Board3D& board, Player p) {
    // Check 49 possible lines
    // Lines along each axis
    // Diagonals on faces
    // Body diagonals
}
```

---

## 14. Performance Benchmarking

### 14.1 Benchmark Targets

| Difficulty | Time per move | Target nodes | Actual nodes |
|------------|---------------|--------------|--------------|
| Random | <1ms | 1 | 1 |
| Easy | <10ms | 50 | 50-100 |
| Medium | <20ms | 500 | 300-700 |
| Hard | <50ms | 5,000 | 3K-8K |
| Impossible | <100ms | 50,000+ | 40K-100K |

### 14.2 Benchmark Suite

```cpp
void benchmark_minimax() {
    std::vector<Board> test_positions = load_positions();

    auto start = std::chrono::high_resolution_clock::now();
    int nodes = 0;

    for (auto& board : test_positions) {
        nodes += minimax(board, ...);
    }

    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::ms>(end - start);

    std::cout << "Nodes/second: " << (nodes / duration.count() * 1000) << '\n';
}
```

### 14.3 Memory Profiling

- Board state: 8-18 bytes
- Transposition table: 16-32 bytes per entry
- Move stack: 16 bytes per move
- Typical game: <10MB total memory

---

## 15. Code Organization

### Directory Structure

```
games/tictactoe-ai/
├── CMakeLists.txt
├── HLD.md
├── README.md
├── src/
│   ├── game/
│   │   ├── Board.h / Board.cpp
│   │   ├── GameState.h / GameState.cpp
│   │   ├── GameEngine.h / GameEngine.cpp
│   │   └── Move.h
│   ├── ai/
│   │   ├── MiniMax.h / MiniMax.cpp
│   │   ├── AlphaBeta.h / AlphaBeta.cpp
│   │   ├── TranspositionTable.h / TranspositionTable.cpp
│   │   ├── Evaluator.h / Evaluator.cpp
│   │   ├── DifficultyScaler.h / DifficultyScaler.cpp
│   │   └── OpeningBook.h / OpeningBook.cpp
│   ├── ui/
│   │   ├── Display.h / Display.cpp
│   │   ├── InputHandler.h / InputHandler.cpp
│   │   └── GameController.h / GameController.cpp
│   ├── util/
│   │   ├── Logger.h / Logger.cpp
│   │   ├── PerformanceMonitor.h / PerformanceMonitor.cpp
│   │   ├── Statistics.h / Statistics.cpp
│   │   └── ZobristHash.h / ZobristHash.cpp
│   └── main.cpp
├── tests/
│   ├── CMakeLists.txt
│   ├── test_board.cpp
│   ├── test_minimax.cpp
│   ├── test_alpha_beta.cpp
│   ├── test_evaluator.cpp
│   ├── test_game_engine.cpp
│   └── benchmark_ai.cpp
├── assets/
│   └── opening_book.dat
└── docs/
    └── MINIMAX_EXPLANATION.md
```

---

## 16. Technical Requirements

### 16.1 Language and Standards
- C++17 or later
- Modern STL containers
- Optional: C++20 concepts for generic templates

### 16.2 Build System
- CMake 3.15+
- Google Test for unit tests
- Optional: Benchmark library for performance tests

### 16.3 Dependencies
- STL only (standard library)
- Google Test (for testing)
- Optional: Benchmark library

### 16.4 Portability
- Cross-platform (Windows, Linux, macOS)
- No platform-specific code
- UTF-8 board display

---

## 17. Design Patterns Used

### 17.1 Strategy Pattern
- Different difficulty levels = different strategies
- `PlayerStrategy` interface for AI algorithms

### 17.2 State Pattern
- `GameState` manages game phases
- Separate states: Setup, Playing, GameOver

### 17.3 Observer Pattern
- UI observes game events
- Statistics track game progress

### 17.4 Factory Pattern
- Create players by difficulty level
- Create boards by variant type

### 17.5 Template Method
- `minimax()` with customizable evaluation
- Reusable algorithm for all board sizes

---

## 18. Algorithm Correctness

### 18.1 Minimax Correctness Proof

**Base Case**:
- Terminal position: evaluate() returns game outcome
- Correct by definition (game rules)

**Inductive Case**:
- Assume children evaluated correctly
- Parent value = max/min of children
- Follows from game theory: both players play optimally
- Theorem: Result is minimax value (best outcome with perfect play)

### 18.2 Alpha-Beta Correctness

**Invariant**:
- Never prune a branch that could change best move
- alpha = lower bound on parent's value
- beta = upper bound on parent's value
- When beta <= alpha, parent won't reach this node

**Proof**:
- If we find eval >= beta, parent will not use this branch
- If we find eval <= alpha, parent prefers other child
- Pruning doesn't change final result

### 18.3 Transposition Table Correctness

**Entry Semantics**:
- EXACT: eval equals true minimax value at this depth
- LOWER: eval is lower bound (alpha cutoff)
- UPPER: eval is upper bound (beta cutoff)

**Lookup Logic**:
- Only use EXACT entries unconditionally
- Use bounds only if stored depth >= current depth

---

## 19. Performance Analysis

### 19.1 Time Complexity

**Minimax without pruning**: O(b^d)
- b = branching factor (~5-7 for Tic-Tac-Toe)
- d = search depth

**Alpha-beta with good ordering**: O(b^(d/2))
- Achieves sqrt reduction with optimal move ordering

**3x3 Tic-Tac-Toe specific**:
- Average branching: 5.2 moves per position
- Max depth: 9 moves
- Minimax: ~395,000 nodes
- Alpha-beta: ~140,000 nodes (35% reduction)
- With TT: ~50,000 lookups avoided (20%)

### 19.2 Space Complexity

**Without Transposition Table**: O(d)
- Recursion depth = d
- ~100 bytes per stack frame

**With Transposition Table**: O(b^d)
- 262,144 entries for 3x3 (18-bit positions)
- ~10-12 MB total memory
- Acceptable for modern systems

### 19.3 Scaling to 4x4

- Branching factor: ~7.5
- Average game length: 12 moves
- Without optimizations: impractical
- With alpha-beta: solvable
- With TT: fast (<100ms)

---

## 20. Known Limitations and Future Work

### 20.1 Current Limitations
- 3x3: Perfectly solvable, game is always draw with perfect play
- 4x4/5x5: No perfect solution, rely on depth limits
- 3D/Ultimate: High complexity, need aggressive pruning
- Opening book limited to first move only

### 20.2 Future Enhancements
- Machine learning evaluator (neural network)
- Parallel minimax (multithreading)
- GPU acceleration for large variants
- Advanced learning (store game results)
- Advanced openings database
- Endgame tablebase generation

---

## 21. Testing Strategy

### 21.1 Unit Test Coverage

- Board operations: 95% coverage
- Win detection: 100% coverage (all 8 lines)
- Move generation: 95% coverage
- Minimax: 90% coverage (all paths)
- Alpha-beta: 85% coverage (pruning verification)
- Evaluator: 95% coverage
- Game engine: 85% coverage
- Input validation: 100% coverage

### 21.2 Integration Tests

- Full game playthrough (human vs AI)
- Game state consistency
- Move history and undo
- Multiple games in sequence
- All difficulty levels

### 21.3 Correctness Tests

- Known game positions (opening, middlegame, endgame)
- Minimax returns move from move list
- Alpha-beta returns same result as minimax
- Win detection matches game rules
- No invalid moves accepted

### 21.4 Performance Tests

- Difficulty levels meet time targets
- Memory usage stays under 20MB
- 1000+ AI games per second
- No memory leaks (Valgrind)

---

## 22. Summary

This Tic-Tac-Toe AI implementation combines:
- Efficient board representation (bitboards for 3x3)
- Optimal game tree search (minimax + alpha-beta)
- Performance optimizations (transposition tables, move ordering)
- Educational interface (show AI thinking)
- Multiple difficulty levels (random to perfect)
- Game variants (3x3, 4x4, 5x5, 3D, Ultimate)

The result is a feature-complete game engine with strong AI, perfect for learning game theory and algorithms.
