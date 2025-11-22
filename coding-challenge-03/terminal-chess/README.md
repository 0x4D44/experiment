# Terminal Chess

A fully-featured, beautiful chess game for your terminal written in Rust! Complete with AI opponent, move validation, special moves, and a gorgeous colored interface.

## Features

### Complete Chess Implementation
- ✅ **All Standard Rules**: Complete implementation of chess rules including:
  - All piece movements (Pawn, Knight, Bishop, Rook, Queen, King)
  - Castling (both kingside and queenside)
  - En passant captures
  - Pawn promotion (to Queen, Rook, Bishop, or Knight)
- ✅ **Check & Checkmate Detection**: Automatically detects check, checkmate, and stalemate
- ✅ **Legal Move Validation**: Only legal moves are allowed
- ✅ **50-Move Rule**: Automatic draw detection

### AI Opponent
- ✅ **Minimax Algorithm with Alpha-Beta Pruning**: Strategic AI that thinks ahead
- ✅ **Multiple Difficulty Levels**:
  - Easy (Depth 1): Good for beginners
  - Medium (Depth 2): Intermediate challenge
  - Hard (Depth 3): Advanced play
  - Expert (Depth 4): Very challenging
- ✅ **Position Evaluation**: Uses piece values and position bonuses for smart play

### Beautiful Terminal UI
- ✅ **Unicode Chess Pieces**: Beautiful Unicode characters (♔♕♖♗♘♙ vs ♚♛♜♝♞♟)
- ✅ **Colored Board**: Alternating light and dark squares with color highlighting
- ✅ **Move Highlighting**: Last move is highlighted in yellow
- ✅ **Captured Pieces Display**: See what pieces have been captured
- ✅ **Coordinate Labels**: Board labeled with a-h and 1-8

### Game Management
- ✅ **Save/Load Games**: Save your game and continue later
- ✅ **Undo Moves**: Made a mistake? Undo it!
- ✅ **Move History**: Complete history of all moves made
- ✅ **Two Game Modes**:
  - Player vs Player: Play against a friend locally
  - Player vs AI: Challenge the computer

## Installation

### Prerequisites
- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs))

### Build from Source

```bash
# Clone the repository (or navigate to the project directory)
cd terminal-chess

# Build the project
cargo build --release

# The binary will be at target/release/terminal-chess
```

## How to Play

### Starting the Game

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/terminal-chess
```

### Main Menu

When you start the game, you'll see a menu with options:

1. **Player vs Player**: Play chess with someone on the same computer
2. **Player vs AI**: Play against the computer AI
3. **Load Game**: Load a previously saved game
4. **Quit**: Exit the game

### Making Moves

Moves are entered in **algebraic notation** (source square + destination square):

- `e2e4` - Move piece from e2 to e4
- `e7e8q` - Move pawn from e7 to e8 and promote to Queen
- `e1g1` - Castling (King from e1 to g1)

#### Promotion

When a pawn reaches the opposite end, add a letter for the piece:
- `q` or `Q` - Queen
- `r` or `R` - Rook
- `b` or `B` - Bishop
- `n` or `N` - Knight

Example: `e7e8q` promotes pawn to queen

### Commands

While playing, you can use these commands:

- **Make a move**: Type the move in algebraic notation (e.g., `e2e4`)
- **u** or **undo**: Undo the last move (in AI mode, undoes both your move and AI's move)
- **s** or **save**: Save the current game to a file
- **q** or **quit**: Quit to main menu

### Board Coordinates

The board is labeled with:
- **Columns (files)**: a-h (left to right)
- **Rows (ranks)**: 1-8 (bottom to top)

White pieces start on ranks 1-2, black pieces on ranks 7-8.

### Understanding the Display

```
Black captured: ♙ ♙ ♘

     a  b  c  d  e  f  g  h
    ┌────────────────────────┐
  8 │ ♜  ♞  ♝  ♛  ♚  ♝  ♞  ♜ │ 8
  7 │ ♟  ♟  ♟  ♟  ♟  ♟  ♟  ♟ │ 7
  6 │                        │ 6
  5 │                        │ 5
  4 │                        │ 4
  3 │                        │ 3
  2 │ ♙  ♙  ♙  ♙  ♙  ♙  ♙  ♙ │ 2
  1 │ ♖  ♘  ♗  ♕  ♔  ♗  ♘  ♖ │ 1
    └────────────────────────┘
     a  b  c  d  e  f  g  h

