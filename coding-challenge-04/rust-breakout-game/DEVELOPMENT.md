# Development Guide

## Project Structure

### Module Hierarchy

```
main.rs
├── game.rs (Game state & logic)
│   ├── physics.rs (Collision detection)
│   ├── level.rs (Brick patterns)
│   └── powerup.rs (Power-up system)
├── renderer.rs (Terminal graphics)
└── input.rs (Input abstraction)
```

### Data Flow

```
Input Events → Game State → Renderer → Terminal
     ↓            ↓
  Controls    Physics
               ↓
           Collisions
               ↓
         Score/Lives
```

## Module Details

### main.rs - Game Loop

**Responsibilities**:
- Terminal initialization/cleanup
- Input event polling
- Game loop timing (60 FPS)
- State machine coordination

**Key Constants**:
- `TARGET_FPS: 60` - Frame rate target
- `FRAME_DURATION: 16.67ms` - Time per frame

**Main Loop**:
1. Poll for input events
2. Update game state (if enough time passed)
3. Render frame
4. Sleep briefly to prevent CPU spinning

### game.rs - Core Game Logic

**Data Structures**:
- `Game` - Main game state container
- `Paddle` - Player paddle with position/size
- `Ball` - Ball with physics
- `GameState` - Enum for game states

**Game States**:
- `Menu` - Title screen
- `Playing` - Active gameplay
- `Paused` - Paused state
- `GameOver` - Lost all lives
- `Victory` - Completed all levels

**Update Cycle**:
1. Update paddle position
2. Update power-up timers
3. Update ball physics
4. Check wall collisions
5. Check paddle collision
6. Check brick collisions
7. Update power-ups falling
8. Check level completion

### physics.rs - Physics Engine

**Core Types**:
- `Vec2` - 2D vector (position/velocity)
- `Rect` - Axis-aligned bounding box
- `Circle` - Circle for ball

**Collision Detection**:
```rust
// Circle-Rectangle collision
pub fn collides_rect(&self, rect: &Rect) -> Option<Vec2>

// Returns collision normal if colliding
```

**Physics Operations**:
- Vector length/normalization
- Dot product
- Reflection calculations
- Closest point on rectangle

### level.rs - Level Design

**Brick Types**:
```rust
enum BrickType {
    Normal,       // 1 hit, 10 points
    Strong,       // 2 hits, 25 points
    Unbreakable,  // Cannot break
    Bonus,        // 1 hit, 50 points, drops power-up
}
```

**Level Creation**:
- Each level has unique pattern
- `create_level_X()` functions define layouts
- Procedural generation for level 6+

**Brick Grid**:
- Width: 80 characters / 8 = 10 columns
- Height: Variable per level
- Spacing: 2 rows per brick

### powerup.rs - Power-up System

**Power-up Types**:
```rust
enum PowerUpType {
    WidePaddle,   // 10s duration
    MultiBall,    // Permanent until lost
    SlowBall,     // 8s duration
    ExtraLife,    // Permanent
    LaserPaddle,  // 12s duration
}
```

**Power-up Lifecycle**:
1. Spawn when brick destroyed
2. Fall downward at constant speed
3. Collect on paddle collision
4. Apply effect immediately
5. Track duration (if temporary)
6. Expire and remove effect

### renderer.rs - Terminal Graphics

**Double Buffering**:
```rust
buffer: Vec<Vec<char>>,        // Character buffer
color_buffer: Vec<Vec<Color>>, // Color buffer
```

**Rendering Pipeline**:
1. Clear buffers
2. Draw game elements into buffers
3. Batch output to terminal
4. Optimize color changes

**Color Scheme**:
- Normal bricks: Blue
- Strong bricks: Magenta
- Unbreakable: Grey
- Bonus: Yellow
- Paddle: Green/Yellow
- Ball: Red

### input.rs - Input Handling

**Current Implementation**:
- Simple input state structure
- Extensible for future features
- Input polling in main loop

**Potential Extensions**:
- Mouse support
- Custom key bindings
- Input recording/playback

## Testing Strategy

### Unit Tests

Each module has comprehensive tests:

**Physics Tests**:
```rust
test_vec2_length()           // Vector math
test_vec2_normalize()        // Normalization
test_rect_intersects()       // Rectangle collision
test_circle_rect_collision() // Circle-rect collision
test_reflect_velocity()      // Physics reflection
```

**Level Tests**:
```rust
test_brick_hit()             // Normal brick destruction
test_strong_brick()          // Multi-hit mechanics
test_unbreakable_brick()     // Indestructible bricks
test_level_creation()        // Level generation
test_level_completion()      // Win condition
```

**Game Tests**:
```rust
test_game_creation()         // Initialization
test_paddle_movement()       // Control mechanics
test_paddle_boundaries()     // Edge cases
test_ball_launch()           // Ball physics
test_powerup_collection()    // Power-up system
test_multiball_powerup()     // Special mechanics
test_score_increases()       // Scoring
```

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test physics

# Specific test
cargo test test_paddle_movement

# Show output
cargo test -- --nocapture

