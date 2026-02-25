package engine

// Command represents a parsed player command
type Command struct {
	Action string   // verb: "take", "drop", "examine", etc.
	Object string   // first noun: object of action
	Target string   // second noun: target of action (optional)
	Args   []string // additional arguments
	Raw    string   // original input
}

// ParseContext provides context for command parsing
type ParseContext struct {
	CurrentRoom      *Room
	AvailableItems   []string
	AvailableNPCs    []string
	AvailableExits   []string
}

// Room represents a location in the game world
type Room struct {
	ID              string              // unique identifier
	Name            string              // display name
	BaseDescription string              // static description
	DescriptionFn   func(*GameState) string // dynamic description function
	Exits           map[string]string   // direction -> room ID
	Items           []*Item             // items in this room
	NPCs            []*NPC              // NPCs in this room
	Visited         bool                // has player been here
	Locked          bool                // is room locked
	LockItem        string              // item ID that unlocks room
	State           map[string]interface{} // room-specific state
}

// Item represents an object in the game world
type Item struct {
	ID          string        // unique identifier
	Name        string        // display name
	Description string        // detailed description
	Takeable    bool          // can player take it
	Useable     bool          // can player use it
	Container   bool          // can hold other items
	Locked      bool          // is container locked
	LockItem    string        // item ID that unlocks it
	Open        bool          // is container open
	Hidden      bool          // is item hidden (until examined)
	Weight      float64       // weight in pounds
	Size        float64       // size in cubic units
	Cursed      bool          // cannot be dropped if cursed
	Stackable   bool          // can items stack
	Quantity    int           // for stackable items
	Contents    []*Item       // items inside container
	Examined    bool          // has player examined it
	State       map[string]interface{} // item-specific state
}

// NPC represents a non-player character
type NPC struct {
	ID          string              // unique identifier
	Name        string              // display name
	Description string              // physical description
	Dialogue    *DialogueTree       // dialogue options
	Inventory   []*Item             // NPC's items
	Location    string              // current room ID
	State       map[string]interface{} // NPC-specific state
	Friendly    bool                // is NPC friendly to player
	CanTrade    bool                // can NPC trade items
}

// DialogueTree represents NPC conversation structure
type DialogueTree struct {
	ID       string            // unique dialogue ID
	NPC      string            // NPC ID this belongs to
	Text     string            // what NPC says
	Options  []*DialogueOption // player response options
}

// DialogueOption represents a dialogue choice
type DialogueOption struct {
	ID        string        // unique option ID
	Text      string        // what player can say
	Condition func(*GameState) bool // condition for showing option
	Response  string        // NPC's response to this option
	Next      *DialogueTree // next dialogue after this option
	Action    func(*GameState) // callback when option is selected
}

// World represents the entire game world
type World struct {
	Rooms map[string]*Room
	Items map[string]*Item
	NPCs  map[string]*NPC
}

// GameState tracks the current state of the game
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
	ItemLocations       map[string]string // item ID -> location (room or inventory)
	RoomStateSnapshots  map[string]map[string]interface{} // room state backups
	ItemStateSnapshots  map[string]map[string]interface{} // item state backups
}

// Script represents game logic
type Script struct {
	ID        string      // unique script ID
	Type      string      // "room", "item", "npc", "global"
	Target    string      // what this script applies to
	Trigger   string      // when to execute (on_enter, on_take, etc.)
	Condition func(*GameState) bool
	Action    func(*GameState) error
}

// Event represents a game event
type Event struct {
	Type      string      // event type (room_entered, item_taken, etc.)
	Target    string      // what triggered the event
	Timestamp int64       // when it occurred
	Data      map[string]interface{} // event-specific data
}

// EventHandler handles game events
type EventHandler struct {
	Subscribers map[string][]func(*Event) // event type -> handlers
}

// SaveGame represents a saved game file
type SaveGame struct {
	Version            string                 // save format version
	Timestamp          int64                  // when saved
	GameFile           string                 // which game definition
	PlayerLocation     string                 // current room
	Inventory          []*Item                // inventory items
	Variables          map[string]interface{} // game variables
	RoomStates         map[string]map[string]interface{}
	ItemStates         map[string]map[string]interface{}
	RoomsVisited       map[string]bool
	ItemsExamined      map[string]bool
}

// Parser handles command parsing
type Parser struct {
	Aliases map[string]string // command aliases
}

// Game represents the main game engine
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
