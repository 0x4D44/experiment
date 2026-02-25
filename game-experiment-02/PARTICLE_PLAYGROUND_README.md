# Particle Playground

A physics-based puzzle game where you manipulate particle flow using attractors, repulsors, and environmental effects to guide particles to goals.

## Game Features

### Core Gameplay
- **Particle Physics Simulation**: Realistic physics with mass, charge, velocity, and acceleration
- **Charge Interactions**: Positive/negative particles repel or attract based on Coulomb force
- **Gravity System**: Gravitational attraction between all particles with mass
- **Collision Detection**: Particles collide and bounce off each other and barriers
- **Environmental Effects**:
  - Attractors: Pull particles toward a point
  - Repulsors: Push particles away from a point
  - Barriers: Physical obstacles particles can't pass through
  - Portals: Teleport particles to different locations

### Game Modes

#### Puzzle Mode
- **16 Levels** with increasing difficulty
- **Goal-Based**: Guide particles to goal zones
- **Progressive Learning**: Early levels teach basic mechanics
- **Time Limits**: Optional time challenges on later levels
- Level progression from simple gravity to complex charge interactions

#### Sandbox Mode
- **Free Experimentation**: Create and manipulate particles freely
- **Particle Types**: Spawn neutral, positive, or negative particles
- **Attractor/Repulsor Placement**: Place custom attractors and repulsors
- **Gravity Toggle**: Turn gravity on/off for experimentation
- **No Goals**: Pure physics exploration

### Visual Features
- **Real-time Rendering**: 60 FPS canvas-based visualization
- **Particle Trails**: Visual paths showing particle movement history
- **Color Coding**:
  - Gray: Neutral particles (gravity only)
  - Red: Positive particles (charge repulsion)
  - Blue: Negative particles (charge attraction)
- **Zone Indicators**:
  - Yellow/Green: Goal zones (yellow = incomplete, green = complete)
  - Gold circles: Attractor range
  - Cyan circles: Repulsor range
  - Green portals: Portal locations
- **Grid Background**: Reference grid for spatial awareness

## Controls

### Keyboard Shortcuts
- **SPACE**: Pause/Resume
- **N**: Next Level
- **P**: Previous Level
- **R**: Reset Current Level
- **C**: Clear All Particles
- **T**: Toggle Particle Trails
- **D**: Toggle Debug Information
- **1/2/3**: Spawn Neutral/Positive/Negative particles (Sandbox only)
- **A**: Add Attractor (Sandbox only)
- **X**: Add Repulsor (Sandbox only)

### Mouse Controls
- **Click on Canvas** (Sandbox Mode):
  - Spawns selected particle type
  - Places selected attractor/repulsor
  - Requires selecting particle/attractor type first

### UI Buttons
- **Mode Toggle**: Switch between Puzzle and Sandbox modes
- **Level Selection**: Choose specific levels
- **Playback Controls**: Pause, resume, reset
- **Display Options**: Show/hide trails, debug info
- **Particle Spawning**: Select particle type (Sandbox)
- **Attractor Placement**: Place attractors/repulsors (Sandbox)

## Puzzle Levels

### Level 1-3: Basic Mechanics
1. **Gravity Basics**: Simple gravity-driven particle guidance
2. **Attractor Force**: Using attractors to pull particles
3. **Charge Repulsion**: Repelling positive charges away from repulsors

### Level 4-7: Intermediate
4. **Opposite Attraction**: Positive/negative particle interaction
5. **Navigate Barriers**: Threading through physical obstacles
6. **Portal Jump**: Using teleportation mechanics
7. **Triforce**: Coordinating three particles to a goal

### Level 8-10: Advanced
8. **Charge Separation**: Routing particles to different goals
9. **Magnetic Maze**: Complex navigation through barrier maze
10. **Gravity Well**: Escaping strong gravitational pull

### Level 11-16: Challenge
- Procedurally generated challenge levels with varying difficulty
- Custom combinations of particles, attractors, and goals

## Physics Model

### Force Calculations
- **Coulomb Force**: F = k × q₁ × q₂ / r²
  - Positive-positive: Repulsion
  - Negative-negative: Repulsion
  - Positive-negative: Attraction

- **Gravity**: F = G × m₁ × m₂ / r²
  - All mass attracts all other mass
  - Global downward acceleration when enabled