# Release mode (faster)
cargo test --release
```

## Adding Features

### Adding a New Power-up

1. **Define the Type** (powerup.rs):
```rust
pub enum PowerUpType {
    // ... existing types
    FireBall,  // New type
}
```

2. **Add Properties**:
```rust
impl PowerUpType {
    pub fn symbol(&self) -> &str {
        match self {
            // ... existing
            PowerUpType::FireBall => "F",
        }
    }

    pub fn duration(&self) -> Option<f32> {
        match self {
            // ... existing
            PowerUpType::FireBall => Some(15.0),
        }
    }
}
```

3. **Implement Collection** (game.rs):
```rust
fn collect_powerup(&mut self, power_type: PowerUpType) {
    match power_type {
        // ... existing
        PowerUpType::FireBall => {
            // Add fireball logic
        }
    }
}
```

4. **Add Rendering** (renderer.rs):
```rust
fn get_powerup_color(&self, powerup_type: &PowerUpType) -> Color {
    match powerup_type {
        // ... existing
        PowerUpType::FireBall => Color::Red,
    }
}
```

5. **Write Tests**:
```rust
#[test]
fn test_fireball_powerup() {
    let mut game = Game::new();
    game.collect_powerup(PowerUpType::FireBall);
    // Assert behavior
}
```

### Adding a New Brick Type

1. **Add to Enum** (level.rs):
```rust
pub enum BrickType {
    // ... existing
    Explosive,  // Destroys nearby bricks
}
```

2. **Define Properties**:
```rust
impl BrickType {
    pub fn hits_required(&self) -> u32 {
        match self {
            // ... existing
            BrickType::Explosive => 1,
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            // ... existing
            BrickType::Explosive => 100,
        }
    }
}
```

3. **Add to Levels**:
```rust
fn create_level_6(game_width: f32) -> Vec<Brick> {
    // ... create bricks with new type
    BrickType::Explosive
}
```

### Adding a New Level

1. **Create Level Function** (level.rs):
```rust
fn create_level_6(game_width: f32) -> Vec<Brick> {
    let mut bricks = Vec::new();
    let brick_width = 8.0;
    let brick_height = 2.0;

    // Design your pattern
    for row in 0..rows {
        for col in 0..cols {
            // Create brick pattern
        }
    }

    bricks
}
```

2. **Add to Level Loader**:
```rust
impl Level {
    pub fn load(level_number: u32, game_width: f32) -> Self {
        let bricks = match level_number {
            // ... existing
            6 => Self::create_level_6(game_width),
            _ => Self::create_level_random(level_number, game_width),
        };
        // ...
    }
}
```

## Performance Optimization

### Profiling

```bash
# Build with debugging symbols
cargo build --release --profile release-with-debug

# Profile with perf (Linux)
perf record ./target/release/rust-breakout-game
perf report

# Profile with Instruments (macOS)
instruments -t "Time Profiler" ./target/release/rust-breakout-game
```

### Common Optimizations

1. **Reduce Allocations**:
   - Reuse vectors
   - Use `Vec::with_capacity()`
   - Avoid cloning in hot paths

2. **Collision Detection**:
   - Early exit conditions
   - Spatial partitioning (future)
   - Only check active bricks

3. **Rendering**:
   - Batch terminal writes
   - Minimize color changes
   - Only render visible area

## Debugging

### Debug Prints

```rust
// Conditional compilation
#[cfg(debug_assertions)]
println!("Ball position: {:?}", ball.circle.center);
```

### Test-Driven Debugging

```rust
#[test]
fn debug_ball_stuck() {
    let mut game = Game::new();
    // Reproduce issue
    // Add assertions
    // Fix and verify
}
```

### Common Issues

**Ball Getting Stuck**:
- Check collision normal calculation
- Verify ball position update
- Add position correction after collision

**Paddle Not Moving**:
- Verify game state is Playing
- Check delta time value
- Ensure movement flags are set

**Bricks Not Breaking**:
- Check collision detection
- Verify hit() logic
- Ensure active flag is set

## Release Checklist

- [ ] All tests pass: `cargo test`
- [ ] No warnings: `cargo build --release`
- [ ] Code formatted: `cargo fmt`
- [ ] Clippy passes: `cargo clippy`
- [ ] Documentation complete
- [ ] README up to date
- [ ] Examples work
- [ ] Performance acceptable

## Contributing Guidelines

### Code Style

- Follow Rust standard style (`cargo fmt`)
- Use descriptive variable names
- Add comments for complex logic
- Document public APIs

### Commit Messages

```
feat: Add fireball power-up
fix: Correct paddle boundary check
docs: Update README with new controls
test: Add tests for level 6
refactor: Extract collision logic
```

### Pull Request Process

1. Create feature branch
2. Write tests first
3. Implement feature
4. Run full test suite
5. Update documentation
6. Submit PR with description

## Resources

### Rust Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

### Dependencies
- [crossterm docs](https://docs.rs/crossterm/)
- [rand docs](https://docs.rs/rand/)

### Game Development
- [Game Programming Patterns](https://gameprogrammingpatterns.com/)
- [Breakout game mechanics](https://en.wikipedia.org/wiki/Breakout_(video_game))

## License

Educational and entertainment purposes.

---

Happy coding! 🦀
