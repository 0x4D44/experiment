# Memory Quest RPG

A memory card game with RPG elements. Match cards to progress through an epic fantasy adventure, unlock spells and items, and defeat mighty bosses!

## Game Overview

Memory Quest RPG combines the classic memory card matching mechanics with RPG progression systems. Players select their character class and complete increasingly difficult levels by matching pairs of spell, item, and monster cards.

### Features

- **Three Character Classes**
  - Warrior: High HP and Defense
  - Mage: High Mana and Attack
  - Rogue: High Speed and mobility

- **Progressive Difficulty**: 5 levels with increasing board sizes
  - Level 1: 2x2 board (4 cards)
  - Level 2: 3x2 board (6 cards)
  - Level 3: 3x3 board (9 cards)
  - Level 4: 4x3 board (12 cards)
  - Level 5: 4x4 board (16 cards)

- **Rich Card System**
  - Spells: Fireball, Ice Storm, Lightning Bolt, Healing Light, Time Warp, Meteor Strike
  - Items: Health Potion, Mana Elixir, Iron Sword, Dragon Scale, Ancient Amulet, Treasure Chest
  - Monsters: Goblin, Orc, Troll, Vampire, Dragon, Ancient Evil

- **Story Progression**: Immersive narrative as you progress through levels

- **Boss Encounters**: Face bosses at each level with unique stats and rewards

## How to Play

### Starting the Game

1. Open `memory-quest-rpg.html` in a web browser
2. Select your character class (Warrior, Mage, or Rogue)
3. The game begins!

### Gameplay

1. **Matching Cards**: Click on cards to flip them and reveal what's underneath
2. **Find Pairs**: Match two cards with the same name to earn a point
3. **Progress Tracking**: The progress bar shows how many matches you've made
4. **Complete the Level**: Match all pairs to complete the level and progress to the next one
5. **Win the Game**: Complete all 5 levels to save Aethoria and win!

### Game Controls

- **Click a card**: Flip it to reveal
- **Show Hint button**: Get helpful tips
- **Continue button**: Move to the next level after completing a level

## Technical Details

### Architecture

The game is built with TypeScript using an object-oriented architecture:

- **Card.ts**: Individual card representation with reveal/match state
- **Board.ts**: Game board management with shuffle and match detection
- **Character.ts**: Player character with stats and progression
- **Battle.ts**: Battle system for future boss encounters
- **Game.ts**: Main game manager orchestrating all systems

### Test Coverage

The project includes comprehensive unit and integration tests:

- **Card Tests** (13 tests): Card creation, revealing, matching
- **Board Tests** (25 tests): Board initialization, card flipping, match detection, progress tracking
- **Character Tests** (24 tests): Character creation, stats, leveling, inventory management
- **Battle Tests** (18 tests): Combat mechanics, damage/healing calculations, battle flow
- **Game Tests** (23 tests): Game flow, level progression, character selection

**Total**: 103 tests, all passing

### Running Tests

To run the test suite:

```bash
npm install
npm test -- --testPathPattern="memory-quest"
```

To run with coverage:

```bash
npm test -- --testPathPattern="memory-quest" --coverage
```

### Building

To compile TypeScript:

```bash
npm run build
```

To watch for changes:

```bash
npm run dev
```

## File Structure

```
memory-quest/
├── Card.ts                 # Card class
├── Card.test.ts           # Card tests
├── Board.ts               # Board class
├── Board.test.ts          # Board tests
├── Character.ts           # Character class
├── Character.test.ts      # Character tests
├── Battle.ts              # Battle system
├── Battle.test.ts         # Battle tests
├── Game.ts                # Main game manager
└── Game.test.ts           # Game tests

../
├── memory-quest-rpg.html  # Playable game interface
└── MEMORY_QUEST_RPG_README.md  # This file
```

## Game Mechanics

### Card Matching

- Cards are shuffled randomly each game
- Match pairs with identical names to score points
- No time limit - take your time to remember card positions

### Character Progression

Characters gain experience and level up:
- Gain 50 XP per level completed (scaled by level difficulty)
- Level up when reaching experience threshold
- Stats improve with each level based on class

### Difficulty Scaling

Each level introduces:
- More cards to match
- New cards from the deck
- Increasing experience requirements

### Boss Battles (Future Feature)

After completing memory challenges:
- Face increasingly difficult bosses
- Use accumulated items and spells
- Earn rewards for victories

## Development Notes

### Design Decisions

1. **Object-Oriented Design**: Clear separation of concerns with Card, Board, Character, and Battle classes
2. **Test-Driven Development**: All systems tested before implementation
3. **Flexible Architecture**: Easy to add new card types, bosses, and abilities
4. **Responsive UI**: HTML5 game works on desktop and mobile

### Known Limitations

- Battle system implemented but not integrated into HTML interface (planned for expansion)
- No persistence/save system (state is cleared on refresh)
- No multiplayer functionality

### Future Enhancements

- Complete boss battle integration into HTML interface
- Inventory system UI
- Character stat customization
- Leaderboard/high scores
- Sound effects and animations
- Difficulty settings
- Custom card themes

## Credits

Created for the Game Development Competition Round 2.

Built with:
- TypeScript
- Jest (testing)
- HTML5/CSS3
- Vanilla JavaScript

## License

MIT License

---

**Enjoy your adventure in Aethoria!** 🎮✨
