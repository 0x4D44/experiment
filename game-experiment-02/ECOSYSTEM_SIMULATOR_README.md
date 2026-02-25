# Ecosystem Balance Simulator

A nature simulation game where players maintain ecological balance by managing predator-prey relationships, population dynamics, and environmental factors.

## Game Overview

In this educational game, you'll experience the delicate balance of nature. Four species interact in a complex ecosystem:

- **Grass** (Plants) - Primary producer, grows naturally
- **Rabbits** (Herbivores) - Eat grass, prey for wolves and birds
- **Wolves** (Carnivores) - Eat rabbits, apex predator
- **Birds** (Omnivores) - Eat grass and small animals

## Win Condition

Maintain all species populations within healthy balance ranges for 50 consecutive cycles:

- Grass: 100 - 1000
- Rabbits: 10 - 200
- Wolves: 2 - 30
- Birds: 5 - 100

## Loss Condition

The game ends if any species goes completely extinct (population reaches 0).

## Game Mechanics

### Population Dynamics

Each species follows realistic ecological principles:

1. **Grass Growth**: Grows automatically based on water availability
   - Growth rate: 5% per cycle
   - Reduced during drought, enhanced in abundant weather
   - Consumed by rabbits and birds

2. **Rabbit Population**: Herbivores that eat grass
   - Energy gain: 10 per grass consumed
   - Energy cost: 1 per cycle
   - Reproduction threshold: 15+ energy
   - Reproduction rate: 30% chance when threshold met
   - Max age: 50 cycles
   - Prey for wolves and birds

3. **Wolf Population**: Carnivores that hunt rabbits
   - Energy gain: 30 per rabbit hunted
   - Energy cost: 2 per cycle
   - Reproduction threshold: 40+ energy
   - Reproduction rate: 20% chance when threshold met
   - Max age: 80 cycles
   - Success rate: 70% hunt success
   - Top predator

4. **Bird Population**: Omnivores that hunt and forage
   - Energy gain: 5 from grass, 7.5 from rabbits
   - Energy cost: 0.8 per cycle
   - Reproduction threshold: 12+ energy
   - Reproduction rate: 35% chance when threshold met
   - Max age: 40 cycles
   - 50% chance to eat grass vs hunt rabbits

### Environmental System

**Weather Events** affect water availability:

- **Normal**: Water level increases by 1% per cycle
- **Drought**: Water level decreases by 5% per cycle
- **Abundant**: Water level increases by 5% per cycle
- **Storm**: Water level decreases by 10% per cycle

**Water Level** affects all species:
- Below 30%: Increased death rate (30% stress)
- Below 50%: Moderate stress (10% death)
- 50-100%: Normal conditions

Weather changes randomly (5% chance per cycle).

## Controls

### Game Controls
- **Start Game**: Begin simulation
- **Pause**: Pause the simulation
- **Resume**: Continue from pause
- **Reset**: Restart the game

### Weather Control
Click weather buttons to change environmental conditions:
- ☀️ **Normal**: Balanced weather
- 🔥 **Drought**: Water scarcity
- 🌧️ **Abundant**: Wet conditions
- ⛈️ **Storm**: Extreme weather

### Species Management
Adjust populations manually:
- Enter quantity in text field
- Click **+** to add individuals
- Click **-** to remove individuals

## How to Play

### Quick Start
1. Click **Start Game** to begin the simulation
2. Watch the population trends on the graph
3. Observe how species interact

### Achieving Balance
1. Monitor population counts on the left panel
2. Check the "Balance Progress" indicator (need 50 consecutive cycles in balance)
3. Use weather controls to manage conditions
4. Add/remove species to prevent extinctions
5. Keep all species within their healthy ranges

### Strategy Tips
- **Prevent grass extinction**: Ensure rabbits don't eat all grass - add more grass if needed
- **Control rabbit population**: Use wolves to regulate rabbit numbers
- **Manage water**: Use weather to help struggling species
- **Early intervention**: Don't wait until a species is critical
- **Balance predators**: Keep wolf-to-rabbit ratio healthy
- **Observe trends**: Use the graph to predict population changes

## Technical Details

### Architecture

The game consists of three components:

1. **Core Engine** (`ecosystem-simulator.ts`)
   - Species and individual management
   - Population dynamics simulation
   - Environmental effects
   - Game state management

