# Word Warrior - Typing Battle Game

A dynamic typing game where you type words to cast spells and defeat enemies! Longer and harder words deal more damage. Build combos for massive damage multipliers and become a true Word Warrior.

## Quick Start - Play Now

### Easiest Way to Play
1. **Open this file in your browser**: `word-warrior.html`
2. **Click "Start Game"**
3. **Type words and press Enter** to cast spells
4. **Defeat all enemies** to win!

That's it! No installation, no build process, just open and play.

## Game Features

### Core Gameplay
- **Type to Attack**: Type dictionary words to cast spells and damage enemies
- **Word Difficulty**: Longer and more complex words deal more damage
- **Combo System**: Chain together successful words to build massive damage multipliers (max 2x at combo 10)
- **Categories**: Words are organized by element (Fire, Ice, Earth, Lightning, Healing)

### Character System
- **Progressive Leveling**: Gain experience and level up during battles
- **Health & Mana**: Manage your health and mana resources
- **Stat Growth**: Health and mana increase as you level up
- **Accuracy Tracking**: Maintain high accuracy for better performance

### Enemy Progression
- **Multiple Enemy Types**: Face regular enemies, bosses, and legendary monsters
- **Increasing Difficulty**: Each enemy is stronger than the last
- **Boss Battles**: Special boss encounters with high damage and health
- **Experience Rewards**: Defeat enemies to gain experience and level up

### Word Categories & Effects
- **Fire Words**: Deal direct damage (damage = basePoints × difficulty × speedBonus × comboMultiplier)
- **Ice Words**: Reduce enemy attack power for the next turn
- **Earth Words**: Solid defense-oriented spells
- **Lightning Words**: High-speed electrical attacks
- **Healing Words**: Restore your health instead of dealing damage

### Combo System
- **Building Combos**: Each successful word increases your combo counter
- **Damage Multiplier**: Every 5 hits adds 0.5x multiplier (max 2.0x at 10 hits)
- **Combo Bonus**: Earn bonus points for maintaining combos
- **Resetting**: Miss a word and lose your combo

### Speed Bonuses
- **Perfect (≤100ms)**: 2.0x damage multiplier
- **Great (≤200ms)**: 1.8x damage multiplier
- **Good (≤300ms)**: 1.5x damage multiplier
- **Fair (≤500ms)**: 1.2x damage multiplier
- **Slow (≤750ms)**: 1.0x damage multiplier
- **Very Slow (>750ms)**: 0.8x damage multiplier

## How to Play

### Starting a Battle
1. Click "Start Game" to begin
2. You'll face your first enemy (Goblin)
3. Type words to attack

### During Battle
1. **Type a word** from the dictionary
2. **Press Enter** or click "Type" button
3. If the word is valid:
   - You deal damage to the enemy
   - You gain experience
   - Your combo increases
4. If the word is invalid:
   - Your combo resets
   - You lose some accuracy

### Enemy Turn
After each valid word:
- The enemy automatically attacks you
- You take damage equal to enemy attack × (1.5 for bosses)
- Battle ends if you die or enemy dies

### Victory
- Defeat all 6 enemies to become a true Word Warrior
- Final boss is the Shadow Lord
- Your final score is displayed

### Defeat
- If your health reaches 0, you're defeated
- Click "Play Again" to restart

## Word Dictionary

The game includes 40+ common words across 5 categories:

### Fire Words (Attack)
flame, burn, fire, spark, heat, scorch, torch, ember, blaze, inferno, etc.

### Ice Words (Defense/Slow)
ice, cold, snow, freeze, frost, chill, frozen, crystal, blizzard, glacial, etc.

### Earth Words (Solid)
stone, earth, rock, ground, quake, boulder, mountain, tremor, solid, granite, etc.

### Lightning Words (Speed)
lightning, thunder, bolt, shock, electric, power, energy, zap, arc, charge, etc.

### Healing Words (Restoration)
heal, cure, mend, help, aid, potion, remedy, restore, revive, blessing, etc.

## Enemies You'll Face

| Enemy | Health | Attack | Level | Type |
|-------|--------|--------|-------|------|
| Goblin | 20 | 5 | 1 | Regular |
| Orc | 35 | 8 | 1 | Regular |
| Troll | 50 | 12 | 2 | Regular |
| Drake | 75 | 15 | 3 | Regular |
| Demon | 100 | 20 | 4 | Regular |
| Shadow Lord | 150 | 25 | 5 | Boss |

Boss enemies deal 1.5x damage!

## Game Statistics

- **Total Words**: 40+ words in base dictionary (500+ with full game)
- **Difficulty Levels**: 1-5 stars
- **Number of Enemies**: 6 (5 regular + 1 boss)
- **Experience Points**: Level up with continuous play
- **Combo Multiplier**: Up to 2.0x

## Strategic Tips

### Effective Tactics
1. **Type Fast**: Shorter time = higher damage bonus (up to 2x)
2. **Build Combos**: Keep hitting valid words to stack multipliers
3. **Use Healing**: When health is low, type healing words
4. **Category Strategy**: Mix different word categories for balanced approach
5. **Plan Ahead**: Know which words are available before you need them

### Winning Strategy
1. Start with simple, fast words (flame, ice, bolt) to build combo
2. Use longer words (inferno, blizzard, lightning) for big damage when combo is high
3. Use healing words (heal, cure) when health drops below 30%
4. Type words quickly - speed equals damage!
5. Don't break your combo early - maintain rhythm

