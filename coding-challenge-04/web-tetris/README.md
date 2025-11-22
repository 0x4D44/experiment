# TETRIS - Classic Block Puzzle Game

A fully-featured, beautiful web-based Tetris game built with pure HTML, CSS, and JavaScript. No dependencies, no build process - just open and play!

![Tetris Game](https://img.shields.io/badge/Game-Tetris-purple?style=for-the-badge)
![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white)
![CSS3](https://img.shields.io/badge/CSS3-1572B6?style=for-the-badge&logo=css3&logoColor=white)
![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?style=for-the-badge&logo=javascript&logoColor=black)

## Features

### Core Gameplay
- **All 7 Classic Tetrominos**: I, O, T, S, Z, J, L pieces with authentic colors
- **Super Rotation System (SRS)**: Professional rotation mechanics with wall kicks
- **Ghost Piece**: Semi-transparent preview showing where piece will land
- **Smooth Controls**: Responsive keyboard controls for precise movement
- **Hard Drop**: Instant piece placement with space bar
- **Soft Drop**: Speed up piece descent with down arrow

### Game Mechanics
- **Line Clearing**: Classic Tetris line clear mechanics
- **Score System**:
  - Single: 100 points × level
  - Double: 300 points × level
  - Triple: 500 points × level
  - Tetris (4 lines): 800 points × level
  - Soft drop: 1 point per cell
  - Hard drop: 2 points per cell
- **Level Progression**: Automatic level increase every 10 lines
- **Increasing Difficulty**: Drop speed increases with each level
- **Next Piece Preview**: See what's coming next
- **Pause Function**: Pause/resume with 'P' key

### Visual Design
- **Beautiful UI**: Modern gradient design with glassmorphism effects
- **3D Block Effects**: Blocks with highlights and shadows for depth
- **Smooth Animations**: CSS animations for all UI elements
- **Responsive Layout**: Adapts to different screen sizes
- **Game Over Screen**: Elegant overlay with final score display
- **Start Screen**: Welcoming intro with animated title

### Technical Features
- **Pure Vanilla JavaScript**: No frameworks or dependencies
- **Clean Code Architecture**: Well-organized, documented, and maintainable
- **Comprehensive Tests**: Full test suite for game logic
- **Performance Optimized**: Smooth 60 FPS gameplay using requestAnimationFrame
- **No Build Required**: Just open index.html in any modern browser

## Quick Start

### Play the Game

1. **Open the game**:
   ```bash
   cd /home/md/language/experiment/coding-challenge-04/web-tetris
   # Open index.html in your browser
   ```

   Or simply double-click `index.html` in your file browser.

2. **Click "START GAME"** and begin playing!

### Run Tests

Open `test.html` in your browser to see the comprehensive test suite:
```bash
# Open test.html in your browser
```

The test suite will automatically run and display results for:
- Grid creation and management
- Tetromino pieces
- Collision detection
- Scoring system
- Level progression
- Line clearing
- Piece rotation
- Game state management

## Controls

| Key | Action |
|-----|--------|
| **←** | Move piece left |
| **→** | Move piece right |
| **↓** | Soft drop (faster descent) |
| **↑** | Rotate piece clockwise |
| **SPACE** | Hard drop (instant placement) |
| **P** | Pause/Resume game |

## Game Rules

### Objective
Stack falling tetromino pieces to create complete horizontal lines. Completed lines disappear, earning you points. The game ends when pieces stack to the top of the playing field.

### Scoring
- **Single Line**: 100 × level
- **Double Line**: 300 × level
- **Triple Line**: 500 × level
- **Tetris (4 lines)**: 800 × level
- **Soft Drop**: 1 point per cell
- **Hard Drop**: 2 points per cell

### Level Progression
- Start at Level 1
- Advance one level for every 10 lines cleared
- Higher levels = faster piece descent
- Minimum drop speed: 100ms (at level 10+)

## Tetrominos

All seven classic Tetris pieces are included:

- **I-Piece** (Cyan): Straight line - perfect for Tetris!
- **O-Piece** (Yellow): Square - doesn't rotate
- **T-Piece** (Purple): T-shape - versatile for tight spaces
- **S-Piece** (Green): S-shape - interlocks with Z
- **Z-Piece** (Red): Z-shape - mirror of S
- **J-Piece** (Blue): J-shape - good for corners
- **L-Piece** (Orange): L-shape - mirror of J

## Project Structure

```
web-tetris/
├── index.html       # Main game HTML with embedded CSS
├── tetris.js        # Complete game engine and logic
├── test.html        # Comprehensive test suite
└── README.md        # This file
```

## Technical Details

### Architecture

The game is built with a clean, object-oriented architecture:

```javascript
class TetrisGame {
    - Grid management (10×20 playing field)
    - Piece generation and spawning
    - Collision detection
    - Movement and rotation logic
    - Line clearing and scoring
    - Level progression
    - Rendering engine
    - Game loop with requestAnimationFrame
}
```

### Key Features Implementation

**Super Rotation System (SRS)**:
- Professional rotation mechanics
- Wall kick algorithms
- Edge case handling for all pieces

**Collision Detection**:
- Boundary checking (left, right, bottom)
- Piece-to-piece collision
- Real-time validation

**Ghost Piece**:
- Calculates landing position
- Renders semi-transparent preview
- Updates in real-time with movement

**Rendering**:
- Canvas-based rendering
- 3D block effects with highlights/shadows
- Grid overlay for visual clarity
- Smooth 60 FPS performance

### Browser Compatibility

Works in all modern browsers:
- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- Opera (latest)

Requires JavaScript enabled and HTML5 Canvas support.

## Development

### Code Quality
- **Clean Code**: Well-structured, readable, and maintainable
- **Documentation**: Comprehensive inline comments
- **Testing**: Full test coverage for game logic
- **Standards**: Follows JavaScript best practices

### Testing
The test suite (`test.html`) includes 40+ tests covering:
- Grid operations
- Piece mechanics
- Collision detection
- Scoring calculations
- Level progression
- Line clearing
- Rotation logic
- Game state management

All tests are automated and run in the browser with visual feedback.

## Tips & Strategies

1. **Plan Ahead**: Use the "Next" preview to plan your moves
2. **Create Tetris**: Try to clear 4 lines at once for maximum points
3. **Avoid Gaps**: Stack pieces carefully to minimize holes
4. **Use Hard Drop**: Master the space bar for quick placement
5. **Ghost Piece**: Leverage the ghost preview for precise positioning
6. **Rotation**: Experiment with rotation positions before committing
7. **Stay Low**: Keep your stack low to have more time to react

## Credits

Built for the Coding Challenge Competition

**Features**:
- Classic Tetris gameplay with modern polish
- Super Rotation System (SRS) implementation
- Beautiful gradient UI design
- Comprehensive game logic testing
- Zero dependencies, pure vanilla JavaScript

## License

This project is created for educational and competition purposes.

---

**Enjoy playing TETRIS!** 🎮

Try to beat your high score and master the art of Tetris!
