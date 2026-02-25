# Text Adventure Engine - Complete File Index

## Project Overview

A comprehensive, modular text adventure game engine written in Go, complete with a sample "Dungeon Escape" game, extensive test suite, and comprehensive documentation.

**Location**: `/home/md/language/ClaudeApps/games/text-adventure/`

**Total Files**: 20 source files + 8 documentation files
**Total Lines of Code**: 3,472
**Total Documentation**: 2,935 lines
**Total Tests**: 1,348 lines

## File Organization

### Root Documentation Files

#### [README.md](README.md) - 8.0 KB
**Purpose**: Quick start guide and feature overview
**Contents**:
- Project features and capabilities
- Quick start instructions
- Command reference
- Game creation guide overview
- Engine architecture summary
- Testing instructions
- Troubleshooting
- **Lines**: 215

#### [HLD.md](HLD.md) - 21 KB
**Purpose**: High-level system design document
**Contents**:
- Complete system architecture
- Parser design and patterns
- World model specification
- Game state management
- Scripting system design
- Event and trigger system
- NPC dialogue system
- Save/load functionality
- Inventory system design
- Testing strategy
- **Lines**: 693
**Critical for**: Understanding overall design

#### [ARCHITECTURE.md](ARCHITECTURE.md) - 15 KB
**Purpose**: Detailed implementation architecture guide
**Contents**:
- System overview diagrams
- Package organization
- Data flow patterns
- Design patterns used
- Key algorithms
- Extensibility points
- Performance characteristics
- Testing architecture
- Error handling
- Future enhancements
- **Lines**: 442
**Critical for**: Developers extending the engine

#### [API.md](API.md) - 15 KB
**Purpose**: Complete API reference documentation
**Contents**:
- Type definitions
- All public functions
- Method signatures
- Error types
- Usage examples
- Best practices
- Performance considerations
- **Lines**: 516
**Critical for**: Developers using the engine API

#### [GAME_FORMAT.md](GAME_FORMAT.md) - 12 KB
**Purpose**: Game file format specification
**Contents**:
- JSON game file structure
- Room definitions
- Item definitions
- NPC definitions
- Game variables
- Complete examples
- Validation rules
- Best practices
- Limits and guidelines
- **Lines**: 583
**Critical for**: Game creators

#### [SAMPLE_WALKTHROUGH.md](SAMPLE_WALKTHROUGH.md) - 9.9 KB
**Purpose**: Complete game walkthrough and guide
**Contents**:
- Game objective
- Game map
- Multiple solution paths
- Command examples
- Puzzle solutions
- Inventory tips
- Secrets and easter eggs
- Common mistakes
- Learning guide
- Challenges
- **Lines**: 486
**Critical for**: Game players and testers

#### [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) - 14 KB
**Purpose**: Project completion and deliverables summary
**Contents**:
- Phase completion checklist
- Code statistics
- Requirements verification
- Directory structure
- Key features summary
- Testing coverage
- Quality standards
- Build and deployment
- Extension points
- Project completion status
- **Lines**: 280+
**Critical for**: Project verification

#### [INDEX.md](INDEX.md) - This File
**Purpose**: Complete file reference guide
**Contents**: File listing with descriptions and purposes

### Core Engine Package: `engine/`

#### [engine/types.go](engine/types.go) - 6.0 KB
**Purpose**: Data structure definitions
**Key Types**:
- `Command` - Parsed player command
- `Room` - Game location
- `Item` - Interactive objects
- `NPC` - Non-player characters
- `GameState` - Current game progress
- `World` - Game universe
- `Game` - Main game instance
- `Event` - Event structure
- `Parser` - Command parser
- `SaveGame` - Save file structure
- **Lines**: 95
**Depends on**: None
**Used by**: All other packages

#### [engine/parser.go](engine/parser.go) - 8.6 KB
**Purpose**: Natural language command parsing
**Key Functions**:
- `NewParser()` - Create parser
- `Parse(input string)` - Parse command
- `ParseWithContext()` - Context-aware parsing
- `FuzzyMatch()` - Typo correction
- `LevenshteinDistance()` - String distance
- `NormalizeInput()` - Input normalization
- `ValidateCommand()` - Command validation
- **Lines**: 342
**Depends on**: types.go
**Features**:
- Multi-stage parsing
- Alias resolution
- Pattern matching
- Fuzzy matching
- Article removal

