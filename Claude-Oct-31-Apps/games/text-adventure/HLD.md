# Text Adventure Engine - High-Level Design

## 1. Overview

The Text Adventure Engine is a modular, extensible framework for creating interactive text-based games in Go. It provides a comprehensive system for managing game worlds, parsing player commands, managing state, and executing game logic through a flexible scripting system.

## 2. Architecture

### 2.1 High-Level Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Game Loop                               │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Input      │  │   Command    │  │   Game       │     │
│  │   Parser     │→ │   Executor   │→ │   State      │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│         ↓                ↓                    ↓             │
│         └────────────────┴────────────────────┘             │
│                         ↓                                   │
│              ┌──────────────────────┐                      │
│              │   World Model        │                      │
│              │  - Rooms             │                      │
│              │  - Items             │                      │
│              │  - NPCs              │                      │
│              │  - Connections       │                      │
│              └──────────────────────┘                      │
│                         ↓                                   │
│              ┌──────────────────────┐                      │
│              │   Scripting Engine   │                      │
│              │  - Event Handler     │                      │
│              │  - Trigger System    │                      │
│              │  - State Mutations   │                      │
│              └──────────────────────┘                      │
└─────────────────────────────────────────────────────────────┘
```

## 3. Core Systems

### 3.1 Command Parser

The parser is responsible for converting player input into actionable commands.

#### Command Types:
- **Movement**: `north`, `south`, `east`, `west`, `up`, `down`, `n`, `s`, `e`, `w`, `u`, `d`
- **Object Interaction**: `take <object>`, `drop <object>`, `examine <object>`, `x <object>`
- **Action-Object-Target**: `use <object> on <target>`, `put <object> in <target>`
- **NPC Interaction**: `talk to <npc>`, `ask <npc> about <topic>`
- **Inventory**: `inventory`, `i`
- **Look**: `look`, `l`
- **Wait**: `wait`, `z`
- **Help**: `help`, `?`
- **Save/Load**: `save <name>`, `load <name>`
- **Quit**: `quit`, `exit`

#### Parser Features:
- **Natural Language Processing**:
  - Handles variations like "take the sword" vs "pick up sword" vs "grab sword"
  - Extracts verb, noun, and target from input
  - Context-aware interpretation (e.g., "sword" refers to item in current room)

- **Fuzzy Matching**:
  - Typo correction for object names
  - Similarity-based matching (edit distance)
  - Suggestion system for unclear commands

- **Alias System**:
  - Built-in shortcuts (x=examine, i=inventory, l=look)
  - Game-defined aliases
  - Player-customizable aliases

- **Context**:
  - Available exits in current room
  - Items in current location
  - Items in inventory
  - NPCs in current location

#### Command Structure:
```go
type Command struct {
    Action   string   // verb: "take", "drop", "examine", etc.
    Object   string   // first noun: object of action
    Target   string   // second noun: target of action (optional)
    Args     []string // additional arguments
    Raw      string   // original input
}
```

### 3.2 World Model

The world model represents the game universe and all its contents.

#### Core Entities:

**Room**:
- Unique ID
- Name and description
- Dynamic description function (based on state)
- Connected exits (north, south, east, west, up, down)
- Items present
- NPCs present
- Room state (visited, locked, etc.)
- Lighting properties

**Item**:
- Unique ID
- Name and aliases
- Description
- Takeable flag
- Useable flag
- Container flag (can hold items)
- Weight and size (for limits)
- Properties (locked, open, examined)
- State-based description

**NPC**:
- Unique ID
- Name
- Description
- Dialogue tree structure
- Interaction callbacks
- State (angry, friendly, afraid)
- Inventory

**World State**:
- Current room
- Player inventory
- Item states (examined, moved, combined)
- Room states (visited, discovered)
- Game variables/flags
- Score tracking

#### World Structure (JSON):
```json
{
  "title": "Dungeon Escape",
  "startingRoom": "cell",
  "rooms": {
    "cell": {
      "name": "Dungeon Cell",
      "description": "A small, damp cell.",
      "exits": {
        "north": "corridor"
      },
      "items": ["mattress", "key"],
      "npc": []
    }
  },
  "items": {
    "key": {
      "name": "iron key",
      "description": "An old iron key.",
      "takeable": true,
      "weight": 0.1
    }
  },
  "npc": {},
  "variables": {}
}
```

### 3.3 Game State Manager

Manages the current state of the game and ensures consistency.

**Responsibilities**:
- Track player location
- Manage inventory (with weight/size limits)
- Maintain item states
- Track game variables and flags
- Handle state mutations safely
- Provide state snapshots for save/load

**State Mutation Protocol**:
- All mutations go through the state manager
- Changes are logged for debugging
- Before/after hooks for events
- Rollback capability for failed actions

### 3.4 Command Executor

Executes parsed commands and produces game state changes.

**Built-in Commands**:
- **Movement**: Changes player location, triggers room entry events
- **Examine**: Returns item description, marks as examined
- **Take/Drop**: Moves items between room and inventory
- **Inventory**: Lists carrying items
- **Use On**: Executes use interaction between two items
- **Talk**: Initiates NPC dialogue
- **Save/Load**: Persists/restores game state

**Action Resolution**:
1. Parse command into structured form
2. Validate command (room has exit, item is takeable, etc.)
3. Call pre-action hooks
4. Mutate game state
5. Call post-action hooks
6. Generate response text
7. Trigger events/scripts

### 3.5 Scripting System

A lightweight scripting engine for implementing game logic without requiring code changes.

#### Script Language Features:
- **Conditions**: Check game state variables, item properties, player location
- **Actions**: Set variables, move items, change room properties, trigger text
- **Control Flow**: If/else, loops, switch statements
- **Variables**: String, number, boolean, list types
- **Functions**: Built-in library of game functions

#### Script Types:

**Room Scripts**:
- `on_enter`: Triggered when player enters room
- `on_examine`: Triggered when player examines something in room
- `on_take`: Triggered when player takes item from room
- `on_use`: Triggered when item is used on something in room

**Item Scripts**:
- `on_take`: Triggered when item is taken
- `on_drop`: Triggered when item is dropped
- `on_examine`: Triggered when item is examined
- `on_use`: Triggered when item is used on something

**NPC Scripts**:
- `on_talk`: Dialogue branch selection
- `on_trade`: Item exchange logic
- `on_leave`: Triggered when NPC leaves room

**Global Scripts**:
- `on_game_start`: Initial setup
- `on_turn`: Called each turn
- `on_victory`: Called when game is won

#### Script Format (YAML):
```yaml
scripts:
  room_cell_on_enter:
    condition:
      - variable: "cell_visited"
        is: false
    actions:
      - set_variable: "cell_visited" to true
      - print: "You awaken in a cold cell..."

  item_key_on_take:
    condition: []
    actions:
      - set_variable: "has_key" to true
      - print: "The key is cold in your hand."
