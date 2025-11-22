# Rust Platformer - Competition Edition

A fully-featured 2D platformer game built in Rust using the Macroquad game engine. This game showcases advanced game development techniques including physics simulation, particle effects, smooth camera movement, and multiple gameplay mechanics.

## Features

### Core Gameplay
- **Smooth Player Movement**: Responsive left/right movement with acceleration
- **Advanced Jump Mechanics**:
  - Single jump with realistic physics
  - Double jump ability (unlockable power-up)
  - Variable jump height based on button hold
- **Physics System**: Custom physics engine with gravity, collision detection, and resolution
- **Health & Lives System**: 3 hearts per life, 3 lives to start
- **Checkpoint System**: Save progress at checkpoints throughout levels

### Platform Types
- **Solid Platforms**: Standard platforms that support the player
- **Moving Platforms**: Platforms that move between two points
- **Disappearing Platforms**: Platforms that vanish after stepping on them

### Enemies
- **Walker Enemies**: Ground-based enemies that patrol between points
- **Flyer Enemies**: Flying enemies that circle in the air
- **Patroller Enemies**: Faster, more aggressive ground enemies

### Collectibles
- **Coins**: Basic collectibles worth 100 points each
- **Gems**: Rare collectibles worth 500 points
- **Health Packs**: Restore one heart
- **Extra Lives**: Gain an additional life
- **Double Jump Power-up**: Unlock double jump ability

### Visual Effects
- **Particle System**:
  - Jump particles when leaving ground
  - Landing particles when hitting platforms
  - Collection particles for items
  - Damage and death effects
- **Parallax Scrolling**: Multi-layer background with depth
- **Smooth Camera**: Follows player with interpolation
- **Animated Sprites**: Character and enemy animations
- **Visual Feedback**: Invulnerability flashing, disappearing platform effects

### Level Design
- **5 Complete Levels**: Progressive difficulty curve
  1. **Tutorial Valley**: Introduction to basic mechanics
  2. **Moving Platforms**: Learn about dynamic obstacles
  3. **Sky High**: Vertical challenge with precision jumping
  4. **Danger Zone**: Multiple enemy types and hazards
  5. **Final Challenge**: Ultimate test of all skills
- **JSON Level Format**: Easy-to-edit level data files
- **Goal System**: Reach the goal to complete each level

### UI/UX
- **Main Menu**: Start game or quit
- **HUD**: Real-time display of health, lives, score, coins, and level
- **Pause System**: Press ESC to pause/resume
- **Game Over Screen**: Shows final score and level reached
- **Victory Screen**: Celebration for completing all levels
- **Flash Messages**: On-screen notifications for checkpoints and power-ups

### Audio
- **Sound Effects System**: Audio feedback for all actions
  - Jump and landing sounds
  - Collection sounds
  - Damage and death sounds
  - Checkpoint activation
  - Level completion

## Building and Running

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

### Quick Start

