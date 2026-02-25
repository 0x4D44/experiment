package tests

import (
	"testing"
	"textadventure/engine"
)

func TestItemTakeDrop(t *testing.T) {
	state := engine.NewGameState("room")
	item := &engine.Item{ID: "sword", Name: "Sword", Takeable: true}

	// Can take
	if !item.Takeable {
		t.Error("item should be takeable")
	}

	state.AddToInventory(item)
	if !state.HasItem("sword") {
		t.Error("item should be in inventory")
	}

	// Can drop
	state.RemoveFromInventory("sword")
	if state.HasItem("sword") {
		t.Error("item should be removed from inventory")
	}
}

func TestUntakeableItems(t *testing.T) {
	state := engine.NewGameState("room")
	wall := &engine.Item{ID: "wall", Name: "Wall", Takeable: false}

	if wall.Takeable {
		t.Error("wall should not be takeable")
	}

	// Should not add to inventory
	state.AddToInventory(wall)

	// We don't enforce this in the state manager, but the executor should check
}

func TestInventoryListDisplay(t *testing.T) {
	state := engine.NewGameState("room")

	items := []*engine.Item{
		{ID: "sword", Name: "Iron Sword", Weight: 2.0},
		{ID: "key", Name: "Golden Key", Weight: 0.1},
		{ID: "shield", Name: "Wooden Shield", Weight: 3.0},
	}

	for _, item := range items {
		state.AddToInventory(item)
	}

	inventory := state.GetInventory()
	if len(inventory) != 3 {
		t.Errorf("inventory size: got %d, want 3", len(inventory))
	}

	// Verify all items present
	names := make(map[string]bool)
	for _, item := range inventory {
		names[item.Name] = true
	}

	expectedNames := map[string]bool{
		"Iron Sword":     true,
		"Golden Key":     true,
		"Wooden Shield":  true,
	}

	for name := range expectedNames {
		if !names[name] {
			t.Errorf("missing item: %s", name)
		}
	}
}

func TestWeightCalculation(t *testing.T) {
	state := engine.NewGameState("room")

	items := []*engine.Item{
		{ID: "sword", Name: "Sword", Weight: 2.0},
		{ID: "shield", Name: "Shield", Weight: 3.0},
		{ID: "key", Name: "Key", Weight: 0.1},
	}

	totalExpected := 5.1
	for _, item := range items {
		state.AddToInventory(item)
	}

	total := state.GetInventoryWeight()
	if total != totalExpected {
		t.Errorf("total weight: got %f, want %f", total, totalExpected)
	}
}

func TestMaxInventoryItems(t *testing.T) {
	state := engine.NewGameState("room")
	state.SetMaxInventorySize(3)

	items := []*engine.Item{
		{ID: "item1", Name: "Item 1"},
		{ID: "item2", Name: "Item 2"},
		{ID: "item3", Name: "Item 3"},
		{ID: "item4", Name: "Item 4"},
	}

	// Add first 3
	for i := 0; i < 3; i++ {
		state.AddToInventory(items[i])
	}

	if len(state.GetInventory()) != 3 {
		t.Errorf("inventory size: got %d, want 3", len(state.GetInventory()))
	}

	// Cannot add 4th without removing
	if state.CanAddToInventory(items[3]) {
		t.Error("should not be able to exceed max inventory size")
	}
}

func TestInventorySearch(t *testing.T) {
	state := engine.NewGameState("room")

	items := []*engine.Item{
		{ID: "sword", Name: "Iron Sword"},
		{ID: "key", Name: "Golden Key"},
		{ID: "shield", Name: "Wooden Shield"},
	}

	for _, item := range items {
		state.AddToInventory(item)
	}

	// Find by ID
	if !state.HasItem("sword") {
		t.Error("should find item by ID")
	}

	// Multiple items
	if len(state.GetInventory()) != 3 {
		t.Errorf("should have 3 items, got %d", len(state.GetInventory()))
	}
}

