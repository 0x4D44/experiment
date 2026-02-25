package engine

import (
	"fmt"
	"strings"
)

// NewGame creates a new game instance
func NewGame(world *World, state *GameState, parser *Parser) *Game {
	return &Game{
		World:        world,
		State:        state,
		Parser:       parser,
		EventHandler: NewEventHandler(),
		Scripts:      make(map[string]*Script),
		TurnCounter:  0,
		GameOver:     false,
		Victory:      false,
	}
}

// Start initializes the game
func (g *Game) Start() error {
	if g.World == nil || g.State == nil || g.Parser == nil {
		return ErrInvalidGameState
	}

	// Mark starting room as visited
	g.State.MarkRoomVisited(g.State.CurrentRoom)

	// Trigger game start event
	g.EventHandler.Emit(&Event{
		Type:      "game_start",
		Timestamp: 0,
	})

	return nil
}

// ExecuteCommand parses and executes a player command
func (g *Game) ExecuteCommand(input string) error {
	if g.GameOver {
		return ErrGameOver
	}

	// Parse the command
	cmd, err := g.Parser.Parse(input)
	if err != nil {
		return err
	}

	// Execute the command
	return g.executeAction(cmd)
}

// executeAction performs the action specified by a command
func (g *Game) executeAction(cmd *Command) error {
	switch cmd.Action {
	case "north", "south", "east", "west", "up", "down":
		return g.moveRoom(cmd.Object)

	case "look":
		return nil // Output handled by caller

	case "examine":
		return g.examine(cmd.Object)

	case "take":
		return g.takeItem(cmd.Object)

	case "drop":
		return g.dropItem(cmd.Object)

	case "inventory":
		return nil // Output handled by caller

	case "use":
		return g.useItem(cmd.Object, cmd.Target)

	case "put":
		return g.putItem(cmd.Object, cmd.Target)

	case "talk":
		return g.talkToNPC(cmd.Object)

	case "open":
		return g.openContainer(cmd.Object)

	case "close":
		return g.closeContainer(cmd.Object)

	case "save":
		return nil // Handled separately

	case "load":
		return nil // Handled separately

	case "wait":
		g.TurnCounter++
		g.State.IncrementTurnCount()
		return nil

	case "quit":
		g.GameOver = true
		return nil

	case "help":
		return nil // Output handled by caller

	default:
		return ErrUnknownCommand
	}
}

// moveRoom moves the player to an adjacent room
func (g *Game) moveRoom(direction string) error {
	currentRoom := g.World.GetRoom(g.State.CurrentRoom)
	if currentRoom == nil {
		return ErrRoomNotFound
	}

	// Check if exit exists
	nextRoomID, ok := currentRoom.Exits[direction]
	if !ok {
		return ErrInvalidExit
	}

	// Check if next room exists
	nextRoom := g.World.GetRoom(nextRoomID)
	if nextRoom == nil {
		return ErrRoomNotFound
	}

	// Check if room is locked
	if nextRoom.Locked {
		return errors.New("the " + direction + " passage is locked")
	}

	// Move to new room
	g.State.CurrentRoom = nextRoomID
	g.State.MarkRoomVisited(nextRoomID)

	// Emit room entered event
	g.EventHandler.Emit(&Event{
		Type:      "room_entered",
		Target:    nextRoomID,
		Timestamp: int64(g.TurnCounter),
	})

	g.TurnCounter++
	return nil
}

// examine shows detailed information about an item
func (g *Game) examine(itemID string) error {
	currentRoom := g.World.GetRoom(g.State.CurrentRoom)
	if currentRoom == nil {
		return ErrRoomNotFound
	}

	// Find item in room or inventory
	var item *Item

	// Check inventory first
	item = g.State.GetItem(itemID)

	// If not in inventory, check room
	if item == nil {
		for _, roomItem := range currentRoom.Items {
			if roomItem.ID == itemID || strings.EqualFold(roomItem.Name, itemID) {
				item = roomItem
				break
			}
		}
	}

	if item == nil {
		return ErrItemNotFound
	}

	// Mark as examined
	g.State.MarkItemExamined(item.ID)

	// Emit item examined event
	g.EventHandler.Emit(&Event{
		Type:      "item_examined",
		Target:    item.ID,
		Timestamp: int64(g.TurnCounter),
	})

	g.TurnCounter++
	return nil
}

