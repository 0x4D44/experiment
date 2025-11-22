# 🚀 Quick Start Guide

## Instant Play (3 seconds to fun!)

### Option 1: Double-Click
```
Double-click: index.html
```
That's it! The game opens in your default browser.

### Option 2: Command Line
```bash
# macOS
open index.html

# Linux
xdg-open index.html

# Windows
start index.html
```

### Option 3: Drag & Drop
Drag `index.html` into any browser window.

---

## 🎮 Controls Cheat Sheet

| Action | Control |
|--------|---------|
| Move Paddle | Mouse or ← → Arrow Keys |
| Launch Ball | SPACE or ↑ Arrow |
| Pause/Resume | SPACE |

---

## 🎯 Quick Tips

1. **Aim for Angles** - Hit the ball with the paddle edges for sharp angles
2. **Collect Power-Ups** - Don't miss the falling colored boxes!
3. **Multi-Ball is King** - Best power-up for clearing levels fast
4. **Red = Hard** - Red bricks take 3 hits, focus on them early
5. **Watch All Balls** - When you have multiple balls, keep track!

---

## 🧪 Run Tests

Want to see the quality? Check the test suite:

```bash
# Open test suite
open test.html

# Click "RUN ALL TESTS" button
# Watch 50+ tests pass with flying colors!
```

---

## 🎨 What You'll See

### Start Screen
- Pulsing START GAME button
- Instructions and objectives
- Beautiful purple/cyan gradient theme

### Gameplay
- Smooth 60 FPS action
- Particle explosions on every brick break
- Glowing ball and paddle effects
- Real-time score and stats
- Power-up notifications

### Power-Ups (catch them!)
- ●● **Multi-Ball** - Purple glow
- ━━ **Big Paddle** - Green glow
- ⏱ **Slow Ball** - Cyan glow
- 🔥 **Fire Ball** - Red glow with trail
- ❤ **Extra Life** - Yellow glow

---

## 📊 Score Guide

| Brick Type | Hits | Points per Hit | Color |
|------------|------|----------------|-------|
| Easy | 1 | 10 × level | Green |
| Medium | 2 | 10 × level | Orange |
| Hard | 3 | 10 × level | Red |

**Level Bonus**: 1000 × remaining lives

Example: Complete Level 3 with 2 lives = 2,000 bonus points!

---

## 🏆 Challenge Yourself

### Beginner Goals
- [ ] Complete Level 1
- [ ] Score 500 points
- [ ] Collect your first power-up

### Intermediate Goals
- [ ] Complete Level 3
- [ ] Score 2,000 points
- [ ] Don't lose any lives in a level

### Expert Goals
- [ ] Complete Level 5
- [ ] Score 5,000 points
- [ ] Complete a level with all 3 lives

### Master Goals
- [ ] Complete Level 10
- [ ] Score 10,000 points
- [ ] Beat the game without losing a life

---

## 🐛 Troubleshooting

### Game won't start?
- Make sure you're using a modern browser (Chrome, Firefox, Safari, Edge)
- Check that JavaScript is enabled

### No sound?
- Click anywhere on the page first (browser security)
- Check your volume settings
- Sound uses Web Audio API (not required for gameplay)

### Controls not working?
- Make sure the game window has focus
- Try clicking on the canvas area
- Check that your browser isn't blocking input

### Game too fast/slow?
- The game runs at 60 FPS on modern hardware
- Try closing other tabs or programs
- Refresh the page to reset

---

## 📱 Mobile Play

The game supports touch controls!

- **Tap and drag** to move paddle
- **Tap** to launch ball
- **Double tap** to pause

Note: Performance may vary on older mobile devices.

---

## 🎓 For Developers

### File Structure
```
breakout-game/
├── index.html    # Main game (just open this!)
├── game.js       # Game engine (1000 lines)
├── test.html     # Test runner
├── tests.js      # Test suite (50+ tests)
├── README.md     # Full documentation
├── FEATURES.md   # Feature checklist
└── QUICKSTART.md # This file
```

### Modify the Game
All game constants are at the top of the `BreakoutGame` class in `game.js`:

```javascript
this.PADDLE_WIDTH = 100;      // Change paddle size
this.PADDLE_SPEED = 8;        // Change paddle speed
this.BALL_SPEED = 5;          // Change ball speed
this.BRICK_ROWS = 6;          // Change number of rows
// ... and many more!
```

### Add Your Own Levels
Edit the `getLevelPattern()` method in `game.js`:
- 0 = No brick
- 1 = 1-hit brick (green)
- 2 = 2-hit brick (orange)
- 3 = 3-hit brick (red)

Example:
```javascript
[
    [3, 3, 3, 3, 3, 3, 3, 3, 3, 3],  // Row of hard bricks
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // Empty row
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],  // Row of easy bricks
]
```

---

## 🎉 Have Fun!

This game was built with ❤️ and lots of attention to detail. Every brick break, every particle, every sound effect was crafted to give you the best arcade experience possible.

**Enjoy breaking bricks!** 🎮✨

---

Need help? Check:
- `README.md` - Full documentation
- `FEATURES.md` - Complete feature list
- `test.html` - See all 50+ tests pass
