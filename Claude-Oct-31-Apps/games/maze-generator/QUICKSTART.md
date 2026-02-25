# Maze Generator - Quick Start Guide

## Installation & Running

```bash
# Navigate to project
cd /home/md/language/ClaudeApps/games/maze-generator

# Build and run (first time)
cargo run --release

# Or just run the binary
./target/release/maze-generator
```

## Main Menu

```
1. Play Interactive Maze    - Solve mazes and track stats
2. Generate and Display     - View mazes with solutions
3. Algorithm Information    - Learn about the algorithms
4. Exit                     - Quit the game
```

## Game Controls

| Control | Action |
|---------|--------|
| ↑ W | Move North |
| ← A | Move West |
| ↓ S | Move South |
| → D | Move East |
| H or "hint" | Show next optimal move |
| S or "show" | Reveal complete solution |
| R or "reset" | Restart same maze |
| Q or "quit" | Exit to menu |

## Difficulty Levels

| Level | Size | Time | Use Case |
|-------|------|------|----------|
| Tiny | 10x10 | <1ms | Learning |
| Small | 25x25 | ~5ms | Quick game |
| Medium | 50x50 | ~20ms | Challenge |
| Large | 100x100 | ~100ms | Real test |
| Huge | 250x250 | ~500ms | View only |
| Custom | Any | Varies | Your choice |

## Algorithm Quick Reference

| Algorithm | Speed | Quality | Best For |
|-----------|-------|---------|----------|
| Binary Tree | ⚡⚡⚡ Fastest | Good | Large mazes |
| Kruskal | ⚡⚡ Fast | Excellent | Balanced |
| Recursive Backtracker | ⚡⚡ Fast | Good | Interesting |
| Prim | ⚡ Moderate | Great | Natural look |
| Wilson | ⚡ Moderate | Excellent | Fair distribution |
| Aldous-Broder | 🐢 Slow | Perfect | Hardest mazes |

## Gameplay Tips

**Better Times:**
- Plan your route before moving
- Look for paths toward 'E' (exit)
- Use hints to learn optimal routes
- Practice on smaller mazes first

**Understanding Your Score:**
- **Steps**: Total moves taken
- **Optimal Path**: Shortest possible solution
- **Efficiency**: (Optimal / Actual) * 100%
- **Hints**: Count against your efficiency

## Example Session

```bash
$ ./target/release/maze-generator

╔════════════════════════════════════╗
║     MAZE GENERATOR - MAIN MENU     ║
╠════════════════════════════════════╣
║ 1. Play Interactive Maze           ║
║ 2. Generate and Display Maze       ║
║ 3. Algorithm Information           ║
║ 4. Exit                            ║
╚════════════════════════════════════╝

Select option (1-4): 1

Select Difficulty:
1. Tiny (10x10)
2. Small (25x25)
...

Choice (1-6): 2

Select Algorithm:
1. Recursive Backtracker (winding)
2. Kruskal's (uniform)
...

Choice (1-7): 2

Generating 25x25 maze using Kruskal's Algorithm...
Maze generated! Seed: 1234567890

[Maze appears on screen with S at start, E at end]

Controls: ↑↓←→/WASD Move | H Hint | S Show Solution | R Reset | Q Quit
Move (↑↓←→ or WASD, or H/S/R/Q): d
[Player moves east]

[Continue playing until reaching exit]

═══════════════════════════════
        MAZE COMPLETE!
═══════════════════════════════
Size: 25x25
Steps Taken: 47
Optimal Path: 35
Efficiency: 74.5%
Time: 2m 15s
Hints Used: 1
═══════════════════════════════
```

## Keyboard Shortcut Reference

### Movement
```
W    ↑
A←   D→
S    ↓
```
Or use arrow keys!

### Commands
- H: Hint
- S: Show solution
- R: Reset
- Q: Quit
- B: Toggle breadcrumbs

## Test the Installation

```bash
# Run tests
cargo test --lib

# Expected output:
# test result: ok. 74 passed; 0 failed
```

## Performance

- Binary size: 541 KB
- Memory for 100x100 maze: ~100 KB
- Generation time: <100ms
- Load time: Instant
- Response time: <1ms

## Visual Legend

| Symbol | Meaning |
|--------|---------|
| S | Start position |
| E | End/Exit position |
| @ | Your current position |
| · | Visited cells |
| * | Solution path (when revealed) |
| ─ | Horizontal wall |
| │ | Vertical wall |
| Space | Open passage |

## Common Issues

**"Maze doesn't fit on screen"**
- Use smaller difficulty level
- Increase terminal window size
- Mazes >100x100 may require scrolling

**"Terminal too small"**
- Resize terminal to at least 80x24
- Use custom size with smaller dimensions

**"Algorithm too slow"**
- Choose Binary Tree (fastest)
- Use smaller maze size
- Aldous-Broder is slowest; avoid for large mazes

**"I can't find the exit"**
- Use "hint" to get next step
- Use "show" to see complete solution
- Look for passages leading away from start

## File Locations

- **Binary**: `/home/md/language/ClaudeApps/games/maze-generator/target/release/maze-generator`
- **Source**: `/home/md/language/ClaudeApps/games/maze-generator/src/`
- **Docs**:
  - `README.md` - Full documentation
  - `DEMO.md` - Detailed tutorial
  - `HLD.md` - Technical design
  - `PROJECT_SUMMARY.md` - Implementation report

## Full Documentation

For complete documentation, see:
- **README.md** - Comprehensive user guide
- **DEMO.md** - Walkthrough and examples
- **HLD.md** - Architecture and design

## Quick Facts

- 2,648 lines of Rust code
- 74 comprehensive tests
- 6 maze generation algorithms
- 3 pathfinding algorithms
- 100% safe code (no unsafe blocks)
- Cross-platform (Linux, macOS, Windows)

## Algorithm Recommendations

**Want to play:**
- Easy: Recursive Backtracker on Tiny
- Medium: Kruskal on Small
- Hard: Aldous-Broder on Medium
- Challenge: Custom 75x75 with Aldous-Broder

**Want to learn:**
- See Option 3 for algorithm details
- Try each algorithm on Small size
- Compare solution lengths

**Want speed:**
- Use Binary Tree algorithm
- Increase difficulty for bigger mazes
- Perfect for comparing performance

## Enjoy!

Have fun exploring mazes. Try different algorithms and difficulty levels to find your perfect challenge!

---

**Need help?** Check README.md for detailed documentation.
