# Text Adventure Engine - Architecture Guide

## System Overview

The Text Adventure Engine is a modular, Go-based framework for creating interactive fiction games. It separates concerns into distinct layers: parsing, world management, state tracking, and command execution.

```
┌────────────────────────────────────────────────────────────┐
│                    User Interface Layer                     │
│              (CLI - cmd/main.go)                           │
│    ┌─────────────────────────────────────────────┐        │
│    │  Input Reading  │  Output Display           │        │
│    │  REPL Loop      │  Formatted Strings        │        │
│    └─────────────────────────────────────────────┘        │
└────────────────────┬───────────────────────────────────────┘
                     │
                     ↓
┌────────────────────────────────────────────────────────────┐
│                Application Layer                           │
│              (engine/engine.go)                            │
│    ┌──────────────────────────────────────────────┐       │
│    │  Game Loop  │  Command Executor  │  Events  │       │
│    │  Turn Count │  Action Dispatcher │  Emitter │       │
│    └──────────────────────────────────────────────┘       │
└────────────────────┬───────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        ↓            ↓            ↓
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│   Parser     │ │   World      │ │    State     │
│ Subsystem    │ │  Subsystem   │ │  Management  │
└──────────────┘ └──────────────┘ └──────────────┘
│ • Lexing     │ │ • Rooms      │ │ • Inventory  │
│ • Tokenizing │ │ • Items      │ │ • Variables  │
│ • Matching   │ │ • NPCs       │ │ • Snapshots  │
│ • Aliases    │ │ • Exits      │ │ • Tracking   │
└──────────────┘ └──────────────┘ └──────────────┘
        │            │            │
        └────────────┼────────────┘
                     ↓
┌────────────────────────────────────────────────────────────┐
│              Data Models & Types                           │
│              (engine/types.go)                             │
│    ┌──────────────────────────────────────────────┐       │
│    │ Room │ Item │ NPC │ Event │ Script │ Command│       │
│    └──────────────────────────────────────────────┘       │
└────────────────────────────────────────────────────────────┘
        │
        ↓
┌────────────────────────────────────────────────────────────┐
│           Data Layer & Persistence                         │
│        (gameformat/loader.go)                              │
│    ┌──────────────────────────────────────────────┐       │
│    │  JSON Loading  │  JSON Saving  │  Validation│       │
│    │  Game Files    │  Save Files   │  Schemas   │       │
│    └──────────────────────────────────────────────┘       │
└────────────────────────────────────────────────────────────┘
```

## Package Organization

### engine/

Core game engine implementation with no external dependencies.

**Files**:
- `types.go` - All data structures and interfaces
- `parser.go` - Natural language command parsing
- `world.go` - World model and entity management
- `state.go` - Game state tracking
- `engine.go` - Main game loop and command execution
- `errors.go` - Error type definitions

**Key Concepts**:
- **Stateless functions**: Parser is pure (no side effects)
- **Immutable references**: Items, rooms, NPCs are references
- **Type safety**: Proper error handling with typed errors
- **Loose coupling**: Event system for inter-component communication

### gameformat/

Game file loading and validation.

**Files**:
- `loader.go` - JSON parsing and game initialization

**Key Concepts**:
- **Validation**: Checks game structure before loading
- **Type conversion**: Converts JSON to engine types
- **Error reporting**: Clear error messages for invalid files
- **Save/load**: Serialization of game state

### cmd/

Command-line interface and entry point.

**Files**:
- `main.go` - Main function, game loop, UI

**Key Concepts**:
- **REPL loop**: Read-Eval-Print-Loop for interactive play
- **Output formatting**: Rich text display
- **Help system**: Built-in command documentation

## Data Flow

### Command Execution Flow

```
User Input
    ↓
[Parser.Parse()]  ← Converts string to Command struct
    ↓
[Game.ExecuteCommand()]  ← Routes to appropriate handler
    ↓
[game.executeAction()]  ← Implements actual action
    ↓
[Game.State mutation]  ← Updates game state
    ↓
[EventHandler.Emit()]  ← Publishes events
    ↓
[Subscribers]  ← React to events (scripts, logging, etc.)
    ↓
Output to UI  ← Display results
```

