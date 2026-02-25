# Text Adventure Engine

A comprehensive, modular text adventure game engine written in Go. Create immersive text-based games with dynamic worlds, NPCs, puzzles, and complex interactions.

## Features

- **Natural Language Parser**: Handles diverse input formats (take sword, pick up key, grab item)
- **Dynamic World Model**: Rooms, items, NPCs, and complex interconnections
- **Game State Management**: Track inventory, visited rooms, examined items, game variables
- **Command Execution**: 20+ built-in commands with extensibility
- **Event System**: Loose coupling with pub-sub event handling
- **Save/Load System**: Persistent game progress
- **Inventory Management**: Weight limits, item combinations, equipment slots
- **NPC Dialogue**: Branching conversations with conditional options
- **Container Objects**: Items that hold other items (chests, bags, etc.)
- **Sample Game**: "Dungeon Escape" with 20+ rooms and complex puzzles

## Quick Start

### Prerequisites

- Go 1.21 or later
- Make (optional, for build targets)

### Building

```bash
# Build the game
make build

# Or manually with Go
go build -o bin/textadventure ./cmd/main.go
```

### Running the Sample Game

```bash
# Using Make
make run

# Or directly
./bin/textadventure games/dungeon-escape/dungeon-escape.json
```

### Running Tests

```bash
# Run all tests
make test

# With coverage report
make coverage
```

## Command Reference

### Movement
- `north`, `south`, `east`, `west` (or `n`, `s`, `e`, `w`)
- `up`, `down` (or `u`, `d`)

### Actions
- `look` / `l` - Examine your surroundings
- `examine <item>` / `x <item>` - Look closely at something
- `take <item>` - Pick up an item
- `drop <item>` - Put down an item
- `use <item> on <target>` - Use one item on another
- `put <item> in <container>` - Place item in container
- `open <container>` - Open a container
- `close <container>` - Close a container
- `talk to <npc>` - Start a conversation

### Inventory
- `inventory` / `i` - Show what you're carrying
- Weight and size limits apply

### System
- `wait` / `z` - Pass a turn
- `save <name>` - Save your progress
- `load <name>` - Load a saved game
- `help` - Show command help
- `quit` / `exit` - Quit the game

## Creating Your Own Game

### Game File Format

Games are defined in JSON format:

```json
{
  "title": "My Adventure",
  "description": "An exciting adventure",
  "version": "1.0",
  "author": "Your Name",
  "startingRoom": "room1",
  "maxInventory": 10,
  "maxCarryWeight": 50.0,
  "variables": {
    "puzzle1_solved": false
  },
  "rooms": {
    "room1": {
      "name": "Starting Room",
      "description": "You are in a room.",
      "exits": {
        "north": "room2"
      },
      "items": ["sword"],
      "npcs": ["guide"]
    }
  },
  "items": {
    "sword": {
      "name": "Iron Sword",
      "description": "A sharp sword",
      "takeable": true,
      "useable": true,
      "weight": 3.0
    }
  },
  "npcs": {
    "guide": {
      "name": "Guide",
      "description": "A helpful guide",
      "location": "room1",
      "friendly": true,
      "canTrade": true
    }
  }
}
```

### Room Definition

```json
{
  "name": "Room Name",
  "description": "Detailed room description",
  "exits": {
    "north": "next_room_id",
    "south": "previous_room_id"
  },
  "items": ["item1", "item2"],
  "npcs": ["npc1"],
  "locked": false,
  "lockItem": "key_id"
}
```

### Item Definition

```json
{
  "name": "Display Name",
  "description": "Detailed description",
  "takeable": true,
  "useable": true,
  "container": false,
  "locked": false,
  "hidden": false,
  "weight": 1.5,
  "size": 0.5,
  "cursed": false,
  "stackable": false,
  "quantity": 1
}
```

### NPC Definition

```json
{
  "name": "Character Name",
  "description": "Character description",
  "location": "room_id",
  "friendly": true,
  "canTrade": true
}
```

