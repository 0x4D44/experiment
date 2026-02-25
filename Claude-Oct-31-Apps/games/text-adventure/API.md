# Text Adventure Engine - API Documentation

## Overview

This document describes the public API for the Text Adventure Engine, allowing developers to extend and customize the engine.

## Package: engine

### Types

#### Command

```go
type Command struct {
    Action string   // verb: "take", "drop", "examine", etc.
    Object string   // first noun: object of action
    Target string   // second noun: target of action (optional)
    Args   []string // additional arguments
    Raw    string   // original input
}
```

#### Room

```go
type Room struct {
    ID              string
    Name            string
    BaseDescription string
    DescriptionFn   func(*GameState) string // optional dynamic description
    Exits           map[string]string       // direction -> room ID
    Items           []*Item
    NPCs            []*NPC
    Visited         bool
    Locked          bool
    LockItem        string
    State           map[string]interface{}  // room-specific state
}
```

#### Item

```go
type Item struct {
    ID          string
    Name        string
    Description string
    Takeable    bool
    Useable     bool
    Container   bool
    Locked      bool
    LockItem    string
    Open        bool
    Hidden      bool
    Weight      float64
    Size        float64
    Cursed      bool
    Stackable   bool
    Quantity    int
    Contents    []*Item                    // for containers
    Examined    bool
    State       map[string]interface{}
}
```

#### NPC

```go
type NPC struct {
    ID          string
    Name        string
    Description string
    Dialogue    *DialogueTree
    Inventory   []*Item
    Location    string
    State       map[string]interface{}
    Friendly    bool
    CanTrade    bool
}
```

#### GameState

```go
type GameState struct {
    CurrentRoom         string
    Inventory           []*Item
    Variables           map[string]interface{}
    RoomsVisited        map[string]bool
    ItemsExamined       map[string]bool
    ItemsCombined       map[string]map[string]bool
    MaxCarryWeight      float64
    MaxInventorySize    int
    Score               int
    TurnCount           int
    EquippedItems       map[string]string
    ItemLocations       map[string]string
    RoomStateSnapshots  map[string]map[string]interface{}
    ItemStateSnapshots  map[string]map[string]interface{}
}
```

#### World

```go
type World struct {
    Rooms map[string]*Room
    Items map[string]*Item
    NPCs  map[string]*NPC
}
```

#### Game

```go
type Game struct {
    World           *World
    State           *GameState
    Parser          *Parser
    EventHandler    *EventHandler
    Scripts         map[string]*Script
    TurnCounter     int
    GameOver        bool
    Victory         bool
}
```

#### Event

```go
type Event struct {
    Type      string                 // "room_entered", "item_taken", etc.
    Target    string                 // what triggered the event
    Timestamp int64                  // when it occurred
    Data      map[string]interface{} // event-specific data
}
```

### World Functions

#### NewWorld

```go
func NewWorld() *World
```

Creates a new empty world.

#### AddRoom

```go
func (w *World) AddRoom(room *Room) error
```

Adds a room to the world. Returns `ErrDuplicateRoom` if room ID already exists.

#### GetRoom

```go
func (w *World) GetRoom(id string) *Room
```

Retrieves a room by ID. Returns nil if not found.

#### AddItem

```go
func (w *World) AddItem(item *Item) error
```

Adds an item to the world registry. Returns `ErrDuplicateItem` if ID already exists.

#### GetItem

```go
func (w *World) GetItem(id string) *Item
```

Retrieves an item by ID. Returns nil if not found.

#### AddNPC

```go
func (w *World) AddNPC(npc *NPC) error
```

Adds an NPC to the world. Returns `ErrDuplicateNPC` if ID already exists.

#### GetNPC

```go
func (w *World) GetNPC(id string) *NPC
```

Retrieves an NPC by ID. Returns nil if not found.

#### GetRoomItems

```go
func (w *World) GetRoomItems(roomID string) []*Item
```

Returns all items in a room.

#### GetRoomNPCs

```go
func (w *World) GetRoomNPCs(roomID string) []*NPC
```

Returns all NPCs in a room.

#### HasExit

```go
func (w *World) HasExit(roomID, direction string) bool
```

Checks if a room has an exit in a direction.

#### GetExit

```go
func (w *World) GetExit(roomID, direction string) string
```

Returns the target room for an exit. Returns empty string if not found.

#### GetAvailableExits

```go
func (w *World) GetAvailableExits(roomID string) []string
```

Returns list of all exit directions from a room.

#### ValidateWorld

```go
func (w *World) ValidateWorld() []string
```

Validates world structure, returns list of errors found.

### GameState Functions

#### NewGameState

```go
func NewGameState(startingRoom string) *GameState
```

Creates a new game state starting in the given room.

#### SetVariable

```go
func (gs *GameState) SetVariable(key string, value interface{})
```

Sets a game variable.

#### GetVariable

```go
func (gs *GameState) GetVariable(key string) (interface{}, bool)
```

