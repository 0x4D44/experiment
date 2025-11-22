# 🎮 START HERE - Web Tetris

Welcome to the **Web Tetris** game! This guide will get you playing in under 30 seconds.

## ⚡ Quick Play (30 seconds)

### Method 1: Direct Open (Recommended)
1. Double-click `index.html` in your file browser
2. Click "START GAME"
3. Play with arrow keys and SPACE!

### Method 2: Command Line
```bash
cd /home/md/language/experiment/coding-challenge-04/web-tetris
xdg-open index.html  # Linux
# or: open index.html (macOS)
# or: start index.html (Windows)
```

### Method 3: Use Launcher Script
```bash
./PLAY.sh
```

## 🎯 Controls

| Key | Action |
|-----|--------|
| **←** | Move left |
| **→** | Move right |
| **↑** | Rotate |
| **↓** | Soft drop (faster) |
| **SPACE** | Hard drop (instant) |
| **P** | Pause |

## 📚 Documentation

Choose your documentation based on your needs:

### For Players
- **[QUICKSTART.md](QUICKSTART.md)** - Quick start guide with tips

### For Judges/Reviewers
- **[COMPETITION_SUMMARY.md](COMPETITION_SUMMARY.md)** - Competition submission details
- **[DEMO_GUIDE.md](DEMO_GUIDE.md)** - Complete demo walkthrough

### For Developers
- **[README.md](README.md)** - Comprehensive technical documentation
- **[FEATURES.md](FEATURES.md)** - Complete feature list

## 🧪 Run Tests

```bash
# Open test.html in browser
xdg-open test.html
```

Or double-click `test.html` - tests run automatically!

## 📦 What's Included

```
web-tetris/
├── index.html    ← MAIN GAME (open this!)
├── tetris.js     ← Game engine
├── test.html     ← Test suite
├── PLAY.sh       ← Launcher script
└── *.md          ← Documentation files
```

## ✨ Highlights

- ✓ **Zero Setup** - Just open and play!
- ✓ **All 7 Tetrominos** - Classic gameplay
- ✓ **Beautiful UI** - Modern gradient design
- ✓ **Ghost Piece** - Landing preview
- ✓ **40+ Tests** - Comprehensive testing
- ✓ **No Dependencies** - Pure vanilla JS

## 🎮 Game Objective

Stack falling pieces to create complete horizontal lines.
Clear lines to score points and level up.
Don't let the stack reach the top!

## 💯 Scoring

- Single line: 100 × level
- Double: 300 × level
- Triple: 500 × level
- **Tetris (4 lines): 800 × level**

## 🚀 Pro Tips

1. Use **ghost piece** (transparent preview) for precision
2. Check **next piece** panel to plan ahead
3. Try for **Tetris** (4 lines) for max points
4. Use **hard drop** (SPACE) for speed
5. **Avoid gaps** in your stack

---

## 🎯 Ready to Play?

**Open `index.html` and click START GAME!**

Have fun! 🎮✨

---

## Need Help?

- **Quick Start**: Read [QUICKSTART.md](QUICKSTART.md)
- **Full Manual**: Read [README.md](README.md)
- **Features**: Read [FEATURES.md](FEATURES.md)
- **Demo Guide**: Read [DEMO_GUIDE.md](DEMO_GUIDE.md)
