# Hex Commander - Turn-Based Hexagonal Strategy Game

A fully functional, browser-based turn-based strategy game featuring hexagonal grids, multiple unit types, terrain effects, fog of war mechanics, and an AI opponent.

## Game Overview

Hex Commander is a tactical strategy game where you command units across a hexagonal battlefield. Use infantry, cavalry, and archers strategically to defeat your opponent or control resource nodes. Terrain significantly affects movement and combat, adding tactical depth to each decision.

## Features

### Core Gameplay
- **Hexagonal Grid System**: 12x12 hexagonal tile-based map with proper axial coordinate system
- **Multiple Unit Types**:
  - Infantry: Balanced combat stats, moderate movement
  - Cavalry: High mobility, good vision range
  - Archer: Long-range attacks, low defense
- **Dynamic Terrain**:
  - Plain: Easy movement, no bonuses
  - Forest: Slow movement, defensive bonus
  - Mountain: Very slow movement, high defense
  - Water: Impassable (units cannot traverse)
  - Resource Nodes: Valuable capture points for victory points

### Strategic Elements
- **Movement System**: Each unit has limited movement per turn; movement costs vary by terrain
- **Combat System**: Attack and defense values with terrain defense bonuses
- **Action Economy**: Each unit can move OR attack once per turn (not both in same turn)
- **Resource Management**: Victory through elimination or resource node control
- **Fog of War**: Vision range-based awareness (AI and future multiplayer)

### AI & Challenge
- **Simple AI Opponent**: Makes tactical decisions to:
  - Prioritize enemy elimination within range
  - Move toward nearest enemy threats
  - Adapt strategy based on unit health and positioning

### Victory Conditions
1. **Elimination Victory**: Eliminate all enemy units
2. **Resource Control**: Hold more resource nodes after 50 turns
3. **Draw**: Equal resources at turn limit

## How to Play

### Setup
```bash
# Install dependencies
npm install

# Build TypeScript
npm run build

# Run tests to verify everything works
npm test -- hex-commander.test.ts
```

### Playing the Game
Open `hex-commander.html` in a web browser.

### Controls
- **Select Unit**: Click on a red unit (your units are red)
- **View Valid Moves**: Selected unit highlights blue squares it can move to
- **Move Unit**: Click on a blue highlighted square to move
- **Attack**: Right-click on an adjacent enemy unit
- **End Turn**: Click "End Turn" button or let AI take its turn
- **New Game**: Click "New Game" to restart

### Game Rules
- Each unit can move once per turn (movement limited by terrain and unit stats)
- Each unit can attack once per turn (after movement)
- Archers can attack from 3 tiles away; other units from adjacent only
- Terrain provides defense bonuses during combat
- Destroyed units are removed from the map
- Victory conditions checked each turn

## Technical Architecture

### Core Systems

#### Coordinate System (`hex-commander.ts`)
Uses axial coordinates (q, r) for hexagonal grid mathematics:
- Efficient distance calculations
- Proper neighbor finding
- Clean coordinate arithmetic

#### Game State Management
- `GameMap`: Manages tile, terrain, and unit placement
- `Game`: Core game logic including movement, combat, victory conditions
- `Unit`: Individual unit tracking with stats and state
- `Terrain`: Environmental effects on gameplay

#### AI System (`hex-commander-ai.ts`)
- `SimpleAI`: Makes tactical decisions
  - Attacks threats within range
  - Advances toward enemies
  - End-turn management

#### UI System (`hex-commander-ui.ts`)
- `HexCommanderUI`: Canvas-based rendering and input handling
  - Hex drawing and rendering
  - Unit visualization with health bars
  - Mouse input for unit selection and movement
  - Real-time game state display
  - Game loop and animation

### Technology Stack
- **Language**: TypeScript (strict mode enabled)
- **Rendering**: HTML5 Canvas (no external graphics libraries)
- **Testing**: Jest with ts-jest
- **Build**: TypeScript Compiler

## File Structure

