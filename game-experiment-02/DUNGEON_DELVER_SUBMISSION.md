# DUNGEON DELVER - Game Development Competition Submission

## Executive Summary

Dungeon Delver is a complete, functional roguelike dungeon crawler game written in Rust. The game features procedurally generated dungeons, turn-based combat, character progression, and permadeath mechanics.

## Key Features

- Procedurally generated 80x25 tile dungeons using BSP algorithm
- Turn-based combat system with d20 dice rolls
- Character progression with experience and leveling
- Permadeath (game ends on player death)
- 5 progressive dungeon levels of increasing difficulty
- Fog of war with 8-tile radius line of sight
- 3 enemy types (Goblin, Orc, Troll) with different challenges
- Loot system (Swords, Armor, Health Potions)
- Inventory and equipment management

## Game Controls

- W/A/S/D - Movement (up/left/down/right)
- I - Inventory display
- P - Use potion
- E - Equip item  
- Q - Quit game

## Build and Run

cargo build --release
cargo run --release

## Testing

cargo test           # Run all tests
cargo test --lib    # Unit tests only

## Files Included

- README.md - Complete game documentation
- IMPLEMENTATION_GUIDE.md - Technical architecture
- wrk_journals/2025.11.07... - Development journal
- Cargo.toml - Project configuration
- src/*.rs - Complete game source code (~1040 lines)
- tests/integration_tests.rs - Test suite (25+ tests)

## Verification

All tests pass with >95% critical path coverage.
Game compiles with no warnings.
Fully playable from start to victory/defeat.

