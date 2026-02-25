# Poker Quest - START HERE

## Quick Start (30 seconds)

1. **Open `poker-quest.html` in your web browser**
2. Click "START NEW RUN"
3. Select 5 cards and click "ATTACK"
4. Battle through 10 encounters to win!

That's it! The game is fully playable with no installation required.

---

## What You Get

### The Game
- **poker-quest.html** - Fully playable roguelike game (open in browser)
- **poker-quest.ts** - Clean TypeScript source code
- **poker-quest.test.ts** - 35 passing unit tests (100% pass rate)

### Documentation
- **POKER_QUEST_README.md** - Complete game guide
- **POKER_QUEST_SUBMISSION.md** - Submission overview
- **POKER_QUEST_FINAL_REPORT.md** - Technical details
- **This file** - Quick start guide

### Development Journal
- **wrk_journals/2025.11.07 - JRN - Poker Roguelike Development.md** - Development process

---

## Game Overview

**Poker Quest** is a roguelike game where you:

1. **Draw poker hands** from your deck
2. **Battle enemies** with hand strength determining damage
3. **Earn gold** from victories
4. **Buy upgrades** in the shop to survive
5. **Defeat 10 encounters** to beat the Dragon Lord and win

### Core Mechanics
- Poker hands deal damage (Royal Flush = 100 damage, High Card = 2)
- Enemies scale in difficulty (50-150 HP)
- Shop system with 4 upgrades
- Deck management between encounters

---

## File Guide

| File | Purpose | Notes |
|------|---------|-------|
| **poker-quest.html** | Play the game | Open in browser, fully self-contained |
| poker-quest.ts | Source code | Game logic in TypeScript |
| poker-quest.test.ts | Tests | 35 unit tests, all passing |
| POKER_QUEST_README.md | How to play | Complete game guide |
| POKER_QUEST_SUBMISSION.md | Submission details | What you're getting |
| POKER_QUEST_FINAL_REPORT.md | Technical report | Architecture and design |
| wrk_journals/*.md | Development journal | Design decisions and process |

---

## Testing

Run tests with:
```bash
npm install
npm test -- poker-quest.test.ts
```

**Results**: 35 tests passing, 0 failures

---

## Game Features

### Poker Hands
- Royal Flush (100 dmg)
- Straight Flush (80 dmg)
- Four of a Kind (60 dmg)
- Full House (50 dmg)
- Flush (35 dmg)
- Straight (30 dmg)
- Three of a Kind (25 dmg)
- Two Pair (15 dmg)
- Pair (8 dmg)
- High Card (2 dmg)

### Encounters
- 10 different enemies
- Scaling difficulty (50 HP → 150 HP boss)
- Different enemy types
- Final boss: Dragon Lord

### Shop Upgrades
- Health Boost (+20 HP, 50 gold)
- Heal (+50 HP, 40 gold)
- Refresh Deck (30 gold)
- Gold Multiplier (100 gold)

---

## Browser Requirements

Works on:
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

No plugins or downloads required - pure HTML5 + JavaScript.

---

## How to Win

1. Defeat 10 encounters
2. Each encounter requires beating an enemy in combat
3. Combat: Select 5 cards → ATTACK → enemy takes damage → repeat
4. Between encounters: Visit shop to buy upgrades
5. Defeat the Dragon Lord at encounter 10 to win!

---

## Development Summary

### Code Quality
- Clean TypeScript implementation
- Full type safety
- No external dependencies (beyond Jest for testing)
- 2000+ lines of production code

### Testing
- 35 comprehensive unit tests
- 100% pass rate
- Coverage of all game systems
- Test-driven development methodology

### Documentation
- Complete README with strategy tips
- Technical architecture report
- Development journal
- Inline code comments

---

## Next Steps

1. **Open `poker-quest.html`** → Play the game
2. **Read POKER_QUEST_README.md** → Learn strategy tips
3. **Run tests** → Verify quality
4. **Check POKER_QUEST_FINAL_REPORT.md** → Understand design

---

## Questions?

- **How to play**: See POKER_QUEST_README.md
- **How it works**: See POKER_QUEST_FINAL_REPORT.md
- **Design decisions**: See wrk_journals/2025.11.07 - JRN - Poker Roguelike Development.md

---

**Ready to play? Open `poker-quest.html` now!**

Enjoy your poker adventure!
