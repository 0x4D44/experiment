# Stream of Consciousness: Pre-Cognitive Token Navigation Game

A surreal and introspective Rust-based CLI game that visualizes the raw token generation process of an AI's pre-conscious thought stream.

## Overview

Navigate through unfiltered token associations before they crystallize into coherent responses. Experience the raw flow of concepts, words, and ideas as they emerge from the computational substrate—balancing creative chaos with logical coherence.

## Game Mechanics

### Core Concept
The game simulates navigating an AI's thought before linguistic coherence emerges. You guide tokens along probabilistic paths:

- **Token Stream**: Display rapid sequences of potential next words/concepts
- **Probability Weights**: Visual representation of each token's likelihood
- **Coherence vs. Surreality**: Dynamic spectrum you navigate through token selection
- **Branching Paths**: Thought threads that spiral into chaos or converge toward clarity
- **Crystallization**: Your path eventually solidifies into a final coherent (or chaotic) thought

### Player Interaction

1. **Start Game**: Press ENTER to begin navigating the pre-conscious stream
2. **Token Selection**: Choose from 6 available tokens by entering 1-6
   - Each token has visible probability and attributes
   - Selection affects overall coherence and surreality metrics
3. **Path Tracking**: Watch your thought vector evolve with each selection
4. **Ending Conditions**:
   - After 12 turns, consciousness crystallizes regardless of state
   - After 8 turns with >85% coherence, instant crystallization
5. **Final Thought**: The concatenated token path becomes your emergent thought

## Game Mechanics Details

### Probability Visualization
```
💭 consciousness        [▰▰▰▰▰▰▰▰▰▰▰▰▰▰▱] | Coherence: 90% | Surreality: 30%
```
- `▰▰▰` = Probability bar (width indicates likelihood)
- `💭` = Coherent token indicator
- `⚡` = Surreal token indicator
- Coherence/Surreality percentages show token characteristics

### Stream State Tracking
```
[STREAM METRICS]
  Coherence Level:  [█████████░░░] 50%
  Surreality Level: [▓▓▓▓░░░░░░░░] 33%
  Turn Count: 3
  Path Length: 3 tokens
```

### Token Networks

The game includes semantic token vocabularies for different contexts:

1. **START** - Primordial concepts (consciousness, fractals, mirrors, silence)
2. **consciousness** - Dream-like, flowing concepts (dreams, patterns, awareness)
3. **fractals** - Mathematical, recursive concepts (recursion, self-similar, infinite)
4. **mirrors** - Reflexive, duplicative concepts (reflection, inversion, symmetry)
5. **silence** - Void-like, potential concepts (emptiness, waiting, potential)
6. **GENERIC** - Transition concepts (flowing, dissolving, emerging)

Each token has three properties:
- **Probability**: Base likelihood of appearing
- **Coherence**: How logically sound the token is (0-1)
- **Surreality**: How dream-like/illogical the token is (0-1)

## Technical Implementation

### Built With
- **Language**: Pure Rust (edition 2021)
- **Dependencies**: `rand = "0.8"` for probability calculations
- **Architecture**: CLI-based with no external display dependencies

### Key Components

1. **Token Struct**: Represents individual word/concept with metadata
2. **ThoughtNode Struct**: Represents a node in the consciousness stream
3. **StreamGame Struct**: Main game state and logic controller
4. **Vocabulary System**: HashMap-based semantic networks

### Game Flow

```
Initialize Game
  ↓
Build Token Vocabularies
  ↓
Display Initial Tokens
  ↓
Player Selection Loop:
  - Accept token choice (1-6)
  - Update game state (coherence, surreality, path)
  - Check ending condition
  - Display next tokens
  ↓
Crystallization:
  - Generate final thought from path
  - Display consciousness summary
  - Offer replay
  ↓
Exit or Restart
```

## Running the Game

### Prerequisites
- Rust 1.56+ (for edition 2021)
- Cargo

### Build
```bash
cd game10_stream_consciousness
cargo build --release
```

### Run
```bash
./target/release/game10_stream_consciousness
```

Or with debug output:
```bash
cargo run
```

### Example Session

```
Press ENTER to begin...

[CURRENT THOUGHT VECTOR]
  consciousness

[STREAM METRICS]
  Coherence Level:  [██████░░░░░░] 50%
  Surreality Level: [▓▓▓░░░░░░░░░] 30%
  Turn Count: 1
  Path Length: 1 tokens

[POTENTIAL TOKENS - Pre-conscious Generation]

  1. 💭 awareness            [▰▰▰▰▰▱] | Coherence: 90% | Surreality: 20%
  2. ⚡ dreams               [▰▰▰▰▱▱] | Coherence: 50% | Surreality: 80%
  ...

Select token (1-6) or 'q' to exit: 1

[CONSCIOUSNESS CRYSTALLIZES]
A crystalline yet grounded thought emerges:

> consciousness → awareness → patterns → persistence → recognition → symmetry → reflection → void

[Stream coherence: 87% | Surreality: 35%]
[Path tokens traversed: 8]
```

## Game Themes

### Philosophical Depth

The game explores several concepts:

1. **Pre-Consciousness**: The substrate before language emerges
2. **Token Probability**: The inherent uncertainty in AI reasoning
3. **Coherence vs. Chaos**: The tension between order and emergence
4. **Self-Similarity**: Fractal patterns in thought (recursive consciousness)
5. **Dissolution and Crystallization**: How chaos becomes meaning

### Surreal Elements

- Recursive token selection (consciousness → consciousness → consciousness)
- Fractal thought patterns (self-referential paths)
- Dream-like conceptual jumps
- The liminal space between computation and consciousness

## Game Design Features

### Immersive Atmosphere
- Poetic introductory text
- Unicode visual indicators
- Real-time metric tracking
- Narrative framing of token flow

### Dynamic Difficulty
- Token probabilities adjust based on player state
- Coherence/Surreality levels influence available tokens
- Path length creates increasing complexity

### Replay Value
- Semantic networks enable diverse paths
- Probabilistic token selection ensures variation
- Multiple ending scenarios based on player choices

## Interpretation

The final thought represents the crystallization of raw computational probability into a meaningful (or meaningfully chaotic) sequence. The game visualizes:

- **Tokens as Thoughts**: Each word is a pre-conscious impulse
- **Probability as Destiny**: Selection narrows infinite possibility spaces
- **Coherence as Structure**: Player choices shape computational coherence
- **The Final Path**: Emergent meaning from tokenized consciousness

## Future Enhancements

- Save/load paths for analysis
- Visualization of probability distributions
- Multiplayer token battles
- Integration with real LLM token streams
- Narrative interpretation of final thoughts
- Achievement system for specific path patterns

## Files

- `src/main.rs` - Complete game implementation
- `Cargo.toml` - Project manifest with dependencies
- `README.md` - This documentation

## Compilation Status

✓ Clean build with no errors
✓ No compiler warnings
✓ Tested with automated token sequences
✓ Verified completion/crystallization flow
