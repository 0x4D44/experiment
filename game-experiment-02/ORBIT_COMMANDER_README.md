# Orbit Commander - Space Navigation Game

A physics-based space navigation game where you command a spacecraft through a realistic solar system using orbital mechanics, gravity assists, and fuel management.

## Overview

Orbit Commander challenges players to complete 10 increasingly difficult missions by navigating a spacecraft through a solar system populated by realistic celestial bodies. The game uses actual orbital mechanics principles, including:

- Newtonian gravity calculations
- Realistic orbital dynamics
- Fuel consumption and management
- Trajectory prediction with real-time visualization
- Time acceleration controls

## Getting Started

### Play the Game

1. Open `orbit-commander.html` in a web browser
2. Click "START GAME" to begin
3. Launch your spacecraft and complete the 10 missions

### Run Tests

```bash
npm test -- orbit-commander.test.ts
```

All 19 unit and integration tests should pass.

### Build from Source

```bash
npm run build
```

This compiles TypeScript to JavaScript for the web.

## Gameplay

### Launch Phase

When starting each mission, you begin in the launch phase:
- **Arrow Keys / A/D**: Rotate launch angle (0-360 degrees)
- **W/S**: Adjust launch velocity (0-100 units/second)
- **SPACEBAR**: Launch spacecraft

The green circle shows your launch angle, the red line indicates direction, and the blue bar shows velocity.

### Flight Phase

After launch, control your spacecraft in flight:
- **W / Arrow Up**: Engage main thruster
- **A / Arrow Left**: Rotate thrust vector counter-clockwise
- **D / Arrow Right**: Rotate thrust vector clockwise
- **1/2/5/0**: Time scale (1x, 2x, 5x, 10x - speeds up simulation for planning)

The white dashed line shows your predicted trajectory 100 simulation steps ahead.

### UI Elements

- **Green dot**: Your spacecraft
- **Colored bodies**: Planets and moon (labeled)
- **Orange glow**: Indicates active thrust
- **White dashed line**: Predicted trajectory path
- **Score display**: Current mission progress and total score

## Missions

### Mission 1: Mars Bound (Difficulty 1.0x)
Reach Mars from Earth orbit. Basic introduction to orbital mechanics.
- Fuel: 500 units
- Target: Get within 200 units of Mars

### Mission 2: Venus Run (Difficulty 1.2x)
Reach Venus with fuel efficiency bonus.
- Fuel: 400 units
- Target: Get within 200 units of Venus

### Mission 3: Lunar Deployment (Difficulty 1.3x)
Deploy satellite to lunar orbit.
- Fuel: 350 units
- Target: Get within 80 units of Moon (closer tolerance)

### Mission 4: Gravity Assist Maneuver (Difficulty 1.5x)
Use gravity assists to reach the outer system.
- Fuel: 600 units
- Target: Reach Jupiter area

### Mission 5: Mercury Challenge (Difficulty 1.6x)
Navigate to the closest planet to the Sun.
- Fuel: 450 units
- Target: Get within 150 units of Mercury

### Mission 6: Fuel Conservation (Difficulty 1.8x)
Complete a mission with minimal fuel use.
- Fuel: 300 units
- Target: Reach any planet with 200+ fuel remaining

### Mission 7: Asteroid Strike (Difficulty 2.0x)
Intercept a moving asteroid.
- Fuel: 400 units
- Target: Get within 100 units of asteroid

### Mission 8: Dual Rendezvous (Difficulty 2.2x)
Visit two planets in one mission.
- Fuel: 550 units
- Target: Reach Mars, then Venus

### Mission 9: Grand Tour (Difficulty 2.5x)
Visit multiple planets in sequence.
- Fuel: 800 units
- Target: Visit Mercury, Venus, Mars, return to start

### Mission 10: Impossible Challenge (Difficulty 3.0x)
Navigate through the asteroid belt safely.
- Fuel: 250 units
- Target: Traverse asteroid field safely

## Physics System

### Orbital Mechanics

The game implements realistic Newtonian physics:

```
Force = G * (m1 * m2) / r²
Acceleration = Force / mass
```

Where:
- G = 5.0 (gravitational constant for game balance)
- m1, m2 = masses of interacting bodies
- r = distance between bodies

### Celestial Bodies

| Body | Mass | Orbital Velocity | Distance from Sun |
|------|------|------------------|--------------------|
| Sun | 1000 | - | 0 |
| Mercury | 50 | 8 u/s | 300 units |
| Venus | 100 | 6 u/s | 500 units |
| Earth | 150 | 4.5 u/s | 700 units |
| Moon | 10 | Orbits Earth | 30 units from Earth |
| Mars | 75 | 3.5 u/s | 1000 units |
| Jupiter | 300 | 2 u/s | 1600 units |

### Thrust System

- Main thruster power: 15 units
- Fuel consumption: 2 * thrustPower * deltaTime
- Fuel can be allocated strategically for course corrections

### Trajectory Prediction

