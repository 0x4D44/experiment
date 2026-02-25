# Pulse Navigator

**Audio Echolocation Rhythm Game**

Navigate through complete darkness using only sonar pulses and rhythm. Master timing precision to survive and find all objectives.

## Concept

In Pulse Navigator, you control a submarine in complete darkness. Your only way to perceive the world is by sending sonar pulses that echo off nearby objects. The catch? Your survival depends on maintaining a steady rhythm. Perfect timing rewards you with energy and extended sonar range, while poor timing drains your energy and limits your perception.

## Features

### High-Precision Timing Engine
- Sub-millisecond timing accuracy using `performance.now()`
- Adaptive tempo tracking with weighted moving averages
- Configurable timing windows (perfect: ±50ms, good: ±100ms, ok: ±150ms)
- Real-time rhythm consistency scoring

### Audio Feedback System
- **Sonar Pulses**: Frequency-swept chirps (1200Hz → 300Hz)
- **Distance-Based Echoes**: Delayed reflections using speed of sound simulation
- **Object Differentiation**:
  - High pitch (600Hz) = Objectives (your goal!)
  - Medium pitch (400Hz) = Obstacles (avoid!)
  - Low pitch (300Hz) = Walls (boundaries)
- **Stereo Panning**: Audio positioning indicates object direction
- **Feedback Sounds**: Ascending arpeggios for good rhythm, descending tones for poor timing

### Rhythm-Based Mechanics
- **Energy System**: Perfect rhythm gains energy, poor timing drains it
- **Dynamic Range**: Sonar range scales with energy (10-30 units)
- **Combo System**: Consecutive accurate beats increase score multipliers
- **Movement Cost**: Navigation consumes energy
- **Adaptive Difficulty**: Game tracks your natural tempo and adjusts expectations

### Game World
- 50x50 unit procedurally generated world
- 10 obstacles with collision detection
- 5 objectives to collect
- Boundary walls with distinct audio signature
- Real-time distance and angle calculations for all echoes

## Installation

### Requirements
- Node.js 18+ (for ES2022 support)
- npm or yarn

### Setup
```bash
# Install dependencies
npm install

# Build the project
npm run build

# Run the game
npm start

# Run tests
npm test

# Lint code
npm run lint
```

## Controls

| Key | Action |
|-----|--------|
| **SPACE** | Send sonar pulse (maintain rhythm!) |
| **W** / **↑** | Move up |
| **S** / **↓** | Move down |
| **A** / **←** | Move left |
| **D** / **→** | Move right |
| **P** | Pause/Resume |
| **Ctrl+C** | Quit |

## Gameplay

### Objective
Find all 5 objectives while maintaining your energy through rhythmic sonar pulses.

### Core Loop
1. **Press SPACE rhythmically** to send sonar pulses
2. **Listen to echoes** to identify nearby objects
3. **Navigate** towards high-pitched echoes (objectives)
4. **Avoid** medium-pitched echoes (obstacles)
5. **Maintain rhythm** to keep energy high and sonar range long

### Timing Mechanics

The game learns YOUR natural tempo:
- First beat: Always PERFECT (establishes baseline)
- Second beat: Sets initial tempo (always PERFECT)
- Subsequent beats: Judged against your established rhythm

**Accuracy Windows:**
- **PERFECT** (±50ms): +10 energy, full score, combo +1
- **GOOD** (±100ms): +5 energy, 75% score, combo +1
- **OK** (±150ms): No energy change, 50% score, combo maintained
- **MISS** (>150ms): -20 energy, no score, combo reset

**Tempo Adaptation:**
The game uses a weighted moving average (last 8 beats) to adapt to tempo changes. More recent beats have higher weight, allowing natural rhythm variations.

### Energy Management
- **Maximum Energy**: 100
- **Recharge Rate**: 5 energy/second (passive)
- **Movement Cost**: 2 energy per unit distance
- **Sonar Range Formula**: `10 + (20 × energyPercentage)`

### Scoring
- Base scores: Perfect=100, Good=75, OK=50, Miss=0
- **Combo Multiplier**: `1 + (combo × 0.1)`
- Example: 10-combo perfect hit = 100 × 2.0 = 200 points

### Win/Loss Conditions
- **Victory**: Collect all 5 objectives
- **Game Over**: Energy depletes to 0

### Performance Grades
- **S**: 50+ max combo, 5000+ score
- **A**: 30+ max combo, 3000+ score
- **B**: 20+ max combo, 2000+ score
- **C**: 10+ max combo, 1000+ score
- **D**: Below grade C requirements