1. **Clone or navigate to the project directory**:
   ```bash
   cd /home/md/language/experiment/coding-challenge-02/platformer-rust
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the game**:
   ```bash
   cargo run --release
   ```

### Development Build
For faster compilation during development (with less optimization):
```bash
cargo run
```

### Running Tests
Execute the comprehensive test suite:
```bash
cargo test
```

The tests cover:
- Physics engine (gravity, velocity, collision)
- AABB collision detection
- Collision resolution from all directions
- Edge cases and boundary conditions

## Controls

### Gameplay
- **Move Left**: Left Arrow or A
- **Move Right**: Right Arrow or D
- **Jump**: Space, W, or Up Arrow
- **Double Jump**: Jump again while in air (after collecting power-up)
- **Pause**: ESC

### Menu Navigation
- **Navigate**: Arrow Keys or W/S
- **Select**: Enter or Space

## Game Mechanics

### Physics
- **Gravity**: 1200 pixels/second²
- **Jump Velocity**: -450 pixels/second (powerful first jump)
- **Double Jump Velocity**: -400 pixels/second (slightly weaker)
- **Max Fall Speed**: 600 pixels/second (terminal velocity)
- **Move Speed**: 200 pixels/second

### Scoring
- Coins: 100 points
- Gems: 500 points
- Health Packs: 50 points
- Extra Lives: 1000 points
- Double Jump Power-up: 200 points
- Defeating Enemies: 200 points (stomp from above)

### Combat
- **Stomp Enemies**: Jump on enemies from above to defeat them
- **Take Damage**: Collision with enemies removes 1 heart
- **Invulnerability**: 1.5 seconds of invulnerability after taking damage
- **Death**: Lose a life when health reaches 0 or fall off the map
- **Game Over**: Occurs when all lives are lost

### Checkpoints
- Touch checkpoint flags to activate them
- Respawn at the last activated checkpoint after death
- Checkpoints save progress within a level

## Architecture

### Project Structure
```
platformer-rust/
├── src/
│   ├── main.rs              # Main game loop and state management
│   ├── lib.rs               # Library exports
│   ├── physics/
│   │   └── mod.rs           # Physics engine and collision detection
│   ├── entities/
│   │   ├── mod.rs           # Entity module exports
│   │   ├── player.rs        # Player character implementation
│   │   ├── platform.rs      # Platform types and behavior
│   │   ├── enemy.rs         # Enemy AI and types
│   │   ├── collectible.rs   # Collectible items
│   │   └── checkpoint.rs    # Checkpoint system
│   ├── level/
│   │   └── mod.rs           # Level loading and management
│   ├── camera/
│   │   └── mod.rs           # Camera system with smooth following
│   ├── particles/
│   │   └── mod.rs           # Particle effects system
│   ├── ui/
│   │   └── mod.rs           # UI components (HUD, menus, backgrounds)
│   └── audio/
│       └── mod.rs           # Audio system
├── tests/
│   └── physics_tests.rs     # Comprehensive physics tests
├── levels/
│   └── level1.json          # Example level data
├── Cargo.toml               # Project configuration
└── README.md                # This file
```

### Design Patterns
- **Entity Component System**: Modular entity design
- **State Machine**: Game states (Menu, Playing, Paused, GameOver, Victory)
- **Observer Pattern**: Event-driven particle and audio systems
- **Data-Driven Design**: JSON-based level format

## Level Format

Levels can be created by editing JSON files in the `levels/` directory:

```json
{
  "name": "Level Name",
  "width": 2000.0,
  "height": 600.0,
  "spawn_x": 100.0,
  "spawn_y": 400.0,
  "platforms": [
    {
      "x": 0.0,
      "y": 500.0,
      "width": 400.0,
      "height": 100.0,
      "platform_type": "Solid",
      "end_x": null,
      "end_y": null,
      "speed": null
    }
  ],
  "enemies": [
    {
      "x": 900.0,
      "y": 450.0,
      "enemy_type": "Walker",
      "patrol_start": 850.0,
      "patrol_end": 1100.0,
      "radius": null
    }
  ],
  "collectibles": [
    {
      "x": 450.0,
      "y": 350.0,
      "collectible_type": "Coin"
    }
  ],
  "checkpoints": [
    {
      "x": 1300.0,
      "y": 400.0
    }
  ],
  "goal_x": 1850.0,
  "goal_y": 450.0
}
```

### Platform Types
- `"Solid"`: Standard platform
- `"Moving"`: Requires `end_x`, `end_y`, and `speed` parameters
- `"Disappearing"`: Vanishes after being stepped on

### Enemy Types
- `"Walker"`: Ground enemy, requires `patrol_start` and `patrol_end`
- `"Flyer"`: Flying enemy, requires `radius` for circular motion
- `"Patroller"`: Fast ground enemy, requires `patrol_start` and `patrol_end`

### Collectible Types
- `"Coin"`: Basic collectible
- `"Gem"`: Valuable collectible
- `"HealthPack"`: Restores health
- `"ExtraLife"`: Grants extra life
- `"DoubleJump"`: Unlocks double jump

## Technical Highlights

### Performance
- **Optimized Collision Detection**: AABB-based collision with minimal checks
- **Efficient Particle System**: Pooled particles with automatic cleanup
- **Release Mode Optimization**: LTO enabled for maximum performance

### Code Quality
- **Comprehensive Tests**: 20+ unit tests covering core physics
- **Modular Design**: Clean separation of concerns
- **Documentation**: Inline comments and doc comments
- **Type Safety**: Leveraging Rust's type system for safety

### Graphics
- **Smooth Rendering**: 60 FPS target with delta time
- **Parallax Layers**: Multi-layer background system
- **Visual Effects**: Alpha blending, color transitions, animations
- **Camera System**: Bounded smooth following camera

## Troubleshooting

### Build Issues
- Ensure Rust is up to date: `rustup update`
- Clean and rebuild: `cargo clean && cargo build --release`

### Performance Issues
- Always run in release mode for games: `cargo run --release`
- Release mode is 10-100x faster than debug mode

### Level Not Loading
- Check JSON syntax with a validator
- Ensure level files are in the `levels/` directory
- Verify file permissions

## Future Enhancements

Potential additions for future versions:
- Sound file integration (WAV/OGG support)
- Level editor GUI
- Save/load game state
- High score table
- More enemy types and bosses
- Power-ups (speed boost, shield, etc.)
- Animated sprites from sprite sheets
- Gamepad support
- Online leaderboards

## License

This project is created for a coding challenge competition.

## Credits

Built with:
- **Rust**: Systems programming language
- **Macroquad**: Simple and easy-to-use game library
- **serde/serde_json**: Serialization framework
- **rand**: Random number generation

---

**Enjoy the game and good luck in the competition!** 🎮🏆