### Integration
- **Velocity Verlet Integration**: Stable physics simulation
- **Friction**: 0.99 multiplier per frame (energy dissipation)
- **Distance Clamping**: Prevents extreme forces at very short distances

### Collision Model
- **Particle-Particle**: Elastic collision with velocity exchange
- **Particle-Wall**: Bounce with 0.8 restitution coefficient
- **Particle-Barrier**: Reflection with velocity reversal
- **Particle-Portal**: Instant teleportation

## Technical Details

### Architecture
- **Physics Engine** (`particle-physics.ts`):
  - Vector2D math utilities
  - Particle class with position/velocity/acceleration
  - Force calculation and integration
  - Collision detection

- **Game Engine** (`particle-game.ts`):
  - Level management and progression
  - Goal checking and win conditions
  - Mode management (Puzzle vs Sandbox)
  - Level definitions with 16+ built-in levels

- **Renderer** (`particle-renderer.ts`):
  - Canvas-based 2D rendering
  - Particle and trail visualization
  - UI HUD rendering
  - Debug information display

- **Main Application** (`particle-main.ts`):
  - Event handling
  - Game loop and frame timing
  - UI interaction and state management

### Performance
- Optimized for 60 FPS gameplay
- O(n²) particle interactions (can be optimized with spatial partitioning if needed)
- Efficient canvas rendering with minimal redraws
- Trail rendering with alpha compositing for visual effect

## Testing

The project includes comprehensive unit tests covering:

### Physics Tests
- Vector math operations (addition, subtraction, magnitude, normalization)
- Particle physics (velocity, acceleration, position updates)
- Force calculations (attractors, repulsors)
- Collision detection and response
- Barrier interaction
- Portal teleportation

### Game Logic Tests
- Level loading and progression
- Goal completion detection
- Particle-goal interaction
- Physics engine integration
- Particle spawning

### Running Tests
```bash
npm test                 # Run all tests
npm run test:watch     # Run in watch mode
npm run test:coverage  # Generate coverage report
```

All tests pass with 100% coverage of critical physics and game systems.

## Build Instructions

### Prerequisites
- Node.js 14+
- npm

### Setup
```bash
npm install
```

### Development
```bash
npm run build          # Compile TypeScript
npm run dev           # Watch mode compilation
```

### Running
Open `particle-playground.html` in a modern web browser after building.

### Browser Requirements
- ES2020 support (modern browsers)
- Canvas 2D API
- RequestAnimationFrame

## Gameplay Tips

### Puzzle Mode
1. **Start Simple**: First levels teach basic mechanics
2. **Observe Behavior**: Watch how forces interact before moving
3. **Use Trails**: Enable particle trails to understand trajectories
4. **Small Adjustments**: Minor changes in attractor strength can make big differences
5. **Time Management**: On timed levels, plan your approach before starting

### Sandbox Mode
1. **Experiment**: Try different particle combinations freely
2. **Learn Physics**: Observe real physics principles in action
3. **Challenge Yourself**: Create puzzle scenarios for others
4. **Optimize Paths**: Try to get particles to specific locations efficiently

## Game Balance

### Difficulty Progression
- Levels 1-3: Learn controls and basic forces
- Levels 4-7: Combine mechanics and navigate obstacles
- Levels 8-10: Complex scenarios requiring planning
- Levels 11-16: Procedural challenges for infinite replayability

### Tuning Parameters
Key constants in `particle-physics.ts`:
- `COULOMB_CONSTANT = 0.5`: Charge force strength
- `GRAVITY_CONSTANT = 0.1`: Gravitational pull strength
- `FRICTION = 0.99`: Energy dissipation per frame
- `MIN_DISTANCE = 20`: Minimum distance for force calculation
- `MAX_DISTANCE = 500`: Maximum distance for force effect

## Future Enhancements

Potential additions for expanded gameplay:
- Score system with time/efficiency ratings
- Level editor for custom puzzles
- Multiplayer challenges
- Advanced particle types (mass variations)
- Magnetic field tiles
- Particle fusion mechanics
- More complex barrier types
- Sound effects and music
- Leaderboard system

## Credits

Created for the Game Development Competition - Round 3

## License

MIT License - Free to use and modify
