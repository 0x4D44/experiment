# Quick Start Guide

## Installation & Running

### Option 1: Using Cargo (Recommended)

```bash
cd /home/md/language/experiment/coding-challenge-04/rust-breakout-game

# Build and run in one command
cargo run --release

# Or build first, then run
cargo build --release
./target/release/rust-breakout-game
```

### Option 2: Development Mode

```bash
# Faster compilation, slower runtime
cargo run
```

## First Game Tutorial

### Step 1: Start Menu
- You'll see the BREAKOUT title screen
- Press **SPACE** to start playing

### Step 2: Launch the Ball
- The ball starts attached to your paddle
- Use **LEFT/RIGHT arrows** to position
- Press **SPACE** to launch

### Step 3: Keep the Ball Alive
- Move paddle with **arrow keys**
- Don't let the ball fall off the bottom
- Hit bricks to score points

### Step 4: Collect Power-ups
- Falling letters are power-ups
- Catch them with your paddle
- **[W]** = Wide Paddle (easiest to use)
- **[M]** = Multi-Ball (more balls!)
- **[S]** = Slow Ball (easier control)
- **[+]** = Extra Life
- **[L]** = Laser indicator

### Step 5: Complete the Level
- Break all colored bricks
- Grey bricks are unbreakable
- Level complete when all breakable bricks are gone

### Step 6: Game Over or Victory
- **Game Over**: Run out of lives
- **Victory**: Complete all 5 levels
- Press **R** to restart
- Press **Q** to quit

## Controls Reference

| Key | Action |
|-----|--------|
| SPACE | Start game / Launch ball |
| LEFT ARROW | Move paddle left |
| RIGHT ARROW | Move paddle right |
| P | Pause / Resume |
| R | Restart (after game over) |
| Q | Quit game |

## Tips for First Game

1. **Take Your Time**: Start slow, learn the controls
2. **Watch the Angle**: Ball direction depends on paddle hit position
3. **Collect Power-ups**: Wide paddle makes the game much easier
4. **Don't Panic**: You have 5 lives to learn
5. **Yellow Bricks**: These drop power-ups, hit them first!

## Common Issues

### Terminal Too Small
- Make your terminal at least 80 columns x 30 rows
- The game will render in available space

### Colors Not Showing
- Use a modern terminal with ANSI color support
- Try: gnome-terminal, iTerm2, Windows Terminal

### Input Not Working
- Make sure terminal has focus
- Some terminals may have key conflicts

### Game Runs Slowly
- Use `cargo run --release` for optimized build
- Close other terminal applications

## Testing

Run the test suite to verify everything works:

```bash
cargo test
```

Expected output: All 21 tests should pass.

## Performance Check

```bash
# Build optimized release
cargo build --release

# Check binary size
ls -lh target/release/rust-breakout-game

# Run with performance monitoring
time cargo run --release
```

## Next Steps

- Read **FEATURES.md** for detailed game mechanics
- Read **README.md** for full documentation
- Try to beat all 5 levels!
- Challenge friends for high scores

## Enjoy the Game!

Good luck breaking those bricks!
