# Terminal Roguelike Dungeon Crawler

A fully-featured, polished terminal-based roguelike dungeon crawler written in Rust. Explore procedurally generated dungeons, battle diverse enemies, collect loot, and see how deep you can descend!

## Features

### Core Gameplay
- **Procedurally Generated Dungeons**: Every playthrough is unique with randomly generated rooms and corridors
- **Turn-Based Combat**: Strategic combat system with attack/defense mechanics
- **Character Progression**: Gain XP, level up, and become stronger
- **Multiple Enemy Types**: Face goblins, orcs, trolls, and dragons - each with unique stats
- **Intelligent AI**: Enemies use A* pathfinding to hunt you down
- **Field of View/Fog of War**: Advanced shadowcasting algorithm reveals only what you can see

### Items & Equipment
- **Weapons**: Daggers, swords, axes, and more to boost your attack
- **Armor**: Leather, chain mail, and plate armor for defense
- **Potions**: Healing items to survive tough battles
- **Inventory System**: Manage up to 10 items with quick-use hotkeys

### Visual & UI
- **Beautiful ASCII Art**: Colorful terminal graphics using Unicode characters
- **Smooth Rendering**: Clean, responsive interface built with crossterm
- **Real-time Stats Display**: Track HP, attack, defense, and XP
- **Message Log**: Detailed combat and action feedback
- **High Score System**: Compete for the best scores across playthroughs

## Installation

### Prerequisites
- Rust 1.70 or higher (install from [rustup.rs](https://rustup.rs/))

### Build from Source

```bash
cd terminal-roguelike
cargo build --release
```

## How to Play

### Starting the Game

```bash
cargo run --release
```

Or run the compiled binary:

```bash
./target/release/terminal_roguelike
```

### Controls

#### Movement
- **Arrow Keys** or **hjkl** (Vi-style): Move in cardinal directions
- **y, u, b, n**: Diagonal movement

#### Actions
- **g** or **,**: Pick up item at your current position
- **>**: Descend stairs to the next dungeon level
- **1-9**: Use/equip item from inventory slot
- **q** or **ESC**: Quit game

#### Game Over
- **n**: Start a new game
- **q**: Quit to desktop

## Gameplay Tips

1. **Manage Your Health**: Don't rush into combat with low HP. Use potions strategically.

2. **Equipment Matters**: Weapons increase attack damage, armor reduces incoming damage. Equip them by using the corresponding inventory number.

3. **Know Your Enemies**:
   - **Goblins** (green 'g'): Weak, good for early XP
   - **Orcs** (orange 'o'): Moderate threat
   - **Trolls** (brown 'T'): Strong, high HP
   - **Dragons** (red 'D'): Extremely dangerous, huge rewards

4. **Explore Thoroughly**: Search every room for items and equipment before descending.

5. **Level Up**: Each level makes you significantly stronger. Don't avoid combat entirely!

6. **Line of Sight**: You can only see enemies within your field of view. They can ambush you from unexplored areas.

7. **Fog of War**: Gray areas have been explored but aren't currently visible. Plan your retreat routes!

## Game Mechanics

### Combat System
- Damage = (Attacker's Attack + Random(0-5)) - Defender's Defense
- Minimum damage is always 1
- Equipment bonuses stack with base stats

### Character Progression
- Gain XP by defeating enemies
- Each level requires: Level × 100 XP
- Level up bonuses:
  - +10 Max HP (fully healed)
  - +2 Attack
  - +1 Defense

### Scoring
- Base Score: Dungeon Level × 100
- XP Score: Total XP earned
- High scores persist between sessions

## Technical Details

### Architecture
- **Entity-Component System**: Clean separation of game logic
- **A* Pathfinding**: Efficient enemy AI
- **Shadowcasting FOV**: Industry-standard field of view algorithm
- **Procedural Generation**: Room-based dungeon creation with corridor connections

### Testing
Run the comprehensive test suite:

```bash
cargo test
```

56 unit tests covering:
- Dungeon generation
- Combat mechanics
- AI pathfinding
- Inventory system
- Field of view calculations
- Item interactions
- Character progression

### Performance
- Lightweight: ~3MB binary
- Fast: 60+ FPS rendering
- Low resource usage
- No external dependencies beyond Rust ecosystem

## Project Structure

```
terminal-roguelike/
├── src/
│   ├── main.rs          # Entry point and input handling
│   ├── game.rs          # Core game state and logic
│   ├── map.rs           # Dungeon generation
│   ├── entity.rs        # Entities, positions, and stats
│   ├── combat.rs        # Combat system
│   ├── items.rs         # Item types and generation
│   ├── ai.rs            # Enemy AI and pathfinding
│   ├── fov.rs           # Field of view calculations
│   ├── ui.rs            # Terminal rendering
│   └── highscore.rs     # Score persistence
├── Cargo.toml           # Dependencies
└── README.md            # This file
```

## Dependencies

- **crossterm** (0.27): Cross-platform terminal manipulation
- **rand** (0.8): Random number generation
- **serde** (1.0): Serialization for high scores
- **serde_json** (1.0): JSON storage

## Building for Distribution

### Release Build
```bash
cargo build --release
```

The optimized binary will be in `target/release/terminal_roguelike`

### Size Optimization (Optional)
For an even smaller binary, add to `Cargo.toml`:

```toml
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
strip = true
```

Then rebuild:
```bash
cargo build --release
```

## Troubleshooting

### Terminal Issues
- **Colors not showing**: Ensure your terminal supports 24-bit color (most modern terminals do)
- **Rendering glitches**: Try resizing your terminal to at least 120x50 characters
- **Input lag**: This shouldn't happen, but try a different terminal emulator if it does

### Game Won't Start
- Verify Rust is installed: `rustc --version`
- Clean and rebuild: `cargo clean && cargo build --release`
- Check terminal size is adequate (minimum 100x50)

## Credits

Created for the Coding Challenge 03 - A fully functional roguelike game built from scratch in Rust.

### Algorithms & Techniques
- Shadowcasting FOV based on work by Björn Bergström
- A* pathfinding implementation
- Room-based dungeon generation

## License

MIT License - Feel free to learn from, modify, and share!

## Future Enhancements (Not Implemented)

Ideas for extending the game:
- Save/load game state
- More item types (scrolls, rings, etc.)
- Character classes
- Spells and magic
- Boss monsters
- Hunger system
- More dungeon generation algorithms
- Sound effects
- Multiple dungeon themes

---

**Enjoy your adventure in the depths!**

For issues or suggestions, please review the code and submit improvements.