## Engine Architecture

### Package Structure

```
textadventure/
├── engine/              # Core engine
│   ├── types.go        # Data structures
│   ├── parser.go       # Command parsing
│   ├── world.go        # World management
│   ├── state.go        # Game state
│   ├── engine.go       # Main game loop
│   └── errors.go       # Error definitions
├── gameformat/         # Game file handling
│   └── loader.go       # Load/save games
├── tools/              # Development tools
│   └── builder.go      # Game builder utilities
├── cmd/                # Command-line interface
│   └── main.go         # Entry point
├── tests/              # Test suite
│   ├── parser_test.go
│   ├── world_test.go
│   └── ...
└── games/              # Sample games
    └── dungeon-escape/
        └── dungeon-escape.json
```

### Core Components

**Parser**: Converts natural language input to commands
- Handles variations and aliases
- Fuzzy matching for typos
- Context-aware interpretation

**World Model**: Represents the game universe
- Rooms with connections
- Items with properties
- NPCs with state
- Validation and integrity checking

**Game State**: Tracks current game progress
- Player location and inventory
- Visited rooms
- Game variables
- Snapshots for save/load

**Command Executor**: Executes parsed commands
- Movement between rooms
- Item manipulation
- NPC interaction
- Event emission

**Event System**: Loose coupling via events
- Room entered
- Item taken/dropped
- NPC talked to
- Custom events

## File Locations

- **Games**: `games/<game-name>/`
- **Documentation**: `docs/`
- **Tests**: `tests/`
- **Built Binary**: `bin/textadventure`

## Game Statistics

The Dungeon Escape sample includes:
- **Rooms**: 9 interconnected locations
- **Items**: 17 different objects to interact with
- **NPCs**: 2 characters
- **Exits**: Multiple paths and puzzles
- **Features**: Hidden items, containers, locked doors

## Testing

The engine includes comprehensive tests:

```bash
# Run all tests
make test

# Generate coverage report
make coverage

# Coverage is tracked for:
# - Parser functionality (command parsing)
# - World navigation
# - State management
# - Command execution
# - Item interaction
# - Inventory management
```

## Development

### Code Standards

- Go idioms and best practices
- Clear naming conventions
- Comprehensive documentation
- Error handling with custom error types
- Modular design for extensibility

### Adding Features

1. **New Commands**: Extend the parser and executor
2. **Game Mechanics**: Use the event system
3. **Scripting**: Implement via event handlers
4. **Dialogue Trees**: Define in game files
5. **Complex Interactions**: Use state variables

## Debugging

Run with debug flag to see additional information:

```bash
./bin/textadventure -debug games/dungeon-escape/dungeon-escape.json
```

Debug output includes:
- Current room ID
- Turn counter
- Player score
- Game state changes

## Performance

Optimized for responsive gameplay:
- Command execution: <100ms typical
- State transitions: Atomic operations
- Memory efficient inventory management
- Lazy evaluation of descriptions

## Limitations & Future Work

### Current Limitations

- No permanent NPC pathfinding
- Simple dialogue system (extendable)
- No combat system (in sample)
- No crafting/combining system

### Planned Features

- Advanced combat mechanics
- Crafting system
- Quest journal
- Time-based events
- Procedural world generation
- Web UI interface
- Multi-player support

## Contributing

This is a sample implementation. Extend it by:
1. Creating new game files
2. Adding custom event handlers
3. Implementing new command types
4. Creating dialogue trees
5. Adding visual elements (ASCII art)

## License

MIT License - Feel free to use and modify

## Credits

Text Adventure Engine v1.0.0
Created as a comprehensive example of Go game development patterns

## Support

For issues or questions:
1. Check the HLD.md for architectural details
2. Review test files for usage examples
3. Examine the sample game JSON for structure
4. Consult the API documentation

---

**Ready to adventure? Run `make run` to begin!**