## Technical Architecture

### Core Systems

#### 1. TimingEngine (`src/timing/TimingEngine.ts`)
High-precision rhythm tracking and beat detection.

**Key Features:**
- `performance.now()` for sub-ms precision
- Weighted tempo calculation (recent beats weighted higher)
- Real-time consistency scoring
- Average deviation tracking
- Complete timing statistics

**Timing Accuracy:**
```typescript
interface TimingWindow {
  perfect: number;  // ±50ms default
  good: number;     // ±100ms default
  ok: number;       // ±150ms default
}
```

#### 2. AudioEngine (`src/audio/AudioEngine.ts`)
Web Audio API synthesis for all game sounds.

**Sound Generation:**
- Oscillator-based synthesis (no audio files needed!)
- Frequency sweeps for sonar pulses
- Distance-based echo delays
- Inverse square law volume attenuation
- Stereo panning for spatial audio

**Echo Calculation:**
```typescript
delay = (distance × 10m) / 343m/s  // Speed of sound
volume = 0.3 / max(1, distance²)   // Inverse square law
```

#### 3. GameWorld (`src/game/GameWorld.ts`)
2D spatial navigation with collision detection.

**Features:**
- Procedural obstacle/objective generation
- Circle-based collision detection
- Distance and angle calculations
- Object detection within sonar range
- Safe spawn point validation

#### 4. RhythmScorer (`src/game/RhythmScorer.ts`)
Energy, combo, and score management.

**Responsibilities:**
- Process beat events into energy changes
- Track combo chains
- Calculate score multipliers
- Manage energy recharge/consumption
- Dynamic sonar range calculation

#### 5. GameController (`src/game/GameController.ts`)
Main game loop coordinator.

**Manages:**
- Game state machine (Menu/Playing/Paused/GameOver/Victory)
- Update loop (60 FPS)
- Input processing
- System coordination
- Win/loss detection

## Testing

Comprehensive test suite with 60 tests covering:

### Timing Engine Tests
- High-precision time tracking
- Beat registration and accuracy detection
- Tempo establishment and adaptation
- Consistency scoring algorithms
- Deviation calculations
- Statistics tracking
- Custom timing windows
- State reset functionality

### Rhythm Scorer Tests
- Energy gain/loss mechanics
- Combo system behavior
- Score calculation with multipliers
- Sonar range calculations
- Energy management (recharge/consumption)
- Performance grading
- State reset

### Game World Tests
- World initialization
- Player movement
- Collision detection (obstacles and walls)
- Object detection within range
- Echo calculations (distance, angle, type)
- Objective collection
- Position management
- Boundary enforcement

**Run tests:**
```bash
npm test
```

**Test Coverage:**
- All core timing mechanics validated
- Edge cases tested (max energy, zero energy, etc.)
- Real timing delays in tests (using busy-wait loops)
- Integration between systems

## Code Quality

### TypeScript Strict Mode
- All strict compiler flags enabled
- No implicit `any` types
- Explicit function return types
- Unused variables/parameters detected

### ESLint
- TypeScript ESLint rules enforced
- No unused variables
- Explicit type checking required
- Consistent code style

### Build Requirements
- Zero TypeScript errors
- Zero ESLint warnings
- All tests passing
- Clean compilation

**Verify build:**
```bash
npm run build  # Must complete with no errors
npm run lint   # Must show no warnings
npm test       # All 60 tests must pass
```

## Development

### Project Structure
```
src/
├── timing/
│   ├── TimingEngine.ts        # High-precision rhythm detection
│   └── TimingEngine.test.ts   # 22 timing tests
├── audio/
│   ├── AudioEngine.ts         # Web Audio synthesis
│   └── web-audio-api.d.ts     # Type definitions
├── game/
│   ├── GameWorld.ts           # Spatial navigation
│   ├── GameWorld.test.ts      # 21 world tests
│   ├── RhythmScorer.ts        # Energy & scoring
│   ├── RhythmScorer.test.ts   # 17 scorer tests
│   └── GameController.ts      # Main coordinator
└── index.ts                    # CLI interface
```

### Adding Features

**New Timing Window:**
```typescript
const customEngine = new TimingEngine({
  perfect: 30,  // Stricter perfect window
  good: 80,     // Tighter good window
  ok: 120       // Narrower ok window
});
```