The game calculates your predicted path 100 simulation steps ahead using:
1. Current position and velocity
2. Gravitational acceleration from all bodies
3. No thrust in prediction (pure gravity calculation)

This helps you plan your maneuvers and gravity assists.

## Scoring System

**Base Score per Mission:**
- 1000 points + Fuel Efficiency Bonus
- Fuel Efficiency Bonus = (Remaining Fuel / Max Fuel) * 500
- Multiplied by Mission Difficulty (1.0x to 3.0x)

**Example:** Complete Mission 5 with 100 fuel remaining out of 450:
- Fuel Bonus = (100 / 450) * 500 = 111 points
- Mission Score = (1000 + 111) * 1.6 = 1777 points

## Technical Details

### Physics Engine

The `PhysicsEngine` class handles all physics calculations:
- **calculateGravitationalForce()**: Newton's law of universal gravitation
- **calculateAcceleration()**: Sum of gravitational forces + thrust
- **updatePhysics()**: Integrates equations of motion each frame
- **predictTrajectory()**: Simulates future path for visualization

### Game Architecture

- **OrbitCommander**: Main game controller
  - State management
  - Event handling
  - Rendering
  - Mission logic

- **Vector Math**: Utility functions for 2D physics
  - Addition, subtraction, scaling
  - Magnitude calculation
  - Normalization
  - Distance calculation

### Data Structures

```typescript
interface GameState {
  spacecraft: Spacecraft;
  bodies: Map<string, CelestialBody>;
  currentMission: Mission;
  score: number;
  totalScore: number;
  missionIndex: number;
  time: number;
  timeScale: number;
  trajectoryPoints: Vector2D[];
  gameRunning: boolean;
  launchPhase: boolean;
  launchAngle: number;
  launchVelocity: number;
}
```

## Testing

The test suite covers:

### Vector Operations (10 tests)
- Vector creation and arithmetic
- Magnitude and normalization
- Distance calculations

### Physics Engine (4 tests)
- Gravitational force calculations
- Inverse square law verification
- Force scaling with mass

### Trajectory Prediction (1 test)
- Multi-point trajectory generation
- Proper spacing and data validation

### Integration Tests (4 tests)
- Energy conservation in stable orbits
- Thrust application and acceleration
- Fuel consumption during thrust
- No fuel consumption during coast

**Total: 19 tests, 100% pass rate**

## Controls Quick Reference

| Control | Launch Phase | Flight Phase |
|---------|--------------|--------------|
| Arrow Up / W | Increase velocity | Main thruster |
| Arrow Down / S | Decrease velocity | N/A |
| Arrow Left / A | Rotate angle left | Rotate thrust left |
| Arrow Right / D | Rotate angle right | Rotate thrust right |
| SPACEBAR | Launch! | N/A |
| 1 | N/A | 1x time scale |
| 2 | N/A | 2x time scale |
| 5 | N/A | 5x time scale |
| 0 | N/A | 10x time scale |

## Tips & Tricks

### Gravity Assists
Use planets as gravity assists to change direction without fuel:
1. Aim to pass near a large planet
2. The planet's gravity will curve your trajectory
3. Exit on the other side with new direction

### Fuel Management
- Time acceleration (1/2/5/0) helps you see long-term trajectories
- Use small thruster bursts for adjustments
- Save fuel for critical course corrections
- The fuel bar shows exactly when you'll run out

### Orbital Timing
- Plan launches to meet planets at optimal points
- Watch the trajectory line - it shows where you'll go
- Remember: all orbits are elliptical if perturbed

### Mission 10 Strategy
The asteroid field is the ultimate challenge:
- Use precise small thrusts
- Navigate between asteroids
- Save fuel for final approach
- Time acceleration helps scout the field

## Files

- `orbit-commander.ts` - Main game source (TypeScript)
- `orbit-commander.test.ts` - Comprehensive test suite
- `orbit-commander.html` - Web interface and launcher
- `orbit-commander.js` - Compiled JavaScript (generated)
- `2025.11.07 - DESIGN - Orbit Commander Game.md` - Design document
- `2025.11.07 - JRN - Orbital Mechanics Development.md` - Development journal

## System Requirements

- Modern web browser with Canvas support
- 800x600 minimum screen resolution
- No external dependencies (pure JavaScript physics)

## Browser Compatibility

- Chrome/Chromium (Latest)
- Firefox (Latest)
- Safari (Latest)
- Edge (Latest)

## Known Limitations

- Maximum 8 celestial bodies in the system
- Trajectory prediction is 100 steps (not infinite)
- No collision detection between spacecraft and asteroids
- No pause functionality during flight

## Development

This game was developed using:
- **Language**: TypeScript
- **Testing**: Jest
- **Physics**: Custom Newtonian implementation
- **Rendering**: HTML Canvas 2D

All code follows TypeScript strict mode with no compiler warnings.

## License

MIT - Created for game development competition Round 3

## Credits

Physics engine inspired by classic orbital mechanics simulations.
Game design emphasizes learning through experimentation with real physics.

---

**Good luck, Commander! May your orbits be true and your fuel abundant!**
