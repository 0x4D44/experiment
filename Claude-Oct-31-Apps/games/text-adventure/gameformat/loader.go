package gameformat

import (
	"encoding/json"
	"errors"
	"io/ioutil"
	"os"
	"textadventure/engine"
)

// GameDefinition represents the structure of a game file
type GameDefinition struct {
	Title         string                 `json:"title"`
	Description   string                 `json:"description"`
	Version       string                 `json:"version"`
	Author        string                 `json:"author"`
	StartingRoom  string                 `json:"startingRoom"`
	Rooms         map[string]RoomDef     `json:"rooms"`
	Items         map[string]ItemDef     `json:"items"`
	NPCs          map[string]NPCDef      `json:"npcs"`
	Variables     map[string]interface{} `json:"variables"`
	MaxInventory  int                    `json:"maxInventory"`
	MaxCarryWeight float64               `json:"maxCarryWeight"`
}

// RoomDef represents a room in the game definition
type RoomDef struct {
	Name        string            `json:"name"`
	Description string            `json:"description"`
	Exits       map[string]string `json:"exits"`
	Items       []string          `json:"items"`
	NPCs        []string          `json:"npcs"`
	Locked      bool              `json:"locked"`
	LockItem    string            `json:"lockItem"`
}

// ItemDef represents an item in the game definition
type ItemDef struct {
	Name        string  `json:"name"`
	Description string  `json:"description"`
	Takeable    bool    `json:"takeable"`
	Useable     bool    `json:"useable"`
	Container   bool    `json:"container"`
	Locked      bool    `json:"locked"`
	LockItem    string  `json:"lockItem"`
	Hidden      bool    `json:"hidden"`
	Weight      float64 `json:"weight"`
	Size        float64 `json:"size"`
	Cursed      bool    `json:"cursed"`
	Stackable   bool    `json:"stackable"`
	Quantity    int     `json:"quantity"`
}

// NPCDef represents an NPC in the game definition
type NPCDef struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	Location    string `json:"location"`
	Friendly    bool   `json:"friendly"`
	CanTrade    bool   `json:"canTrade"`
}

// LoadGameFromFile loads a game from a JSON file
func LoadGameFromFile(filename string) (*engine.Game, error) {
	// Check if file exists
	if _, err := os.Stat(filename); err != nil {
		return nil, errors.New("game file not found: " + filename)
	}

	// Read file
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	// Parse JSON
	var gameDef GameDefinition
	if err := json.Unmarshal(data, &gameDef); err != nil {
		return nil, errors.New("invalid game file format: " + err.Error())
	}

	// Build world from definition
	world := engine.NewWorld()

	// Create all items first
	itemMap := make(map[string]*engine.Item)
	for itemID, itemDef := range gameDef.Items {
		item := &engine.Item{
			ID:          itemID,
			Name:        itemDef.Name,
			Description: itemDef.Description,
			Takeable:    itemDef.Takeable,
			Useable:     itemDef.Useable,
			Container:   itemDef.Container,
			Locked:      itemDef.Locked,
			LockItem:    itemDef.LockItem,
			Hidden:      itemDef.Hidden,
			Weight:      itemDef.Weight,
			Size:        itemDef.Size,
			Cursed:      itemDef.Cursed,
			Stackable:   itemDef.Stackable,
			Quantity:    itemDef.Quantity,
			Contents:    make([]*engine.Item, 0),
			State:       make(map[string]interface{}),
		}
		itemMap[itemID] = item
		world.AddItem(item)
	}

	// Create NPCs
	npcMap := make(map[string]*engine.NPC)
	for npcID, npcDef := range gameDef.NPCs {
		npc := &engine.NPC{
			ID:          npcID,
			Name:        npcDef.Name,
			Description: npcDef.Description,
			Location:    npcDef.Location,
			Friendly:    npcDef.Friendly,
			CanTrade:    npcDef.CanTrade,
			Inventory:   make([]*engine.Item, 0),
			State:       make(map[string]interface{}),
		}
		npcMap[npcID] = npc
		world.AddNPC(npc)
	}

	// Create all rooms
	for roomID, roomDef := range gameDef.Rooms {
		room := &engine.Room{
			ID:              roomID,
			Name:            roomDef.Name,
			BaseDescription: roomDef.Description,
			Exits:           roomDef.Exits,
			Items:           make([]*engine.Item, 0),
			NPCs:            make([]*engine.NPC, 0),
			Locked:          roomDef.Locked,
			LockItem:        roomDef.LockItem,
			State:           make(map[string]interface{}),
		}

		// Add items to room
		for _, itemID := range roomDef.Items {
			if item, ok := itemMap[itemID]; ok {
				room.Items = append(room.Items, item)
			}
		}

		// Add NPCs to room
		for _, npcID := range roomDef.NPCs {
			if npc, ok := npcMap[npcID]; ok {
				room.NPCs = append(room.NPCs, npc)
			}
		}

		world.AddRoom(room)
	}

	// Create game state
	startingRoom := gameDef.StartingRoom
	if startingRoom == "" {
		startingRoom = "start"
	}

	state := engine.NewGameState(startingRoom)

	// Set inventory limits
	if gameDef.MaxInventory > 0 {
		state.SetMaxInventorySize(gameDef.MaxInventory)
	}
	if gameDef.MaxCarryWeight > 0 {
		state.SetMaxCarryWeight(gameDef.MaxCarryWeight)
	}

	// Set initial variables
	for key, value := range gameDef.Variables {
		state.SetVariable(key, value)
	}

	// Create parser
	parser := engine.NewParser()

	// Create game
	game := engine.NewGame(world, state, parser)

	return game, nil
}

