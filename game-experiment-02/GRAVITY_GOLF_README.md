# Gravity Golf - Physics-Based Mini Golf Game

A unique take on mini-golf that combines classic golf mechanics with advanced physics simulations. Navigate through 9 challenging holes featuring gravity wells, obstacles, wormholes, and black holes.

## Game Overview

Gravity Golf is a strategic mini-golf game where you must use gravity manipulation to guide your ball to the hole. Each hole presents unique physics challenges, from simple straight shots to complex scenarios with multiple gravity sources and hazards.

### Key Features

- **Physics-Based Gameplay**: Realistic gravity simulation with Newton's laws
- **9 Unique Holes**: Progressive difficulty from beginner to expert
- **Gravity Modifiers**: Place attractive and repulsive gravity wells strategically
- **Dynamic Obstacles**:
  - Walls: Static barriers to bounce off
  - Black Holes: Destructive gravitational anomalies
  - Wormholes: Teleport across the map
  - Asteroids: Bounceable obstacles
- **Par System**: Classic golf scoring with strokes and par targets
- **Trajectory Prediction**: Visual guides to help you aim

## How to Play

### Basic Controls

1. **Adjust Angle**: Use the angle slider (0-360 degrees)
2. **Set Power**: Use the power slider (0-100%)
3. **Hit Ball**: Click the "HIT BALL" button to shoot
4. **Place Modifiers**: Click "Add Attractor" or "Add Repulsor" buttons, then click on the canvas to place
5. **Navigate**: Use "Next Hole" when ball is in the hole, "Restart Hole" to try again

### Scoring

- Strokes are counted each time you hit the ball
- Par is the target number of strokes for each hole
- Score relative to par:
  - Under par (below target) = better score
  - Over par (above target) = worse score
- Total score is the sum of all strokes for 9 holes

### Game Strategy

1. **Early Holes (1-3)**: Learn basic physics and aiming
   - Hole 1: Simple straight shot
   - Hole 2: Single gravity attractor
   - Hole 3: Dual gravity wells

2. **Intermediate Holes (4-6)**: Introduce obstacles and modifiers
   - Hole 4: Wall maze navigation
   - Hole 5: Black hole danger zone
   - Hole 6: Wormhole teleportation

3. **Advanced Holes (7-9)**: Complex multi-element challenges
   - Hole 7: Asteroid field with gravity
   - Hole 8: Mixed attractive/repulsive wells
   - Hole 9: Ultimate challenge with all elements

## Gravity Mechanics

### Attractive Gravity Wells (Blue)

- Pull the ball toward them
- Strength decreases with distance squared (realistic physics)
- Useful for redirecting shots around obstacles
- Marked with "A" label

### Repulsive Gravity Wells (Red)

- Push the ball away from them
- Create "force fields" to avoid dangerous areas
- Can be used to escape black holes
- Marked with "R" label

### Gravity Well Placement

- Limited modifiers per hole (0-4 depending on difficulty)
- Click modifier button to place
- Click on canvas where you want the well
- Can only place within allowed limits

## Hole Guide

### Hole 1: The Gentle Start
- **Par**: 2
- **Challenge**: Simple straight shot
- **Strategy**: Get comfortable with controls
- **Modifiers**: 0 available

### Hole 2: Single Attractor
- **Par**: 2
- **Challenge**: Gravity well in the middle
- **Strategy**: Time shot to curve around well
- **Modifiers**: 0 available

### Hole 3: Dual Wells
- **Par**: 3
- **Challenge**: Two attractors creating complex path
- **Strategy**: Navigate between wells carefully
- **Modifiers**: 0 available

### Hole 4: Wall Maze
- **Par**: 3
- **Challenge**: Multiple wall obstacles
- **Strategy**: Use gravity modifiers to create curved paths
- **Modifiers**: 2 available

### Hole 5: Black Hole Danger
- **Par**: 3
- **Challenge**: Dangerous black hole in the middle
- **Strategy**: Use repulsive well to stay away
- **Modifiers**: 2 available

### Hole 6: Wormhole Portal
- **Par**: 2
- **Challenge**: Jump through wormhole to opposite side
- **Strategy**: Time entrance to wormhole correctly
- **Modifiers**: 0 available

### Hole 7: Asteroid Field
- **Par**: 4
- **Challenge**: Many asteroids with dual gravity wells
- **Strategy**: Careful navigation and bouncing
- **Modifiers**: 3 available

### Hole 8: The Gravity Gauntlet
- **Par**: 4
- **Challenge**: Mixed attractive/repulsive wells
- **Strategy**: Use opposing forces to your advantage
- **Modifiers**: 3 available

### Hole 9: Cosmic Challenge
- **Par**: 5
- **Challenge**: Everything combined - black hole, wormhole, asteroids, complex gravity
- **Strategy**: Master all mechanics
- **Modifiers**: 4 available

## Physics Engine

### Implementation Details

- **Gravity Calculation**: F = G × m₁ × m₂ / r²
- **Time Step**: 0.016 seconds (60 FPS simulation)
- **Damping**: 0.98 per frame (98% velocity retention)
- **Ground Friction**: Applied when ball moving slowly
- **Max Velocity**: 500 units per second (clamped)
- **Stop Threshold**: 0.1 units per second

### Collision System

- Circular collision detection (ball radius vs object radius)
- Bounce response with energy loss (80% retention)
- Boundary detection and bouncing
- Wormhole teleportation on entrance
- Black hole destruction