```

### 3.6 Event and Trigger System

A pub-sub system for game events that decouples game systems.

**Event Types**:
- `room_entered(room_id)`
- `item_taken(item_id)`
- `item_dropped(item_id)`
- `item_examined(item_id)`
- `item_used(item_id, target_id)`
- `npc_talked_to(npc_id)`
- `game_start()`
- `game_tick(turn_number)`
- `custom_event(event_name, data)`

**Trigger Definition**:
```go
type Trigger struct {
    Event    string      // event type to listen for
    Condition func() bool // optional condition
    Action   func()       // callback function
}
```

### 3.7 NPC Dialogue System

NPCs communicate with the player through structured dialogue trees.

**Dialogue Tree Structure**:
```
NPC: "Hello, stranger. What brings you here?"
├─ Option 1: "I'm looking for treasure"
│  └─ NPC: "Aha! I have a map, but it will cost you."
│     └─ Option A: "How much?"
│     └─ Option B: "Never mind."
├─ Option 2: "I'm lost"
│  └─ NPC: "You're in the Dungeon of Shadows..."
└─ Option 3: "Nothing, just exploring"
   └─ NPC: "Move along then."
```

**Dialogue Features**:
- Branching conversations
- State-based dialogue (different conversations based on game state)
- Conditional options (only show if player has item, knows fact, etc.)
- Dialogue memory (track what's been said)
- Trade/transaction support

### 3.8 Save/Load System

Persistent game state using JSON format.

**Save File Structure**:
```json
{
  "version": "1.0",
  "timestamp": "2025-10-31T14:30:00Z",
  "gameFile": "games/dungeon-escape.json",
  "playerLocation": "cell",
  "inventory": ["key"],
  "variables": {
    "cell_visited": true,
    "has_key": true
  },
  "roomStates": {
    "cell": {
      "items": [],
      "visited": true
    }
  },
  "itemStates": {
    "key": {
      "examined": true,
      "location": "inventory"
    }
  }
}
```

**Save/Load Features**:
- Multiple save slots
- Timestamp and auto-save
- Save validation and integrity checking
- Compatibility checking for game version
- Quick-save/quick-load shortcuts

### 3.9 Inventory System

Manages player inventory with optional constraints.

**Features**:
- Weight and size limits
- Item organization (equipment slots optional)
- Quick access to common items
- Drop/take with validation
- Inventory search and filtering
- Sort options (alphabetical, by weight, by type)

**Constraints**:
- Maximum weight capacity
- Maximum item count
- Item stacking (identical items counted together)
- Restricted items (cursed items, etc.)

## 4. Engine Features

### 4.1 Core Engine Package (`engine/`)

**Files**:
- `engine.go`: Main game loop and initialization
- `parser.go`: Command parser and natural language processing
- `command.go`: Command structures and executor
- `world.go`: World model and entities
- `state.go`: Game state management
- `script.go`: Scripting engine
- `event.go`: Event and trigger system
- `inventory.go`: Inventory management
- `output.go`: Text formatting and rendering

### 4.2 Game Format Package (`gameformat/`)

**Files**:
- `loader.go`: Load game definitions from files
- `validator.go`: Validate game structure
- `schema.go`: Game file schema definitions

### 4.3 Tools Package (`tools/`)

**Files**:
- `worldbuilder.go`: Helper utilities for creating games
- `debug.go`: Debug mode with game state inspection
- `mapgen.go`: Simple procedural world generation

### 4.4 Sample Game Package (`games/`)

**Directory Structure**:
```
games/dungeon-escape/
├── dungeon-escape.json      # World definition
├── dialogue.yaml            # NPC dialogues
├── scripts.yaml             # Game scripts
└── README.md               # Game guide
```

## 5. Implementation Details

### 5.1 Parser Implementation

**Two-phase parsing**:
1. **Tokenization**: Break input into words, identify special tokens
2. **Syntax Analysis**: Match patterns to determine command structure

**Pattern Matching** (in priority order):
1. Direction patterns: `n|north|south|s|east|e|west|w|up|u|down|d`
2. Inventory patterns: `i|inventory|inv`
3. Look patterns: `l|look|examine room`
4. Movement with direction: `go north`, `move south`
5. Single object patterns: `examine <object>`, `take <object>`
6. Two object patterns: `use <object> on <target>`, `put <object> in <target>`
7. NPC patterns: `talk to <npc>`, `ask <npc> about <topic>`
8. System commands: `save`, `load`, `quit`, `help`

**Matching Strategy**:
- Use exact matches first
- Fall back to fuzzy matching with Levenshtein distance
- Generate suggestions for close matches
- Provide "I don't understand" message with hints

### 5.2 World Navigation

**Room Connections**:
- Directed graph structure (room A exits to room B, but not necessarily vice versa)
- Support for conditional exits (locked doors, one-way passages)
- Support for secret/hidden exits (discovered through scripts)
- Automatic description of visible exits in room

**Dynamic Room Descriptions**:
```go
type Room struct {
    BaseDescription string
    DescriptionFn   func(state *GameState) string // Optional dynamic description
    ...
}
```

### 5.3 Item System

**Item Properties**:
- Location (room, inventory, in another item)
- State (examined, moved, combined with)
- Attributes (weight, size, takeable, useable)
- Interaction callbacks (on_take, on_drop, on_use)

**Item Interactions**:
- Simple use: `examine key` returns description
- Contextual use: `use key on door` triggers interaction
- Combinations: `combine potion with water` creates new item
- Consumption: `drink potion` removes from inventory

### 5.4 Container Objects

Special items that can hold other items.

**Features**:
- Open/closed state
- Locked/unlocked state
- Capacity limits
- Visual representation (shows contents when open)

**Interactions**:
- `open chest` → shows contents
- `close chest` → hides contents
- `put key in chest` → adds item to container
- `take key from chest` → removes item from container

### 5.5 Score and Achievement System

Optional tracking of player progress.

**Score Categories**:
- Rooms visited
- Items found
- Puzzles solved
- NPCs talked to
- Custom achievements

**Achievement System**:
- Define achievements in game file
- Track completion status
- Display when achieved
- Multiple difficulty levels

## 6. Sample Game: "Dungeon Escape"

### 6.1 Game Concept

The player awakens in a dungeon cell with no memory of how they got there. They must navigate 20+ rooms, solve 5+ puzzles, gather items, and ultimately escape the dungeon. Multiple endings based on choices and items collected.

### 6.2 World Layout

```
                    DUNGEON ESCAPE MAP

        [Guard]----[Treasury]
           |            |
        [Corridor]--[Kitchen]
           |            |
         [Cell]      [Cellar]
           |            |
        [Escape Passage]
