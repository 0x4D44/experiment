package tests

import (
	"testing"
	"textadventure/engine"
)

func TestCommandMovement(t *testing.T) {
	game := setupTestGame()

	// Move north
	err := game.ExecuteCommand("north")
	if err != nil {
		t.Errorf("movement failed: %v", err)
	}

	if game.State.CurrentRoom != "corridor" {
		t.Errorf("current room: got %q, want %q", game.State.CurrentRoom, "corridor")
	}
}

func TestCommandTakeItem(t *testing.T) {
	game := setupTestGame()

	// In starting room there should be items
	err := game.ExecuteCommand("take key")
	if err != nil {
		t.Errorf("take command failed: %v", err)
	}

	if !game.State.HasItem("key") {
		t.Error("key should be in inventory")
	}
}

func TestCommandDropItem(t *testing.T) {
	game := setupTestGame()

	// First take an item
	game.ExecuteCommand("take key")

	// Now drop it
	err := game.ExecuteCommand("drop key")
	if err != nil {
		t.Errorf("drop command failed: %v", err)
	}

	if game.State.HasItem("key") {
		t.Error("key should not be in inventory after drop")
	}
}

func TestCommandExamine(t *testing.T) {
	game := setupTestGame()

	err := game.ExecuteCommand("examine key")
	if err != nil {
		t.Errorf("examine failed: %v", err)
	}

	if !game.State.IsItemExamined("key") {
		t.Error("key should be marked as examined")
	}
}

func TestCommandInventory(t *testing.T) {
	game := setupTestGame()

	game.ExecuteCommand("take key")
	game.ExecuteCommand("take sword")

	// Get inventory display
	inventory := game.GetInventoryDisplay()
	if inventory == "" {
		t.Error("inventory display should not be empty")
	}

	if !game.State.HasItem("key") || !game.State.HasItem("sword") {
		t.Error("items should be in inventory")
	}
}

func TestCommandLook(t *testing.T) {
	game := setupTestGame()

	description := game.GetCurrentRoomDescription()
	if description == "" {
		t.Error("room description should not be empty")
	}
}

func TestCommandInvalidAction(t *testing.T) {
	game := setupTestGame()

	err := game.ExecuteCommand("pickpocket guard")
	if err == nil {
		t.Error("invalid command should return error")
	}
}

func TestCommandSequence(t *testing.T) {
	game := setupTestGame()

	// Sequence of commands
	commands := []string{
		"look",
		"examine key",
		"take key",
		"inventory",
		"north",
		"look",
		"south",
	}

	for _, cmd := range commands {
		err := game.ExecuteCommand(cmd)
		if err != nil {
			t.Errorf("command %q failed: %v", cmd, err)
		}
	}
}

func TestInvalidMovement(t *testing.T) {
	game := setupTestGame()

	// Try to go east (no exit)
	err := game.ExecuteCommand("east")
	if err == nil {
		t.Error("invalid movement should return error")
	}
}

func TestUseItemOn(t *testing.T) {
	game := setupTestGame()

	// Take key first
	game.ExecuteCommand("take key")

	// Try to use key on door
	err := game.ExecuteCommand("use key on door")
	if err != nil {
		t.Logf("use command result: %v", err)
	}
}

func TestWaitCommand(t *testing.T) {
	game := setupTestGame()
	initialTurn := game.TurnCounter

	err := game.ExecuteCommand("wait")
	if err != nil {
		t.Errorf("wait command failed: %v", err)
	}

	if game.TurnCounter <= initialTurn {
		t.Error("turn counter should increase")
	}
}

func TestHelpCommand(t *testing.T) {
	game := setupTestGame()

	err := game.ExecuteCommand("help")
	if err != nil {
		t.Errorf("help command failed: %v", err)
	}
}

