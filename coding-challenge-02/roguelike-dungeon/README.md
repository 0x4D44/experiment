# Roguelike Dungeon Crawler

A fully-featured terminal-based roguelike dungeon crawler written in Rust. Explore procedurally generated dungeons, fight monsters, collect loot, and defeat the Dragon Boss!

## Features

### Core Gameplay
- **Procedurally Generated Dungeons**: Each level features unique room layouts connected by corridors using BSP room generation
- **Multi-Level Progression**: Descend through 10 increasingly difficult dungeon levels
- **Turn-Based Combat**: Strategic combat system with attack and defense stats
- **Experience & Leveling**: Gain XP from defeated enemies and level up to become stronger
- **Inventory System**: Collect and manage up to 10 items including potions, weapons, armor, and shields

### Advanced Features
- **Fog of War**: Realistic line-of-sight system - only see what your character can see
- **Multiple Enemy Types**:
  - **Zombies** (Z): Slow but durable
  - **Goblins** (g): Fast and aggressive
  - **Orcs** (O): Heavy hitters
  - **Demons** (D): Powerful late-game enemies
  - **Dragon Boss** (X): Final challenge on level 10
- **Equipment System**: Equip weapons for increased attack and armor/shields for better defense
- **Smart AI**: Enemies chase players with varying behaviors based on type
- **Combat Log**: Track all combat events and messages
- **Colorful Terminal Graphics**: Beautiful colored ASCII art using crossterm

### Win Condition
Reach dungeon level 10, defeat all enemies including the Dragon Boss, and descend the final stairs to claim victory!

## Building and Running

### Prerequisites
- Rust 1.70 or later
- A terminal that supports ANSI color codes

### Build
```bash
cd /home/md/language/experiment/coding-challenge-02/roguelike-dungeon
cargo build --release
```

### Run
```bash
cargo run --release
```

### Run Tests
```bash
cargo test
```

## Controls

### Movement
- **W** or **Up Arrow**: Move North
- **S** or **Down Arrow**: Move South
- **A** or **Left Arrow**: Move West
- **D** or **Right Arrow**: Move East

### Actions
- **G**: Pick up item at current position
- **I**: Open inventory
- **>**: Descend stairs (must defeat all enemies first)
- **Q** or **ESC**: Quit game

### Inventory (when open)
- **1-9**: Use or equip the corresponding item
- **E** or **ESC**: Close inventory

## Gameplay Tips

1. **Explore Carefully**: Use the fog of war to your advantage - enemies can't see you through walls
2. **Manage Your Health**: Pick up health potions before engaging in difficult fights
3. **Equip Gear**: Weapons increase attack, while armor and shields increase defense
4. **Level Up**: Gain XP by defeating enemies to increase your stats
5. **Clear Levels**: You must defeat all enemies before descending to the next level
6. **Boss Strategy**: The Dragon on level 10 is extremely powerful - come prepared with equipment and full health!

## Game Mechanics

### Combat System
- Damage = Attacker's Attack - (Defender's Defense / 2) + Random Variance
- 10% chance of critical hits (double damage)
- Equipment bonuses apply to your attack and defense stats

### Experience and Leveling
- Each level requires 100 * current_level XP
- Leveling up increases:
  - Max HP: +5
  - Attack: +2
  - Defense: +1
- HP is fully restored on level up

### Item Types
- **Health Potion** (!): Restores 20 HP
- **Mana Potion** (~): Reserved for future magic system
- **Sword** (/): +5 Attack when equipped
- **Shield** ([): +3 Defense when equipped
- **Armor** (]): +4 Defense when equipped

### Enemy Scaling
Enemies become stronger with each dungeon level:
- HP increases by 2 per level
- Attack increases by 1 every 2 levels
- More dangerous enemy types appear in deeper levels

## Architecture

The game is organized into modular components:

- **dungeon**: Procedural dungeon generation with rooms and corridors
- **entity**: Player and enemy entities with stats and behavior
- **combat**: Combat system with damage calculation and combat log
- **inventory**: Item management and equipment system
- **fov**: Field of view calculation using Bresenham's line algorithm
- **ai**: Enemy AI with different behavior patterns per enemy type
- **render**: Terminal rendering using crossterm with colorful output

## Technical Details

- **Language**: Rust (Edition 2021)
- **Dependencies**:
  - `crossterm 0.27`: Cross-platform terminal manipulation
  - `rand 0.8`: Random number generation for procedural content
- **Testing**: Comprehensive unit tests for all core systems
- **Performance**: Optimized for smooth gameplay even with many entities

## Development

The codebase is well-documented and includes:
- Comprehensive unit tests for all modules (run with `cargo test`)
- Clear separation of concerns between modules
- Idiomatic Rust with proper error handling
- Production-quality code suitable for extension and modification

## Future Enhancements

Potential additions for future versions:
- Magic system with spells
- More item types and rarities
- Save/load game functionality
- Multiple character classes
- Ranged weapons
- Additional enemy types
- Boss fights with special mechanics
- Procedural item generation

## License

Created for a coding challenge competition. Free to use and modify.

## Credits

Built with Rust and inspired by classic roguelike games like Rogue, NetHack, and Dungeon Crawl Stone Soup.