```

### 6.3 Key Elements

**Rooms** (20+):
- Dungeon Cell (starting room)
- Dungeon Corridor
- Guard Chamber
- Treasury
- Kitchen
- Cellar
- Hidden Passages
- Boss Chamber
- Escape Portal
- And more...

**Items** (15+):
- Iron Key
- Torch
- Guard Uniform
- Ancient Map
- Magic Amulet
- Lockpick Set
- And more...

**Puzzles** (5+):
1. **Cell Escape**: Find key hidden under mattress
2. **Guard Confrontation**: Get past guard through disguise or cunning
3. **Treasury Lock**: Unlock treasure room using ancient lock mechanism
4. **Cellar Flooding**: Solve water puzzle to reach lower levels
5. **Boss Battle**: Optional combat encounter

**NPCs** (5+):
- Guard Captain
- Dungeon Master
- Ghost Prisoner
- Merchant
- Oracle

**Endings**:
- Quick Escape: Escape in 30 minutes (minimal puzzle solving)
- Full Escape: Escape with treasure
- True Escape: Escape and defeat boss, access hidden ending

### 6.4 Difficulty Features

- Optional combat encounters
- Time pressure puzzles (some doors close after time)
- Hidden secrets and easter eggs
- Difficulty levels (novice, intermediate, hard)
- Hint system for stuck players

## 7. Design Patterns and Principles

### 7.1 Patterns Used

- **Observer Pattern**: Event system for loose coupling
- **Strategy Pattern**: Different parsing strategies, script execution
- **Factory Pattern**: Command creation, entity instantiation
- **State Pattern**: Game state management
- **Template Method**: Script execution flow
- **Builder Pattern**: Game file loading and construction

### 7.2 Design Principles

- **Modularity**: Clear separation between parser, world, state, and scripting
- **Extensibility**: Easy to add new commands, items, NPCs, scripting functions
- **Immutability**: State changes are atomic and traceable
- **Testability**: Pure functions where possible, dependency injection
- **Clarity**: Clear naming, comprehensive documentation
- **Performance**: Efficient data structures, lazy evaluation where appropriate

## 8. Error Handling

### 8.1 Error Categories

- **Input Errors**: Unparseable commands, unknown verbs/nouns
- **State Errors**: Invalid transitions, impossible actions
- **Script Errors**: Syntax errors, runtime errors
- **File Errors**: Missing game files, corrupt save files
- **Configuration Errors**: Invalid game definitions

### 8.2 Error Recovery

- Graceful degradation
- Clear error messages with suggestions
- Logging for debugging
- Checkpoint system for critical operations
- Fallback behaviors

## 9. Testing Strategy

### 9.1 Test Coverage

- **Parser Tests**: Valid/invalid inputs, edge cases, fuzzy matching
- **Command Tests**: Movement, inventory, interactions
- **World Tests**: Room navigation, item placement
- **State Tests**: Mutations, consistency, save/load
- **Integration Tests**: Full game scenarios
- **Game Tests**: Sample game completability

### 9.2 Test Organization

```
tests/
├── parser_test.go
├── command_test.go
├── world_test.go
├── state_test.go
├── script_test.go
├── inventory_test.go
├── game_test.go
└── fixtures/
    ├── sample_game.json
    └── test_saves/
