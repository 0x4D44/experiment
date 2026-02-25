# Chess Puzzle Rush - Quick Start Guide

## Play Right Now!

Simply open **`chess-puzzle-rush.html`** in your web browser. That's it!

No installation, no build process, no dependencies needed.

## How to Play (30 seconds)

1. **See a chess puzzle?** You need to deliver checkmate in 1-3 moves
2. **Click a piece** to select it (it turns blue and shows valid moves in green)
3. **Click a destination** to move the piece
4. **Make the exact sequence of moves** shown in the solution
5. **Click "Submit Solution"** when done
6. **Get points!** Score = Difficulty × Speed × Combo Multiplier

## Game Controls

| Action | How |
|--------|-----|
| **Move piece** | Click piece → Click destination |
| **Submit** | Click "✓ Submit Solution" button |
| **Get hint** | Click "💡 Get Hint" (-100 point penalty) |
| **Reset puzzle** | Click "↺ Reset Puzzle" to start over |
| **Next puzzle** | Click "→ Next Puzzle" to continue |

## Scoring Examples

### Example 1: Solve Mate-in-1 in 10 seconds
- Base: 50 points (mate in 1)
- Speed bonus: 250 points (50 seconds remaining × 5)
- Combo: 1.0x (first solve)
- **Total: 300 points**

### Example 2: Solve Mate-in-2 in 30 seconds with 5x combo
- Base: 100 points (mate in 2)
- Speed bonus: 150 points (30 seconds remaining × 5)
- Combo: 1.4x (5 in a row = 1 + 0.4)
- **Total: 350 points**

### Example 3: Use hint, solve Mate-in-3 in 45 seconds
- Base: 150 points (mate in 3)
- Speed bonus: 75 points (15 seconds remaining × 5)
- Hint penalty: -100 points
- Combo: 1.0x (first solve)
- **Total: 125 points**

## Tips for Success

1. **Start slow** - Read the puzzle, understand the pattern
2. **Think ahead** - Preview your moves before clicking submit
3. **Build combos** - Chain correct solutions for 2x multiplier
4. **Save hints** - Only use when truly stuck (-100 penalty is steep)
5. **Watch the timer** - Red zone (< 10 seconds) = rush decisions pay off

## Common Chess Checkmate Patterns

### Back Rank Mate
- King trapped on edge with own pieces blocking
- Solution: Move rook/queen to back rank

### Smothered Mate
- King blocked by own pawns
- Solution: Knight delivers mate nearby

### Two Rook Mate
- Rooks control ranks/files
- Solution: Rooks work together to trap king

### Diagonal Mate
- Bishop or queen controls long diagonal
- Solution: Move to deliver mate on color

## Puzzle Difficulty

| Level | Type | Difficulty |
|-------|------|------------|
| ⭐ | Mate in 1 | Easy - Find the winning move |
| ⭐⭐ | Mate in 2 | Medium - Calculate one move ahead |
| ⭐⭐⭐ | Mate in 3 | Hard - Need deep calculation |

## Browser Support

Works in:
- Chrome/Chromium 90+
- Firefox 88+
- Safari 14+
- Microsoft Edge 90+

Requires HTML5 Canvas.

## Not Working?

Try:
1. Refresh the page (F5)
2. Clear browser cache
3. Disable browser extensions
4. Try a different browser
5. Make sure JavaScript is enabled

## Want to Know More?

- **Full game rules** → See `CHESS_PUZZLE_RUSH_README.md`
- **Technical details** → See `CHESS_PUZZLE_RUSH_SUBMISSION.md`
- **Test results** → Run: `npm test -- chess-puzzle-rush.test.ts`

## Challenge Yourself

Can you:
- [ ] Solve all 25 puzzles?
- [ ] Get a 10x combo streak?
- [ ] Score over 5,000 points?
- [ ] Solve a mate-in-3 in under 15 seconds?
- [ ] Get a perfect score (no hints)?

## Game Features

✅ 25+ unique checkmate puzzles
✅ 60-second timer per puzzle (creates pressure!)
✅ Score multiplier for consecutive solves (combos)
✅ Difficulty progression (1-move → 3-move mates)
✅ Hint system (reveals first move)
✅ Visual board with move validation
✅ Real-time score tracking
✅ Multiple puzzle categories

## Have Fun!

This game combines:
- **Chess knowledge** - Know your checkmate patterns
- **Calculation skill** - Think ahead several moves
- **Time pressure** - Fast solving = more points
- **Consistency** - Building combos rewards streaks

Good luck, and enjoy the puzzle rush!

---

**Pro Tip**: Watch the combo counter! Every correct solution increases your multiplier. Miss one and it resets to 1.0x. This creates exciting risk/reward decisions on tricky puzzles.

**Strategy**: Start with Difficulty 1 (mate in 1) to build score and combo, then tackle harder puzzles to rack up big points!

Open `chess-puzzle-rush.html` now and start solving! ♞
