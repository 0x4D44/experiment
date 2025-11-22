# Quick Start Guide

## Play the Game

### Option 1: Open Directly
Simply double-click `index.html` or open it in your browser:
```bash
# macOS
open index.html

# Linux
xdg-open index.html

# Windows
start index.html
```

### Option 2: Use a Local Server (Recommended)
For the best experience, use a local web server:

```bash
# Python 3
python -m http.server 8000

# Python 2
python -m SimpleHTTPServer 8000

# Node.js (if you have http-server installed)
npx http-server

# PHP
php -S localhost:8000
```

Then open http://localhost:8000 in your browser.

## Run Tests

### Browser Tests
Open `test.html` in your browser to see the interactive test suite.

### Command Line Tests
```bash
node test.js
```

## Controls

- **Arrow Keys**: Move tiles (↑ ↓ ← →)
- **Touch Gestures**: Swipe in any direction (mobile)
- **New Game Button**: Start fresh
- **Undo Button**: Take back your last move

## Game Rules

1. Use arrow keys to move tiles
2. When two tiles with the same number touch, they merge into one
3. After every move, a new tile appears (2 or 4)
4. Reach 2048 to win!
5. Game ends when no moves are available

## Features

- Smooth 60fps animations
- Beautiful gradient colors
- Score tracking with localStorage persistence
- Undo up to 10 moves
- Win and game over detection
- Mobile-friendly responsive design
- Continue playing after winning

## Tips for Success

1. **Keep your highest tile in a corner** - Don't let it get surrounded
2. **Build up tiles systematically** - Create patterns that flow
3. **Think ahead** - Consider where tiles will move before making a move
4. **Use undo wisely** - Learn from mistakes and try different strategies
5. **Stay calm** - Even losing positions can sometimes be saved!

Enjoy the game!
