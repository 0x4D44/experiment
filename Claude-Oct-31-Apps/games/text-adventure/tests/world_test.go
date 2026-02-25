package tests

import (
	"testing"
	"textadventure/engine"
)

func TestCreateRoom(t *testing.T) {
	room := &engine.Room{
		ID:          "cell",
		Name:        "Dungeon Cell",
		Description: "A small, damp cell",
	}

	if room.ID != "cell" {
		t.Errorf("room ID: got %q, want %q", room.ID, "cell")
	}
	if room.Name != "Dungeon Cell" {
		t.Errorf("room name: got %q, want %q", room.Name, "Dungeon Cell")
	}
}

func TestWorldNavigation(t *testing.T) {
	world := engine.NewWorld()

	// Add rooms
	cell := &engine.Room{
		ID:   "cell",
		Name: "Cell",
		Exits: map[string]string{"north": "corridor"},
	}
	corridor := &engine.Room{
		ID:   "corridor",
		Name: "Corridor",
		Exits: map[string]string{
			"south": "cell",
			"north": "guardroom",
		},
	}
	guardroom := &engine.Room{
		ID:   "guardroom",
		Name: "Guard Room",
		Exits: map[string]string{"south": "corridor"},
	}

	world.AddRoom(cell)
	world.AddRoom(corridor)
	world.AddRoom(guardroom)

	// Test retrieval
	if room := world.GetRoom("cell"); room == nil {
		t.Error("failed to retrieve cell room")
	}

	// Test exit navigation
	corridorRoom := world.GetRoom("corridor")
	northExit := corridorRoom.Exits["north"]
	if northExit != "guardroom" {
		t.Errorf("north exit: got %q, want %q", northExit, "guardroom")
	}
}

func TestRoomExits(t *testing.T) {
	room := &engine.Room{
		ID:   "test",
		Name: "Test Room",
		Exits: map[string]string{
			"north": "room1",
			"south": "room2",
			"east":  "room3",
			"west":  "room4",
		},
	}

	tests := []struct {
		direction string
		expected  string
	}{
		{"north", "room1"},
		{"south", "room2"},
		{"east", "room3"},
		{"west", "room4"},
		{"up", ""},
		{"down", ""},
	}

	for _, tt := range tests {
		t.Run(tt.direction, func(t *testing.T) {
			if got := room.Exits[tt.direction]; got != tt.expected {
				t.Errorf("got %q, want %q", got, tt.expected)
			}
		})
	}
}

func TestItemPlacement(t *testing.T) {
	world := engine.NewWorld()

	// Create items
	sword := &engine.Item{
		ID:       "sword",
		Name:     "Iron Sword",
		Takeable: true,
		Weight:   2.0,
	}
	key := &engine.Item{
		ID:       "key",
		Name:     "Iron Key",
		Takeable: true,
		Weight:   0.1,
	}

	// Create room with items
	room := &engine.Room{
		ID:    "room1",
		Name:  "Room 1",
		Items: []*engine.Item{sword, key},
	}

	world.AddRoom(room)
	world.AddItem(sword)
	world.AddItem(key)

	// Verify items in room
	if len(room.Items) != 2 {
		t.Errorf("room items: got %d, want 2", len(room.Items))
	}

	// Verify item retrieval
	if item := world.GetItem("sword"); item == nil {
		t.Error("failed to retrieve sword")
	}
}

func TestDynamicRoomDescription(t *testing.T) {
	room := &engine.Room{
		ID:              "cell",
		Name:            "Cell",
		BaseDescription: "A cell",
		DescriptionFn: func(state *engine.GameState) string {
			if state.GetVariable("door_unlocked") == true {
				return "A cell with an open door"
			}
			return "A cell with a locked door"
		},
	}

	state := engine.NewGameState("cell")

	// Test locked description
	desc := room.DescriptionFn(state)
	if desc != "A cell with a locked door" {
		t.Errorf("got %q, want %q", desc, "A cell with a locked door")
	}

	// Set variable and test unlocked description
	state.SetVariable("door_unlocked", true)
	desc = room.DescriptionFn(state)
	if desc != "A cell with an open door" {
		t.Errorf("got %q, want %q", desc, "A cell with an open door")
	}
}

