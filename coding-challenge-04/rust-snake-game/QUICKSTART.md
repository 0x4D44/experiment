# Quick Start Guide

## Install and Run

```bash
cd /home/md/language/experiment/coding-challenge-04/rust-snake-game
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| ↑ ↓ ← → | Move snake |
| P | Pause/Resume |
| Q | Quit |
| R | Restart (after game over) |

## Quick Tips

1. Start with **Medium** difficulty to learn the game
2. The snake can't reverse direction (no 180° turns)
3. Speed increases as your score goes up
4. Each food gives you 10 points
5. Try to create a pattern to fill the board efficiently

## Difficulty Comparison

- **Easy**: 150ms → 80ms (beginner-friendly)
- **Medium**: 100ms → 50ms (balanced)
- **Hard**: 70ms → 30ms (challenging)
- **Extreme**: 50ms → 20ms (expert only!)

## Testing

```bash
cargo test              # Run all 18 unit tests
cargo build --release   # Build optimized binary
./target/release/snake  # Run the compiled binary
```

Enjoy the game!