// takeItem adds an item to the player's inventory
func (g *Game) takeItem(itemID string) error {
	currentRoom := g.World.GetRoom(g.State.CurrentRoom)
	if currentRoom == nil {
		return ErrRoomNotFound
	}

	// Find item in room
	var item *Item
	var itemIndex int

	for i, roomItem := range currentRoom.Items {
		if roomItem.ID == itemID || strings.EqualFold(roomItem.Name, itemID) {
			item = roomItem
			itemIndex = i
			break
		}
	}

	if item == nil {
		return ErrItemNotFound
	}

	// Check if item is takeable
	if !item.Takeable {
		return ErrItemNotTakeable
	}

	// Check if can add to inventory
	if !g.State.CanAddToInventory(item) {
		if g.State.GetInventoryWeight()+item.Weight > g.State.MaxCarryWeight {
			return ErrCarryWeightExceeded
		}
		return ErrInventoryFull
	}

	// Add to inventory
	g.State.AddToInventory(item)

	// Remove from room
	currentRoom.Items = append(currentRoom.Items[:itemIndex], currentRoom.Items[itemIndex+1:]...)

	// Emit item taken event
	g.EventHandler.Emit(&Event{
		Type:      "item_taken",
		Target:    item.ID,
		Timestamp: int64(g.TurnCounter),
	})

	g.TurnCounter++
	return nil
}

// dropItem removes an item from inventory and places it in the current room
func (g *Game) dropItem(itemID string) error {
	// Check if item is in inventory
	item := g.State.GetItem(itemID)
	if item == nil {
		return ErrItemNotFound
	}

	// Check if item is cursed (cannot drop)
	if item.Cursed {
		return errors.New("the item is cursed and cannot be dropped")
	}

	// Remove from inventory
	g.State.RemoveFromInventory(itemID)

	// Add to current room
	currentRoom := g.World.GetRoom(g.State.CurrentRoom)
	if currentRoom != nil {
		currentRoom.Items = append(currentRoom.Items, item)
	}

	// Emit item dropped event
	g.EventHandler.Emit(&Event{
		Type:      "item_dropped",
		Target:    itemID,
		Timestamp: int64(g.TurnCounter),
	})

	g.TurnCounter++
	return nil
}

// useItem uses an item on a target
func (g *Game) useItem(itemID, targetID string) error {
	// Check if item is in inventory
	item := g.State.GetItem(itemID)
	if item == nil {
		return ErrItemNotFound
	}

	if !item.Useable {
		return ErrItemNotUseable
	}

	// If target specified, check if it exists
	if targetID != "" {
		// Could be another item or part of the environment
		_ = targetID // TODO: Implement target interaction
	}

	// Emit item used event
	g.EventHandler.Emit(&Event{
		Type:      "item_used",
		Target:    itemID,
		Timestamp: int64(g.TurnCounter),
		Data: map[string]interface{}{
			"target": targetID,
		},
	})

	g.TurnCounter++
	return nil
}

// putItem places an item into a container
func (g *Game) putItem(itemID, containerID string) error {
	// Check if item is in inventory
	item := g.State.GetItem(itemID)
	if item == nil {
		return ErrItemNotFound
	}

	// Find container (could be in room or inventory)
	var container *Item

	// Check inventory
	container = g.State.GetItem(containerID)

	// If not in inventory, check room
	if container == nil {
		currentRoom := g.World.GetRoom(g.State.CurrentRoom)
		for _, roomItem := range currentRoom.Items {
			if roomItem.ID == containerID {
				container = roomItem
				break
			}
		}
	}

	if container == nil {
		return ErrItemNotFound
	}

	// Check if container can hold items
	if !container.Container {
		return errors.New("this item cannot hold other items")
	}

	// Check if container is locked
	if container.Locked {
		return errors.New("the container is locked")
	}

	// Add item to container
	container.Contents = append(container.Contents, item)

	// Remove from inventory
	g.State.RemoveFromInventory(itemID)

	g.TurnCounter++
	return nil
}

// openContainer opens a closed container
func (g *Game) openContainer(containerID string) error {
	// Find container
	var container *Item

	container = g.State.GetItem(containerID)

	if container == nil {
		currentRoom := g.World.GetRoom(g.State.CurrentRoom)
		if currentRoom != nil {
			for _, item := range currentRoom.Items {
				if item.ID == containerID {
					container = item
					break
				}
			}
		}
	}

	if container == nil {
		return ErrItemNotFound
	}

	if !container.Container {
		return errors.New("this item is not a container")
	}

	if container.Open {
		return errors.New("the container is already open")
	}

	container.Open = true
	g.TurnCounter++
	return nil
}

