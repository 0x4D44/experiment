# Quantum Blocks - Quick Start Guide

## Play the Game Now

### Option 1: Using Python (Recommended)
```bash
cd quantum-blocks
python -m http.server 8000
# Then open http://localhost:8000 in your browser
```

### Option 2: Using Node.js HTTP Server
```bash
cd quantum-blocks
npx http-server . -p 8000 -o
```

### Option 3: Direct Browser
Simply open `index.html` in any modern web browser.

## Controls

| Key | Action |
|-----|--------|
| **LEFT ARROW** | Move block left |
| **RIGHT ARROW** | Move block right |
| **DOWN ARROW** | Speed up falling |
| **SPACE** | Pause/Resume |
| **Start Button** | Begin new game |
| **Reset Button** | Clear board |

## How to Play

1. **Watch blocks fall**: Colored blocks drop from the top
2. **Position them**: Use arrow keys to move left/right
3. **Make matches**: Align 3+ blocks of the same color
4. **Clear & chain**: Consecutive clears give bonus points (chain multiplier)
5. **Earn points**: Score = blocks × 10 × chain multiplier
6. **Game ends**: Top row fills up with no space for new blocks

## Game Modes

### Zen Mode
- Relaxed, endless gameplay
- No time pressure
- Build up your score gradually
- Perfect for learning the mechanics

### Time Attack (3 Minutes)
- Race against the clock
- Earn the highest score possible
- Focus on quick multi-matches
- High-intensity challenge

## Tips for High Scores

1. **Plan ahead**: Look for chain reaction opportunities
2. **Use gravity**: Let blocks settle naturally before placing new ones
3. **Create cascades**: One clear can trigger others for massive multipliers
4. **Watch the queue**: Next color is shown in the UI panel
5. **Speed is skill**: Only speed up when you're confident in placement

## Testing

Run the comprehensive test suite:
```bash
cd quantum-blocks
node tests/test.js
```

Expected output: **23 tests passed, 0 failed**

## Game Features

- 6 vibrant colors
- Horizontal, vertical, and diagonal matching
- Chain multiplier system for consecutive clears
- Particle effects for visual feedback
- Bomb blocks with explosion mechanics
- Zen and Time Attack modes
- Responsive design for mobile and desktop

## Browser Support

Works on:
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## File Structure

```
quantum-blocks/
├── index.html          # Play the game here
├── game.js             # Complete game engine
├── package.json        # Project metadata
├── README.md           # Full documentation
├── QUICKSTART.md       # This file
└── tests/
    └── test.js         # Test suite
```

## Troubleshooting

### Game won't load
- Make sure you're opening via HTTP (http://localhost), not file://
- Check that JavaScript is enabled in your browser
- Try a different browser

### Game is too slow/fast
- This is normal behavior for different devices
- Game speed adjusts automatically as you progress
- Use DOWN arrow to manually speed up

### Controls not responding
- Click on the game canvas first to give it focus
- Make sure CAPS LOCK is off
- Check browser console for errors (F12)

## Next Steps

1. Learn the basics in Zen Mode
2. Achieve your first 1000 points
3. Try Time Attack mode for a challenge
4. Master chain reactions for big scores
5. Share your high scores!

---

**Ready? Open index.html and start clearing blocks!**

For questions or feedback, check the main README.md file.