### Parsing Strategy

The parser uses a multi-stage approach:

```
Input: "use key on door"
    ↓
[Tokenize]  → ["use", "key", "on", "door"]
    ↓
[Alias Resolution]  → Check if "use" has aliases
    ↓
[Pattern Matching]  → Match against known patterns
    ↓
[Extraction]  → Extract verb, object, target
    ↓
Command {
    Action: "use",
    Object: "key",
    Target: "door",
}
```

### World Access Pattern

```
World Model (game.World)
    ├── Rooms (map[string]*Room)
    │   ├── Room.Items []*Item
    │   └── Room.NPCs []*NPC
    ├── Items (map[string]*Item)
    │   └── Item.Contents []*Item (for containers)
    └── NPCs (map[string]*NPC)
        └── NPC.Inventory []*Item

Game State (game.State)
    ├── CurrentRoom string
    ├── Inventory []*Item
    ├── Variables map[string]interface{}
    ├── RoomsVisited map[string]bool
    └── ItemsExamined map[string]bool
```

## Design Patterns

### 1. Observer Pattern (Event System)

```go
// Publisher (Game)
game.EventHandler.Emit(&Event{
    Type: "item_taken",
    Target: "key",
})

// Subscribers register handlers
game.EventHandler.Subscribe("item_taken", func(e *Event) {
    // React to event
})
```

**Benefits**:
- Loose coupling between components
- Easy to add new behaviors
- Testable without mocking

### 2. Strategy Pattern (Command Execution)

```go
switch cmd.Action {
case "take":
    return g.takeItem(cmd.Object)
case "drop":
    return g.dropItem(cmd.Object)
case "examine":
    return g.examine(cmd.Object)
// ...
}
```

**Benefits**:
- Easy to add new commands
- Encapsulated behavior
- Clear action mapping

### 3. Factory Pattern (Game Creation)

```go
// Create from JSON
game, err := gameformat.LoadGameFromFile("game.json")

// Create manually
game := engine.NewGame(world, state, parser)
```

**Benefits**:
- Multiple construction paths
- Validation during creation
- Consistent initialization

### 4. State Pattern (Game State)

```go
type GameState struct {
    CurrentRoom      string
    Inventory        []*Item
    Variables        map[string]interface{}
    RoomsVisited     map[string]bool
    // ...
}
```

**Benefits**:
- Encapsulated state
- Snapshot/restore capability
- Type-safe state access

## Key Algorithms

### 1. Fuzzy Matching

Uses Levenshtein distance for typo correction:

```
Input: "sord"
Candidates: ["sword", "shield", "key"]

Distance("sord", "sword") = 1  ← Best match
Distance("sord", "shield") = 3
Distance("sord", "key") = 4

Result: "sword"
```

### 2. Command Parsing

Multi-pass approach:
1. Check if entire input is a direction
2. Check if single word matches a command
3. Split into words and pattern match
4. Extract objects and targets
5. Remove articles from nouns

### 3. Inventory Management

```
Can add item?
    ↓
Check inventory size < max ✓
    ↓
Calculate total weight:
    sum(item.Weight for item in inventory)
    ↓
New weight = current + item.Weight
    ↓
Check new weight <= max ✓
    ↓
Add to inventory
```

## Extensibility Points

### 1. Adding Commands

Edit `engine/engine.go` method `executeAction()`:

```go
case "meditate":
    g.State.IncrementScore(5)
    return nil
```

### 2. Custom Logic via Events

Subscribe to events in your game initialization:

```go
game.EventHandler.Subscribe("room_entered", func(e *Event) {
    if e.Target == "treasure_room" {
        game.State.SetVariable("treasure_seen", true)
    }
})
```

### 3. Dynamic Room Descriptions

