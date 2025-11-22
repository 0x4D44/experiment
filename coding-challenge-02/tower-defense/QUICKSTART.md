# 🚀 Quick Start Guide

## Play Now (3 Simple Steps)

### Option 1: Direct Browser Open
```bash
cd /home/md/language/experiment/coding-challenge-02/tower-defense
# Then open index.html in your browser
```

### Option 2: Use Launch Script
```bash
cd /home/md/language/experiment/coding-challenge-02/tower-defense
./launch.sh
# Opens a local server at http://localhost:8000
```

### Option 3: Python Server
```bash
cd /home/md/language/experiment/coding-challenge-02/tower-defense
python3 -m http.server 8000
# Visit http://localhost:8000 in your browser
```

## First Game (2 Minutes)

1. **Welcome Screen** - Click "🚀 Start Playing"

2. **Select Tower** - Click on "🎯 Basic Tower" card (50 gold)

3. **Place Tower** - Click on any empty green grid cell (NOT on the gray path)

4. **Start Wave** - Click "🚀 Start Wave" button

5. **Watch** - Your towers will automatically attack enemies!

6. **Build More** - Earn gold from kills, place more towers

7. **Upgrade** - Click existing tower, then "⬆️ Upgrade" button

8. **Survive** - Defend through all 10 waves to win!

## Tower Strategy Guide

### Best First Tower: Basic Tower (50g)
- Good balance of damage and cost
- Place at corner where path turns
- Enemies slow down naturally = more shots

### Early Game (Waves 1-3)
```
Gold: 200 → 300+
Strategy: Place 3-4 Basic Towers covering the path
```

### Mid Game (Waves 4-7)
```
Gold: 500+
Strategy: Add Rapid Towers, start upgrading
```

### Late Game (Waves 8-10)
```
Gold: 1000+
Strategy: Sniper Towers for bosses, Frost Towers to slow
```

## Quick Reference

### Tower Types (Press 1-5 to select)
| Key | Tower | Cost | Best For |
|-----|-------|------|----------|
| 1 | 🎯 Basic | 50g | All-around |
| 2 | ⚡ Rapid | 70g | Fast enemies |
| 3 | 💣 Splash | 100g | Groups |
| 4 | 🔭 Sniper | 120g | Bosses |
| 5 | ❄️ Frost | 80g | Slowing |

### Keyboard Shortcuts
- `Space` / `P` - Pause
- `S` - Start Wave
- `Esc` - Deselect
- `1-5` - Select tower type

### Game Stats
- **Starting Gold:** 200
- **Starting Lives:** 20
- **Total Waves:** 10
- **Win Condition:** Survive all waves
- **Lose Condition:** Lives reach 0

## Test the Game

### Run Test Suite
```bash
# Open test.html in browser
cd /home/md/language/experiment/coding-challenge-02/tower-defense
open test.html  # or xdg-open test.html on Linux
```

Expected: 30+ tests, 100% pass rate

## Troubleshooting

### Game won't load
- ✅ Check browser console (F12) for errors
- ✅ Try different browser (Chrome recommended)
- ✅ Make sure JavaScript is enabled

### No sound
- ✅ Check browser volume settings
- ✅ Web Audio API requires user interaction first
- ✅ Click "Start Playing" button to enable audio

### Performance issues
- ✅ Close other tabs
- ✅ Try game speed control (toggle with speed button)
- ✅ Reduce browser zoom level

### Towers won't place
- ✅ Make sure you selected a tower from left panel
- ✅ Don't place on gray path (only on green grid)
- ✅ Check if you have enough gold
- ✅ Can't place where tower already exists

## File Overview

```
tower-defense/
├── index.html      # Main game (open this!)
├── style.css       # Visual styling
├── game.js         # Game engine (1,550 lines)
├── test.html       # Test suite
├── README.md       # Full documentation
├── FEATURES.md     # Feature checklist
├── QUICKSTART.md   # This file
└── launch.sh       # Server launcher
```

## What to Show Judges

1. **Visual Polish**
   - Open game, show gradient UI
   - Demonstrate smooth animations
   - Show particle effects when towers shoot

2. **Complete Features**
   - All 5 tower types working
   - All 5 enemy types in different waves
   - Upgrade system with visual feedback
   - Economy system with gold/lives

3. **Code Quality**
   - Show game.js organization (classes, comments)
   - Run test.html (30+ passing tests)
   - Show README.md (comprehensive docs)

4. **Gameplay**
   - Play through 2-3 waves
   - Show tower placement and upgrades
   - Demonstrate pause, speed control
   - Show win/lose conditions

5. **Bonus Features**
   - Sound effects (procedural audio)
   - Keyboard shortcuts
   - Game speed control
   - Tower statistics

## Win Strategy

**Wave 1-2:** Basic Towers only, save gold
**Wave 3-4:** Add 1-2 Rapid Towers
**Wave 5-6:** Upgrade best positioned towers
**Wave 7-8:** Add Frost Tower + Sniper Tower
**Wave 9-10:** Max upgrades, strategic selling/replacing

**Key Tip:** Corners are golden! Place towers where enemies turn.

## Need Help?

- 📖 Read README.md for detailed info
- 🧪 Check test.html to verify functionality
- 📝 See FEATURES.md for complete feature list
- 🎮 Practice mode: Just restart and experiment!

---

**Good luck defending your base! 🏰⚔️**

**Estimated reading time:** 5 minutes
**Estimated first game time:** 10 minutes
**Estimated mastery time:** 30 minutes