## Technical Architecture

### Core Systems

1. **Vector2D**: 2D vector math library
   - All physics calculations use vector operations
   - Supports magnitude, normalization, dot product, distance
   - Optimized with squared magnitude comparisons

2. **PhysicsEngine**: Main physics simulation
   - Gravity application from multiple wells
   - Velocity and position updates
   - Collision detection and response
   - Damping and friction application
   - Trajectory prediction

3. **GameManager**: Game logic and state
   - Hole management and progression
   - Score tracking
   - Gravity modifier placement
   - Game flow control

4. **CanvasRenderer**: Graphics system
   - 2D canvas rendering
   - Gravity well visualization
   - Obstacle and hazard drawing
   - Ball and hole rendering
   - UI overlay

### File Structure

```
gravity-golf/
├── src/
│   ├── utils/
│   │   └── Vector2D.ts          # 2D vector mathematics
│   ├── types/
│   │   └── Physics.ts            # Type definitions
│   ├── systems/
│   │   ├── PhysicsEngine.ts      # Physics simulation
│   │   ├── GameManager.ts        # Game logic
│   │   ├── CanvasRenderer.ts     # Graphics rendering
│   │   └── *.test.ts             # Unit tests
│   └── entities/
│       └── HoleConfigurations.ts # Level definitions
├── dist/                          # Compiled JavaScript
├── gravity-golf.html              # Main game interface
└── GRAVITY_GOLF_README.md         # This file
```

## Testing

Comprehensive test suite with 81+ tests covering:

- **Vector2D Operations** (25 tests)
  - Vector addition, subtraction, scaling
  - Magnitude, normalization, dot product
  - Distance calculations, rotation
  - Clamping and equality checks

- **Physics Engine** (30 tests)
  - Gravity application and attractive/repulsive wells
  - Velocity damping and friction
  - Position updates and boundary collisions
  - Impulse application
  - Trajectory prediction
  - Multi-well interactions

- **Game Manager** (26 tests)
  - Game initialization and state management
  - Scoring and stroke counting
  - Ball physics and motion state
  - Gravity modifier placement and limits
  - Hole completion and progression
  - Trajectory preview

### Running Tests

```bash
npm test
```

Filter to Gravity Golf tests:
```bash
npm test -- src/utils/Vector2D.test.ts src/systems/PhysicsEngine.test.ts src/systems/GameManager.test.ts
```

## Building and Running

### Prerequisites
- Node.js 14+
- npm or yarn

### Setup

```bash
npm install
npm run build
```

### Playing the Game

1. Open `gravity-golf.html` in a web browser (Chrome, Firefox, Safari, Edge)
2. Or use a local web server:
   ```bash
   npx serve .
   ```
   Then navigate to `http://localhost:3000/gravity-golf.html`

## Game Controls Summary

| Action | Control |
|--------|---------|
| Adjust Aim Angle | Angle Slider (0-360°) |
| Adjust Shot Power | Power Slider (0-200) |
| Hit Ball | Click "HIT BALL" button |
| Place Attractor | Click "Add Attractor" then click canvas |
| Place Repulsor | Click "Add Repulsor" then click canvas |
| Next Hole | Click "NEXT HOLE" (when ball in hole) |
| Restart Hole | Click "Restart Hole" button |
| New Game | Click "Restart Game" button |

## Tips and Tricks

1. **Watch the Circles**: Gravity well radius shows area of effect
2. **Chain Bounces**: Use obstacles to bounce around angles
3. **Angle Matters**: Even small angle changes create big differences
4. **Power Planning**: High power sometimes overshoots; lower power gives more control
5. **Wormhole Strategy**: Use wormholes to skip difficult sections
6. **Repulsor Shields**: Place repulsive wells to protect from black holes
7. **Practice Aiming**: The angle slider wraps around 360°

## Scoring Guide

### Target Scores by Difficulty

**Par or Better** (Expert): Score ≤ Par
**1-3 Over Par** (Good): Score = Par + 1-3
**4-5 Over Par** (Average): Score = Par + 4-5
**6+ Over Par** (Needs Practice): Score > Par + 5

### Overall Game Scores

- **Tournament Score** = Total strokes / Total par
- **Example**: 40 strokes on 30 par = 133% or +10

## Performance

- **Frame Rate**: 60 FPS target
- **Physics Updates**: Per-frame simulation
- **Collision Detection**: Circle-circle detection (O(n) per object)
- **Memory Usage**: ~2-5 MB for game state and canvas

## Known Limitations

- Single player only
- No persistence/save games
- No sound or music
- No mobile touch controls
- Limited to 9 holes (by design)

## Future Enhancements

Potential features for future versions:
- Leaderboard/score tracking
- Mobile-friendly touch controls
- Sound effects and background music
- Custom hole designer
- Multiplayer turn-based mode
- Power-ups and special abilities
- Procedurally generated holes
- Physics parameter editor

## Credits

Developed as part of Round 2 of the game development competition.

**Technologies Used**:
- TypeScript for type-safe development
- Jest for comprehensive testing
- HTML5 Canvas for graphics
- Pure JavaScript/Canvas (no external game engine)

## License

This project is created for educational and competition purposes.

## Support

For issues or questions about gameplay:
1. Review the "Tips and Tricks" section
2. Check individual hole guides
3. Review the testing suite to understand physics behavior
4. Read the source code comments in the physics engine

---

**Enjoy the game! May your gravity golf score shine!**
