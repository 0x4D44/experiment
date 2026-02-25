# Cyber Breach - Quick Start Guide

## What is Cyber Breach?

A sophisticated hacking-themed puzzle game where you infiltrate virtual networks, solve mini-puzzles, and extract data while evading security detection systems.

## Files Included

### Game Files
- **cyber-breach.html** - Open this in your browser to PLAY the game
- **cyber-breach.ts** - Main game engine (703 lines, TypeScript)
- **cyber-breach.test.ts** - Test suite (64 tests, all passing)

### Documentation
- **CYBER_BREACH_README.md** - Complete gameplay guide and mechanics
- **CYBER_BREACH_SUBMISSION.md** - Competition submission summary
- **CYBER_BREACH_START_HERE.md** - This file

### Development
- **wrk_journals/2025.11.07 - JRN - Network Hacker Development.md** - Full development journal

## How to Play

### Quick Start (2 minutes)
1. Open `cyber-breach.html` in any web browser
2. Select **Level 1** from the dropdown
3. Click **LOAD LEVEL**
4. Click on network nodes to hack them
5. Try to extract enough data before time or detection limit

### Game Overview
- **12 Progressive Levels** - Tutorial to expert difficulty
- **5 Puzzle Types** - Different challenges for different node types
- **5 Hacking Tools** - Limited resources you must use strategically
- **Stealth System** - Detection meter (0-100%, game over at 100%)
- **Time Pressure** - Decreasing time limits push you to act fast

### Controls
- **Click on node** - Hack that network system
- **Type node ID** - Enter node ID and press ENTER to hack
- **Click tool button** - Activate that hacking tool
- **Level selector** - Choose which level to play
- **SCAN NODE** - Get info about a specific node
- **SHOW NETWORK** - View network topology
- **RESTART** - Start over from Level 1

## Game Mechanics

### Node Types
- **Gateway** - Entry point, already hacked
- **Server** - Requires password cracking
- **Database** - Requires encryption breaking
- **SecurityHub** - Requires privilege escalation
- **DataCache** - Requires firewall bypass

### Tools (Limited Uses)
- **Port Scanner** (3 uses) - Reveals vulnerability info
- **Password Cracker** (2 uses) - Cracks passwords
- **Firewall Bypass** (1 use) - Bypasses security hubs
- **Privilege Exploit** (2 uses) - Escalates permissions
- **Network Mask** (infinite) - Reduces detection increase

### Detection System
- Successful hack = +3× node security to detection
- Failed attempt = +2× node security to detection
- Network Mask active = 50% detection reduction
- Game over at 100% detection (alarm triggers)

## Difficulty Progression

| Level | Difficulty | Time | Nodes | Security |
|-------|-----------|------|-------|----------|
| 1-3   | Beginner  | 280s | 3-4   | 2-3      |
| 4-7   | Medium    | 200s | 5-6   | 5-7      |
| 8-10  | Hard      | 100s | 8-9   | 9-10     |
| 11-12 | Insane    | 60s  | 10    | 10       |

## Tips for Success

### Beginner Strategy
- Start with Level 1 to learn mechanics
- Focus on low-security nodes first
- Watch the detection meter
- Use tools on difficult nodes

### Intermediate Strategy
- Plan your infiltration path
- Use tools strategically on hard targets
- Activate Network Mask before risky hacks
- Balance speed vs. safety

### Advanced Strategy
- Optimize node order to minimize detection
- Chain tool usage for maximum effect
- Extract exactly enough data to win
- Speedrun levels for mastery

## Testing

```bash
npm test -- cyber-breach.test.ts
```

Results: **64/64 tests passing** (100% success rate)

## Game Features

✓ 12 complete levels with unique networks
✓ 5 distinct puzzle types (contextual)
✓ 5 specialized hacking tools
✓ Stealth/detection mechanics
✓ Progressive difficulty curve
✓ Time pressure system
✓ Strategic depth and replay value
✓ Professional codebase with full tests
✓ Beautiful terminal-styled UI
✓ Clear gameplay guide

## Technical Details

- **Language**: TypeScript
- **Testing**: Jest (64 tests)
- **Architecture**: Modular systems design
- **Code**: 703 lines (game) + 677 lines (tests)
- **Performance**: 1.5s test execution
- **Quality**: 100% type safety, fully documented

## Game States

- **Playing**: Actively hacking nodes
- **Level Complete**: Extracted enough data
- **Game Over (Time)**: Time limit expired
- **Game Over (Detection)**: Alarm triggered

## What Makes Cyber Breach Special

1. **Sophisticated AI**: Dynamic difficulty through procedural generation
2. **Strategic Depth**: Multiple valid approaches to each level
3. **Balanced Mechanics**: Detection system creates meaningful tension
4. **Professional Quality**: Comprehensive testing and documentation
5. **Thematic Design**: Authentic hacker aesthetic throughout
6. **Scalable Architecture**: Easy to extend with new puzzles/tools/levels

## Files Summary

| File | Purpose | Size |
|------|---------|------|
| cyber-breach.html | Playable game | 28 KB |
| cyber-breach.ts | Game engine | 19 KB |
| cyber-breach.test.ts | Test suite | 21 KB |
| CYBER_BREACH_README.md | Full guide | 9.5 KB |
| Development Journal | Notes | 14 KB |

## Ready to Play?

1. Open **cyber-breach.html** in your browser
2. Select a level and click LOAD LEVEL
3. Click on network nodes to start hacking
4. Good luck, hacker!

---

**Cyber Breach** - Infiltrate networks. Bypass security. Extract data. Evade detection.

For complete information, see **CYBER_BREACH_README.md**