Retrieves a variable. Returns (value, true) if found, (nil, false) otherwise.

#### GetVariableAsString / GetVariableAsInt / GetVariableAsBool

```go
func (gs *GameState) GetVariableAsString(key string, defaultVal string) string
func (gs *GameState) GetVariableAsInt(key string, defaultVal int) int
func (gs *GameState) GetVariableAsBool(key string, defaultVal bool) bool
```

Type-safe variable retrieval with default values.

#### AddToInventory

```go
func (gs *GameState) AddToInventory(item *Item) bool
```

Adds an item to inventory. Returns false if inventory full or weight exceeded.

#### RemoveFromInventory

```go
func (gs *GameState) RemoveFromInventory(itemID string) bool
```

Removes an item from inventory by ID. Returns false if not found.

#### HasItem

```go
func (gs *GameState) HasItem(itemID string) bool
```

Checks if item is in inventory.

#### GetItem

```go
func (gs *GameState) GetItem(itemID string) *Item
```

Retrieves an item from inventory.

#### CanAddToInventory

```go
func (gs *GameState) CanAddToInventory(item *Item) bool
```

Checks if an item can be added (size and weight limits).

#### GetInventoryWeight

```go
func (gs *GameState) GetInventoryWeight() float64
```

Returns total weight of items in inventory.

#### GetInventorySortedByWeight

```go
func (gs *GameState) GetInventorySortedByWeight() []*Item
```

Returns inventory sorted by item weight (ascending).

#### SetMaxCarryWeight / SetMaxInventorySize

```go
func (gs *GameState) SetMaxCarryWeight(weight float64)
func (gs *GameState) SetMaxInventorySize(size int)
```

Sets inventory constraints.

#### MarkRoomVisited / IsRoomVisited

```go
func (gs *GameState) MarkRoomVisited(roomID string)
func (gs *GameState) IsRoomVisited(roomID string) bool
```

Track which rooms the player has visited.

#### MarkItemExamined / IsItemExamined

```go
func (gs *GameState) MarkItemExamined(itemID string)
func (gs *GameState) IsItemExamined(itemID string) bool
```

Track which items have been examined.

#### MarkItemsCombined / AreItemsCombined

```go
func (gs *GameState) MarkItemsCombined(item1, item2 string)
func (gs *GameState) AreItemsCombined(item1, item2 string) bool
```

Track item combinations.

#### CreateSnapshot / RestoreSnapshot

```go
func (gs *GameState) CreateSnapshot() *GameState
func (gs *GameState) RestoreSnapshot(snapshot *GameState)
```

Create and restore full game state snapshots.

### Parser Functions

#### NewParser

```go
func NewParser() *Parser
```

Creates a new command parser with default aliases.

#### Parse

```go
func (p *Parser) Parse(input string) (*Command, error)
```

Parses a string input into a Command. Returns error if input is invalid.

#### ParseWithContext

```go
func (p *Parser) ParseWithContext(input string, ctx *ParseContext) (*Command, error)
```

Parses input with additional game context for better matching.

#### FuzzyMatch

```go
func FuzzyMatch(input string, candidates []string, maxDistance int) (string, int)
```

Finds best matching string from candidates using Levenshtein distance.

#### LevenshteinDistance

```go
func LevenshteinDistance(a, b string) int
```

Calculates edit distance between two strings.

### Game Functions

#### NewGame

```go
func NewGame(world *World, state *GameState, parser *Parser) *Game
```

Creates a new game instance.

#### Start

```go
func (g *Game) Start() error
```

Initializes the game and triggers game_start event.

#### ExecuteCommand

```go
func (g *Game) ExecuteCommand(input string) error
```

Parses and executes a player command.

#### GetCurrentRoomDescription

```go
func (g *Game) GetCurrentRoomDescription() string
```

Returns description of current room.

#### GetCurrentRoomName

```go
func (g *Game) GetCurrentRoomName() string
```

Returns name of current room.

#### GetCurrentRoomExits

```go
func (g *Game) GetCurrentRoomExits() []string
```

Returns available exits from current room.

#### GetCurrentRoomItems

```go
func (g *Game) GetCurrentRoomItems() []*Item
```

Returns items in current room.

#### GetCurrentRoomNPCs

```go
func (g *Game) GetCurrentRoomNPCs() []*NPC
```

Returns NPCs in current room.

#### GetInventoryDisplay

```go
func (g *Game) GetInventoryDisplay() string
```

Returns formatted string of inventory.

#### IsGameOver / IsVictory

```go
func (g *Game) IsGameOver() bool
func (g *Game) IsVictory() bool
```

Check game status.

#### SetVictory / SetGameOver

```go
func (g *Game) SetVictory()
func (g *Game) SetGameOver()
```

Update game status.

### EventHandler Functions

#### NewEventHandler

```go
func NewEventHandler() *EventHandler
```

Creates a new event handler.

#### Subscribe

