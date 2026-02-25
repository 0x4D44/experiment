# Snake CLI Game - High Level Design

## 1. Game Architecture Overview

The Snake CLI game will be a terminal-based implementation with the following architecture:

```
┌─────────────────────────────────────────┐
│         Game Loop Controller             │
├─────────────────────────────────────────┤
│  - Frame timing (10+ FPS)                │
│  - Input processing                      │
│  - Game state updates                    │
│  - Rendering coordination                │
└─────────────────────────────────────────┘
         ↓              ↓              ↓
    ┌────────┐    ┌────────┐    ┌─────────┐
    │  Snake │    │  Food  │    │  Board  │
    │ Logic  │    │ Spawner│    │ Manager │
    └────────┘    └────────┘    └─────────┘
         ↓              ↓              ↓
    ┌──────────────────────────────────────┐
    │     Collision Detection Engine       │
    ├──────────────────────────────────────┤
    │  - Wall collision                    │
    │  - Self-collision                    │
    │  - Food consumption                  │
    └──────────────────────────────────────┘
         ↓
    ┌──────────────────────────────────────┐
    │      Terminal Renderer               │
    ├──────────────────────────────────────┤
    │  - ANSI escape codes                 │
    │  - Double buffering                  │
    │  - Clear/redraw logic                │
    └──────────────────────────────────────┘
```

## 2. Core Components

### 2.1 Game Loop
- **Tick Rate**: 10 FPS (100ms per frame)
- **Configurable Speed**: Base speed with progressive difficulty
- **Non-blocking Input**: Separate goroutine for keyboard input
- **State Management**: Game states (Running, Paused, GameOver)

### 2.2 Snake Logic
- **Movement**: Processes direction queue, prevents reverse movement
- **Growth**: Extends snake when food is consumed
- **Collision**: Detects self-collision by checking head position against body
- **Direction Queue**: Handles input buffering to prevent lag-induced reverse moves

### 2.3 Food Spawning
- **Random Generation**: Generates random coordinates within game bounds
- **Validation**: Ensures food doesn't spawn on snake body
- **Respawn**: Creates new food after consumption
- **Simple Algorithm**: Uses modulo-based random positioning

### 2.4 Collision Detection
- **Wall Collision**: Head position exceeds board boundaries
- **Self-Collision**: Head position matches any body segment
- **Food Collision**: Head position equals food position triggers growth
- **Edge Cases**: Handles boundary wrap-around if enabled (optional)

### 2.5 Input Handling
- **Non-blocking Input**: Uses goroutine with channel communication
- **Arrow Keys & WASD**: Both control schemes supported
- **Direction Validation**: Prevents moving 180 degrees (reverse)
- **Input Buffering**: Queue stores next intended direction
- **Pause**: Spacebar toggles pause state

### 2.6 Rendering
- **Terminal Control**:
  - ANSI escape codes for cursor positioning
  - Clear screen using `\033[2J`
  - Move cursor using `\033[H`
  - Hide/show cursor with `\033[?25l` and `\033[?25h`
- **Double Buffering**: Build frame in memory before output to avoid flicker
- **Visual Elements**:
  - `█` (U+2588) for snake body
  - `●` (U+25CF) for food
  - `═║╔╗╚╝` for borders
  - ` ` (space) for empty cells

### 2.7 Score Tracking
- **Points**: 10 points per food consumed
- **Speed Bonus**: Speed increases every 5 foods (5% speed increase)
- **High Score**: Persisted to `~/.snake-cli-highscore`
- **Display**: Current score and high score shown during gameplay

## 3. Data Structures

### 3.1 Point/Coordinate
```go
type Point struct {
    X, Y int
}
```

### 3.2 Snake
```go
type Snake struct {
    Body      []Point    // Head is Body[0], tail is Body[len-1]
    Direction Direction  // Current direction
    NextDir   Direction  // Buffered next direction
}
```

### 3.3 Direction Enum
```go
type Direction int

const (
    Up Direction = iota
    Down
    Left
    Right
    None // For initial state
)
```

### 3.4 Game State
```go
type GameState struct {
    Board       *Board
    Snake       *Snake
    Food        Point
    Score       int
    HighScore   int
    GameOver    bool
    Paused      bool
    TickCount   int
    SpeedLevel  int
}
```

### 3.5 Board
```go
type Board struct {
    Width  int
    Height int
}
```

## 4. Input Handling Strategy

### 4.1 Keyboard Input Architecture
- **Input Goroutine**: Separate goroutine reads keyboard input asynchronously
- **Channel Communication**: Sends input events to main game loop via channel
- **Raw Mode Terminal**: Use `termios` or cross-platform library (`tcell`, `bubbletea`)
- **Non-blocking**: Prevents game freeze while waiting for input

### 4.2 Input Processing
1. Read raw keyboard input (arrow keys, WASD, spacebar, Q)
2. Convert to Direction enum or control command
3. Send to direction channel
4. Game loop applies validated direction changes

### 4.3 Library Choice
- **Primary**: `github.com/gdamore/tcell/v2` (cross-platform, robust)
- **Alternative**: Pure Go with syscalls for raw mode
- **Fallback**: `golang.org/x/term` for terminal control

## 5. Rendering Approach

### 5.1 Screen Management
1. Hide cursor at game start: `\033[?25l`
2. Clear screen: `\033[2J`
3. Move cursor home: `\033[H`
4. Render game frame
5. Move cursor to score position
6. Display score