func TestContainerItems(t *testing.T) {
	chest := &engine.Item{
		ID:        "chest",
		Name:      "Wooden Chest",
		Takeable:  false,
		Container: true,
		Contents:  make([]*engine.Item, 0),
	}

	key := &engine.Item{
		ID:       "key",
		Name:     "Key",
		Takeable: true,
	}

	// Add item to container
	chest.Contents = append(chest.Contents, key)

	if len(chest.Contents) != 1 {
		t.Errorf("container contents: got %d, want 1", len(chest.Contents))
	}

	if chest.Contents[0].ID != "key" {
		t.Errorf("contained item: got %q, want %q", chest.Contents[0].ID, "key")
	}
}

func TestLockedRooms(t *testing.T) {
	room := &engine.Room{
		ID:       "treasury",
		Name:     "Treasury",
		Locked:   true,
		LockItem: "gold_key",
	}

	if !room.Locked {
		t.Error("room should be locked")
	}

	if room.LockItem != "gold_key" {
		t.Errorf("lock item: got %q, want %q", room.LockItem, "gold_key")
	}
}

func TestHiddenItems(t *testing.T) {
	item := &engine.Item{
		ID:       "key",
		Name:     "Hidden Key",
		Hidden:   true,
		Takeable: true,
	}

	if !item.Hidden {
		t.Error("item should be hidden")
	}

	// After examining or finding, item becomes visible
	item.Hidden = false
	if item.Hidden {
		t.Error("item should no longer be hidden")
	}
}

func TestRoomState(t *testing.T) {
	room := &engine.Room{
		ID:   "room1",
		Name: "Room 1",
	}

	state := engine.NewGameState("room1")

	// Initially room not visited
	if state.IsRoomVisited("room1") {
		t.Error("room should not be visited initially")
	}

	// Mark as visited
	state.MarkRoomVisited("room1")

	// Now should be visited
	if !state.IsRoomVisited("room1") {
		t.Error("room should be visited after marking")
	}
}

func TestMultipleLevelNavigation(t *testing.T) {
	world := engine.NewWorld()

	// Create rooms with up/down navigation
	surface := &engine.Room{
		ID:   "surface",
		Name: "Surface",
		Exits: map[string]string{"down": "underground"},
	}
	underground := &engine.Room{
		ID:   "underground",
		Name: "Underground",
		Exits: map[string]string{
			"up":   "surface",
			"down": "deep_cave",
		},
	}
	deepCave := &engine.Room{
		ID:   "deep_cave",
		Name: "Deep Cave",
		Exits: map[string]string{"up": "underground"},
	}

	world.AddRoom(surface)
	world.AddRoom(underground)
	world.AddRoom(deepCave)

	// Test vertical navigation
	if underRoom := world.GetRoom("underground"); underRoom.Exits["up"] != "surface" {
		t.Error("vertical navigation failed")
	}
}

func TestItemProperties(t *testing.T) {
	tests := []struct {
		name        string
		takeable    bool
		useable     bool
		container   bool
		weight      float64
	}{
		{"sword", true, true, false, 2.0},
		{"wall", false, false, false, 1000.0},
		{"chest", false, true, true, 10.0},
		{"feather", true, false, false, 0.01},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			item := &engine.Item{
				Name:      tt.name,
				Takeable:  tt.takeable,
				Useable:   tt.useable,
				Container: tt.container,
				Weight:    tt.weight,
			}

			if item.Takeable != tt.takeable {
				t.Errorf("takeable: got %v, want %v", item.Takeable, tt.takeable)
			}
			if item.Weight != tt.weight {
				t.Errorf("weight: got %f, want %f", item.Weight, tt.weight)
			}
		})
	}
}