Use `DescriptionFn` in rooms:

```json
// In code when creating a game
room.DescriptionFn = func(state *GameState) string {
    if state.GetVariableAsBool("door_opened", false) {
        return "The door is now open."
    }
    return "The door is locked."
}
```

### 4. Item Interaction System

Add to items when creating them:

```go
item.State["interacted"] = false

// In event handler
game.EventHandler.Subscribe("item_used", func(e *Event) {
    item := game.World.GetItem(e.Target)
    if item != nil {
        item.State["interacted"] = true
    }
})
```

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Get room by ID | O(1) | Map lookup |
| Get item in world | O(1) | Map lookup |
| Find item in room | O(n) | Linear search, n = items in room |
| Find item in inventory | O(n) | Linear search, n = inventory size |
| Parse command | O(1) avg, O(m) worst | m = word count |
| Fuzzy match | O(n×m) | n = candidates, m = input length |

### Space Complexity

| Structure | Space | Scaling |
|-----------|-------|---------|
| World | O(r + i + n) | r=rooms, i=items, n=npcs |
| GameState | O(i + v) | i=inventory items, v=variables |
| Parser | O(a) | a=aliases (constant, ~20) |
| Total | O(r + i + n + v) | Typically <10MB |

### Optimization Techniques

1. **Inventory limit** (default 20) prevents O(n) searches becoming slow
2. **Map-based lookups** for world entities ensure O(1) access
3. **Event subscriptions** are O(k) where k is subscribers (typically small)
4. **State snapshots** only store necessary data

## Testing Architecture

Tests are organized by component:

```
tests/
├── parser_test.go          # Command parsing tests
├── world_test.go           # World model tests
├── state_test.go           # State management tests
├── inventory_test.go       # Inventory system tests
├── command_test.go         # Command execution tests
└── fixtures/
    └── sample_game.json    # Test game file
```

**Testing Strategy**:
- Unit tests for individual components
- Integration tests for command execution
- Fixtures for reproducible test scenarios
- Coverage targets: >80% of engine code

## Error Handling

### Error Types

Structured error types for proper error handling:

```go
// Item not in room
if item == nil {
    return ErrItemNotFound
}

// Inventory full
if !canAdd {
    return ErrInventoryFull
}

// Invalid move
if !exists {
    return ErrInvalidExit
}
```

### Error Recovery

Commands fail gracefully:

```go
// If take fails, item stays in room
// If move fails, player stays in room
// Invalid commands show help instead of crashing
```

## Future Architecture Enhancements

### 1. Dialogue System Enhancement

Current: Simple NPC presence
Planned: Branching conversation trees

```go
type DialogueNode struct {
    Text    string
    Options []*DialogueOption
    Next    map[string]*DialogueNode
}
```

### 2. Script Engine

Current: Inline event handlers
Planned: Game-defined scripts

```go
// Load from game file
scripts := loadScriptsFromYAML("scripts.yaml")
```

### 3. Async Events

Current: Synchronous event emission
Planned: Deferred/async event handling

```go
game.EventHandler.EmitAsync("delayed_event", delay)
```

### 4. Spatial Queries

Current: Linear search for items/NPCs
Planned: Spatial indexing for large worlds

```go
items := world.GetItemsInRadius(room, 50)
```

## Deployment Architecture

### Single Binary

```
text-adventure          (Main binary)
├── Game files (JSON)   (Embedded or external)
├── Resources           (Images, sounds - future)
└── Data                (Save games)
```

### Distributed Potential

```
┌──────────────────────┐
│   Web Client         │
│   (WebAssembly)      │
└──────────┬───────────┘
           │ WebSocket/HTTP
           ↓
┌──────────────────────┐
│   Game Server        │
│   (Go binary)        │
└──────────────────────┘
           │
           ↓
┌──────────────────────┐
│   Game Database      │
│   (Save states)      │
└──────────────────────┘
```

---

**For implementation details, see API.md. For game creation, see GAME_FORMAT.md.**