// SaveGameToFile saves a game state to a file
func SaveGameToFile(game *engine.Game, filename string) error {
	// Create save game structure
	saveGame := engine.SaveGame{
		Version:        "1.0",
		GameFile:       filename,
		PlayerLocation: game.State.CurrentRoom,
		Inventory:      game.State.GetInventory(),
		Variables:      game.State.Variables,
		RoomsVisited:   game.State.RoomsVisited,
		ItemsExamined:  game.State.ItemsExamined,
	}

	// Marshal to JSON
	data, err := json.MarshalIndent(saveGame, "", "  ")
	if err != nil {
		return err
	}

	// Write to file
	if err := ioutil.WriteFile(filename, data, 0644); err != nil {
		return err
	}

	return nil
}

// LoadGameFromSave loads a saved game
func LoadGameFromSave(gameFile, saveFile string) (*engine.Game, error) {
	// Load the game definition
	game, err := LoadGameFromFile(gameFile)
	if err != nil {
		return nil, err
	}

	// Load save file
	data, err := ioutil.ReadFile(saveFile)
	if err != nil {
		return nil, err
	}

	var saveGame engine.SaveGame
	if err := json.Unmarshal(data, &saveGame); err != nil {
		return nil, errors.New("invalid save file format: " + err.Error())
	}

	// Restore game state
	game.State.CurrentRoom = saveGame.PlayerLocation

	// Restore inventory
	game.State.ClearInventory()
	for _, item := range saveGame.Inventory {
		game.State.AddToInventory(item)
	}

	// Restore variables
	for key, value := range saveGame.Variables {
		game.State.SetVariable(key, value)
	}

	// Restore room visits
	game.State.RoomsVisited = saveGame.RoomsVisited

	// Restore item examinations
	game.State.ItemsExamined = saveGame.ItemsExamined

	return game, nil
}

// ValidateGameDefinition checks if a game definition is valid
func ValidateGameDefinition(def GameDefinition) []string {
	errors := []string{}

	// Check required fields
	if def.Title == "" {
		errors = append(errors, "missing title")
	}

	if def.StartingRoom == "" {
		errors = append(errors, "missing startingRoom")
	}

	if len(def.Rooms) == 0 {
		errors = append(errors, "no rooms defined")
	}

	// Check that starting room exists
	if _, ok := def.Rooms[def.StartingRoom]; !ok {
		errors = append(errors, "starting room not defined: "+def.StartingRoom)
	}

	// Check that all room exits point to valid rooms
	for roomID, room := range def.Rooms {
		for direction, targetRoom := range room.Exits {
			if _, ok := def.Rooms[targetRoom]; !ok {
				errors = append(errors, "room "+roomID+" has invalid exit "+direction+" to "+targetRoom)
			}
		}
	}

	// Check that all referenced items exist
	for roomID, room := range def.Rooms {
		for _, itemID := range room.Items {
			if _, ok := def.Items[itemID]; !ok {
				errors = append(errors, "room "+roomID+" references non-existent item "+itemID)
			}
		}
	}

	// Check that all referenced NPCs exist
	for roomID, room := range def.Rooms {
		for _, npcID := range room.NPCs {
			if _, ok := def.NPCs[npcID]; !ok {
				errors = append(errors, "room "+roomID+" references non-existent NPC "+npcID)
			}
		}
	}

	return errors
}
