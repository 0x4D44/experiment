# Platform Builder - Game & Level Editor

A fully-functional platformer game with an integrated level editor. Create, play-test, and share custom levels with a comprehensive tile-based system including physics, collision detection, and multiple tile types.

## Features

### Game Features
- Realistic physics engine with gravity, acceleration, and friction
- Pixel-perfect collision detection
- Multiple tile types:
  - **Solid Blocks** - Platforms and walls
  - **Spikes** - Hazards that reset player to checkpoint
  - **Springs** - Bounce pads for reaching high areas
  - **Moving Platforms** - Dynamic platforms with custom paths
  - **Checkpoints** - Save points to reset to on death
- Enemy patrol system with custom pathing
- Level completion detection
- 10 pre-made levels with increasing difficulty
- Pause/Resume functionality

### Editor Features
- Drag-and-drop tile placement
- Real-time level preview
- Adjustable tile sizes
- Undo/Redo functionality
- Save/Load custom levels
- Playtest mode for instant testing
- Difficulty rating system (1-5)
- Enemy path editing

### Data Management
- Export levels as JSON
- Import levels from JSON
- Share code generation for level distribution
- Bulk download all levels

## Quick Start

### Playing the Game

1. **Open the Game**
   - Open `platform-builder.html` in a modern web browser
   - Click the "Play Game" tab

2. **Select a Level**
   - Choose any level from the dropdown menu
   - Level name includes difficulty rating

3. **Controls**
   - **Move Left**: Arrow Left or A
   - **Move Right**: Arrow Right or D
   - **Jump**: Arrow Up or W
   - **Restart**: R
   - **Pause**: Click Pause button

4. **Goal**
   - Navigate from the green player square to the pink goal square
   - Avoid spikes, reach checkpoints, navigate obstacles
   - Complete the level to see your time!

### Creating Levels

1. **Open Editor**
   - Click the "Level Editor" tab
   - Give your level a name
   - Set difficulty (1-5)

2. **Select Tile Type**
   - Choose from: Solid, Spike, Spring, Checkpoint
   - Click the tile option to select it (highlighted in blue)

3. **Place Tiles**
   - Click on the canvas to place tiles
   - Tiles snap to grid (size adjustable)
   - See real-time preview

4. **Set Player Position**
   - Click "Set Start" to mark player start position
   - Click "Set Goal" to mark the level goal
   - These define where the player spawns and the objective

5. **Add Enemies (Optional)**
   - Currently, enemies must be added via JSON import
   - Each enemy needs a path defined as arrays of X and Y coordinates

6. **Test Your Level**
   - Click "Playtest Level" to immediately test
   - Switch to "Play Game" tab
   - Confirm your level plays as intended

7. **Save Your Level**
   - Enter level name and difficulty
   - Click "Save Level"
   - Level appears in the "Manage Levels" tab

### Managing Levels

1. **Export Level**
   - Go to "Manage Levels" tab
   - Click "Export Level as JSON"
   - Copy the JSON for sharing or backup

2. **Import Level**
   - Click "Import Level from JSON"
   - Paste JSON from exported level
   - Level is instantly available for play

3. **Download All Levels**
   - Click "Download All Levels"
   - All levels saved as a single JSON file
   - Import later to restore

## Pre-Made Levels

### Level 1: Tutorial - Basic Movement
Difficulty: 1 (Beginner)
- Learn basic controls
- Simple platform jumping
- No hazards

### Level 2: Jump Practice
Difficulty: 2 (Easy)
- Series of increasingly high platforms
- Practice timing jumps
- Build platform jumping skills

### Level 3: Spike Course
Difficulty: 2 (Easy)
- Navigate between spike hazards
- Use checkpoints strategically
- Avoid hazards

### Level 4: Spring Bounce
Difficulty: 2 (Easy)
- Use spring tiles to bounce high
- Master spring mechanics
- Reach elevated platforms

### Level 5: Moving Platforms
Difficulty: 3 (Moderate)
- Time jumps with moving platforms
- Requires platform prediction
- More complex obstacle design

### Level 6: Avoid the Guard
Difficulty: 3 (Moderate)
- Navigate around patrolling enemies
- Use timing to avoid enemies
- Checkpoints for safety

### Level 7: Obstacle Course
Difficulty: 4 (Hard)
- Combination of all mechanics
- Spikes, springs, moving platforms
- Multiple checkpoints for progression

### Level 8: The Gauntlet
Difficulty: 4 (Hard)
- Intense obstacle gauntlet
- Timed enemy patrols
- Requires skill and practice

### Level 9: Guard Patrol
Difficulty: 4 (Hard)
- Multiple enemies with different paths
- Requires careful navigation
- Strategic checkpoint usage

### Level 10: Ultimate Challenge
Difficulty: 5 (Expert)
- All game mechanics combined
- Difficult enemy patterns
- Extreme precision required
- Fastest players will race for best times

## Tile Types Reference

### Solid Block (Brown)
- Default platform/wall
- Blocks player movement
- Supports player standing
- Most common tile type

### Spike (Red)
- Hazard tile
- Instantly resets player to last checkpoint
- Use to create danger zones
- Forces careful navigation

### Spring (Gold)
- Bounce pad
- Propels player upward with high velocity
- Allows reaching otherwise inaccessible areas
- Great for vertical challenges