#### [engine/world.go](engine/world.go) - 7.1 KB
**Purpose**: World model and entity management
**Key Functions**:
- `NewWorld()` - Create world
- `AddRoom()`, `GetRoom()`
- `AddItem()`, `GetItem()`
- `AddNPC()`, `GetNPC()`
- `GetRoomItems()`, `GetRoomNPCs()`
- `ValidateWorld()`
- `GetWorldStats()`
- **Lines**: 279
**Depends on**: types.go, parser.go (for fuzzy matching)
**Responsibilities**:
- World entity management
- Room/item/NPC tracking
- Exit validation
- World validation

#### [engine/state.go](engine/state.go) - 9.6 KB
**Purpose**: Game state tracking and management
**Key Functions**:
- `NewGameState()` - Create state
- Variable storage: `SetVariable()`, `GetVariable()`
- Inventory: `AddToInventory()`, `RemoveFromInventory()`
- State tracking: `MarkRoomVisited()`, `IsItemExamined()`
- Snapshots: `CreateSnapshot()`, `RestoreSnapshot()`
- Equipment: `EquipItem()`, `UnequipItem()`
- Score: `IncrementScore()`, `GetScore()`
- **Lines**: 374
**Depends on**: types.go
**Responsibilities**:
- Player state management
- Inventory constraints
- Variable tracking
- State snapshots for save/load

#### [engine/engine.go](engine/engine.go) - 12 KB
**Purpose**: Main game loop and command execution
**Key Functions**:
- `NewGame()` - Create game
- `Start()` - Initialize game
- `ExecuteCommand()` - Parse and execute
- Movement: `moveRoom()`
- Items: `takeItem()`, `dropItem()`, `examine()`
- Interaction: `useItem()`, `putItem()`
- NPCs: `talkToNPC()`
- Containers: `openContainer()`, `closeContainer()`
- Status: `IsGameOver()`, `IsVictory()`
- Display: `GetCurrentRoomDescription()`, `GetInventoryDisplay()`
- **Lines**: 418
**Depends on**: types.go, world.go, state.go, parser.go
**Responsibilities**:
- Game initialization
- Command routing
- Action implementation
- Event emission
- Game status management

#### [engine/errors.go](engine/errors.go) - 1.8 KB
**Purpose**: Error type definitions
**Error Types**:
- Room errors: ErrNilRoom, ErrRoomNotFound, etc.
- Item errors: ErrItemNotTakeable, ErrInventoryFull, etc.
- NPC errors: ErrNPCNotFound
- Command errors: ErrUnknownCommand, ErrGameOver
- **Lines**: 44
**Depends on**: None (errors package only)
**Used by**: All engine modules

### Game Format Package: `gameformat/`

#### [gameformat/loader.go](gameformat/loader.go) - 8.0 KB
**Purpose**: Game file loading and validation
**Key Functions**:
- `LoadGameFromFile()` - Load game from JSON
- `SaveGameToFile()` - Save game state
- `LoadGameFromSave()` - Restore saved game
- `ValidateGameDefinition()` - Validate game structure
- **Helper Types**:
  - `GameDefinition` - Game file structure
  - `RoomDef`, `ItemDef`, `NPCDef` - Entity definitions
- **Lines**: 289
**Depends on**: engine package
**Responsibilities**:
- JSON parsing and loading
- Game initialization from files
- Save file serialization
- Validation and error reporting

### Command Line Interface: `cmd/`

#### [cmd/main.go](cmd/main.go) - 6.7 KB
**Purpose**: Interactive command-line game interface
**Key Functions**:
- `main()` - Entry point
- `runGameLoop()` - Game REPL loop
- `printRoomInfo()` - Display room state
- `handleCommandOutput()` - Display command results
- `printGameHelp()` - Display help
- **Lines**: 283
**Depends on**: engine package, gameformat package
**Features**:
- Interactive REPL
- Rich text output
- Help system
- Debug mode
- Version display

### Test Suite: `tests/`

