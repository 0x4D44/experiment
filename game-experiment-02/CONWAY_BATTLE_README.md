# Conway's Battle Arena

A competitive twist on Conway's Game of Life where two players battle for cellular dominance!

## Game Overview

Conway's Battle Arena combines the mathematical elegance of Conway's Game of Life with competitive strategy gaming. Players place cells of their color during a deployment phase, then watch as the cells evolve using Conway's rules in a battle for territory.

### Quick Start

1. **Open the Game**: Open `conway-battle-game.html` in your web browser
2. **Select Game Mode**: Choose "PvP" (player vs player) or "vs AI" (against AI opponents)
3. **Choose Difficulty** (if vs AI): Select Easy, Normal, or Hard
4. **Start New Game**: Click the "Start New Game" button
5. **Deploy Cells**: Click on the grid to place your colored cells (30 per player)
6. **Watch Battle**: Once both players finish deploying, the cells evolve automatically
7. **Win**: First player to achieve a victory condition wins!

## Game Modes

### PvP (Player vs Player)
- Two human players take turns placing cells on the grid
- Player 1 controls blue cells (0088ff)
- Player 2 controls red cells (ff4488)
- Each player can place 30 cells in total
- Players alternate taking turns

### vs AI (Player vs AI)
- Player 1 (human) competes against an AI opponent
- Three difficulty levels:
  - **Easy**: AI places cells randomly
  - **Normal**: AI clusters cells near existing cells and near edges
  - **Hard**: AI attempts to control the board center and create strategic patterns

## Game Phases

### Phase 1: Deployment (Manual)
- Players place cells strategically on the 40x40 grid
- Each player has 30 cells to place
- Players alternate turns placing one cell at a time
- The game displays whose turn it is and how many cells remain
- Once both players place all 30 cells, the battle begins automatically

### Phase 2: Battle (Automatic)
- Cells evolve according to Conway's Game of Life rules with player ownership
- Generation counter shows how many evolution steps have occurred
- Game continuously evaluates victory conditions
- Cells create stunning emergent patterns as they interact
- Battle continues until a victory condition is met

### Phase 3: Game Over
- Winner is announced with the victory condition
- Final statistics are displayed
- Player can start a new game

## Victory Conditions

The game checks for victory each generation. First condition met wins:

1. **Elimination Victory** (Immediate): One player has 0 cells remaining
   - The opponent wins immediately

2. **Domination Victory** (70%+ of alive cells):
   - One player controls 70% or more of all alive cells on the board
   - Shows overwhelming superiority

3. **Time-Based Victory** (After 500 generations):
   - If neither player achieves elimination or domination
   - Player with more cells wins
   - If equal, it's a draw

## Conway's Game of Life Rules

The core rules applied each generation:

1. **Survival**: Any alive cell with 2-3 neighbors survives
2. **Death**: Any alive cell with fewer than 2 or more than 3 neighbors dies
3. **Birth**: Any dead cell with exactly 3 neighbors becomes alive

### Multi-Player Ownership Modification

In Battle Arena, cells inherit the color of their "birth parents":

- **When a cell is born** (from 3 neighbors), it takes the color of the **majority** of its neighbors
- **If neighbors are tied** (1.5-1.5 split), no cell is born (ambiguous ownership)
- This creates interesting territorial dynamics where players compete for cellular territory

## Statistics Tracked

The stats panel shows:
- **Phase**: Current game phase (SETUP, DEPLOYMENT, BATTLE, GAME_OVER)
- **Player 1 Cells**: Number of blue cells currently alive
- **Player 2 Cells**: Number of red cells currently alive
- **Generation**: How many evolution cycles have completed
- **Turn**: Whose turn it is in deployment phase (during battle shows generation number)

## Controls

### Game Mode Selection
- **PvP Button**: Local two-player game
- **vs AI Button**: Play against AI opponent

### Difficulty Selection (vs AI only)
- **Easy**: Random cell placement
- **Normal**: Clustered strategic placement
- **Hard**: Center-focused strategic placement with pattern recognition

### Game Controls
- **Start New Game**: Initialize a fresh game with current settings
- **Pause/Resume**: Pause the battle phase and resume later
- **Reset Game**: Stop current game and return to setup

### In-Game Interaction
- **Click on Grid** (Deployment Phase): Place a cell at that location
  - Only works during your turn
  - Only on empty cells
  - Turns alternate automatically
  - Shows remaining cells you can place

## Technical Details

### Architecture

The game is built with clean separation of concerns:

```
GameBoard          - Grid state and cell management
Cell               - Individual cell with owner tracking
DeploymentManager  - Turn management and placement rules
VictoryChecker     - Victory condition evaluation
AIPlayer           - AI strategy and placement logic
ConwayBattleGame   - Main game engine and state machine
GameRenderer       - Canvas rendering and visualization
```

