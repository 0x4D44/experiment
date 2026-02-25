# Rhythm Runner - Game Development Competition Entry

A rhythm-based endless runner game where players must sync their actions to the beat of background music.

## Game Overview

Rhythm Runner is an arcade-style game that combines music, rhythm mechanics, and reflexes. Players control a character that automatically runs forward and must jump, slide, and move to avoid obstacles while maintaining perfect timing with the beat.

## Features

- **Rhythm-Based Gameplay**: Obstacles are generated in sync with the BPM (beats per minute)
- **Three Difficulty Levels**: Easy (100 BPM), Normal (120 BPM), Hard (140 BPM)
- **Timing System**:
  - Perfect: Hit within 50ms of the beat (+100 points)
  - Good: Hit within 100ms of the beat (+50 points)
  - Miss: Hit outside the timing window (no points, lose life)
- **Combo Multiplier**: Chain consecutive perfect/good hits to increase score multiplier
- **Procedural Obstacles**: Randomly generated obstacles that sync with rhythm
- **Lives System**: Start with 3 lives, lose one per collision
- **Score Tracking**: Real-time score, combo counter, and accuracy calculation

## How to Play

### Starting the Game
1. Open `public/index.html` in a web browser
2. Click on your desired difficulty level:
   - **Easy (100 BPM)**: Best for learning
   - **Normal (120 BPM)**: Standard difficulty
   - **Hard (140 BPM)**: Expert challenge

### Controls
- **Space**: Jump over obstacles
- **S**: Slide under obstacles
- **Arrow Keys**: Move left/right to navigate

### Objective
- Avoid obstacles for as long as possible
- Maintain high accuracy for better scores
- Build combos for increased point multiplier
- Survive all three lives

## Architecture

### Core Systems

#### AudioSystem
- Manages BPM tracking and beat synchronization
- Calculates timing accuracy based on beat position
- Supports dynamic BPM changes for difficulty levels

#### Player
- Controls character position and state
- Implements jump and slide mechanics
- Handles physics (gravity, collision bounds)
- Manages movement constraints

#### ObstacleGenerator
- Procedurally generates obstacles
- Synchronizes obstacle timing with music beat
- Adjusts gap sizes and speeds based on difficulty

#### ScoringSystem
- Tracks score, combo, and multiplier
- Calculates accuracy statistics
- Manages hit registration

#### Renderer
- Canvas-based 2D graphics rendering
- Real-time UI updates
- Visual feedback for timing hits

#### Game Engine
- Orchestrates all systems
- Manages game state (menu, playing, game over)
- Handles user input and collision detection
- Controls game loop

## Project Structure

```
/c/language/experiment/02/
├── public/
│   ├── index.html          # Main game HTML
│   └── game.js             # Bundled game code
├── src/
│   └── index.ts           # TypeScript source (compiles to game.js)
├── wrk_journals/
│   └── 2025.11.07 - JRN - Rhythm Runner Development.md
├── README.md               # This file
├── package.json            # NPM dependencies
├── tsconfig.json           # TypeScript configuration
└── webpack.config.js       # Webpack bundler config
```

## Running the Game

### Play the Game
```bash
# Open in browser
open public/index.html
# or
firefox public/index.html
```

### Development

#### Build TypeScript
```bash
npm run build
```

#### Watch Mode
```bash
npm run dev
```

#### Run Tests
```bash
npm test
```

## Technical Stack

- **Language**: TypeScript/JavaScript
- **Graphics**: Canvas 2D API
- **Build**: webpack, esbuild, TypeScript Compiler
- **Testing**: Jest
- **Package Manager**: npm

## Game Mechanics in Detail

### Timing System
The game uses a beat-based timing window:
- **Perfect Window**: ±50ms from beat center
- **Good Window**: ±100ms from beat center
- **Miss**: Outside timing windows

Beat position is calculated based on elapsed time and current BPM:
```
beatPosition = (elapsedTime % beatDuration) / beatDuration
```

### Score Calculation
```
basePoints = 100 (perfect) or 50 (good) or 0 (miss)
multiplier = 1 + floor(combo / 5) * 0.5
finalScore = floor(basePoints * multiplier * difficultyFactor)
```

### Obstacle Generation
- Obstacles are generated at random intervals (100-180ms gap for normal)
- Types: Blocks (jump) and Spikes (avoid)
- Speed increases with difficulty
- Position timing syncs with beat rhythm

## Testing

The game includes comprehensive unit tests for:
- Audio timing accuracy
- Player physics and collision
- Scoring calculations
- Obstacle generation

Run tests with:
```bash
npm test
```

## Performance

- Target: 60 FPS (16.67ms frame time)
- Optimized collision detection
- Efficient DOM updates (only UI changes)
- Canvas rendering optimizations

## Difficulties & Lessons Learned

### Challenge: Bash String Escaping
Working with complex TypeScript/JavaScript strings in bash heredocs proved problematic with backticks and quotes. Solution: Used direct Node.js file writing instead.

### Lesson: Pragmatic Development
When technical obstacles emerge, pragmatic solutions (using JavaScript directly vs forced TypeScript compilation) can be more effective than fighting the environment.

### Design Decision: Simplicity
Focused on core gameplay mechanics rather than elaborate visual effects to ensure stability and complete implementation.

## Future Enhancements

Potential improvements for future versions:
- Actual audio/music files instead of timer-based rhythm
- More obstacle types (moving obstacles, platforms)
- Power-ups and special effects
- Leaderboard system
- Multiple characters with different abilities
- Level-based progression
- Sound effects and music
- Mobile touch controls
- Accessibility improvements

## Credits

Rhythm Runner - Game Competition Entry
Created: 2025-11-07

## License

MIT License - Open for learning and modification

---

**Play Well!** Keep the beat, chain the combos, and reach for the high score!