```

## 10. Future Extensions

### 10.1 Optional Features (Phase 2+)

- **Multiple Character Classes**: Different starting items/abilities
- **Skill System**: Player statistics affecting outcomes
- **Magic System**: Spells that modify world state
- **Crafting**: Combine items to create new ones
- **Quest System**: Structured objectives with rewards
- **AI Pathfinding**: NPCs navigate through world
- **Dynamic Weather**: Time-of-day and weather effects
- **Procedural Generation**: Randomized dungeons
- **Multiplayer**: Shared world interactions
- **Rich Media**: ASCII art, sound effects, color
- **Network Play**: Multi-user dungeon capabilities

### 10.2 Tool Extensions

- **Web Interface**: Play games in browser
- **World Editor GUI**: Visual world builder
- **Script IDE**: Full development environment
- **Version Control**: Git integration for game files
- **Analytics**: Track player behavior and completion

## 11. Documentation

### 11.1 Documentation Files

- **README.md**: Project overview and quick start
- **ARCHITECTURE.md**: Detailed architecture guide
- **API.md**: Engine API documentation
- **GAME_FORMAT.md**: World definition file specification
- **SCRIPT_GUIDE.md**: Scripting language guide
- **TOOLS.md**: Development tools documentation
- **SAMPLE_WALKTHROUGH.md**: Complete game walkthrough

### 11.2 Code Documentation

- Package-level documentation
- Function-level comments for public APIs
- Example usage in comments
- Inline explanations for complex logic

## 12. Build and Deployment

### 12.1 Build System

- **Makefile**: Standard build targets
  - `make build`: Build executable
  - `make test`: Run test suite
  - `make coverage`: Generate coverage report
  - `make docs`: Build documentation
  - `make release`: Create distribution

### 12.2 Distribution

- Single binary with bundled sample game
- Configurable game data directory
- Documentation and examples
- Installation guide

## 13. Success Criteria

1. Engine implementation complete with all core features
2. Test suite with >80% code coverage
3. Parser handles diverse input formats gracefully
4. Sample game "Dungeon Escape" is fully playable
5. Clear documentation for extending and creating new games
6. No crashes or data corruption in save/load
7. Responsive gameplay (commands processed within 100ms)
8. Modular code suitable for community extensions