// closeContainer closes an open container
func (g *Game) closeContainer(containerID string) error {
	var container *Item

	container = g.State.GetItem(containerID)

	if container == nil {
		currentRoom := g.World.GetRoom(g.State.CurrentRoom)
		if currentRoom != nil {
			for _, item := range currentRoom.Items {
				if item.ID == containerID {
					container = item
					break
				}
			}
		}
	}

	if container == nil {
		return ErrItemNotFound
	}

	if !container.Container {
		return errors.New("this item is not a container")
	}

	if !container.Open {
		return errors.New("the container is already closed")
	}

	container.Open = false
	g.TurnCounter++
	return nil
}

// talkToNPC initiates a conversation with an NPC
func (g *Game) talkToNPC(npcID string) error {
	currentRoom := g.World.GetRoom(g.State.CurrentRoom)
	if currentRoom == nil {
		return ErrRoomNotFound
	}

	// Find NPC in room
	var npc *NPC
	for _, roomNPC := range currentRoom.NPCs {
		if roomNPC.ID == npcID || strings.EqualFold(roomNPC.Name, npcID) {
			npc = roomNPC
			break
		}
	}

	if npc == nil {
		return ErrNPCNotFound
	}

	// Emit NPC talked to event
	g.EventHandler.Emit(&Event{
		Type:      "npc_talked_to",
		Target:    npc.ID,
		Timestamp: int64(g.TurnCounter),
	})

	g.TurnCounter++
	return nil
}

// GetCurrentRoomDescription returns the description of the current room
func (g *Game) GetCurrentRoomDescription() string {
	room := g.World.GetRoom(g.State.CurrentRoom)
	if room == nil {
		return "Unknown location."
	}

	return g.World.GetRoomDescription(g.State.CurrentRoom, g.State)
}

// GetCurrentRoomName returns the name of the current room
func (g *Game) GetCurrentRoomName() string {
	room := g.World.GetRoom(g.State.CurrentRoom)
	if room == nil {
		return "Unknown"
	}
	return room.Name
}

// GetCurrentRoomExits returns available exits from the current room
func (g *Game) GetCurrentRoomExits() []string {
	return g.World.GetAvailableExits(g.State.CurrentRoom)
}

// GetCurrentRoomItems returns items in the current room
func (g *Game) GetCurrentRoomItems() []*Item {
	return g.World.GetRoomItems(g.State.CurrentRoom)
}

// GetCurrentRoomNPCs returns NPCs in the current room
func (g *Game) GetCurrentRoomNPCs() []*NPC {
	return g.World.GetRoomNPCs(g.State.CurrentRoom)
}

// GetInventoryDisplay returns a formatted string of the inventory
func (g *Game) GetInventoryDisplay() string {
	inventory := g.State.GetInventory()
	if len(inventory) == 0 {
		return "You are carrying nothing."
	}

	var result strings.Builder
	result.WriteString("You are carrying:\n")
	for _, item := range inventory {
		result.WriteString(fmt.Sprintf("  - %s\n", item.Name))
	}
	return result.String()
}

// IsGameOver checks if the game has ended
func (g *Game) IsGameOver() bool {
	return g.GameOver
}

// IsVictory checks if the player has won
func (g *Game) IsVictory() bool {
	return g.Victory
}

// SetVictory marks the game as won
func (g *Game) SetVictory() {
	g.Victory = true
	g.GameOver = true
}

// SetGameOver marks the game as over
func (g *Game) SetGameOver() {
	g.GameOver = true
}

// NewEventHandler creates a new event handler
func NewEventHandler() *EventHandler {
	return &EventHandler{
		Subscribers: make(map[string][]func(*Event)),
	}
}

// Subscribe registers a handler for an event type
func (eh *EventHandler) Subscribe(eventType string, handler func(*Event)) {
	eh.Subscribers[eventType] = append(eh.Subscribers[eventType], handler)
}

// Emit publishes an event to all subscribers
func (eh *EventHandler) Emit(event *Event) {
	if handlers, ok := eh.Subscribers[event.Type]; ok {
		for _, handler := range handlers {
			handler(event)
		}
	}
}

// import needed for error handling
import "errors"
