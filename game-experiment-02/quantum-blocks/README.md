# Quantum Blocks

A fun and engaging puzzle game combining the mechanics of Tetris with Match-3 gameplay!

## Overview

**Quantum Blocks** is a web-based puzzle game where falling colored blocks drop down a grid. When 3 or more blocks of the same color align (horizontally, vertically, or diagonally), they clear and you earn points. Chain multiple clears for bonus multipliers!

## Features

### Core Gameplay
- **Falling Blocks**: Colored blocks fall one at a time, Tetris-style
- **Match-3 Mechanics**: Clear blocks when 3+ same colors align in any direction
- **Gravity System**: Blocks naturally settle after matches are cleared
- **Chain Reactions**: Consecutive clears multiply your score

### Game Modes
- **Zen Mode**: Endless gameplay with no time limit - relax and enjoy
- **Time Attack**: 3-minute challenge - earn the highest score you can in the time limit

### Special Features
- **Bomb Blocks**: Explode when cleared, destroying all blocks in a 2-radius area
- **Particle Effects**: Satisfying visual feedback when blocks are cleared
- **Progressive Difficulty**: Game speed increases as you play
- **Chain Multiplier**: Clear multiple matches in succession for exponential score bonuses

## How to Play

### Controls
- **LEFT / RIGHT ARROW**: Move the falling block left or right
- **DOWN ARROW**: Speed up the falling block
- **SPACE**: Pause/Resume the game
- **Start Button**: Begin a new game
- **Reset Button**: Clear the board and restart

### Strategy
1. Plan your moves to create matches of 3 or more
2. Look for chain reaction opportunities for bonus points
3. Use the space around the grid to position blocks strategically
4. In Time Attack mode, focus on quick multi-matches for maximum score

## Technical Details

### Architecture
- **game.js**: Complete game engine with all logic
  - GameEngine: Core game state and rules
  - GameController: Game loop and input handling
  - Renderer: Canvas rendering system

- **index.html**: Interactive web interface with custom CSS styling
- **tests/test.js**: Comprehensive test suite (23 tests)

### Game Grid
- 10 columns × 20 rows
- 30-pixel blocks for clear visibility
- Supports both normal and reversed gravity

### Scoring System
- Base points: 10 per block cleared
- Chain multiplier: Increases with consecutive clears
- Final score = blocks × 10 × chain_multiplier

## Installation & Running

### To Play in Browser
1. Open `index.html` in a modern web browser
2. Click "Start Game" to begin
3. Use arrow keys and mouse for controls

### To Run Tests
```bash
node tests/test.js
```

Expected output: 20+ tests passing

### Browser Requirements
- Modern HTML5 Canvas support
- ES6+ JavaScript support
- Tested on: Chrome, Firefox, Safari, Edge

## Project Structure

```
quantum-blocks/
├── index.html          # Main game interface
├── game.js             # Complete game implementation
├── package.json        # Project metadata
├── README.md           # This file
├── tests/
│   └── test.js         # Test suite
└── wrk_journals/       # Development documentation
```

## Game Logic

### Block Spawning
- Random color selected from 6 available colors
- New block spawns at top center of grid
- Game ends if no space to spawn (top row full)

### Movement System
- Blocks fall automatically based on game speed
- Player can move left/right while falling
- Blocks stop falling when hitting bottom or another block

### Match Detection
- Scans entire grid after each block placement
- Detects 3+ aligned blocks in 4 directions:
  - Horizontal
  - Vertical
  - Diagonal (both directions)

### Gravity & Settling
- After matches clear, remaining blocks fall
- Process repeats until no more matches exist
- Happens automatically with animation

### Special Blocks
- **Bomb Block**: Clears 5×5 area when matched
- Future: Color-changer (changes surrounding blocks), Gravity-reverser

## Test Coverage

The game includes 23 comprehensive tests covering:
- Grid initialization
- Block spawning and limits
- Block movement (left, right, down)
- Gravity simulation
- Match detection (horizontal, vertical, diagonal)
- Block clearing and scoring
- Chain multiplier system
- Bomb mechanics
- Game state management

Run tests: `node tests/test.js`

## Future Enhancements

- Sound effects for blocks, matches, and game over
- Difficulty progression with level system
- Leaderboard / high scores
- Mobile touch controls
- More special block types
- Power-ups and boosters
- Animated backgrounds
- Combo system with visual indicators

## Code Quality

- Clean, modular architecture
- Comprehensive test suite
- Well-commented code
- ESLint compatible
- TypeScript-ready structure

## Performance

- 60 FPS target on modern browsers
- Efficient grid-based collision detection
- Optimized rendering with canvas
- Minimal memory overhead

## Browser Compatibility

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## License

ISC

## Author

Created for the Game Development Competition 2025

---

**Ready to play? Open `index.html` in your browser and start clearing blocks!**
