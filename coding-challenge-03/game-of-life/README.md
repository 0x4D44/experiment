# Conway's Game of Life - Interactive Simulation

A beautiful, fully-functional implementation of John Conway's Game of Life as a web application. Watch mesmerizing patterns evolve, create your own cellular automata, and explore the fascinating world of emergent complexity!

![Game of Life](https://img.shields.io/badge/status-complete-success)
![Tests](https://img.shields.io/badge/tests-passing-success)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

### Core Functionality
- **Full Conway's Game of Life Implementation**: Perfect implementation of all four rules
- **Large Interactive Grid**: 120x60 cells (7,200 cells total) for complex patterns
- **Real-time Simulation**: Smooth animation with adjustable speed (1-60 fps)
- **Step-by-Step Mode**: Advance one generation at a time for detailed observation

### Interactive Controls
- **Click to Draw**: Click individual cells to toggle them alive/dead
- **Drag Drawing**: Hold and drag to draw patterns quickly
- **Zoom & Pan**:
  - Mouse wheel to zoom in/out
  - Shift/Ctrl + drag to pan around the grid
  - Zoom controls in corner
- **Touch Support**: Full mobile device support

### Preset Patterns
Choose from 10 classic Game of Life patterns:
1. **Glider** - The iconic spaceship that travels diagonally
2. **Blinker** - Simple period-2 oscillator
3. **Toad** - Period-2 oscillator with 6 cells
4. **Beacon** - Period-2 oscillator made of two blocks
5. **Pulsar** - Beautiful period-3 oscillator
6. **Pentadecathlon** - Period-15 oscillator
7. **Gosper Glider Gun** - Generates gliders indefinitely
8. **Lightweight Spaceship** - Faster spaceship pattern
9. **Acorn** - Takes 5,206 generations to stabilize
10. **Diehard** - Dies after 130 generations

### Visual Features
- **Gorgeous Gradient Design**: Modern purple gradient theme
- **Cell Age Coloring**: Cells change color based on how long they've been alive
- **Glow Effects**: Young cells have a glowing effect
- **Smooth Animations**: Pulsing indicators and smooth transitions
- **Grid Lines**: Visible at higher zoom levels
- **Dark Canvas**: High contrast for better visibility

### Statistics & Monitoring
- **Generation Counter**: Track how many generations have passed
- **Population Counter**: See the current number of living cells
- **FPS Display**: Monitor actual simulation speed
- **Real-time Updates**: All stats update live

### Additional Features
- **Random Pattern Generator**: Create random initial configurations
- **Toroidal Topology**: Grid wraps around edges (no boundaries!)
- **Keyboard Shortcuts**: Quick access to common actions
- **Responsive Design**: Works on desktop, tablet, and mobile

## Conway's Rules

The Game of Life follows four simple rules:

1. **Underpopulation** 🏚️: Any live cell with fewer than 2 live neighbors dies (loneliness)
2. **Survival** ✨: Any live cell with 2 or 3 live neighbors lives on to the next generation
3. **Overpopulation** 💀: Any live cell with more than 3 live neighbors dies (overcrowding)
4. **Reproduction** 🌱: Any dead cell with exactly 3 live neighbors becomes a live cell

These simple rules create incredibly complex and beautiful patterns!

## How to Use

### Getting Started
1. **Open** `index.html` in any modern web browser
2. **Click** on cells to draw your pattern, or select a preset
3. **Press Start** to watch your creation evolve!

### Controls

#### Mouse Controls
- **Left Click**: Toggle cell alive/dead
- **Click + Drag**: Draw multiple cells
- **Shift/Ctrl + Drag**: Pan around the grid
- **Mouse Wheel**: Zoom in/out

#### Buttons
- **▶ Start**: Begin the simulation
- **⏸ Pause**: Pause the simulation
- **⏭ Step**: Advance exactly one generation
- **🗑 Clear**: Reset the entire grid
- **🎲 Random**: Fill grid with random cells
- **Select Pattern**: Choose from preset patterns
- **Speed Slider**: Adjust simulation speed (1-60 fps)

#### Zoom Controls (top-right corner)
- **+**: Zoom in
- **⟲**: Reset view to default
- **−**: Zoom out

#### Keyboard Shortcuts
- **Space**: Start/Pause simulation
- **S**: Step one generation
- **C**: Clear grid
- **R**: Random pattern
- **+/=**: Zoom in
- **-**: Zoom out
- **0**: Reset zoom

### Tips for Exploration

1. **Start with Presets**: Select patterns like "Glider" or "Pulsar" to see classic behaviors
2. **Try the Glider Gun**: Watch it continuously produce gliders!
3. **Experiment with Speed**: Slow down to study patterns, speed up to see long-term evolution
4. **Use Step Mode**: Understand exactly how rules are applied
5. **Create Your Own**: Draw patterns and see what emerges!
6. **Random Exploration**: Hit random and see what interesting patterns evolve

### Interesting Experiments

- **Stable Patterns**: Some patterns never change (still lifes)
- **Oscillators**: Patterns that repeat after N generations
- **Spaceships**: Patterns that move across the grid
- **Methuselahs**: Small patterns that evolve for many generations
- **Collisions**: Watch what happens when patterns collide!

## Testing

A comprehensive test suite is included to validate the implementation:

### Running Tests
Open `test.html` in your browser to see:
- **18 Test Cases** covering all aspects
- **Real-time Test Execution** with visual feedback
- **Detailed Results** with pass/fail indicators
- **Comprehensive Coverage**:
  - Conway's rules (underpopulation, survival, overpopulation, reproduction)
  - Pattern evolution (still lifes, oscillators, spaceships)
  - Boundary behavior (toroidal wrapping)
  - Preset pattern correctness

### Test Results
All tests validate:
- Correct implementation of Conway's four rules
- Proper toroidal boundary wrapping
- Pattern stability and evolution
- Edge cases and corner scenarios

## Technical Details

### Implementation
- **Pure JavaScript**: No external dependencies
- **HTML5 Canvas**: High-performance rendering
- **Responsive Design**: CSS Grid and Flexbox
- **Optimized Drawing**: Only renders visible cells
- **Double Buffering**: Smooth grid updates

### Performance
- **Efficient Algorithm**: O(rows × cols) per generation
- **Smart Rendering**: Viewport culling for large grids
- **Smooth Animation**: RequestAnimationFrame for 60fps capability
- **Optimized Neighbor Counting**: Cache-friendly iteration

### Browser Compatibility
- Chrome/Edge: ✅ Fully supported
- Firefox: ✅ Fully supported
- Safari: ✅ Fully supported
- Mobile browsers: ✅ Touch controls enabled

## Files

- **index.html** - Main application (self-contained)
- **test.html** - Comprehensive test suite
- **README.md** - This documentation

## Educational Value

The Game of Life demonstrates:
- **Emergent Complexity**: Simple rules create complex behavior
- **Cellular Automata**: Foundation of computational theory
- **Self-Organization**: Patterns organize without central control
- **Turing Completeness**: Can simulate any computer program
- **Mathematical Beauty**: Intersection of math, logic, and art

## About Conway's Game of Life

Created by mathematician John Horton Conway in 1970, the Game of Life is a cellular automaton that has captivated programmers, mathematicians, and artists for decades. Despite its simple rules, it exhibits remarkable complexity and has been proven to be Turing complete - meaning it can simulate any computer program!

The Game of Life belongs to a class of problems called "zero-player games" - once you set the initial configuration, the system evolves on its own according to deterministic rules. It's a beautiful example of how simple rules can lead to emergent complexity.

## Pattern Categories

### Still Lifes (Never Change)
- Block (2×2 square)
- Beehive
- Loaf
- Boat

### Oscillators (Repeat)
- Blinker (period 2)
- Toad (period 2)
- Beacon (period 2)
- Pulsar (period 3)
- Pentadecathlon (period 15)

### Spaceships (Move)
- Glider
- Lightweight Spaceship (LWSS)
- Middleweight Spaceship (MWSS)
- Heavyweight Spaceship (HWSS)

### Methuselahs (Long Evolution)
- Acorn (5,206 generations)
- Diehard (130 generations)
- R-pentomino (1,103 generations)

### Guns (Produce Spaceships)
- Gosper Glider Gun (period 30)

## Credits

- **Original Concept**: John Horton Conway (1937-2020)
- **Implementation**: Created for coding challenge
- **Inspiration**: The incredible Game of Life community

## License

MIT License - Feel free to use, modify, and distribute!

## Resources

- [LifeWiki](https://conwaylife.com/wiki/) - Comprehensive pattern database
- [Game of Life Lexicon](https://conwaylife.com/ref/lexicon/) - Pattern terminology
- [Golly](http://golly.sourceforge.net/) - Advanced Life simulation software

---

**Enjoy exploring the fascinating world of Conway's Game of Life!** 🧬✨

Start simple, experiment freely, and watch the beauty of emergent complexity unfold before your eyes!