```go
func (eh *EventHandler) Subscribe(eventType string, handler func(*Event))
```

Registers a handler for an event type.

#### Emit

```go
func (eh *EventHandler) Emit(event *Event)
```

Publishes an event to all subscribers.

## Package: gameformat

### Functions

#### LoadGameFromFile

```go
func LoadGameFromFile(filename string) (*engine.Game, error)
```

Loads and initializes a game from a JSON file.

#### SaveGameToFile

```go
func SaveGameToFile(game *engine.Game, filename string) error
```

Saves game state to a file.

#### LoadGameFromSave

```go
func LoadGameFromSave(gameFile, saveFile string) (*engine.Game, error)
```

Loads a game and restores a saved state.

#### ValidateGameDefinition

```go
func ValidateGameDefinition(def GameDefinition) []string
```

Validates a game definition. Returns list of errors if any.

## Error Types

```go
// Room errors
ErrNilRoom         = errors.New("room cannot be nil")
ErrInvalidRoomID   = errors.New("room ID cannot be empty")
ErrDuplicateRoom   = errors.New("room with this ID already exists")
ErrRoomNotFound    = errors.New("room not found")
ErrInvalidExit     = errors.New("exit does not exist in this direction")

// Item errors
ErrNilItem         = errors.New("item cannot be nil")
ErrInvalidItemID   = errors.New("item ID cannot be empty")
ErrDuplicateItem   = errors.New("item with this ID already exists")
ErrItemNotFound    = errors.New("item not found")
ErrItemNotTakeable = errors.New("item cannot be taken")
ErrInventoryFull   = errors.New("inventory is full")
ErrCarryWeightExceeded = errors.New("carrying too much weight")

// NPC errors
ErrNPCNotFound     = errors.New("NPC not found")

// Command errors
ErrUnknownCommand  = errors.New("unknown command")
ErrGameOver        = errors.New("game is over")
```

## Usage Examples

### Creating a Game Programmatically

```go
package main

import (
    "fmt"
    "textadventure/engine"
)

func main() {
    // Create world
    world := engine.NewWorld()

    // Create and add rooms
    room1 := &engine.Room{
        ID:   "start",
        Name: "Starting Room",
        Description: "You are in a room.",
        Exits: map[string]string{"north": "room2"},
        Items: make([]*engine.Item, 0),
        NPCs: make([]*engine.NPC, 0),
        State: make(map[string]interface{}),
    }
    world.AddRoom(room1)

    // Create items
    key := &engine.Item{
        ID:       "key",
        Name:     "Key",
        Takeable: true,
        Weight:   0.1,
        State:    make(map[string]interface{}),
    }
    world.AddItem(key)
    room1.Items = append(room1.Items, key)

    // Create game state
    state := engine.NewGameState("start")

    // Create parser
    parser := engine.NewParser()

    // Create and start game
    game := engine.NewGame(world, state, parser)
    game.Start()

    // Execute commands
    game.ExecuteCommand("take key")
    fmt.Println(game.GetInventoryDisplay())
}
```

### Event Handling

```go
// Subscribe to events
game.EventHandler.Subscribe("item_taken", func(event *engine.Event) {
    fmt.Printf("Item %s was taken\n", event.Target)
})

game.EventHandler.Subscribe("room_entered", func(event *engine.Event) {
    fmt.Printf("Entered room %s\n", event.Target)
})

// Emit custom event
game.EventHandler.Emit(&engine.Event{
    Type:   "custom_puzzle_solved",
    Target: "puzzle_1",
})
```

### Extending the Game

```go
// Add custom command handler (extend ExecuteCommand)
func customCommand(game *engine.Game, cmd *engine.Command) error {
    if cmd.Action == "meditate" {
        game.State.IncrementScore(10)
        return nil
    }
    return nil
}
```

## Testing

Example test structure:

```go
func TestGameInitialization(t *testing.T) {
    world := engine.NewWorld()
    state := engine.NewGameState("start")
    parser := engine.NewParser()

    game := engine.NewGame(world, state, parser)

    if game.State.CurrentRoom != "start" {
        t.Error("game state not initialized correctly")
    }
}
```

## Best Practices

1. **Always check for nil**: Game functions can return nil for not found
2. **Handle errors**: Check error returns from AddRoom, AddItem, etc.
3. **Use type assertions**: For GetVariable, use GetVariableAsXxx helpers
4. **Subscribe to events**: Use EventHandler for loose coupling
5. **Validate world**: Call world.ValidateWorld() before starting
6. **Create snapshots**: For save functionality, use CreateSnapshot()
7. **Clean up**: Remove items and NPCs properly when needed

## Performance Considerations

- Rooms: O(1) lookup by ID
- Items: O(1) lookup in world, O(n) in room/inventory
- Inventory: Limited size (default 20) for performance
- Events: O(k) where k is number of subscribers
- Parser: O(1) for exact matches, O(n) for fuzzy matching

---

**For complete examples, see the tests and sample game!**
