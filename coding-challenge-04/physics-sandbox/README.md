# Physics Sandbox - Interactive Physics Simulation

An AMAZING interactive physics sandbox web application featuring real-time physics simulation, stunning visual effects, and intuitive controls. Built with custom physics engine implementation for a coding challenge competition.

![Physics Sandbox](https://img.shields.io/badge/Physics-Sandbox-blue)
![HTML5 Canvas](https://img.shields.io/badge/HTML5-Canvas-orange)
![JavaScript](https://img.shields.io/badge/JavaScript-ES6+-yellow)
![Tests](https://img.shields.io/badge/Tests-Passing-green)

## Features

### Physics Simulation
- **Real Physics Engine**: Custom implementation with accurate gravity, collision detection, and momentum
- **Gravity Simulation**: Toggle-able gravity with realistic acceleration (9.8 m/s²)
- **Collision Detection**: Advanced collision detection and response for circles and boxes
- **Momentum & Velocity**: Realistic momentum conservation and velocity calculations
- **Wall Boundaries**: Objects bounce off screen edges with configurable restitution

### Visual Effects
- **Particle Trails**: Beautiful motion trails that fade over time
- **Glow Effects**: Dynamic glow and shadow effects for all objects
- **Gradient Backgrounds**: Animated gradient background with floating particles
- **Grid Overlay**: Semi-transparent grid for depth perception
- **Velocity Arrows**: Visual indicators showing object velocity and direction
- **Color Variations**: Every object gets a unique, vibrant HSL color

### Interactive Controls
- **Mouse Drag**: Click and drag objects to move them around
- **Throw Mechanic**: Release dragged objects to throw them with velocity
- **Click to Spawn**: Click empty space to create new objects
- **Multiple Shapes**: Spawn circles or boxes with adjustable sizes
- **Touch Support**: Full touch screen support for mobile devices

### User Interface
- **Real-time FPS Counter**: Monitor performance at 60fps
- **Object Counter**: Track number of active physics objects
- **Tool Selection**: Easy switching between circle and box tools
- **Size Slider**: Adjust spawn size from 10 to 80 pixels
- **Control Buttons**: Quick access to all features
- **Keyboard Shortcuts**: Power user shortcuts for all actions

## Getting Started

### Installation

No installation or build process required! This is a pure HTML5/JavaScript application.

```bash
# Clone or download the project
cd physics-sandbox

# Open in browser (any of these methods work)
open index.html                    # macOS
xdg-open index.html               # Linux
start index.html                  # Windows
python3 -m http.server 8000       # Or use a simple HTTP server
```

### Usage

1. **Open `index.html`** in your web browser
2. **Click** anywhere on the canvas to spawn objects
3. **Drag** objects to move them around
4. **Release** while dragging to throw objects
5. **Experiment** with different settings and tools!

## Controls

### Mouse Controls
- **Left Click**: Spawn new object at cursor position
- **Click + Drag**: Pick up and move objects
- **Drag + Release**: Throw objects with velocity

### Keyboard Shortcuts
| Key | Action |
|-----|--------|
| `1` | Select Circle tool |
| `2` | Select Box tool |
| `G` | Toggle Gravity on/off |
| `T` | Toggle Trails on/off |
| `Space` | Pause/Resume simulation |
| `C` | Clear all objects |
| `R` | Spawn Rain (50 falling objects) |

### UI Buttons
- **Circle/Box**: Switch between object types
- **Size Slider**: Adjust object spawn size (10-80px)
- **Gravity**: Toggle gravity simulation
- **Trails**: Toggle motion trails
- **Pause**: Pause/resume physics simulation
- **Spawn Rain**: Create a rain of 50 falling objects
- **Clear All**: Remove all objects from canvas

## Technical Details

### Architecture

The application is built with three main modules:

1. **`physics-engine.js`** - Custom physics engine
   - Vector2 math utilities
   - PhysicsObject base class
   - Circle and Box implementations
   - PhysicsEngine with collision detection
   - Gravity and force calculations

2. **`renderer.js`** - Canvas rendering system
   - Beautiful visual effects
   - Trail rendering
   - Shadow and glow effects
   - Gradient backgrounds
   - Animated particles

3. **`app.js`** - Main application and interaction
   - Game loop at 60fps
   - Mouse and touch event handling
   - UI control management
   - State management

### Physics Engine Features

#### Vector Mathematics
- 2D vector operations (add, subtract, multiply, divide)
- Magnitude and normalization
- Dot product for collision detection
- Distance calculations

#### Collision Detection
- **Circle-Circle**: Accurate distance-based detection with separation
- **Circle-Box**: AABB (Axis-Aligned Bounding Box) collision
- **Box-Box**: AABB collision with overlap resolution
- Impulse-based collision response
- Configurable restitution (bounciness)

#### Physics Calculations
- Force application (F = ma)
- Acceleration integration
- Velocity integration with friction
- Position updates with fixed time step
- Angular velocity for boxes
- Wall boundary constraints

### Performance

- **Target**: 60 FPS
- **Fixed Time Step**: 16.67ms (1/60 second)
- **Optimizations**:
  - Efficient collision detection
  - Spatial partitioning could be added for 100+ objects
  - Canvas rendering optimizations
  - Trail length limiting
  - Background particle pooling

### Visual Effects Details

- **Gradient Shading**: Radial gradients for 3D appearance
- **Glow Effect**: Larger transparent circles behind objects
- **Shadows**: Offset shadows with blur for depth
- **Trails**: Fading alpha particles following fast objects
- **Animated Background**: Moving particle field
- **Velocity Arrows**: Yellow arrows showing movement direction

## Testing

The project includes comprehensive unit tests for all physics calculations.

### Running Tests

```bash
# Run tests with Node.js
node physics-engine.test.js
```

### Test Coverage

The test suite includes 40+ tests covering:
- Vector2 mathematics (addition, subtraction, magnitude, normalization, dot product)
- PhysicsObject behavior (force application, velocity, acceleration)
- Circle and Box properties (collision detection, containment)
- PhysicsEngine features (gravity, collisions, boundaries)
- Collision response and momentum conservation
- Wall boundary constraints
- Object grabbing mechanics

All tests use a custom test runner with clear output:

```
✓ Vector2: Constructor creates vector with correct x and y
✓ Vector2: Add returns correct sum
✓ PhysicsEngine: Gravity applies force to objects
✓ PhysicsEngine: Conservation of momentum in collision
...
==================================================
Tests passed: 40
Tests failed: 0
Total tests: 40
==================================================
```

## Project Structure

```
physics-sandbox/
├── index.html              # Main HTML file with UI
├── physics-engine.js       # Custom physics engine
├── renderer.js             # Canvas rendering system
├── app.js                  # Main application logic
├── physics-engine.test.js  # Comprehensive test suite
└── README.md              # This file
```

## Browser Compatibility

- Chrome/Edge: Full support
- Firefox: Full support
- Safari: Full support
- Mobile browsers: Full touch support

Requires HTML5 Canvas support (all modern browsers).

## Features Showcase

### 1. Realistic Physics
Objects fall with gravity (9.8 m/s²), bounce off walls and each other, and conserve momentum in collisions.

### 2. Beautiful Visuals
Every object has unique colors with gradients, glows, and shadows. Fast-moving objects leave colorful trails.

### 3. Interactive Fun
Drag objects to throw them, spawn rain of falling objects, toggle gravity to float objects, or pause to admire your creation.

### 4. Multiple Object Types
- **Circles**: Smooth rolling motion with accurate physics
- **Boxes**: Rotation on impact with angular velocity

### 5. Adjustable Parameters
- Object size (10-80 pixels)
- Restitution (bounciness)
- Friction (air resistance)
- Gravity toggle

## Performance Tips

For the smoothest experience:

1. **Optimal Object Count**: 20-50 objects for 60fps on most devices
2. **Maximum Objects**: Can handle 100+ on modern hardware
3. **Clear When Needed**: Use the "Clear All" button to reset
4. **Disable Trails**: Turn off trails for better performance with many objects

## Code Quality

- Clean, readable code with JSDoc comments
- Modular architecture with separation of concerns
- No external dependencies (pure JavaScript)
- Comprehensive test coverage
- Responsive design for all screen sizes

## Customization

Want to modify the simulation? Here are some fun tweaks:

### Change Gravity
```javascript
// In physics-engine.js, line ~165
this.gravity = new Vector2(0, 980); // Increase for stronger gravity
```

### Adjust Bounciness
```javascript
// In physics-engine.js, line ~45
this.restitution = 0.7; // 0 = no bounce, 1 = perfect bounce
```

### Modify Trail Length
```javascript
// In physics-engine.js, line ~48
this.maxTrailLength = 30; // Longer trails
```

### Change Colors
```javascript
// In physics-engine.js, generateColor() function
const hue = Math.floor(Math.random() * 360); // Full rainbow
```

## Known Limitations

- Very high object counts (200+) may impact performance
- Box rotation uses simplified physics (no full rigid body dynamics)
- Circle-Box collisions assume AABB (no rotated boxes)
- Mobile performance depends on device capabilities

## Future Enhancements

Potential features for future versions:
- [ ] Spatial partitioning for better collision detection
- [ ] Additional shapes (triangles, polygons)
- [ ] Springs and constraints
- [ ] Particle effects on collision
- [ ] Save/load scenes
- [ ] Adjustable gravity direction
- [ ] Object properties panel
- [ ] Slow motion mode
- [ ] Recording/playback

## License

This project is created for a coding challenge competition. Feel free to use, modify, and learn from the code!

## Author

Built with passion for physics simulation and beautiful visuals.

## Acknowledgments

- HTML5 Canvas API for rendering
- Physics inspiration from Box2D and Matter.js
- Color theory for HSL gradients

---

## Quick Start Guide

1. Open `index.html` in your browser
2. Click to spawn circles
3. Press `2` to switch to boxes
4. Drag objects to throw them
5. Press `R` for a rain of objects
6. Press `G` to toggle gravity and watch objects float
7. Have fun experimenting!

Enjoy the physics sandbox!