func TestSpecialItems(t *testing.T) {
	state := engine.NewGameState("room")

	// Cursed item that cannot be dropped
	curse := &engine.Item{
		ID:       "cursed_ring",
		Name:     "Cursed Ring",
		Takeable: true,
		Cursed:   true,
	}

	state.AddToInventory(curse)

	if !state.HasItem("cursed_ring") {
		t.Error("cursed item should be in inventory")
	}

	// Game should prevent dropping it (enforced in executor)
}

func TestStackableItems(t *testing.T) {
	state := engine.NewGameState("room")

	// Coins should be stackable
	coins := &engine.Item{
		ID:        "coin",
		Name:      "Gold Coin",
		Stackable: true,
		Quantity:  10,
	}

	state.AddToInventory(coins)

	if coins.Quantity != 10 {
		t.Errorf("quantity: got %d, want 10", coins.Quantity)
	}

	// Swords should not stack
	sword := &engine.Item{
		ID:        "sword",
		Name:      "Sword",
		Stackable: false,
		Quantity:  1,
	}

	if sword.Stackable {
		t.Error("swords should not be stackable")
	}
}

func TestInventoryOrganization(t *testing.T) {
	state := engine.NewGameState("room")

	// Create items in specific order
	items := []*engine.Item{
		{ID: "sword", Name: "Sword", Weight: 2.0},
		{ID: "key", Name: "Key", Weight: 0.1},
		{ID: "shield", Name: "Shield", Weight: 3.0},
	}

	for _, item := range items {
		state.AddToInventory(item)
	}

	inventory := state.GetInventory()

	// Check order is maintained
	if inventory[0].ID != "sword" {
		t.Errorf("first item: got %q, want %q", inventory[0].ID, "sword")
	}
	if inventory[1].ID != "key" {
		t.Errorf("second item: got %q, want %q", inventory[1].ID, "key")
	}
	if inventory[2].ID != "shield" {
		t.Errorf("third item: got %q, want %q", inventory[2].ID, "shield")
	}
}

func TestInventorySorting(t *testing.T) {
	state := engine.NewGameState("room")

	items := []*engine.Item{
		{ID: "sword", Name: "Sword", Weight: 2.0},
		{ID: "key", Name: "Key", Weight: 0.1},
		{ID: "shield", Name: "Shield", Weight: 3.0},
	}

	for _, item := range items {
		state.AddToInventory(item)
	}

	// Sort by weight
	sorted := state.GetInventorySortedByWeight()

	if sorted[0].Weight > sorted[1].Weight || sorted[1].Weight > sorted[2].Weight {
		t.Error("items not sorted by weight")
	}

	if sorted[0].Weight != 0.1 || sorted[1].Weight != 2.0 || sorted[2].Weight != 3.0 {
		t.Error("weight sort order incorrect")
	}
}

func TestEquipmentSlots(t *testing.T) {
	state := engine.NewGameState("room")

	sword := &engine.Item{
		ID:      "sword",
		Name:    "Sword",
		Equip:   true,
		Slot:    "hand",
	}

	state.AddToInventory(sword)
	state.EquipItem("sword")

	if !state.IsItemEquipped("sword") {
		t.Error("sword should be equipped")
	}
}

func TestCarryCapacity(t *testing.T) {
	state := engine.NewGameState("room")
	state.SetMaxCarryWeight(5.0)

	items := []*engine.Item{
		{ID: "item1", Name: "Item 1", Weight: 2.0},
		{ID: "item2", Name: "Item 2", Weight: 2.0},
		{ID: "item3", Name: "Item 3", Weight: 2.0},
	}

	// Can carry 2
	state.AddToInventory(items[0])
	state.AddToInventory(items[1])

	if state.GetInventoryWeight() > 5.0 {
		t.Error("inventory exceeds max weight")
	}

	// Cannot carry 3
	canAdd := state.CanAddToInventory(items[2])
	if canAdd {
		t.Error("should not exceed carry capacity")
	}
}

func TestEmptyInventory(t *testing.T) {
	state := engine.NewGameState("room")

	if len(state.GetInventory()) != 0 {
		t.Error("inventory should start empty")
	}

	if state.GetInventoryWeight() != 0 {
		t.Error("empty inventory should have zero weight")
	}
}
