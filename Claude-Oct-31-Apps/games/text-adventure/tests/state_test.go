package tests

import (
	"testing"
	"textadventure/engine"
)

func TestGameStateCreation(t *testing.T) {
	state := engine.NewGameState("start_room")

	if state.CurrentRoom != "start_room" {
		t.Errorf("current room: got %q, want %q", state.CurrentRoom, "start_room")
	}

	if len(state.GetInventory()) != 0 {
		t.Errorf("initial inventory: got %d items, want 0", len(state.GetInventory()))
	}
}

func TestVariableStorage(t *testing.T) {
	state := engine.NewGameState("room")

	// Test setting and getting variables
	state.SetVariable("key_found", true)
	if val, ok := state.GetVariable("key_found"); !ok || val != true {
		t.Error("failed to store/retrieve boolean variable")
	}

	state.SetVariable("player_gold", 100)
	if val, ok := state.GetVariable("player_gold"); !ok || val != 100 {
		t.Error("failed to store/retrieve numeric variable")
	}

	state.SetVariable("location", "dungeon")
	if val, ok := state.GetVariable("location"); !ok || val != "dungeon" {
		t.Error("failed to store/retrieve string variable")
	}

	// Test non-existent variable
	if _, ok := state.GetVariable("nonexistent"); ok {
		t.Error("should return false for non-existent variable")
	}
}

func TestInventoryManagement(t *testing.T) {
	state := engine.NewGameState("room")

	item1 := &engine.Item{ID: "sword", Name: "Sword", Weight: 2.0}
	item2 := &engine.Item{ID: "key", Name: "Key", Weight: 0.1}

	// Add items
	state.AddToInventory(item1)
	state.AddToInventory(item2)

	inventory := state.GetInventory()
	if len(inventory) != 2 {
		t.Errorf("inventory size: got %d, want 2", len(inventory))
	}

	// Check item presence
	if !state.HasItem("sword") {
		t.Error("inventory should contain sword")
	}

	if !state.HasItem("key") {
		t.Error("inventory should contain key")
	}

	if state.HasItem("shield") {
		t.Error("inventory should not contain shield")
	}

	// Remove item
	state.RemoveFromInventory("sword")
	if state.HasItem("sword") {
		t.Error("sword should be removed from inventory")
	}

	if len(state.GetInventory()) != 1 {
		t.Errorf("inventory size after removal: got %d, want 1", len(state.GetInventory()))
	}
}

func TestInventoryWeightLimit(t *testing.T) {
	state := engine.NewGameState("room")
	state.SetMaxCarryWeight(10.0)

	item1 := &engine.Item{ID: "sword", Name: "Sword", Weight: 8.0}
	item2 := &engine.Item{ID: "key", Name: "Key", Weight: 3.0}

	// Add first item (under limit)
	ok := state.CanAddToInventory(item1)
	if !ok {
		t.Error("should be able to add item under weight limit")
	}
	state.AddToInventory(item1)

	// Try to add second item (would exceed limit)
	ok = state.CanAddToInventory(item2)
	if ok {
		t.Error("should not be able to add item exceeding weight limit")
	}

	// Add lighter item
	item3 := &engine.Item{ID: "paper", Name: "Paper", Weight: 0.5}
	ok = state.CanAddToInventory(item3)
	if !ok {
		t.Error("should be able to add light item")
	}
	state.AddToInventory(item3)

	// Check total weight
	totalWeight := state.GetInventoryWeight()
	expectedWeight := 8.5
	if totalWeight != expectedWeight {
		t.Errorf("total weight: got %f, want %f", totalWeight, expectedWeight)
	}
}

func TestRoomVisitedTracking(t *testing.T) {
	state := engine.NewGameState("start")

	// Check unvisited rooms
	if state.IsRoomVisited("start") {
		t.Error("starting room should not be marked visited initially")
	}

	// Mark room visited
	state.MarkRoomVisited("start")
	if !state.IsRoomVisited("start") {
		t.Error("room should be marked visited")
	}

	// Check different room
	if state.IsRoomVisited("other") {
		t.Error("other room should not be marked visited")
	}
}

