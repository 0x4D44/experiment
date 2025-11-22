# 🎮 Game Flow & Features Demonstration

This document walks through the complete game experience.

## 🌟 Initial Screen

When you first open `index.html`, you'll see:

```
┌─────────────────────────────────────────┐
│           💣 Minesweeper                │
│                                         │
│  [Beginner] [Intermediate] [Expert]    │
│                                         │
│  🚩 010        😊         ⏱️ 000       │
│                                         │
│  ┌───────────────────────────┐         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  │ ░ ░ ░ ░ ░ ░ ░ ░ ░         │         │
│  └───────────────────────────┘         │
│                                         │
│     🏆 Best Times                       │
│     Beginner: N/A                       │
│     Intermediate: N/A                   │
│     Expert: N/A                         │
└─────────────────────────────────────────┘
```

**Features visible:**
- Title with mine emoji
- Three difficulty buttons (Beginner selected by default)
- Mine counter (shows remaining flags)
- Smiley face button (restart)
- Timer (shows elapsed time)
- 9×9 grid of covered cells (beginner)
- High scores section

## 🎯 First Click - Always Safe!

When you click any cell for the first time:

```
Timer starts: 001
Cell reveals safely
If empty area → flood fills!

Example after clicking center:
┌───────────────────────────┐
│ ░ ░ ░ ░ ░ ░ ░ ░ ░         │
│ ░ ░ ░ 1 1 1 ░ ░ ░         │
│ ░ ░ 1       1 ░ ░ ░        │
│ ░ 1       1 1 ░ ░ ░        │
│ ░ 1       1 2 ░ ░ ░        │
│ ░ 1       1 ░ ░ ░ ░        │
│ ░ ░ 1 1 1 1 ░ ░ ░         │
│ ░ ░ ░ ░ ░ ░ ░ ░ ░         │
│ ░ ░ ░ ░ ░ ░ ░ ░ ░         │
└───────────────────────────┘
```

**What happened:**
- First click is guaranteed safe
- Empty cells (blank) auto-reveal
- Numbers show adjacent mine counts
- Flood-fill reveals connected empty area
- Animation plays (scale + fade effect)
- Sound effect plays

## 🚩 Flagging Mines

Right-click cells you think are mines:

```
🚩 008 (counter decreased)

┌───────────────────────────┐
│ 🚩 ░ ░ 1 1 1 ░ ░ ░        │
│ ░ 🚩 ░ 1       1 ░ ░       │
│ ░ ░ 1       1 1 ░ ░ ░      │
│ ░ 1       1 1 ░ ░ ░        │
│ ░ 1       1 2 ░ ░ ░        │
│ ░ 1       1 ░ ░ ░ ░        │
│ ░ ░ 1 1 1 1 ░ ░ ░         │
│ ░ ░ ░ ░ ░ ░ ░ ░ ░         │
│ ░ ░ ░ ░ ░ ░ ░ ░ ░         │
└───────────────────────────┘
```

**Features:**
- Right-click places flag
- Gold/orange highlight
- Mine counter decreases
- Right-click again removes flag
- Flagged cells can't be clicked
- Sound effect plays

## 🎲 Playing the Game

Continue revealing safe cells and flagging mines:

```
⏱️ 045 seconds elapsed
🚩 003 mines remaining

┌───────────────────────────┐
│ 🚩 2 1 1 1 1 ░ 1          │
│ 2 🚩 1   1 2 2 1 1         │
│ 2 2 1   1 🚩 2   1         │
│ 🚩 1     1 2 2 1 1         │
│ 1 1     1 2 🚩 1            │
│   1     1 🚩 2 1            │
│ 1 2 1 1 1 2 🚩 1           │
│ 🚩 2 ░ ░ 1 2 2 1           │
│ 2 🚩 ░ ░ 🚩 1   1          │
└───────────────────────────┘
```

**Number Colors:**
- 1 = Blue
- 2 = Green
- 3 = Red
- 4 = Dark Blue
- 5 = Maroon
- 6 = Cyan
- 7 = Black
- 8 = Gray

## 💥 Clicking a Mine - Game Over

If you click a mine:

```
Smiley: 😵 (game over face)
Timer: Stops
All mines revealed

┌───────────────────────────┐
│ 💣 2 1 1 1 1 ░ 1          │
│ 2 💣 1   1 2 2 1 1         │
│ 2 2 1   1 💣 2   1         │
│ 💣 1     1 2 2 1 1         │
│ 1 1     1 2 💣 1            │
│   1     1 💣 2 1            │
│ 1 2 1 1 1 2 💣 1           │
│ 💣 2 ░ ░ 1 2 2 1           │
│ 2 💣 ░ ░ ❌ 1   1          │
└───────────────────────────┘
```

**What happens:**
- Clicked mine turns RED (explosion animation)
- All other mines revealed (💣)
- Wrong flags show ❌
- Explosion sound plays
- Modal appears after 0.5s

```
┌─────────────────────┐
│                     │
│   💥 Game Over      │
│                     │
│ Better luck next    │
│      time!          │
│                     │
│   [Play Again]      │
│                     │
└─────────────────────┘
```

## 🏆 Winning the Game

When all non-mine cells are revealed:

```
Smiley: 😎 (cool sunglasses)
Timer: Stops at your time
All mines auto-flagged
🚩 010 (all flags placed)

┌───────────────────────────┐
│ 🚩 2 1 1 1 1   1          │
│ 2 🚩 1     1 2 2 1 1       │
│ 2 2 1     1 🚩 2   1       │
│ 🚩 1         1 2 2 1 1     │
│ 1 1         1 2 🚩 1       │
│     1         1 🚩 2 1     │
│ 1 2 1 1 1 2 🚩 1           │
│ 🚩 2 1 1 1 2 2 1           │
│ 2 🚩 1 1 🚩 1     1        │
└───────────────────────────┘
```

**What happens:**
- Victory fanfare (3-tone melody)
- All remaining mines auto-flagged
- Time recorded if new best
- Modal appears

```
┌─────────────────────┐
│                     │
│   🎉 You Won!       │
│                     │
│  Time: 45s          │
│  Difficulty:        │
│    Beginner         │
│  🏆 NEW RECORD!     │
│                     │
│   [Play Again]      │
│                     │
└─────────────────────┘
```

## ⚡ Advanced Feature: Chord Clicking

Middle-click or Shift+Left-click on a revealed number:

```
Before chord click on "2":
┌───────────┐
│ ░ ░ ░     │
│ 🚩 2 ░     │  <- Click here with correct flags
│ 🚩 ░ ░     │
└───────────┘

After chord click:
┌───────────┐
│ 1 1 1     │
│ 🚩 2 1     │  <- All safe neighbors revealed
│ 🚩 1 1     │
└───────────┘
```

**Requirements:**
- Number must be revealed
- Adjacent flags = number shown
- Reveals all non-flagged neighbors
- If wrong flags → detonates mines!

## 📊 Difficulty Comparison

### Beginner (9×9, 10 mines)
```
Grid: Small and manageable
Density: ~12% mines
Time: Typically 30-120 seconds
Ideal for: Learning, quick games
```

### Intermediate (16×16, 40 mines)
```
Grid: Medium challenge
Density: ~16% mines
Time: Typically 120-300 seconds
Ideal for: Experienced players
```

### Expert (30×16, 99 mines)
```
Grid: Large and challenging
Density: ~21% mines
Time: Typically 300-600 seconds
Ideal for: Masters, speedruns
```

## 🎨 Visual States

### Cell States
1. **Covered** - Gray 3D raised button
2. **Revealed** - Flat white/gray surface
3. **Flagged** - Gold/orange with 🚩
4. **Mine** - Red background with 💣
5. **Exploded** - Bright red with pulse animation
6. **Wrong Flag** - Shows ❌ on game over

### Smiley States
1. **😊** - Normal/Playing
2. **😵** - Dead (clicked mine)
3. **😎** - Cool (won game)
4. **Hover** - Scales larger
5. **Click** - Scales smaller

### Animations
1. **Cell Reveal** - Scale up, fade in (0.2s)
2. **Mine Explosion** - Pulse scale (0.5s)
3. **Modal Appear** - Zoom in from center (0.3s)
4. **Button Hover** - Lift up with shadow
5. **Grid Load** - Smooth fade in

## 🔊 Sound Effects

1. **Reveal** - Soft beep (800 Hz, 0.05s)
2. **Flag** - Confirmation beep (1000 Hz, 0.05s)
3. **Win** - 3-note ascending melody
   - Note 1: C5 (523 Hz)
   - Note 2: E5 (659 Hz)
   - Note 3: G5 (784 Hz)
4. **Lose** - Deep explosion (200 Hz, 0.5s)

## 💾 Persistence

### High Scores (localStorage)
```javascript
{
  "beginner": 45,      // Best time in seconds
  "intermediate": 178,
  "expert": 412
}
```

**Features:**
- Saved per browser
- Persists across sessions
- Updates on new records only
- Display on game screen

## 🎯 Win Condition Logic

```
Total Cells = Rows × Cols
Mine Cells = Mine Count
Safe Cells = Total Cells - Mine Cells

Win when: Revealed Cells = Safe Cells

Example (Beginner):
- Total: 9 × 9 = 81 cells
- Mines: 10 cells
- Need to reveal: 71 cells
```

## 🔄 Restart Options

1. **Smiley Button** - Restart current difficulty
2. **Difficulty Button** - Change difficulty (auto-restarts)
3. **Play Again** (modal) - Restart after game over

All reset:
- Timer to 000
- Flags to 000
- Grid to covered
- Generate new mine layout

---

## 🎮 Complete Game Session Example

```
1. Open index.html
   → Beginner mode (9×9, 10 mines)
   → 😊 010 ⏱️ 000

2. Click center cell
   → Timer starts
   → Safe reveal + flood fill
   → 😊 010 ⏱️ 001

3. Right-click suspected mines
   → Flags placed
   → 😊 007 ⏱️ 015

4. Continue revealing safe cells
   → Numbers guide decisions
   → 😊 003 ⏱️ 042

5. Click last safe cell
   → 🎉 You Won!
   → 😎 000 ⏱️ 045
   → New record saved!
   → Modal appears

6. Click "Play Again"
   → New game starts
   → Try to beat 45s!
```

---

🎉 **Enjoy the game!** 🎉
