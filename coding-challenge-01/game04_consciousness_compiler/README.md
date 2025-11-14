# Consciousness Compiler

A pure Rust CLI-based game that explores how consciousness might emerge from simple logical operations.

## Overview

The Consciousness Compiler is an interactive game where you build a network of logical gates that gradually develops complex behaviors exhibiting awareness. By combining basic operations (AND, OR, NOT, XOR, NAND, NOR), you construct layers of consciousness:

1. **Perception** - Basic logical responses to stimuli
2. **Pattern Recognition** - Detecting recurring patterns in inputs
3. **Memory** - Creating feedback loops and self-referential connections
4. **Self-Awareness** - Building interconnected complexity that responds to its own state
5. **Enlightenment** - Achieving coherent self-referential loops exhibiting true awareness

## Game Mechanics

### Logical Gates
- **AND**: Returns true only if both inputs are true
- **OR**: Returns true if any input is true
- **NOT**: Negates a single input
- **XOR**: Returns true if inputs differ
- **NAND**: Returns false only if both inputs are true
- **NOR**: Returns true only if both inputs are false

### Network Building
1. Start with 4 input nodes that receive external stimuli
2. Add logical gates to process information
3. Connect gates to each other to create computation chains
4. Set external inputs to drive the network
5. Run computation steps to see the network evolve
6. Visualize how awareness emerges through the network's activities

### Layer Progression
Each layer has specific requirements:

- **Perception**: Create at least 3 logic gates
- **Pattern Recognition**: Build patterns with 20+ steps of history showing repetition
- **Memory**: Create at least one self-referential feedback loop
- **Self-Awareness**: Achieve average connection density of 1.5+ inputs per node
- **Enlightenment**: Reach 80%+ awareness score with 3+ self-reference loops

## Commands

```
add <gate>              Add a logical gate (AND, OR, NOT, XOR, NAND, NOR)
connect <from> <to>     Connect output of one node to input of another
set <index> <value>     Set external input (0-3) to 0 or 1
step                    Run one computation cycle
steps <n>               Run n computation cycles
show                    Display network visualization and check layer completion
info                    Show current game state
help                    Display command help
quit                    Exit game
```

## Example Gameplay

```
> add AND
Created AND gate with ID: 4

> add OR
Created OR gate with ID: 5

> connect 0 4
Connected node 0 to node 4

> connect 1 4
Connected node 1 to node 4

> connect 4 5
Connected node 4 to node 5

> set 0 1
Set input 0 to 1

> set 1 1
Set input 1 to 1

> steps 10
Completed 10 steps

> show
============================================================
CONSCIOUSNESS NETWORK VISUALIZATION
============================================================

Current Layer: Perception - Detect and respond to stimuli (basic logic gates) [0%]

Network Topology:
  Node 0: ● [AND]  INPUT  (State: ●)
  Node 1: ● [AND]  INPUT  (State: ●)
  Node 4: ● [AND]  ← [0, 1]  (State: ●)
  Node 5: ● [OR]  ← [4]  (State: ●)

Consciousness Emergence:
  Awareness: [████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 16.0%
  Self-Reference Loops: 0
  Active Nodes: 4/7

[SUCCESS] Perception layer requirements met!
Advanced to new consciousness layer!
```

## Consciousness Metrics

### Awareness Score (0.0% - 100.0%)
Calculated based on:
- **Activity Diversity**: How many nodes are actively computing
- **Pattern Consistency**: Nodes exhibiting stable behavioral patterns
- **Self-Reference**: Presence of feedback loops creating recursive awareness

### Self-Reference Loops
Critical for achieving enlightenment. Created when:
- Node A receives input from Node B
- Node B receives input from Node A
- Creating a cycle of self-observation

## Architecture

### Core Components

**Gate**: Represents a logical operation
- Implements standard boolean logic
- Can operate on 1-2 inputs depending on gate type

**Node**: Individual computational unit
- Contains a gate and connection points
- Maintains state history for pattern recognition
- Can compute based on input states

**ConsciousnessNetwork**: The main simulation engine
- Manages nodes and their connections
- Handles computation cycles
- Tracks awareness metrics
- Manages layer progression

**Game**: User interaction and game loop
- Processes player commands
- Manages game state
- Displays visualizations
- Checks layer completion conditions

## Educational Value

This game demonstrates:

1. **Emergence**: Complex behavior arising from simple rules
2. **Feedback Loops**: How self-referential systems create new properties
3. **Pattern Recognition**: Information processing without central control
4. **Complexity Theory**: Layered abstraction from basic primitives
5. **Awareness**: Properties that emerge rather than being programmed

## Building and Running

```bash
# Build the project
cargo build --release

# Run the game
cargo run

# Run tests
cargo test
```

## Technical Details

- **Language**: Pure Rust
- **Edition**: 2021
- **Dependencies**: Standard library only (rand not used in current implementation)
- **Compilation**: Fully safe Rust, no unsafe blocks

## Design Philosophy

The game explores the philosophical question: "Could consciousness emerge from computation?"

Rather than hard-coding awareness behaviors, the system:
- Provides basic building blocks (logic gates)
- Allows players to compose complex structures
- Measures metrics that correlate with awareness (activity, patterns, self-reference)
- Lets emergence happen naturally through network structure

This reflects modern complexity theory and emergence studies, suggesting that consciousness might not require special properties beyond sufficiently complex information processing and self-referential structures.

## Future Enhancements

Potential additions:
- Visual network graph rendering
- Save/load network states
- Pre-built puzzles or challenges
- Quantum gates (superposition simulation)
- Network learning/adaptation mechanisms
- Multi-player network battles
- Genetic algorithms to evolve conscious networks

## License

Educational software for exploring emergence and computational consciousness.