2. **UI** (`ecosystem-simulator.html`)
   - Interactive controls
   - Real-time population display
   - Population trend graphing
   - Responsive design

3. **Tests** (`ecosystem-simulator.test.ts`)
   - Comprehensive test coverage
   - Species interaction validation
   - Environmental system testing
   - Win/loss condition testing

### Technology Stack
- **Language**: TypeScript
- **Rendering**: HTML5 Canvas
- **Testing**: Jest
- **Platform**: Web-based (browser)

### File Structure
```
ecosystem-simulator.ts       - Core game engine
ecosystem-simulator.html     - Interactive UI
ecosystem-simulator.test.ts  - Test suite
ECOSYSTEM_SIMULATOR_README.md - This file
```

## Installation & Running

### Play the Game
1. Open `ecosystem-simulator.html` in a web browser
2. Click "Start Game" to begin
3. Manage populations and weather to achieve balance

### Run Tests
```bash
npm install --save-dev jest @types/jest ts-jest typescript
npx jest ecosystem-simulator.test.ts
```

## Test Coverage

The test suite includes:

- **Initialization Tests**: Verify starting state
- **Game Control Tests**: Start, pause, resume, reset
- **Population Management**: Add/remove species
- **Simulation Cycle Tests**: Core game loop
- **Weather System**: Weather and water level mechanics
- **Loss Conditions**: Species extinction detection
- **Win Conditions**: Balance progress tracking
- **Population Dynamics**: Species interactions
- **Integration Tests**: Multi-cycle scenarios

Run tests with: `npm test` or `npx jest ecosystem-simulator.test.ts`

## Educational Value

This game teaches:
- **Predator-Prey Relationships**: How predators and prey regulate populations
- **Energy Flow**: Energy gain from eating, energy cost of living
- **Reproduction Mechanics**: How energy relates to breeding
- **Environmental Impact**: How weather affects populations
- **Ecosystem Balance**: The delicate equilibrium needed for stability
- **Systems Thinking**: How small changes cascade through systems

## Gameplay Example

### Scenario: Rabbit Explosion
1. Initial state: 50 rabbits, 500 grass
2. Rabbits eat grass → grass population drops
3. As grass increases, rabbit population increases
4. Wolves hunt rabbits → rabbit population drops
5. With fewer rabbits, wolves starve → wolf population drops
6. Grass recovers with fewer rabbits eating it
7. Cycle repeats in oscillating pattern

**Player Action**: Monitor and adjust populations to prevent extremes.

## Scoring & Progress

- **Balance Cycles**: Consecutive cycles with all species in healthy ranges
- **Balance Progress**: Visual bar showing progress toward 50-cycle win
- **Population Display**: Real-time species counts
- **Water Level**: Environmental stress indicator
- **Weather**: Current environmental conditions
- **Cycle Counter**: Game progression indicator

## Known Mechanics

### Predation
- Wolves hunt rabbits with 70% success rate
- Birds hunt rabbits with 50% success rate
- Hunting consumes time and effort (energy cost)

### Reproduction
- Species reproduce when energy exceeds threshold
- Offspring inherit half of parent's reproduction energy
- Reproduction rate varies by species

### Aging & Death
- All individuals age each cycle
- Death from starvation (energy <= 0) or old age (exceed max age)
- Removed individuals free resources for others

### Starvation Cascade
- When grass is depleted, rabbits starve
- As rabbits starve, wolves starve
- As predators starve, prey populations recover
- Natural cycle of predator-prey dynamics

## Bugs & Limitations

None known. The game has been thoroughly tested for:
- Population consistency (no negative counts)
- Balance detection accuracy
- Win/loss condition detection
- Environmental system functionality
- Save/load of game state

## Future Enhancement Ideas

- Seasonal cycles with predictable weather patterns
- Mutant species with different traits
- Disease outbreaks affecting populations
- Player achievements and leaderboards
- Difficulty levels (easy, normal, hard)
- Multiple ecosystem maps
- Science-based educational mode
- Sound effects and animations

## Credits

Developed for the Game Development Competition Round 3.

**Game Design**: Ecosystem simulation with predator-prey dynamics
**Programming**: TypeScript, HTML5 Canvas
**Testing**: Jest test framework
**Educational Focus**: Ecology and systems dynamics

## License

This game is provided as-is for educational and competition purposes.

---

**Enjoy the simulation and maintain the balance of nature!** 🌍
