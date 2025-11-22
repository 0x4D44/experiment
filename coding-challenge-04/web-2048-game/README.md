# 2048 Game

A beautiful, fully-featured implementation of the classic 2048 sliding tile puzzle game. Built with vanilla JavaScript, HTML5, and CSS3 for a smooth 60fps gaming experience.

## Features

### Core Game Mechanics
- **4x4 Grid**: Classic grid size with smooth tile sliding
- **Tile Merging**: Tiles with the same number merge when they touch
- **Number Doubling**: 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, and beyond!
- **Random Tile Generation**: New tiles (90% chance of 2, 10% chance of 4) appear after each move
- **Win Condition**: Reach the 2048 tile to win
- **Continue Playing**: Keep playing after winning to reach higher scores
- **Game Over Detection**: Automatically detects when no valid moves remain

### User Interface
- **Beautiful Gradient Tiles**: Each tile value has a unique color gradient
- **Smooth Animations**:
  - Tile sliding with CSS transitions
  - Tile appearance with scale animation
  - Tile merging with pop animation
  - Score increase with floating animation
- **Score Tracking**:
  - Current score with animated increments
  - Best score persistence using localStorage
- **Responsive Design**: Works perfectly on desktop and mobile devices

### Controls
- **Keyboard**: Use arrow keys (↑ ↓ ← →) to move tiles
- **Touch**: Swipe in any direction on mobile devices
- **New Game Button**: Start a fresh game anytime
- **Undo Button**: Undo up to 10 previous moves

### Special Features
- **Undo System**: Made a mistake? Undo your last move (up to 10 moves)
- **Win/Lose Overlays**: Beautiful overlays when you win or lose
- **LocalStorage Persistence**: Your best score is saved automatically
- **No Build Required**: Pure vanilla JavaScript - just open and play!

## Getting Started

### Installation

1. **Clone or download** this repository
2. **Navigate** to the project directory
3. **Open** `index.html` in your web browser

That's it! No build process, no dependencies, no installation required.

### Quick Start

```bash
# Navigate to the project directory
cd web-2048-game

# Open the game in your default browser
# On macOS:
open index.html

# On Linux:
xdg-open index.html

# On Windows:
start index.html
```

## How to Play

### Objective
Combine tiles with the same number to create a tile with the number **2048**!

### Rules
1. Use arrow keys (or swipe on mobile) to move all tiles in that direction
2. When two tiles with the same number touch, they **merge into one**
3. After each move, a new tile (2 or 4) appears in a random empty spot
4. The game ends when the grid is full and no valid moves remain
5. You win when you create a **2048** tile (but you can keep playing!)

### Scoring
- Each time two tiles merge, you gain points equal to the new tile's value
- Example: Merging two 16 tiles gives you 32 points
- Your best score is automatically saved

### Tips
- Plan ahead! Think about where tiles will move before making a move
- Keep your highest tile in a corner
- Build up tiles in a systematic pattern
- Use the undo button to try different strategies
- Don't worry about losing - you can always start a new game!

## Project Structure

```
web-2048-game/
├── index.html          # Main game HTML structure
├── styles.css          # Complete styling with animations
├── game.js             # Core game logic (Game class)
├── app.js              # UI management and controls (GameApp class)
├── test.html           # Comprehensive test suite
└── README.md           # This file
```

## Technical Details

### Architecture

**Game.js** - Core Game Logic
- Grid management (4x4 array)
- Tile movement algorithms
- Merging logic
- Win/lose detection
- Score calculation
- Undo state management
- LocalStorage integration

**App.js** - UI & Controls
- DOM manipulation
- Event handling (keyboard & touch)
- Animation triggers
- Score display updates
- Game state visualization

**Styles.css** - Beautiful Design
- Responsive grid layout
- Gradient tile colors
- CSS transitions and keyframe animations
- Mobile-responsive breakpoints

### Game Logic Highlights

#### Tile Movement Algorithm
1. **Transform Grid**: Rotate/flip grid so all moves become "left" operations
2. **Compress**: Slide all tiles to the left
3. **Merge**: Combine adjacent matching tiles
4. **Compress Again**: Slide merged tiles
5. **Transform Back**: Restore original orientation

#### Undo System
- Saves up to 10 previous game states
- Stores grid, score, won, and over states
- FIFO queue to limit memory usage

#### Win/Lose Detection
- **Win**: Checks for 2048 tile after each move
- **Lose**: Checks if any moves possible (empty cells or matching adjacent tiles)

## Testing

Open `test.html` in your browser to run the comprehensive test suite:

```bash
open test.html
```

### Test Coverage
- Game initialization
- New game start
- Tile movement in all directions (up, down, left, right)
- Tile merging logic
- Multiple merges in one move
- Score tracking
- Win condition detection
- Game over detection
- Undo functionality
- Edge cases and boundary conditions

All tests are written using a custom lightweight test framework and display results with visual feedback.

## Browser Compatibility

Works in all modern browsers:
- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- Mobile browsers (iOS Safari, Chrome Mobile)

Requires:
- ES6 JavaScript support
- CSS Grid support
- LocalStorage support
- Touch events support (for mobile)

## Performance

- **60fps animations**: Smooth CSS transitions using GPU acceleration
- **Efficient rendering**: Only updates changed tiles
- **Memory optimized**: Limited undo history to prevent memory leaks
- **No dependencies**: Lightweight and fast loading

## Future Enhancements

Possible features for future versions:
- Multiple grid sizes (3x3, 5x5, 6x6)
- Different game modes (time attack, limited moves)
- Leaderboards with name tracking
- Sound effects
- Themes and color schemes
- Animation speed controls
- Tutorial mode
- Statistics tracking (games played, win rate, etc.)

## Credits

Inspired by the original 2048 game by Gabriele Cirulli.

This implementation is a complete rewrite with:
- Enhanced animations and visual effects
- Improved mobile touch controls
- Comprehensive undo system
- Full test coverage
- Modern responsive design

## License

Free to use for personal and educational purposes.

## Development

This game was built as a coding challenge submission to demonstrate:
- Clean, maintainable JavaScript code
- Modern CSS techniques (Grid, Flexbox, animations)
- Comprehensive testing practices
- Responsive design principles
- Game development fundamentals
- State management patterns

Enjoy the game! Try to reach 2048 (and beyond)!