func TestMultipleMovements(t *testing.T) {
	game := setupTestGame()

	// Start at cell
	if game.State.CurrentRoom != "cell" {
		t.Errorf("starting room: got %q, want %q", game.State.CurrentRoom, "cell")
	}

	// Go north to corridor
	game.ExecuteCommand("north")
	if game.State.CurrentRoom != "corridor" {
		t.Error("should be in corridor")
	}

	// Go back south to cell
	game.ExecuteCommand("south")
	if game.State.CurrentRoom != "cell" {
		t.Error("should be back in cell")
	}
}

func TestNoExitInDirection(t *testing.T) {
	game := setupTestGame()

	// From starting room, east doesn't exist
	err := game.ExecuteCommand("east")
	if err == nil {
		t.Error("should fail when no exit in direction")
	}
}

func TestTakeNonExistentItem(t *testing.T) {
	game := setupTestGame()

	err := game.ExecuteCommand("take unicorn")
	if err == nil {
		t.Error("should fail when taking non-existent item")
	}
}

func TestDropNonExistentItem(t *testing.T) {
	game := setupTestGame()

	err := game.ExecuteCommand("drop unicorn")
	if err == nil {
		t.Error("should fail when dropping non-existent item")
	}
}

func TestTakeUnliftableItem(t *testing.T) {
	game := setupTestGame()

	err := game.ExecuteCommand("take wall")
	if err == nil {
		t.Error("should fail when taking non-takeable item")
	}
}

func TestCommandCase(t *testing.T) {
	game := setupTestGame()

	tests := []string{
		"north",
		"NORTH",
		"North",
		"NoRtH",
	}

	for _, cmd := range tests {
		game.State.CurrentRoom = "cell"
		err := game.ExecuteCommand(cmd)
		if err != nil {
			t.Errorf("case variant %q failed: %v", cmd, err)
		}
		if game.State.CurrentRoom != "corridor" {
			t.Errorf("movement with %q failed", cmd)
		}
	}
}

func TestLookVariants(t *testing.T) {
	game := setupTestGame()

	variants := []string{"look", "l", "Look", "L"}

	for _, variant := range variants {
		err := game.ExecuteCommand(variant)
		if err != nil {
			t.Errorf("look variant %q failed: %v", variant, err)
		}
	}
}

func TestExamineVariants(t *testing.T) {
	game := setupTestGame()

	tests := []struct {
		command string
		object  string
	}{
		{"examine key", "key"},
		{"x key", "key"},
		{"inspect key", "key"},
		{"look at key", "key"},
	}

	for _, tt := range tests {
		err := game.ExecuteCommand(tt.command)
		if err != nil {
			t.Errorf("examine variant %q failed: %v", tt.command, err)
		}
	}
}

// Helper function to set up a test game
func setupTestGame() *engine.Game {
	world := engine.NewWorld()

	// Create test rooms
	cell := &engine.Room{
		ID:          "cell",
		Name:        "Dungeon Cell",
		Description: "A cold, dark cell",
		Exits:       map[string]string{"north": "corridor"},
		Items: []*engine.Item{
			{ID: "key", Name: "Iron Key", Takeable: true, Weight: 0.1},
			{ID: "sword", Name: "Rusty Sword", Takeable: true, Weight: 2.0},
		},
	}

	corridor := &engine.Room{
		ID:          "corridor",
		Name:        "Dungeon Corridor",
		Description: "A long, dark corridor",
		Exits: map[string]string{
			"south": "cell",
			"north": "guardroom",
		},
		Items: []*engine.Item{
			{ID: "torch", Name: "Burning Torch", Takeable: true, Weight: 0.5},
		},
	}

	guardroom := &engine.Room{
		ID:          "guardroom",
		Name:        "Guard Room",
		Description: "A room with a guard",
		Exits:       map[string]string{"south": "corridor"},
	}

	world.AddRoom(cell)
	world.AddRoom(corridor)
	world.AddRoom(guardroom)

	// Add items to world
	for _, room := range []*engine.Room{cell, corridor, guardroom} {
		for _, item := range room.Items {
			world.AddItem(item)
		}
	}

	state := engine.NewGameState("cell")
	parser := engine.NewParser()
	game := engine.NewGame(world, state, parser)

	return game
}