### Technology Stack
- **Language**: TypeScript (compiled to JavaScript)
- **Runtime**: Browser (HTML5 Canvas)
- **Graphics**: HTML5 Canvas 2D Context
- **Testing**: Jest (56 unit tests included)
- **No External Dependencies**: Pure JavaScript/TypeScript

### Performance
- 40x40 grid = 1,600 cells maximum
- O(n) generation evolution
- Target 60 FPS rendering with 30 frame interval for evolution (~2 generations/sec)
- Efficient neighbor counting with bounds checking

### Grid Specifications
- **Size**: 40x40 cells (1,600 total cells)
- **Cells Per Player**: 30 (placement phase)
- **Max Generations**: 500
- **Evolution Speed**: ~2 generations per second

## Testing

Comprehensive test suite with 56 tests covering:

### Unit Tests
- **Cell Tests** (5 tests): Cell creation, ownership, state management
- **GameBoard Tests** (15 tests): Grid operations, bounds checking, Conway rules
- **DeploymentManager Tests** (9 tests): Turn management, placement validation
- **VictoryChecker Tests** (8 tests): All victory conditions
- **AIPlayer Tests** (6 tests): Placement strategies and difficulty levels
- **ConwayBattleGame Tests** (13 tests): Complete game flow and state transitions

### Test Results
All 56 tests pass successfully, validating:
- Core game mechanics
- Conway's Game of Life rule implementation
- Two-player ownership system
- Victory condition logic
- AI strategy execution
- Complete game flow

Run tests with:
```bash
npm test -- conway-battle.test.ts
```

## Game Strategy Tips

### Early Placement Strategy

1. **Clustering**: Group cells together to create stable patterns
   - Blocks (2x2 squares) are completely stable
   - Blinkers (3-cell rows) oscillate predictably

2. **Edge Control**: Place cells near board edges
   - Hard to for opponent to attack from outside
   - Gives you control of territory

3. **Center Territory**: In Hard AI mode, expect AI to control center
   - Consider edge and corner strategies
   - Prevent center dominance early

### Pattern Recognition

Common patterns to look for:

1. **Still Life** (stable, never changes):
   - Block (2x2 square) - extremely stable
   - Beehive, loaf, tub - other stable patterns

2. **Oscillators** (repeat patterns):
   - Blinker (period 2)
   - Toad, beacon (period 2)
   - Pulsar (period 3)

3. **Spaceships** (moving patterns):
   - Glider (moves diagonally)
   - LWSS, MWSS (move horizontally)

### Winning Tactics

1. **Create Robust Structures**: Use stable patterns that last
2. **Control Territory**: Spread influence across the grid
3. **Anticipate Growth**: Plan where cells will spawn
4. **Block Opponent**: Place cells to prevent opponent's growth
5. **Create Gliders**: Advanced players can create moving patterns

## Browser Compatibility

Works on any modern browser with:
- HTML5 Canvas support
- ES6+ JavaScript support

Tested on:
- Chrome/Chromium (recommended)
- Firefox
- Safari
- Edge

## File Structure

```
conway-battle-game.html          - Playable game (standalone)
src/conway-battle.ts             - TypeScript source code
conway-battle.test.ts            - Unit tests (56 tests)
CONWAY_BATTLE_README.md          - This file
wrk_journals/[journal].md        - Development journal
```

## Troubleshooting

### Game won't start
- Make sure JavaScript is enabled
- Try a different browser
- Check browser console for errors

### Cells not placing
- Verify it's your turn (turn indicator shows your color)
- Make sure you click on an empty cell (not occupied)
- Check you haven't placed all 30 cells yet

### Performance issues
- Close other browser tabs
- Disable browser extensions
- Try a different browser
- Check your system's CPU/GPU usage

## Future Enhancements

Potential features for future versions:
- Undo placement during deployment
- Replay system with playback speed control
- Custom grid sizes
- Network multiplayer
- Leaderboard and stats tracking
- Save/load games
- Custom AI opponents with learning
- More game modes (free-for-all 4-player, etc.)
- Sound effects and music
- Advanced pattern placement tools

## Development

### Running Tests
```bash
npm test -- conway-battle.test.ts
```

### Project Structure
- **src/conway-battle.ts**: Core game logic (1000+ lines)
- **conway-battle.test.ts**: Test suite (800+ lines)
- **conway-battle-game.html**: Full game with rendering (1400+ lines)

### Code Quality
- 56 passing unit tests
- Comprehensive test coverage
- Clean architecture with clear separation of concerns
- No external dependencies
- Pure TypeScript/JavaScript implementation

## Credits

Conway's Battle Arena - Round 2 Game Development Competition

Inspired by:
- John Horton Conway's Game of Life (1970)
- The mathematical elegance of cellular automata
- The strategic depth of competitive gaming

## License

Created for competition purposes. Feel free to use, modify, and share!

---

**Enjoy the Battle! May your cells thrive and your strategies prevail! 🧬⚔️**