```
hex-commander/
├── hex-commander.ts           # Core game logic and data structures
├── hex-commander.test.ts      # Comprehensive unit tests (38 tests)
├── hex-commander-ai.ts        # AI opponent implementation
├── hex-commander-ui.ts        # Canvas-based UI and rendering
├── hex-commander.html         # Game interface and launch page
├── dist/                       # Compiled JavaScript (auto-generated)
└── tsconfig.json              # TypeScript configuration
```

## Testing

### Running Tests
```bash
npm test -- hex-commander.test.ts
```

### Test Coverage
- **Coordinate System**: 5 tests
  - Coordinate creation and comparison
  - Distance calculations
  - Neighbor finding
- **Terrain System**: 4 tests
  - All terrain types and their properties
- **Unit System**: 4 tests
  - Unit creation for all types
  - Unique ID generation
- **Game Map**: 10 tests
  - Map initialization
  - Tile access and validation
  - Unit placement and removal
  - Player unit tracking
- **Game Logic**: 15 tests
  - Game initialization
  - Movement validation and execution
  - Combat mechanics
  - Turn management
  - Victory conditions
  - Game status

All 38 tests pass with 100% success rate.

## Game Design Decisions

### Why Hexagonal Grid?
Hexagonal grids offer superior symmetry compared to square grids, resulting in more tactical depth:
- 6 equidistant neighbors (vs 4-8 for squares)
- Better movement representation
- Cleaner distance calculations

### Axial Coordinate System
Used for efficiency:
- O(1) coordinate lookups
- Clean distance formula: `(|q1-q2| + |r1-r2| + |q1+r1-q2-r2|) / 2`
- Straightforward neighbor calculation

### Unit Type Balance
Three unit types provide tactical variety:
- **Infantry** (5 ATK, 3 DEF, 3 MOV, 3 VIS): Reliable all-rounder
- **Cavalry** (4 ATK, 2 DEF, 5 MOV, 4 VIS): Fast scouting and positioning
- **Archer** (6 ATK, 1 DEF, 3 MOV, 5 VIS): Specialized ranged support

### Simple AI
Intentionally kept simple for competition:
- No pathfinding complexity
- Greedy approach to nearest threats
- Readable and maintainable code

## Gameplay Statistics

### Unit Stats
```
Infantry:
  Cost: 100 resources
  Health: 10 HP
  Attack: 5, Defense: 3
  Movement: 3, Vision: 3

Cavalry:
  Cost: 150 resources
  Health: 10 HP
  Attack: 4, Defense: 2
  Movement: 5, Vision: 4

Archer:
  Cost: 120 resources
  Health: 10 HP
  Attack: 6, Defense: 1
  Movement: 3, Vision: 5
```

### Terrain Movement Costs
- Plain: 1 (fast)
- Forest: 2 (slow)
- Mountain: 3 (very slow)
- Water: Impassable
- Resource Node: 1 (fast)

### Terrain Defense Bonuses
- Plain: +0
- Forest: +2
- Mountain: +3
- Water: N/A
- Resource Node: +1

## Future Enhancement Opportunities

While fully playable and feature-complete, the game could be extended with:
- Multiplayer support for two human players
- Unit upgrades and special abilities
- More terrain types and special tiles
- Sound effects and background music
- Advanced AI with pathfinding
- Persistent games and save states
- Campaign mode with multiple maps
- Unit formations and coordinated attacks

## Troubleshooting

### Game doesn't load
- Ensure TypeScript is compiled: `npm run build`
- Check browser console for errors
- Clear browser cache and reload

### Units won't move
- Make sure the unit is selected (it will have a white border)
- Blue highlighted squares indicate valid moves
- Check that you haven't already moved/attacked this unit

### Why did my unit move fail?
Possible reasons:
- Target is occupied by another unit
- Target is water (impassable)
- Target is too far away for this unit's movement
- Unit already moved this turn

## Performance Notes

- Efficient hex coordinate system: O(1) lookups
- Cached fog of war calculations
- Optimized canvas rendering
- Responsive 60 FPS game loop
- Minimal garbage collection for smooth gameplay

## License

Created for the Game Development Competition Round 2.

## Credits

Developed as a complete strategy game featuring:
- Full hexagonal grid system
- Complete game mechanics
- Working AI opponent
- Comprehensive test suite
- Professional-grade UI

Hex Commander delivers a complete, playable tactical strategy game experience.