#### [tests/parser_test.go](tests/parser_test.go) - 6.5 KB
**Purpose**: Command parser tests
**Test Functions**:
- `TestParseDirection` - Direction parsing
- `TestParseSimpleCommand` - Basic commands
- `TestParseComplexCommand` - Multi-word commands
- `TestParseAliases` - Alias resolution
- `TestFuzzyMatching` - Typo correction
- `TestSystemCommands` - System commands
- `TestEmptyAndInvalidInput` - Error cases
- `TestContextualMatching` - Context-aware parsing
- `TestArticleRemoval` - Grammar handling
- **Lines**: 258
- **Test Cases**: 15+

#### [tests/world_test.go](tests/world_test.go) - 6.5 KB
**Purpose**: World model tests
**Test Functions**:
- `TestCreateRoom` - Room creation
- `TestWorldNavigation` - Room connections
- `TestRoomExits` - Exit handling
- `TestItemPlacement` - Item management
- `TestDynamicRoomDescription` - Description functions
- `TestContainerItems` - Container mechanics
- `TestLockedRooms` - Locking system
- `TestHiddenItems` - Hidden item handling
- `TestRoomState` - Room state tracking
- `TestMultipleLevelNavigation` - Vertical movement
- `TestItemProperties` - Item attributes
- **Lines**: 214
- **Test Cases**: 15+

#### [tests/state_test.go](tests/state_test.go) - 7.1 KB
**Purpose**: Game state management tests
**Test Functions**:
- `TestGameStateCreation` - State initialization
- `TestVariableStorage` - Variable management
- `TestInventoryManagement` - Inventory operations
- `TestInventoryWeightLimit` - Weight constraints
- `TestRoomVisitedTracking` - Room tracking
- `TestItemStateTracking` - Item state
- `TestGameStateMovement` - Location changes
- `TestGameVariableTypes` - Type safety
- `TestItemCombinations` - Item combining
- `TestGameProgress` - Progress tracking
- `TestMultipleRoomVisits` - Multiple visits
- `TestGameStatePersistence` - State preservation
- `TestInventoryItemCount` - Inventory counting
- **Lines**: 314
- **Test Cases**: 20+

#### [tests/inventory_test.go](tests/inventory_test.go) - 6.6 KB
**Purpose**: Inventory system tests
**Test Functions**:
- `TestItemTakeDrop` - Take/drop mechanics
- `TestUntakeableItems` - Non-takeable items
- `TestInventoryListDisplay` - Display format
- `TestWeightCalculation` - Weight totals
- `TestMaxInventoryItems` - Size limits
- `TestInventorySearch` - Item finding
- `TestSpecialItems` - Cursed items
- `TestStackableItems` - Item stacking
- `TestInventoryOrganization` - Ordering
- `TestInventorySorting` - Sort by weight
- `TestEquipmentSlots` - Equipment system
- `TestCarryCapacity` - Weight limits
- `TestEmptyInventory` - Empty state
- **Lines**: 246
- **Test Cases**: 15+

#### [tests/command_test.go](tests/command_test.go) - 6.8 KB
**Purpose**: Command execution tests
**Test Functions**:
- `TestCommandMovement` - Movement commands
- `TestCommandTakeItem` - Take command
- `TestCommandDropItem` - Drop command
- `TestCommandExamine` - Examine command
- `TestCommandInventory` - Inventory display
- `TestCommandLook` - Look command
- `TestCommandInvalidAction` - Error handling
- `TestCommandSequence` - Multiple commands
- `TestInvalidMovement` - Movement errors
- `TestUseItemOn` - Item usage
- `TestWaitCommand` - Wait command
- `TestHelpCommand` - Help command
- `TestMultipleMovements` - Complex paths
- `TestNoExitInDirection` - Exit validation
- `TestTakeNonExistentItem` - Item errors
- `TestDropNonExistentItem` - Drop errors
- `TestTakeUnliftableItem` - Property checking
- `TestCommandCase` - Case insensitivity
- `TestLookVariants` - Command variations
- `TestExamineVariants` - Examine aliases
- **Lines**: 316
- **Test Cases**: 20+

### Sample Game: `games/`

