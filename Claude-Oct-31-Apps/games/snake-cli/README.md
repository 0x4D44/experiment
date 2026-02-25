# Snake CLI - A Terminal-Based Snake Game

A fully-featured Snake game implemented in Go for the command line. Play the classic snake game directly in your terminal with smooth controls, progressive difficulty, and high score tracking.

## Features

- **Classic Gameplay**: Move the snake to eat food and avoid collisions
- **Progressive Difficulty**: Speed increases as you eat more food
- **Score Tracking**: Current score, high score, and difficulty level display
- **Smooth Controls**: Non-blocking keyboard input with arrow keys or WASD
- **Pause/Resume**: Press spacebar to pause and resume the game
- **High Score Persistence**: Your best score is saved and loaded automatically
- **Cross-Platform**: Works on Linux, macOS, and Windows
- **Configurable Board**: Customize board size via command-line flags
- **Terminal Graphics**: Clean ASCII and Unicode visualization

## Installation

### Prerequisites

- Go 1.21 or later
- A terminal that supports ANSI escape codes

### Building from Source

```bash
cd games/snake-cli
make build
```

Or build manually with Go:

```bash
go build -o snake-cli
```

### Using Make

Build and run in one command:

```bash
make run
```

Install to your GOPATH/bin:

```bash
make install
```

## Usage

### Basic Commands

```bash
# Run with default settings (40x20 board)
./snake-cli

# Run with custom board size
./snake-cli -width 60 -height 30

# Run with custom initial speed
./snake-cli -speed 8
```

### In-Game Controls

- **Arrow Keys** or **WASD**: Move the snake
  - `↑` / `W`: Move up
  - `↓` / `S`: Move down
  - `←` / `A`: Move left
  - `→` / `D`: Move right
- **Space**: Pause/Resume game
- **Q**: Quit the game
- **R**: Restart after game over

### Command-Line Flags

```
-width int
    Board width in characters (default: 40, min: 10, max: 200)

-height int
    Board height in characters (default: 20, min: 5, max: 50)

-speed int
    Initial ticks per move (default: 10, min: 1)
    Lower values = faster gameplay
```

## Gameplay

### Objective
- Eat the food (●) to grow your snake
- Avoid hitting walls or the snake's own body
- Earn points for each food consumed
- Beat your high score!

### Scoring
- **Base Points**: 10 points per food eaten
- **Difficulty Bonus**: 5 × (difficulty level) bonus per food
- **Speed Progression**: Speed increases every 5 foods consumed

### Game States
- **Running**: Normal gameplay in progress
- **Paused**: Game is paused, press Space to resume
- **Game Over**: Snake hit a wall or itself, press R to restart or Q to quit

## Game Mechanics

### Snake Movement
- Snake moves continuously in the current direction
- Direction changes are applied immediately
- Cannot move directly backward (prevents accidental self-collision)
- Movement is buffered to handle rapid input

### Food Spawning
- Food appears at random valid positions on the board
- Food never spawns on the snake's body
- New food appears after each consumption

### Collision Detection
- **Wall Collision**: Snake head touches boundary
- **Self-Collision**: Snake head touches its own body
- **Food Collision**: Snake head touches food (increases score and length)

### Difficulty Progression
```
Food Count | Speed Level | Speed Increase | Points per Food
    0-4    |      0      |      10 ticks  |       10
    5-9    |      1      |       9.5 ticks|       15
   10-14   |      2      |       9.0 ticks|       20
   15-19   |      3      |       8.5 ticks|       25
   (continues until minimum speed of 3 ticks is reached)
```

## Development

### Running Tests

```bash
# Run all tests with coverage
make test

# Generate detailed coverage report
make coverage

# Run linter
make lint

# Format code
make fmt
```

### Project Structure

```
games/snake-cli/
├── main.go           # Entry point and game loop
├── game.go           # Game state and core logic
├── input.go          # Keyboard input handling
├── render.go         # Terminal rendering
├── game_test.go      # Comprehensive test suite
├── go.mod            # Go module definition
├── Makefile          # Build automation
├── HLD.md            # High-level design document
└── README.md         # This file
```

### Architecture Overview

```
Input Handler → Direction Processing → Game State Update
                                            ↓
                                    Collision Detection
                                            ↓
                                      Spawn Food
                                            ↓
                                        Renderer
```

## Testing

The project includes a comprehensive test suite with >80% code coverage:

- **Unit Tests**: Snake movement, collision detection, food spawning
- **State Tests**: Game state management, score tracking, speed progression
- **Integration Tests**: Full tick processing with movement and collisions
- **Edge Cases**: Boundary conditions, reverse prevention, speed limits

Run tests:

```bash
make test
coverage    # Generate HTML coverage report
```

## Building for Distribution

### Create a Release Binary

```bash
make clean
make build
```

The executable will be `snake-cli` (or `snake-cli.exe` on Windows).

### Cross-Platform Compilation

```bash
# Build for Linux
GOOS=linux GOARCH=amd64 go build -o snake-cli-linux

# Build for macOS
GOOS=darwin GOARCH=amd64 go build -o snake-cli-macos

# Build for Windows
GOOS=windows GOARCH=amd64 go build -o snake-cli.exe
```

## Performance

- **Frame Rate**: 10 FPS (100ms per frame)
- **Input Response**: <100ms
- **Rendering**: <50ms per frame
- **Memory Usage**: <10MB
- **CPU Usage**: <2% at idle

## Troubleshooting

### Game doesn't respond to input
- Ensure terminal supports raw input mode
- Try using a different terminal emulator
- On Windows, use Windows Terminal or ConEmu for better compatibility

### Colors not appearing
- Some terminals don't support ANSI colors
- The game should still work in monochrome mode

### Screen flickering
- Increase terminal buffer size if available
- Close other applications using the terminal

## Known Limitations

- Requires terminal with ANSI escape sequence support
- Mouse input not supported (keyboard only)
- No networking/multiplayer features
- Terminal resize during gameplay not handled

## Future Enhancements

- [ ] Color support (red snake, yellow food, etc.)
- [ ] Obstacles on the board
- [ ] Power-ups (speed boost, slow-mo, etc.)
- [ ] Leaderboard system
- [ ] Multiple game modes (classic, timed, survival)
- [ ] Sound effects (if terminal supports it)
- [ ] Mouse support for direction
- [ ] Network multiplayer

## License

This project is provided as-is for educational and entertainment purposes.

## Credits

Implemented in Go as a complete from-scratch game engine with non-blocking input, custom rendering, and comprehensive test coverage.

## Contact & Support

For issues, suggestions, or contributions, please refer to the project repository.

---

Enjoy the game! Try to beat your high score!