### Moving Platform (Blue)
- Moves along a defined path
- Player can stand on it
- Requires timing for navigation
- Create dynamic level sections

### Checkpoint (Green)
- Save point for the player
- Player respawns here on hazard contact
- Multiple checkpoints per level
- Essential for longer levels

## Game Mechanics

### Physics
- **Gravity**: Constant downward acceleration (0.6 units/frame)
- **Friction**: Horizontal deceleration when moving (85% per frame)
- **Max Fall Speed**: Terminal velocity of 15 units/frame
- **Jump Power**: Upward velocity of 12 units/frame
- **Max Horizontal Speed**: 8 units/frame

### Collision Detection
- Pixel-perfect AABB (Axis-Aligned Bounding Box) collisions
- Automatic collision direction detection
- Proper resolution based on collision side
- No clipping through walls

### Level Design Tips
1. **Difficulty Progression**
   - Start with simple tutorials
   - Gradually introduce new mechanics
   - Combine mechanics in later levels

2. **Checkpoint Placement**
   - Place after difficult sections
   - Use to "save" player progress
   - Too many makes game too easy
   - Too few frustrates players

3. **Enemy Design**
   - Create patrol paths that are avoidable
   - Use multiple checkpoints before enemies
   - Give player time to predict patterns

4. **Pacing**
   - Mix difficult sections with easier ones
   - Use springs for momentum/fun
   - Vary tile patterns

## Technical Details

### File Structure
```
/
├── platform-builder.html          # Main game/editor UI
├── platform-builder-game.js        # Game logic and mechanics
├── platform-builder-core.ts        # Core engine (TypeScript)
├── platform-builder.test.ts        # Comprehensive test suite (34 tests)
└── PLATFORM_BUILDER_README.md      # This file
```

### Test Coverage
- **Physics Engine**: 9 tests
  - Gravity, jumping, movement, friction, acceleration
- **Collision Detection**: 7 tests
  - Collision detection, direction determination
- **Game Logic**: 5 tests
  - Spike hazards, checkpoints, level completion, bounds, fall detection
- **Level Building**: 6 tests
  - Level creation, tile management, enemy placement, difficulty
- **Serialization**: 3 tests
  - JSON serialization, deserialization, round-trip preservation
- **Integration**: 4 tests
  - Complete gameplay sequences, obstacle avoidance, checkpoints, enemies

### Running Tests
```bash
npm test -- platform-builder.test.ts
```

All 34 tests should pass, covering:
- Core physics calculations
- Collision resolution for all tile types
- Complete gameplay scenarios
- Level save/load functionality

### Browser Compatibility
- Chrome/Chromium (Recommended)
- Firefox
- Safari
- Edge
- Any modern ES6+ compatible browser

## Level Format (JSON)

```json
{
  "id": "level_name",
  "name": "Level Display Name",
  "width": 800,
  "height": 600,
  "difficulty": 3,
  "playerStart": { "x": 40, "y": 500 },
  "playerEnd": { "x": 750, "y": 400 },
  "tiles": [
    {
      "type": "solid",
      "x": 0,
      "y": 550,
      "width": 800,
      "height": 50,
      "pathIndex": 0
    }
  ],
  "enemies": [
    {
      "id": "enemy_1",
      "x": 300,
      "y": 400,
      "width": 20,
      "height": 20,
      "velocityX": 0,
      "velocityY": 0,
      "grounded": true,
      "pathX": [250, 400, 250],
      "pathY": [400, 400, 400],
      "pathIndex": 0,
      "speed": 2
    }
  ]
}
```

## Performance Notes
- Game runs at 60 FPS on modern hardware
- Physics updates once per frame
- Collision detection uses broad-phase filtering
- No memory leaks in extended play sessions
- Mobile devices may experience occasional frame drops

## Troubleshooting

### Player Falls Through Platforms
- Ensure tile type is "solid" or "moving_platform"
- Check collision detection isn't disabled
- Verify platform height > player height

### Spikes Not Resetting Player
- Confirm spike tile type is set to "spike"
- Check spike collision area overlaps player
- Verify checkpoint is set before spike

### Enemies Not Moving
- Ensure pathX and pathY arrays have at least 2 points
- Check pathIndex is properly managed
- Verify enemy x/y match path coordinates initially

### Level Won't Save
- Check level has a name
- Ensure difficulty is 1-5
- Browser local storage may be full

### Game Runs Slowly
- Close other browser tabs
- Reduce number of enemies
- Simplify level complexity (fewer tiles)

## Future Enhancement Ideas
1. Slope tiles for sliding mechanics
2. Moving spike hazards
3. Power-ups (invincibility, speed boost)
4. Multiple players/co-op
5. Collectible items for scoring
6. Time trial/speedrun mode
7. Leaderboards
8. Level sharing via QR code
9. Custom sprite support
10. Sound effects and music

## License
Created for the Game Development Competition - Round 2

## Credits
- Physics Engine: Custom implementation with gravity and friction
- Collision Detection: Separating Axis Theorem (AABB optimization)
- UI Framework: Vanilla HTML/CSS/JavaScript
- Test Framework: Jest

---

**Happy Level Creating! Have Fun!**