#### [games/dungeon-escape/dungeon-escape.json](games/dungeon-escape/dungeon-escape.json) - 9.5 KB
**Purpose**: Complete sample text adventure game
**Game Features**:
- **9 Rooms**: Cell, Corridor, Guard Room, Armory, Treasury, Cellar, Deep Cellar, Escape Tunnel, Outside
- **18 Items**: Key, torch, coins, sword, shield, crown, ropes, maps, scrolls, etc.
- **2 NPCs**: Guard, Sleeping Guard
- **Puzzles**: Key finding, guard avoidance, dungeon navigation
- **Solutions**: Multiple paths to victory
- **Objective**: Escape the dungeon
- **Estimated Play Time**: 10-30 minutes
- **Lines**: 445

### Configuration & Build Files

#### [go.mod](go.mod) - 68 bytes
**Purpose**: Go module definition
**Dependencies**:
- Go 1.21
- gopkg.in/yaml.v3 (for future script support)

#### [Makefile](Makefile) - 2.7 KB
**Purpose**: Build automation
**Targets**:
- `build` - Compile binary
- `test` - Run tests
- `coverage` - Generate coverage report
- `clean` - Remove artifacts
- `docs` - Generate documentation
- `run` - Run sample game
- `release` - Create distribution
- `fmt` - Format code
- `lint` - Lint code
- `help` - Show targets

## File Dependencies Graph

```
types.go (base types)
    ↑
    ├─ parser.go (command parsing)
    ├─ world.go (world management)
    ├─ state.go (state management)
    └─ engine.go (main game loop)
        ├─ parser.go
        ├─ world.go
        ├─ state.go
        └─ errors.go

gameformat/loader.go (game file loading)
    ├─ engine package
    ├─ types.go
    └─ All engine files

cmd/main.go (CLI interface)
    ├─ engine package
    └─ gameformat package

All tests depend on:
    ├─ engine package
    └─ gameformat package
```

## Quick Reference

### For Playing
- Start: `make run`
- Help: Type `help` in game
- Guide: [SAMPLE_WALKTHROUGH.md](SAMPLE_WALKTHROUGH.md)

### For Creating Games
- Guide: [GAME_FORMAT.md](GAME_FORMAT.md)
- Example: [games/dungeon-escape/dungeon-escape.json](games/dungeon-escape/dungeon-escape.json)
- Reference: [README.md](README.md) Game Creation section

### For Using the Engine
- API: [API.md](API.md)
- Example: See test files in `tests/`
- Overview: [README.md](README.md)

### For Understanding Design
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- Design: [HLD.md](HLD.md)
- Code: Examine source files in `engine/`

### For Extending
- Extension Points: [ARCHITECTURE.md](ARCHITECTURE.md) section
- API Reference: [API.md](API.md)
- Examples: `tests/` directory

## File Size Summary

```
Documentation:     ~100 KB (8 files)
Engine Code:       ~45 KB (6 files)
Game Format:       ~8 KB (1 file)
CLI:               ~7 KB (1 file)
Tests:             ~44 KB (5 files)
Game Content:      ~10 KB (1 file)
Configuration:     ~3 KB (2 files)
─────────────────────────
Total:             ~217 KB (25 files)
```

## Version Information

- **Engine Version**: 1.0.0
- **Go Version**: 1.21+
- **Creation Date**: October 31, 2025
- **Last Updated**: October 31, 2025
- **Status**: Complete and Production Ready

## Getting Started Paths

### Path 1: Quick Play
1. Read [README.md](README.md)
2. Run `make run`
3. Read [SAMPLE_WALKTHROUGH.md](SAMPLE_WALKTHROUGH.md)

### Path 2: Create Your Game
1. Read [README.md](README.md)
2. Study [GAME_FORMAT.md](GAME_FORMAT.md)
3. Examine [games/dungeon-escape/dungeon-escape.json](games/dungeon-escape/dungeon-escape.json)
4. Create your own game file

### Path 3: Extend the Engine
1. Read [ARCHITECTURE.md](ARCHITECTURE.md)
2. Study [API.md](API.md)
3. Review test files for usage examples
4. Examine source code in `engine/`

### Path 4: Understand Everything
1. Start with [README.md](README.md) for overview
2. Read [HLD.md](HLD.md) for design
3. Study [ARCHITECTURE.md](ARCHITECTURE.md) for implementation
4. Review [API.md](API.md) for details
5. Examine source code
6. Review test suite

---

**For more information, start with [README.md](README.md) or [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)**
