# 🐍 Rust Snake Game - Terminal Edition

An amazing, feature-rich terminal-based Snake game implementation in Rust with smooth gameplay, beautiful colors, multiple difficulty levels, and comprehensive test coverage.

## Features

- **Smooth Gameplay**: Precise timing and responsive controls
- **Multiple Difficulty Levels**: Easy, Medium, Hard, and Extreme modes
- **Progressive Difficulty**: Game speed increases as you score more points
- **Beautiful Terminal UI**: Colorful graphics with borders and emojis
- **Score Tracking**: Real-time score and snake length display
- **Collision Detection**: Wall and self-collision detection
- **Pause Functionality**: Pause and resume at any time
- **Game Over & Restart**: Quick restart without leaving the game
- **Comprehensive Tests**: Extensive unit tests for game logic

## Requirements

- Rust 1.70 or later
- A terminal that supports ANSI colors and Unicode (most modern terminals)

## Installation

### Option 1: Clone and Build

```bash
cd /home/md/language/experiment/coding-challenge-04/rust-snake-game
cargo build --release
```

### Option 2: Run Directly

```bash
cd /home/md/language/experiment/coding-challenge-04/rust-snake-game
cargo run --release
```

## How to Play

1. **Start the Game**: Run `cargo run --release`
2. **Select Difficulty**: Use arrow keys (↑/↓) to select, Enter to start
3. **Control the Snake**:
   - ↑ (Up Arrow) - Move up
   - ↓ (Down Arrow) - Move down
   - ← (Left Arrow) - Move left
   - → (Right Arrow) - Move right
4. **Game Controls**:
   - P - Pause/Resume
   - Q - Quit game
   - R - Restart (after game over)

## Game Rules

- Guide your snake to eat the food (🍎)
- Each food item increases your score by 10 points
- Your snake grows longer with each food eaten
- Game speed increases progressively as you score more
- Avoid hitting walls or your own body
- The game ends when you collide with a wall or yourself

## Difficulty Levels

| Difficulty | Initial Speed | Min Speed | Description |
|------------|---------------|-----------|-------------|
| **Easy** | 150ms | 80ms | Relaxed pace, perfect for beginners |
| **Medium** | 100ms | 50ms | Balanced challenge for casual players |
| **Hard** | 70ms | 30ms | Fast-paced action for experienced players |
| **Extreme** | 50ms | 20ms | Blazing speed for true snake masters |

## Building

```bash
# Debug build
cargo build

# Release build (optimized, recommended for playing)
cargo build --release
```

## Running

```bash
# Run with cargo
cargo run --release

# Or run the compiled binary directly
./target/release/snake
```

## Testing

The project includes comprehensive unit tests covering all game logic:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_snake_movement
```

### Test Coverage

- Direction handling and opposite detection
- Position movement in all directions
- Snake creation, movement, and growth
- Snake self-collision detection
- Prevention of 180-degree turns
- Game state management (running, paused, game over)
- Wall collision detection
- Food spawning (not on snake)
- Score increase on food consumption
- Difficulty level configurations
- Pause/resume functionality

## Architecture

The project is organized into clean, modular components:

```
rust-snake-game/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # This file
└── src/
    ├── main.rs         # Entry point, game loop, and input handling
    ├── game.rs         # Core game logic (Snake, Game, Position, Direction)
    └── renderer.rs     # Terminal rendering with crossterm
```

### Key Components

- **game.rs**: Core game logic including:
  - `Snake`: Snake entity with body, movement, and collision detection
  - `Game`: Main game controller managing state, score, and updates
  - `Direction`: Movement directions with opposite detection
  - `Position`: 2D position on the game board
  - `Difficulty`: Difficulty levels with speed configurations
  - `GameState`: Game state enumeration (Running, Paused, GameOver)

- **renderer.rs**: Terminal rendering using crossterm:
  - Colorful UI with borders and emojis
  - Separate rendering for title, info, board, and instructions
  - Difficulty selection menu
  - Clean terminal management with proper cleanup

- **main.rs**: Application entry point:
  - Menu system for difficulty selection
  - Main game loop with precise timing
  - Input handling for all controls
  - Game restart functionality

## Dependencies

- **crossterm** (0.27): Cross-platform terminal manipulation
- **rand** (0.8): Random number generation for food spawning

## Technical Highlights

- **Zero Unsafe Code**: Pure safe Rust implementation
- **No Unwrap() Abuse**: Proper error handling throughout
- **Comprehensive Tests**: 15+ unit tests with high coverage
- **Clean Architecture**: Well-organized, modular code structure
- **Efficient Rendering**: Smart terminal updates without flickering
- **Precise Timing**: Consistent game speed across different systems
- **Resource Management**: Proper terminal cleanup on exit

## Performance

- Minimal CPU usage thanks to efficient event polling
- Smooth rendering at all difficulty levels
- No memory leaks or resource issues
- Fast startup time

## Future Enhancements

Potential features for future versions:
- High score persistence
- Multiple game modes (walls off, speed mode)
- Obstacles and power-ups
- Two-player mode
- Level system with increasing complexity
- Sound effects (terminal bell)
- Customizable board size
- Color themes

## License

This project is created for a coding challenge competition.

## Credits

Built with ❤️ using Rust and crossterm.

---

**Enjoy the game! Can you beat the high score?** 🏆