func TestItemStateTracking(t *testing.T) {
	state := engine.NewGameState("room")

	item := &engine.Item{ID: "sword", Name: "Sword"}
	state.AddToInventory(item)

	// Mark item as examined
	state.MarkItemExamined("sword")
	if !state.IsItemExamined("sword") {
		t.Error("item should be marked examined")
	}

	// Non-existent item
	if state.IsItemExamined("shield") {
		t.Error("non-existent item should not be marked examined")
	}
}

func TestGameStateMovement(t *testing.T) {
	state := engine.NewGameState("cell")

	if state.CurrentRoom != "cell" {
		t.Errorf("initial room: got %q, want %q", state.CurrentRoom, "cell")
	}

	// Move to new room
	state.CurrentRoom = "corridor"
	if state.CurrentRoom != "corridor" {
		t.Errorf("current room: got %q, want %q", state.CurrentRoom, "corridor")
	}
}

func TestGameVariableTypes(t *testing.T) {
	state := engine.NewGameState("room")

	// Test different variable types
	testCases := []struct {
		name     string
		value    interface{}
		expected interface{}
	}{
		{"bool_var", true, true},
		{"int_var", 42, 42},
		{"float_var", 3.14, 3.14},
		{"string_var", "hello", "hello"},
	}

	for _, tc := range testCases {
		state.SetVariable(tc.name, tc.value)
		retrieved, _ := state.GetVariable(tc.name)
		if retrieved != tc.expected {
			t.Errorf("%s: got %v, want %v", tc.name, retrieved, tc.expected)
		}
	}
}

func TestItemCombinations(t *testing.T) {
	state := engine.NewGameState("room")

	item1 := &engine.Item{ID: "potion", Name: "Potion"}
	item2 := &engine.Item{ID: "water", Name: "Water"}

	state.AddToInventory(item1)
	state.AddToInventory(item2)

	// Mark as combined
	state.MarkItemsCombined("potion", "water")

	if !state.AreItemsCombined("potion", "water") {
		t.Error("items should be marked as combined")
	}

	if !state.AreItemsCombined("water", "potion") {
		t.Error("combined items should be symmetric")
	}
}

func TestGameProgress(t *testing.T) {
	state := engine.NewGameState("start")

	// Track completion milestones
	state.SetVariable("puzzle_1_solved", false)
	state.SetVariable("puzzle_2_solved", false)

	state.SetVariable("puzzle_1_solved", true)
	if val, _ := state.GetVariable("puzzle_1_solved"); val != true {
		t.Error("puzzle 1 should be solved")
	}

	// Check overall progress
	val1, _ := state.GetVariable("puzzle_1_solved")
	val2, _ := state.GetVariable("puzzle_2_solved")

	if val1 != true || val2 != false {
		t.Error("inconsistent puzzle state")
	}
}

func TestMultipleRoomVisits(t *testing.T) {
	state := engine.NewGameState("start")

	rooms := []string{"start", "corridor", "room1", "room2"}

	for _, room := range rooms {
		state.MarkRoomVisited(room)
	}

	for _, room := range rooms {
		if !state.IsRoomVisited(room) {
			t.Errorf("room %s should be marked visited", room)
		}
	}
}

func TestGameStatePersistence(t *testing.T) {
	state := engine.NewGameState("start")
	state.SetVariable("progress", 50)

	item := &engine.Item{ID: "key", Name: "Key"}
	state.AddToInventory(item)
	state.MarkRoomVisited("corridor")

	// Verify all state is preserved
	if state.CurrentRoom != "start" {
		t.Error("current room not persisted")
	}

	val, _ := state.GetVariable("progress")
	if val != 50 {
		t.Error("variable not persisted")
	}

	if !state.HasItem("key") {
		t.Error("item not persisted")
	}

	if !state.IsRoomVisited("corridor") {
		t.Error("room visit not persisted")
	}
}

func TestInventoryItemCount(t *testing.T) {
	state := engine.NewGameState("room")

	for i := 0; i < 5; i++ {
		item := &engine.Item{
			ID:   string(rune(i)),
			Name: string(rune(i)),
		}
		state.AddToInventory(item)
	}

	if len(state.GetInventory()) != 5 {
		t.Errorf("inventory count: got %d, want 5", len(state.GetInventory()))
	}
}