### Combo Management
- First 5 words: 1.0x multiplier (0 bonus points)
- Words 6-10: Up to 1.5x multiplier (8-10 bonus points)
- Words 11+: Up to 2.0x multiplier (18+ bonus points)

## Scoring

Your score is calculated as:
```
Points = Damage + ComboBonus + SpeedBonus
Score = Sum of all Points
```

## Controls

| Input | Action |
|-------|--------|
| Type | Enter word |
| Enter Key | Submit word |
| Click "Type" | Submit word |
| Click "Start Game" | Begin game |
| Click "Play Again" | Restart after defeat |

## File Structure

```
word-warrior/
├── word-warrior.html          ← Main game file (open this!)
├── src/
│   └── word-warrior-core.ts   ← Game engine source (TypeScript)
├── word-warrior.test.ts       ← Comprehensive test suite (73 tests)
├── WORD_WARRIOR_README.md     ← This file
├── package.json               ← Dependencies
└── wrk_journals/              ← Development journal
```

## Technical Details

### Technology Stack
- **Frontend**: HTML5, CSS3, Canvas API, Vanilla JavaScript
- **Language**: TypeScript (compiled to JavaScript)
- **Testing**: Jest (73 comprehensive tests)
- **No Dependencies**: Game runs completely standalone

### Game Engine Architecture
- **Core Systems**: Word, Enemy, Player, Battle, ComboSystem, WordDatabase
- **Game Manager**: Handles progression through multiple battles
- **Rendering**: Canvas-based 2D graphics
- **Input Handling**: Keyboard and mouse controls

### Performance
- **Target FPS**: 60 FPS (16.67ms frame time)
- **Bundle Size**: Single HTML file (~50KB)
- **Browser Support**: All modern browsers (Chrome, Firefox, Safari, Edge)
- **Load Time**: Instant (no network requests)

## Testing

The game includes 73 comprehensive unit and integration tests covering:
- Word creation and damage calculation
- Enemy health and combat mechanics
- Player stats and progression
- Combo system and multipliers
- Word database validation
- Full battle mechanics
- Game progression and completion

Run tests with:
```bash
npm install
npm test
```

All tests pass!

## Development

### Built with TDD
This game was developed using Test-Driven Development (TDD):
1. Write failing test → 2. Make it pass → 3. Refactor → 4. Repeat

### See the Source
- **Game Engine**: `src/word-warrior-core.ts` (1150 lines)
- **Tests**: `word-warrior.test.ts` (600+ lines, 73 tests)
- **HTML Game**: `word-warrior.html` (800 lines, embedded game)

## Game Flow

```
START
  ↓
Select Difficulty (Implicit)
  ↓
Battle 1: Goblin (Health: 20, Attack: 5)
  ↓
Battle 2: Orc (Health: 35, Attack: 8)
  ↓
Battle 3: Troll (Health: 50, Attack: 12)
  ↓
Battle 4: Drake (Health: 75, Attack: 15)
  ↓
Battle 5: Demon (Health: 100, Attack: 20)
  ↓
BOSS BATTLE: Shadow Lord (Health: 150, Attack: 25)
  ↓
VICTORY!
```

## Tips for High Scores

### Scoring Multipliers
1. **Base Damage**: Word difficulty × word basePoints
2. **Speed Bonus**: 0.8x - 2.0x depending on typing speed
3. **Combo Multiplier**: 1.0x - 2.0x (one multiplier per 5 hits)
4. **Combo Bonus**: 0-18+ points per 5-hit combo

### Maximizing Score
- Type as fast as possible (≤100ms = 2.0x bonus)
- Build long combos (every 5 hits adds 0.5x multiplier)
- Use difficulty-5 words when combo is high (Conflagration, etc.)
- Never break your combo unnecessarily

## Troubleshooting

### Game Won't Start
- Make sure JavaScript is enabled in your browser
- Try a different browser (Chrome, Firefox, Safari, Edge)
- Refresh the page (F5 or Cmd+R)

### Words Not Being Recognized
- Check spelling (must match dictionary exactly)
- Dictionary is case-insensitive (FLAME = flame = Flame)
- Common words: flame, ice, thunder, heal, stone, bolt, etc.

### Can't Type Faster
- Make sure input focus is on the text field
- Click on the input box first if not focusing
- Try clearing browser extensions

### Performance Issues
- Close other browser tabs
- Reduce browser rendering quality
- Try a different browser

## Browser Compatibility

**Fully Supported:**
- Google Chrome 90+
- Mozilla Firefox 88+
- Apple Safari 14+
- Microsoft Edge 90+

**Requirements:**
- HTML5 Canvas support
- ES2020 JavaScript support
- No plugins needed

## Credits

**Word Warrior** - Game Competition Entry
Created: 2025-11-07
Language: TypeScript
Platform: Web (HTML5 Canvas)
Testing: Jest
Status: Complete & Fully Tested

## License

MIT License - Open for learning and modification

---

## Quick Reference

### Game States
- **Menu**: Before starting
- **Battle**: Fighting enemies
- **Victory**: Defeated boss
- **Defeat**: Lost all health

### Key Statistics to Track
- **Health**: Current/Max
- **Mana**: Current/Max
- **Level**: Player level
- **Combo**: Current hit count
- **Score**: Total points
- **Accuracy**: Success rate

### Always Remember
- Type fast = more damage (up to 2x)
- Build combos for multipliers (up to 2x)
- Use healing words strategically
- Different words have different power
- Bosses deal extra damage (1.5x)

---

**Ready to become a Word Warrior? Open word-warrior.html and start typing!**

Type fast. Type smart. Type to victory.