**New Audio Feedback:**
```typescript
// In AudioEngine.ts
playCustomSound(): void {
  const oscillator = this.audioContext!.createOscillator();
  const gainNode = this.audioContext!.createGain();
  // Configure oscillator and envelope...
  oscillator.start(now);
  oscillator.stop(now + duration);
}
```

**New Game Mode:**
```typescript
// In GameController.ts
enum GameMode {
  NORMAL = 'NORMAL',
  HARD = 'HARD',     // Stricter timing windows
  ENDLESS = 'ENDLESS' // Infinite objectives
}
```

## Performance Characteristics

### Timing Precision
- **Resolution**: Sub-millisecond (performance.now())
- **Accuracy**: ±1-5ms typical deviation on modern hardware
- **Update Rate**: 60 Hz game loop
- **Latency Compensation**: Direct measurement, no prediction

### Audio Performance
- **Sample Rate**: 44.1kHz (Web Audio API default)
- **Latency**: ~10-50ms (system dependent)
- **Polyphony**: Unlimited (Web Audio handles mixing)
- **CPU Usage**: Low (oscillator-based synthesis)

### Memory Usage
- **Beat History**: Last 16 beats stored
- **Tempo History**: Last 8 intervals tracked
- **Object Count**: ~40 static objects (10 obstacles, 5 objectives, ~25 walls)
- **Total Memory**: <10MB typical

## Audio Design Philosophy

### Why Synthesized Audio?
- **Zero Latency**: No file loading/decoding delays
- **Procedural**: Infinitely variable without assets
- **Precise Timing**: Sample-accurate scheduling
- **Small Bundle**: No audio file overhead

### Sound Design Choices
- **Sonar Chirp**: Downward sweep mimics real sonar
- **Echo Timing**: Physics-based for spatial realism
- **Pitch Coding**: Intuitive high=target, low=danger
- **Stereo Panning**: Essential for blind navigation

## Design Decisions

### Why Rhythm-Based Navigation?
Traditional rhythm games test your ability to match a fixed pattern. Pulse Navigator inverts this: YOU set the rhythm, and consistency is rewarded. This creates a unique gameplay flow where player expression matters.

### Why Audio-Only?
By removing visual information, players must develop genuine echolocation skills. The stereo panning and distance-based delays create a true 3D audio space that's surprisingly navigable once mastered.

### Why Energy Mechanics?
Energy creates tension. Perfect rhythm is safety; poor rhythm is danger. This transforms timing from a score mechanic into a survival mechanic, raising stakes without adding complexity.

## Known Limitations

### Platform Compatibility
- **Node.js Only**: Uses `web-audio-api` npm package for Node
- **Terminal Required**: No browser/GUI version
- **Windows/Mac/Linux**: Cross-platform Node support

### Audio Library
- The `web-audio-api` npm package has some rough edges
- Limited spatial audio (stereo panning only, no HRTF)
- No audio file playback (synthesis only)

### Gameplay
- Fixed world size (50×50)
- No difficulty progression
- Single-player only
- No persistent save system

## Future Enhancements

### Potential Features
- Multiple difficulty levels with adjusted timing windows
- Procedurally generated worlds
- Multiplayer cooperative mode
- Achievement system
- Persistent high scores
- Additional game modes (time trial, survival, zen)
- Visual mode for accessibility
- MIDI controller support
- Custom audio themes

### Technical Improvements
- WebGL visualization mode
- Browser version with Web Audio API
- Mobile support with touch controls
- Replay system
- Telemetry for timing analysis
- Adaptive difficulty based on performance

## Credits

**Game Design & Implementation**: Claude (Anthropic)
**Audio Engine**: Web Audio API
**Timing System**: performance.now() API
**Challenge**: Anthropic Rhythm & Timing Game Coding Challenge

## License

MIT License - Feel free to use, modify, and distribute!

---

## Quick Start Summary

```bash
# Install and run
npm install && npm run build && npm start

# Test timing precision
npm test

# Start playing
# 1. Press SPACE to establish your rhythm
# 2. Listen for high-pitched echoes (objectives)
# 3. Use WASD to navigate
# 4. Maintain your rhythm to keep energy high
# 5. Collect all 5 objectives to win!
```

**Pro Tips:**
- Start slow! The game adapts to YOUR tempo
- Listen for stereo panning to determine direction
- Higher energy = better sonar range
- Consistency matters more than speed
- Let echoes finish before moving

**Have fun navigating in the dark!** 🎵🔊🎮
