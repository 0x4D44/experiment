# Rust Breakout Game

A beautiful, feature-rich terminal-based Breakout/Arkanoid game written in Rust!

## Features

### Core Gameplay
- Classic Breakout mechanics with smooth 60 FPS gameplay
- Paddle control with arrow keys
- Realistic ball physics with angle-based bouncing
- Multiple brick types with different hit points
- 5 unique hand-crafted levels plus procedurally generated bonus levels
- Lives system (5 lives to start)
- Score tracking and level progression

### Brick Types
- **Normal Bricks** (Blue) - 1 hit, 10 points
- **Strong Bricks** (Magenta) - 2 hits, 25 points
- **Unbreakable Bricks** (Grey) - Cannot be destroyed
- **Bonus Bricks** (Yellow) - 1 hit, 50 points, guaranteed power-up drop

### Power-ups
- **[W] Wide Paddle** - Increases paddle width for easier catches
- **[M] Multi-Ball** - Duplicates all active balls
- **[S] Slow Ball** - Reduces ball speed temporarily
- **[+] Extra Life** - Adds one life
- **[L] Laser Paddle** - Special laser capability (visual indicator)

### Visual Features
- Beautiful ASCII art title screens
- Color-coded bricks and power-ups
- Animated ball and paddle
- Victory and Game Over screens with ASCII art
- Real-time HUD showing score, lives, level, and active power-ups
- Smooth animations and effects

## Installation

### Prerequisites
- Rust 1.70 or higher
- Terminal with color support

### Build from Source

```bash
cd /home/md/language/experiment/coding-challenge-04/rust-breakout-game
cargo build --release
```

### Run the Game

```bash
cargo run --release
```

## Controls

- **Arrow Left/Right** - Move paddle
- **SPACE** - Start game / Launch ball
- **P** - Pause / Resume
- **R** - Restart (on Game Over / Victory screen)
- **Q** - Quit game

## Gameplay Tips

1. **Angle Control**: The ball's angle changes based on where it hits the paddle. Hit near the edges for sharper angles!

2. **Power-up Strategy**:
   - Catch the Wide Paddle power-up early for easier ball control
   - Save Multi-Ball for difficult brick patterns
   - Slow Ball is great when you need precision

3. **Brick Priority**: Focus on breaking bonus bricks (yellow) first for more power-ups and higher scores

4. **Level Progression**: Each level introduces new challenges:
   - Level 1: Simple grid with bonus bricks
   - Level 2: Strong bricks in alternating rows
   - Level 3: Pyramid pattern with strategic strong bricks
   - Level 4: Checkerboard with unbreakable obstacles
   - Level 5: Complex walls and mixed brick types
   - Level 6+: Procedurally generated challenges

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Architecture

The game is organized into clean, modular components:

- **main.rs** - Game loop, input handling, and terminal setup
- **game.rs** - Core game logic, state management, and game rules
- **physics.rs** - Collision detection, vector math, and physics calculations
- **level.rs** - Level definitions and brick management
- **powerup.rs** - Power-up types, spawning, and effects
- **renderer.rs** - Terminal rendering with colors and ASCII art
- **input.rs** - Input state management

## Technical Highlights

### Physics Engine
- Custom 2D vector math for ball movement
- Circle-rectangle collision detection
- Realistic reflection calculations
- Angle-based paddle bouncing

### Rendering System
- Double-buffered rendering for smooth visuals
- Separate character and color buffers
- Optimized terminal output with minimal flicker
- Beautiful ASCII art and Unicode box-drawing characters

### Game Loop
- Fixed 60 FPS update rate
- Delta-time based physics
- Smooth paddle movement
- Responsive input handling

## Performance

- Optimized for 60 FPS gameplay
- Minimal CPU usage with smart frame timing
- Efficient collision detection
- Clean terminal handling with no artifacts

## Development

### Project Structure
```
rust-breakout-game/
├── src/
│   ├── main.rs          # Entry point and game loop
│   ├── game.rs          # Game state and logic
│   ├── physics.rs       # Physics and collision
│   ├── level.rs         # Level definitions
│   ├── powerup.rs       # Power-up system
│   ├── renderer.rs      # Terminal rendering
│   └── input.rs         # Input handling
├── Cargo.toml
└── README.md
```

### Dependencies
- `crossterm` - Cross-platform terminal manipulation
- `rand` - Random number generation for power-ups and procedural levels

## Future Enhancements

Potential features for future versions:
- High score persistence
- Sound effects (terminal bell)
- More power-up types
- Boss levels
- Combo multipliers
- Particle effects
- Custom level editor

## License

This project is created for educational and entertainment purposes.

## Credits

Developed as a coding challenge demonstration showcasing:
- Clean Rust code architecture
- Terminal UI development
- Game physics implementation
- Comprehensive testing practices

Enjoy the game!
