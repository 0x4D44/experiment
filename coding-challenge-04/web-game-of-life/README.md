# Conway's Game of Life - Interactive Simulator

An educational, visually stunning, and feature-rich implementation of Conway's Game of Life cellular automaton. Built for a coding challenge competition with smooth 60 FPS rendering, interactive controls, and a beautiful modern UI.

![Game of Life](https://img.shields.io/badge/Status-Complete-brightgreen) ![License](https://img.shields.io/badge/License-MIT-blue)

## Features

### Core Functionality
- **Efficient Simulation Engine**: Optimized algorithm that only checks active cells
- **Smooth 60 FPS Rendering**: High-performance Canvas rendering with requestAnimationFrame
- **Toroidal Topology**: Wrap-around edges for infinite-like grid behavior
- **Double Buffering**: Prevents flickering and ensures accurate simulation

### Interactive Controls
- **Click & Drag Drawing**: Click or drag to create patterns
- **Draw/Erase Modes**: Toggle between drawing and erasing cells
- **Play/Pause/Step**: Full control over simulation playback
- **Speed Control**: Adjustable simulation speed (1-60 generations/second)
- **Clear & Randomize**: Quick grid manipulation tools

### Visual Features
- **Age-Based Coloring**: Cells change color based on how long they've been alive
  - Cyan → Green → Yellow → Orange → Red → Purple gradient
  - Visual feedback showing pattern evolution
- **Smooth Animations**: Beautiful color transitions and effects
- **Grid Lines**: Optional grid overlay for precise editing
- **Zoom Controls**: Zoom in/out to view details or big picture
- **Real-time Statistics**: Generation counter, population count, and FPS display

### Pattern Library
Pre-loaded with 15+ famous Game of Life patterns:

#### Oscillators (Repeating Patterns)
- **Blinker** (Period 2): The simplest oscillator
- **Toad** (Period 2): Compact period-2 oscillator
- **Beacon** (Period 2): Four-cell oscillator
- **Pulsar** (Period 3): Beautiful symmetric oscillator
- **Pentadecathlon** (Period 15): Long-period oscillator

#### Spaceships (Moving Patterns)
- **Glider**: The smallest spaceship, moves diagonally
- **LWSS** (Lightweight Spaceship): Moves horizontally
- **MWSS** (Middleweight Spaceship): Larger horizontal spaceship
- **HWSS** (Heavyweight Spaceship): Largest standard spaceship

#### Still Lifes (Stable Patterns)
- **Block**: Simple 2×2 square
- **Beehive**: Hexagonal still life
- **Loaf**: Asymmetric stable pattern
- **Boat**: Small stable configuration

#### Guns (Pattern Generators)
- **Gosper Glider Gun**: Shoots gliders every 30 generations
- **Simkin Glider Gun**: Smaller gun with period 120

### Grid Options
- **30×30**: Small, fast simulation
- **50×50**: Default balanced size
- **75×75**: Large detailed grid
- **100×100**: Maximum complexity

### Keyboard Shortcuts
- **Space**: Play/Pause simulation
- **Enter**: Step one generation
- **C**: Clear grid
- **R**: Randomize grid

## Conway's Game of Life Rules

The Game of Life is a cellular automaton devised by mathematician John Conway in 1970. Despite simple rules, it produces incredibly complex and beautiful patterns.

### The Rules
1. **Birth**: A dead cell with exactly 3 live neighbors becomes alive
2. **Survival**: A live cell with 2 or 3 live neighbors stays alive
3. **Death**: All other cells die (from underpopulation or overcrowding)

### Why It's Fascinating
- **Turing Complete**: Can simulate any computational process
- **Emergent Complexity**: Simple rules create complex behaviors
- **Self-Organization**: Patterns organize themselves without external input
- **Mathematical Beauty**: Demonstrates cellular automaton principles

## Project Structure

```
web-game-of-life/
├── index.html              # Main application page
├── css/
│   └── styles.css          # Modern dark theme styling
├── js/
│   ├── game-of-life.js     # Core Game of Life logic
│   ├── patterns.js         # Pattern library definitions
│   ├── renderer.js         # Canvas rendering engine
│   └── app.js              # Main application controller
├── tests/
│   ├── index.html          # Test runner page
│   └── game-of-life.test.js # Comprehensive test suite
└── README.md               # This file
```

## Technical Implementation

### Game of Life Engine
- **Double Buffering**: Uses two grids to prevent conflicts during updates
- **Active Cell Optimization**: Only processes cells that could change state
- **Age Tracking**: Separate grid tracks how long cells have been alive
- **Efficient Neighbor Counting**: O(1) neighbor lookup with boundary checks

### Rendering System
- **Canvas API**: Hardware-accelerated 2D rendering
- **Color Gradients**: Dynamic age-based coloring system
- **Responsive Design**: Adapts to container size
- **Grid Centering**: Automatically centers grid in viewport
- **Glow Effects**: Subtle shadows for visual appeal

### Performance Optimizations
- **Active Cell Set**: Only checks cells near live cells
- **RequestAnimationFrame**: Syncs with display refresh rate
- **Efficient Updates**: Minimal DOM manipulation
- **Smart Redraws**: Only redraws when necessary

## How to Use

### Running the Application
1. Open `index.html` in a modern web browser
2. No build process or dependencies required - pure vanilla JavaScript!
3. For local development, use a simple HTTP server:
   ```bash
   python -m http.server 8000
   # or
   npx serve
   ```
4. Navigate to `http://localhost:8000`

### Running Tests
1. Open `tests/index.html` in a web browser
2. Tests run automatically and display results
3. 20 comprehensive tests covering all rules and edge cases
4. Console output shows detailed test results

### Creating Patterns

#### Manual Drawing
1. Click individual cells to toggle them on/off
2. Click and drag to draw continuous patterns
3. Use Draw mode to create cells
4. Use Erase mode to remove cells

#### Using Pattern Library
1. Click any pattern button to load it
2. Pattern appears at grid center
3. Patterns can be combined and modified
4. Experiment to create hybrid patterns!

#### Tips for Experimentation
- **Start with oscillators**: See how patterns repeat
- **Try spaceships**: Watch them travel across the grid
- **Combine patterns**: Place multiple patterns to interact
- **Use randomize**: Discover emergent patterns
- **Adjust speed**: Slow down to study individual generations

## Pattern Descriptions

### Glider
The most iconic pattern in Game of Life. This 5-cell pattern moves diagonally across the grid, traveling one cell every 4 generations. It's the smallest spaceship and fundamental to many complex constructions.

### Gosper Glider Gun
Discovered by Bill Gosper in 1970, this was the first known pattern that grows indefinitely. It produces a new glider every 30 generations, proving that Game of Life patterns can grow without bound.

### Pulsar
A beautiful period-3 oscillator with 48 cells. It's one of the most symmetric patterns and demonstrates how complex periodic behavior emerges from simple rules.

### Pentadecathlon
A period-15 oscillator that's common in random starting configurations. Its long period makes it interesting to watch evolve.

### Blinker
The simplest oscillator with just 3 cells. It alternates between horizontal and vertical orientations every generation, making it perfect for understanding basic oscillation.

## Educational Value

This simulator is perfect for:
- **Computer Science Education**: Demonstrates algorithms, data structures, and optimization
- **Mathematics**: Explores cellular automata, emergence, and complexity theory
- **Biology**: Models population dynamics and self-organizing systems
- **Art & Design**: Creates mesmerizing patterns and animations
- **Philosophy**: Raises questions about life, complexity, and computation

## Browser Compatibility

Works on all modern browsers:
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Opera 76+

Requires JavaScript enabled and HTML5 Canvas support.

## Performance Notes

- **60 FPS** rendering on modern hardware
- **100×100 grid** handles ~1000 active cells smoothly
- **Optimized algorithm** scales well with grid size
- **Responsive** remains smooth even during complex simulations

## Future Enhancements

Potential additions for future versions:
- Save/load custom patterns
- Export patterns to RLE format
- Share patterns via URL
- More pattern categories (gardens, fuses, etc.)
- Pattern editor with symmetry tools
- Time-lapse recording
- Population graphs over time
- Multi-state cellular automata

## Credits

- **John Conway** (1937-2020): Creator of the Game of Life
- **Bill Gosper**: Discoverer of the Glider Gun
- **LifeWiki**: Pattern references and documentation
- **Cellular Automaton Community**: Decades of pattern discovery

## License

MIT License - Free to use, modify, and distribute.

## Learn More

- [LifeWiki](https://conwaylife.com/wiki/): Comprehensive pattern database
- [John Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life): Wikipedia article
- [The Game of Life](https://www.youtube.com/watch?v=R9Plq-D1gEk): Numberphile video explanation

---

**Built with ❤️ for the coding challenge competition**

Enjoy exploring the fascinating world of cellular automata! 🧬
