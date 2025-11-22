# 2048 Game

A complete, fully functional implementation of the classic 2048 puzzle game built with vanilla JavaScript, HTML, and CSS.

![2048 Game](https://img.shields.io/badge/Status-Complete-brightgreen) ![Tests](https://img.shields.io/badge/Tests-16%2F16%20Passing-success) ![JavaScript](https://img.shields.io/badge/JavaScript-ES6+-yellow)

## Features

- **Full 2048 gameplay mechanics** - Slide tiles in four directions, merge matching numbers
- **Smooth animations** - Beautiful tile sliding and merging effects
- **Score tracking** - Current score and best score (persisted in localStorage)
- **Win detection** - Automatically detects when you reach 2048
- **Game over detection** - Knows when no more moves are possible
- **Responsive design** - Works on desktop and mobile devices
- **Keyboard controls** - Arrow keys or WASD for movement
- **Modern UI** - Beautiful gradient background and color-coded tiles
- **Comprehensive test suite** - 16 unit tests covering all game logic
- **Local storage** - Remembers your best score between sessions

## How to Play

### Goal
Join the numbers and get to the **2048 tile**!

### Controls
- **Arrow Keys** (↑ ↓ ← →) or **WASD** - Move tiles
- **New Game button** - Restart the game
- **R key** - Quick restart

### Rules
1. Use arrow keys to move all tiles in one direction
2. When two tiles with the same number touch, they merge into one
3. After each move, a new tile (2 or 4) appears randomly
4. Keep merging tiles to reach 2048
5. Game ends when the board is full and no merges are possible

### Scoring
- Every merge adds the value of the new tile to your score
- Try to beat your high score!

## Quick Start

### Option 1: Open Directly in Browser
Simply open `index.html` in any modern web browser:
```bash
# Open in your default browser
open index.html        # macOS
start index.html       # Windows
xdg-open index.html   # Linux
```

### Option 2: Run with a Local Server
For best results, serve the files with a local web server:

```bash
# Using Python 3
python3 -m http.server 8000

# Using Python 2
python -m SimpleHTTPServer 8000

# Using Node.js (http-server)
npx http-server -p 8000
```

Then open: http://localhost:8000

### Option 3: Use Live Server (VS Code)
1. Install the "Live Server" extension in VS Code
2. Right-click on `index.html`
3. Select "Open with Live Server"

## Running Tests

### Browser Tests
Open `test.html` in your browser to run the interactive test suite with visual results.

### Command Line Tests
Run the Node.js test suite from the terminal:

```bash
node test.js
```

Expected output:
```
Running 2048 Game Tests...

✓ Tile: should create tile with correct properties
✓ Tile: should save and update position
✓ Grid: should create empty grid
...
✓ GameManager: should detect when no moves available

==================================================
Tests passed: 16
Tests failed: 0
Total tests: 16
Pass rate: 100.0%
==================================================
```

## Project Structure

```
2048-game/
├── index.html      # Main game page
├── style.css       # Game styling and animations
├── game.js         # Core game logic (Grid, Tile, GameManager)
├── app.js          # UI controller and event handling
├── test.html       # Interactive browser test suite
├── test.js         # Node.js command-line test suite
└── README.md       # This file
```

## Technical Details

### Architecture

The game is built with a clean separation of concerns:

1. **Game Logic (`game.js`)**
   - `Tile` class - Represents individual tiles
   - `Grid` class - Manages the game board
   - `GameManager` class - Handles game state and rules

2. **UI Layer (`app.js`)**
   - `HTMLActuator` - Updates the DOM
   - `KeyboardInputManager` - Handles user input
   - `LocalStorageManager` - Persists data
   - `Application` - Coordinates everything

3. **Presentation (`style.css`)**
   - Modern gradient background
   - Smooth CSS animations
   - Responsive grid layout
   - Color-coded tiles

### Key Algorithms

**Tile Movement:**
- Builds traversal order based on direction
- Finds farthest available position for each tile
- Handles merging when tiles collide

**Win/Lose Detection:**
- Win: Any tile reaches 2048
- Lose: Grid is full AND no adjacent tiles match

**Animation System:**
- Uses CSS transforms for smooth movement
- Tracks previous positions for animation
- Special animations for new and merged tiles

### Browser Compatibility

Works in all modern browsers:
- Chrome 60+
- Firefox 55+
- Safari 12+
- Edge 79+

## Test Coverage

The test suite covers:
- Tile creation and positioning
- Grid operations (insert, remove, available cells)
- Game initialization and restart
- Tile movement in all four directions
- Tile merging logic
- Score calculation
- Win condition detection
- Game over detection
- Edge cases (no movement possible, double merges, etc.)

## Performance

- Lightweight: ~500 lines of JavaScript
- No dependencies or frameworks required
- Smooth 60fps animations
- Fast startup time
- Minimal memory footprint

## Development

### Code Style
- ES6+ JavaScript
- Clear, descriptive variable names
- Comprehensive comments
- Modular class-based architecture

### Adding Features

The modular design makes it easy to extend:
- Add new tile animations in `style.css`
- Modify game rules in `GameManager`
- Add new controls in `KeyboardInputManager`
- Create new UI elements in `HTMLActuator`

### Debugging

Open browser dev tools and check console for any errors. The game state can be inspected:

```javascript
// In browser console after game loads
window.game = new GameManager(4);
console.log(game.getState());
```

## Known Limitations

- No undo functionality
- No touch/swipe controls (keyboard only)
- Fixed 4x4 grid size (not configurable via UI)
- No animations between page loads

## Credits

Created for Coding Challenge 03. Built from scratch with vanilla JavaScript - no frameworks, no libraries, just pure web technologies.

## License

This is a coding challenge submission. Feel free to use it as a learning resource!

---

**Enjoy the game! Can you reach 2048?** 🎮