### 5.2 Buffering Strategy
- Build complete frame in string buffer or 2D slice
- Write entire frame to stdout in single operation
- Prevents flickering from partial updates
- ~60-100ms rendering time acceptable

### 5.3 Render Cycle
```
Clear Screen → Draw Borders → Draw Snake → Draw Food
→ Draw Score → Flush to Terminal
```

### 5.4 Performance
- Only redraw changed regions (optional optimization)
- Frame cap at game tick rate (10 FPS)
- Minimize syscalls per frame

## 6. Score Tracking and Game Over

### 6.1 Score Calculation
- **Base Points**: 10 points per food
- **Bonus**: Additional 5 points per difficulty level
- **Formula**: `score += 10 + (speedLevel * 5)`

### 6.2 Speed Progression
- **Initial Speed**: 10 ticks per movement (100ms at 10 FPS)
- **Speed Increase**: Every 5 foods consumed, reduce ticks by 10%
- **Max Speed**: 3 ticks per movement (hard cap to prevent unplayability)

### 6.3 Game Over Conditions
1. **Wall Collision**: Head moves outside board boundaries
2. **Self-Collision**: Head position matches any body segment
3. **Player Quit**: User presses Q key

### 6.4 Game Over Screen
- Display final score
- Display high score (with notification if beaten)
- Show replay option (press R to restart, Q to quit)
- Cursor visible on game over screen

### 6.5 High Score Persistence
- Store in `~/.snake-cli-highscore` as simple integer text file
- Load on game start
- Update and save on game end if score > high score
- Handle missing file gracefully (first run)

## 7. Configuration Options

### 7.1 Board Configuration
- **Default**: 40x20 playable area
- **Minimum**: 10x10 (prevents unplayable board)
- **Maximum**: 200x50 (terminal limitation)
- **Adjustable**: Via command-line flags

### 7.2 Speed Configuration
- **Initial Speed**: 100ms between moves (10 ticks per move at 10 FPS)
- **Speed Increase Rate**: 5% per difficulty level
- **Min Ticks**: 3 (prevents game from becoming unplayable)

### 7.3 Visual Configuration
- **Color Support**: Auto-detect terminal capability
- **Unicode Support**: Fall back to ASCII if needed
- **Border Styles**: Configurable ASCII vs Unicode

### 7.4 Command-Line Flags
```
-width int       : Board width (default: 40)
-height int      : Board height (default: 20)
-speed int       : Initial ticks per move (default: 10)
-color           : Enable colors (default: true)
-highscore string: High score file path
```

## 8. Edge Cases and Considerations

### 8.1 Edge Cases
1. **Food Spawning on Snake**: Retry random position until valid
2. **Initial Snake Length**: 3 segments to prevent immediate self-collision
3. **Direction Queue**: Prevent queuing reverse direction
4. **Pause While Paused**: Toggle off without duplicate state
5. **Resume After Pause**: Apply buffered direction immediately
6. **Terminal Resize**: Detect and adjust board (advanced feature)

### 8.2 Performance Considerations
- Use slices efficiently (pre-allocate for snake growth)
- Avoid excessive string allocations in render loop
- Cache render strings where possible
- Use sync.Mutex for thread-safe state if needed

### 8.3 Cross-Platform Compatibility
- **Linux**: Full support via termios
- **macOS**: Full support via termios
- **Windows**: Full support via Windows Console API (or tcell library)
- **Testing**: Test on all three platforms if possible

### 8.4 Game Feel
- Smooth movement: Constant tick rate, no jitter
- Responsive controls: Input processed every frame
- Visual feedback: Immediate direction change on display
- No lag: Buffered input prevents input delay

## 9. Testing Strategy

### 9.1 Unit Tests
- **Snake Movement**: Test 4 directions, boundary wrapping
- **Collision Detection**: Wall, self, food collisions
- **Food Spawning**: Valid positions, no overlap with snake
- **Score Calculation**: Points and speed level progression
- **Direction Validation**: Prevent reverse movements

### 9.2 Integration Tests
- **Game Tick**: Complete movement → collision → food spawn → render cycle
- **Game State Transitions**: Running → Paused → Running → GameOver
- **High Score Persistence**: Load, update, save

### 9.3 Coverage Target
- Aim for >80% code coverage
- Focus on critical gameplay logic
- Mock terminal I/O for rendering tests

## 10. File Structure

```
games/snake-cli/
├── main.go              # Game entry point and main loop
├── game.go              # Game state and core logic
├── snake.go             # Snake movement and logic
├── food.go              # Food generation
├── collision.go         # Collision detection
├── input.go             # Keyboard input handling
├── render.go            # Terminal rendering
├── config.go            # Configuration management
├── game_test.go         # Comprehensive test suite
├── HLD.md               # This document
├── Makefile             # Build automation
└── README.md            # User documentation
```

## 11. Summary

The Snake CLI game will be a robust, well-structured Go application with:
- **Clean Architecture**: Separated concerns with focused components
- **Non-blocking Input**: Responsive keyboard controls
- **Smooth Rendering**: Flicker-free terminal output
- **Progressive Difficulty**: Speed increases with score
- **Persistent High Scores**: Game statistics tracking
- **Cross-Platform Support**: Works on Linux, macOS, Windows
- **Comprehensive Tests**: >80% code coverage with unit and integration tests
- **Professional Quality**: Well-documented, error-handling, edge cases managed
