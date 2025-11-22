# Quick Start Guide

## Instant Play (No Setup Required!)

1. **Navigate to the game directory:**
   ```bash
   cd /home/md/language/experiment/coding-challenge-04/memory-match-game
   ```

2. **Open in your browser:**
   - Simply double-click `index.html`
   - Or run: `open index.html` (macOS) / `xdg-open index.html` (Linux)

3. **Start playing immediately!**

## With Local Server (Recommended)

### Python
```bash
cd /home/md/language/experiment/coding-challenge-04/memory-match-game
python3 -m http.server 8000
# Open http://localhost:8000
```

### Node.js
```bash
cd /home/md/language/experiment/coding-challenge-04/memory-match-game
npx http-server -p 8000 -o
# Browser opens automatically
```

## Run Tests

```bash
cd /home/md/language/experiment/coding-challenge-04/memory-match-game
npm test
```

All 24 tests should pass!

## Features to Try

1. **Change Difficulty**: Easy (4×4) → Medium (6×6) → Hard (8×8)
2. **Switch Themes**: Try all 5 themes (Emojis, Animals, Food, Space, Sports)
3. **Beat Your Score**: Try to complete with fewer moves and less time
4. **Sound Effects**: Click the speaker icon to toggle sound on/off
5. **Win Celebration**: Complete the game to see confetti!

## Keyboard Shortcuts

- Press `N` to start a new game
- Press `Escape` to close the win modal

## Tips

- Start with Easy difficulty to learn the game
- Remember card positions after flipping
- Try to beat your personal best score!

---

**Enjoy the game!** 🧠🎮