White captured: ♟ ♟

Current player: White
Status: Playing
```

- **Yellow highlighted squares**: Show the last move made
- **White pieces** (♔♕♖♗♘♙): Your pieces (or white's pieces in PvP)
- **Black pieces** (♚♛♜♝♞♟): AI's pieces (or black's pieces in PvP)

## Game Examples

### Opening Move

```
  Enter move: e2e4
```

Moves the white pawn from e2 to e4 (classic opening).

### Castling

```
  Enter move: e1g1
```

Castles kingside (king moves from e1 to g1, rook automatically moves).

### Promotion

```
  Enter move: e7e8q
```

Moves pawn from e7 to e8 and promotes it to a Queen.

### Save Game

```
  Enter move: s
  Enter filename to save: mygame.json
```

### Load Game

From the main menu, select "Load Game" and enter the filename.

## Strategy Tips

### Against the AI

1. **Start Easy**: Begin with Easy or Medium difficulty to learn
2. **Control the Center**: Move your pawns to e4, d4, e5, d5 positions
3. **Develop Pieces**: Get your knights and bishops out early
4. **Protect Your King**: Castle early for safety
5. **Think Ahead**: The AI thinks several moves ahead - plan your strategy!

### Classic Openings

- **Italian Game**: `e2e4 e7e5 g1f3 b8c6 f1c4`
- **Sicilian Defense** (as Black): After `e2e4`, respond with `c7c5`
- **French Defense** (as Black): After `e2e4`, respond with `e7e6`

## Technical Details

### Architecture

The project is organized into clean modules:

- **chess/**: Core chess logic
  - `board.rs`: Board representation and move generation
  - `piece.rs`: Piece types and values
  - `position.rs`: Board position handling
  - `moves.rs`: Move representation
  - `game.rs`: Game state management
- **ai/**: AI engine
  - `engine.rs`: Minimax algorithm with alpha-beta pruning
- **ui/**: Terminal user interface
  - `terminal.rs`: Display and input handling

### Testing

The project includes comprehensive unit tests. Run them with:

```bash
cargo test
```

Tests cover:
- Piece movement rules
- Special moves (castling, en passant, promotion)
- Check and checkmate detection
- Board evaluation
- Move validation
- Game state management

### Performance

- **Fast Move Generation**: Efficient algorithms for legal move generation
- **Alpha-Beta Pruning**: Dramatically reduces search space for AI
- **Optimized for Terminal**: Smooth rendering with minimal screen updates

## Troubleshooting

### Terminal Display Issues

If the Unicode pieces don't display correctly:
- Make sure your terminal supports UTF-8
- Try a different terminal (Windows Terminal, iTerm2, etc.)
- Check that your font supports Unicode chess pieces

### Game Crashes or Freezes

- Try reducing AI difficulty
- Make sure you have the latest version
- Check that you have enough stack space

### Move Not Accepted

- Verify you're using algebraic notation (e.g., `e2e4`, not `e4`)
- Make sure it's a legal move (not putting your king in check)
- For promotion, add the piece letter (e.g., `e7e8q`)

## Contributing

This is a challenge project, but improvements and bug fixes are welcome!

## License

This project is provided as-is for the coding challenge.

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [Serde](https://serde.rs/) - Serialization framework

---

**Enjoy your game!** ♔♕♖♗♘♙
