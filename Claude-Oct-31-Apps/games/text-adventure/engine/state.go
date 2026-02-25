package engine

// NewGameState creates a new game state starting in the given room
func NewGameState(startingRoom string) *GameState {
	return &GameState{
		CurrentRoom:        startingRoom,
		Inventory:          make([]*Item, 0),
		Variables:          make(map[string]interface{}),
		RoomsVisited:       make(map[string]bool),
		ItemsExamined:      make(map[string]bool),
		ItemsCombined:      make(map[string]map[string]bool),
		MaxCarryWeight:     50.0, // Default 50 pounds
		MaxInventorySize:   20,   // Default 20 items
		Score:              0,
		TurnCount:          0,
		EquippedItems:      make(map[string]string),
		ItemLocations:      make(map[string]string),
		RoomStateSnapshots: make(map[string]map[string]interface{}),
		ItemStateSnapshots: make(map[string]map[string]interface{}),
	}
}

// SetVariable sets a game variable
func (gs *GameState) SetVariable(key string, value interface{}) {
	gs.Variables[key] = value
}

// GetVariable retrieves a game variable
func (gs *GameState) GetVariable(key string) (interface{}, bool) {
	val, ok := gs.Variables[key]
	return val, ok
}

// GetVariableAsString retrieves a variable as a string
func (gs *GameState) GetVariableAsString(key string, defaultVal string) string {
	if val, ok := gs.Variables[key]; ok {
		if str, ok := val.(string); ok {
			return str
		}
	}
	return defaultVal
}

// GetVariableAsInt retrieves a variable as an int
func (gs *GameState) GetVariableAsInt(key string, defaultVal int) int {
	if val, ok := gs.Variables[key]; ok {
		switch v := val.(type) {
		case int:
			return v
		case float64:
			return int(v)
		}
	}
	return defaultVal
}

// GetVariableAsBool retrieves a variable as a bool
func (gs *GameState) GetVariableAsBool(key string, defaultVal bool) bool {
	if val, ok := gs.Variables[key]; ok {
		if b, ok := val.(bool); ok {
			return b
		}
	}
	return defaultVal
}

// GetInventory returns the player's inventory
func (gs *GameState) GetInventory() []*Item {
	return gs.Inventory
}

// AddToInventory adds an item to the inventory
func (gs *GameState) AddToInventory(item *Item) bool {
	if !gs.CanAddToInventory(item) {
		return false
	}

	gs.Inventory = append(gs.Inventory, item)
	gs.ItemLocations[item.ID] = "inventory"
	return true
}

// RemoveFromInventory removes an item from the inventory by ID
func (gs *GameState) RemoveFromInventory(itemID string) bool {
	for i, item := range gs.Inventory {
		if item.ID == itemID {
			gs.Inventory = append(gs.Inventory[:i], gs.Inventory[i+1:]...)
			delete(gs.ItemLocations, itemID)
			return true
		}
	}
	return false
}

// HasItem checks if an item is in the inventory
func (gs *GameState) HasItem(itemID string) bool {
	for _, item := range gs.Inventory {
		if item.ID == itemID {
			return true
		}
	}
	return false
}

// GetItem retrieves an item from the inventory
func (gs *GameState) GetItem(itemID string) *Item {
	for _, item := range gs.Inventory {
		if item.ID == itemID {
			return item
		}
	}
	return nil
}

// CanAddToInventory checks if an item can be added
func (gs *GameState) CanAddToInventory(item *Item) bool {
	if item == nil {
		return false
	}

	// Check size limit
	if len(gs.Inventory) >= gs.MaxInventorySize {
		return false
	}

	// Check weight limit
	totalWeight := gs.GetInventoryWeight() + item.Weight
	if totalWeight > gs.MaxCarryWeight {
		return false
	}

	return true
}

// GetInventoryWeight returns total weight of items
func (gs *GameState) GetInventoryWeight() float64 {
	total := 0.0
	for _, item := range gs.Inventory {
		total += item.Weight
	}
	return total
}

// GetInventorySortedByWeight returns inventory sorted by weight (ascending)
func (gs *GameState) GetInventorySortedByWeight() []*Item {
	sorted := make([]*Item, len(gs.Inventory))
	copy(sorted, gs.Inventory)

	// Bubble sort (simple for small inventories)
	for i := 0; i < len(sorted); i++ {
		for j := i + 1; j < len(sorted); j++ {
			if sorted[j].Weight < sorted[i].Weight {
				sorted[i], sorted[j] = sorted[j], sorted[i]
			}
		}
	}

	return sorted
}

// SetMaxCarryWeight sets the maximum weight the player can carry
func (gs *GameState) SetMaxCarryWeight(weight float64) {
	gs.MaxCarryWeight = weight
}

// SetMaxInventorySize sets the maximum number of items
func (gs *GameState) SetMaxInventorySize(size int) {
	gs.MaxInventorySize = size
}

// MarkRoomVisited marks a room as visited
func (gs *GameState) MarkRoomVisited(roomID string) {
	gs.RoomsVisited[roomID] = true
}

// IsRoomVisited checks if a room has been visited
func (gs *GameState) IsRoomVisited(roomID string) bool {
	return gs.RoomsVisited[roomID]
}

// MarkItemExamined marks an item as examined
func (gs *GameState) MarkItemExamined(itemID string) {
	gs.ItemsExamined[itemID] = true
}

// IsItemExamined checks if an item has been examined
func (gs *GameState) IsItemExamined(itemID string) bool {
	return gs.ItemsExamined[itemID]
}

// MarkItemsCombined marks two items as having been combined
func (gs *GameState) MarkItemsCombined(item1, item2 string) {
	if gs.ItemsCombined[item1] == nil {
		gs.ItemsCombined[item1] = make(map[string]bool)
	}
	if gs.ItemsCombined[item2] == nil {
		gs.ItemsCombined[item2] = make(map[string]bool)
	}

	gs.ItemsCombined[item1][item2] = true
	gs.ItemsCombined[item2][item1] = true
}

// AreItemsCombined checks if two items have been combined
func (gs *GameState) AreItemsCombined(item1, item2 string) bool {
	if combinations, ok := gs.ItemsCombined[item1]; ok {
		return combinations[item2]
	}
	return false
}

// EquipItem equips an item
func (gs *GameState) EquipItem(itemID string) bool {
	if !gs.HasItem(itemID) {
		return false
	}

	item := gs.GetItem(itemID)
	if item == nil {
		return false
	}

	gs.EquippedItems[itemID] = itemID
	return true
}

// UnequipItem unequips an item
func (gs *GameState) UnequipItem(itemID string) bool {
	delete(gs.EquippedItems, itemID)
	return true
}

// IsItemEquipped checks if an item is equipped
func (gs *GameState) IsItemEquipped(itemID string) bool {
	_, equipped := gs.EquippedItems[itemID]
	return equipped
}

// IncrementScore increases the player's score
func (gs *GameState) IncrementScore(points int) {
	gs.Score += points
}

// GetScore returns the current score
func (gs *GameState) GetScore() int {
	return gs.Score
}

// IncrementTurnCount increments the turn counter
func (gs *GameState) IncrementTurnCount() {
	gs.TurnCount++
}

// GetTurnCount returns the current turn count
func (gs *GameState) GetTurnCount() int {
	return gs.TurnCount
}

// SaveRoomState saves the current state of a room
func (gs *GameState) SaveRoomState(roomID string, state map[string]interface{}) {
	snapshot := make(map[string]interface{})
	for k, v := range state {
		snapshot[k] = v
	}
	gs.RoomStateSnapshots[roomID] = snapshot
}

// LoadRoomState restores a previously saved room state
func (gs *GameState) LoadRoomState(roomID string) map[string]interface{} {
	if snapshot, ok := gs.RoomStateSnapshots[roomID]; ok {
		state := make(map[string]interface{})
		for k, v := range snapshot {
			state[k] = v
		}
		return state
	}
	return nil
}

// SaveItemState saves the current state of an item
func (gs *GameState) SaveItemState(itemID string, state map[string]interface{}) {
	snapshot := make(map[string]interface{})
	for k, v := range state {
		snapshot[k] = v
	}
	gs.ItemStateSnapshots[itemID] = snapshot
}

// LoadItemState restores a previously saved item state
func (gs *GameState) LoadItemState(itemID string) map[string]interface{} {
	if snapshot, ok := gs.ItemStateSnapshots[itemID]; ok {
		state := make(map[string]interface{})
		for k, v := range snapshot {
			state[k] = v
		}
		return state
	}
	return nil
}

// CreateSnapshot creates a full game state snapshot
func (gs *GameState) CreateSnapshot() *GameState {
	snapshot := NewGameState(gs.CurrentRoom)
	snapshot.Score = gs.Score
	snapshot.TurnCount = gs.TurnCount
	snapshot.MaxCarryWeight = gs.MaxCarryWeight
	snapshot.MaxInventorySize = gs.MaxInventorySize

	// Copy inventory
	for _, item := range gs.Inventory {
		snapshot.Inventory = append(snapshot.Inventory, item)
	}

	// Copy variables
	for k, v := range gs.Variables {
		snapshot.Variables[k] = v
	}

	// Copy room visits
	for k, v := range gs.RoomsVisited {
		snapshot.RoomsVisited[k] = v
	}

	// Copy item examinations
	for k, v := range gs.ItemsExamined {
		snapshot.ItemsExamined[k] = v
	}

	// Copy equipped items
	for k, v := range gs.EquippedItems {
		snapshot.EquippedItems[k] = v
	}

	return snapshot
}

// RestoreSnapshot restores a game state from a snapshot
func (gs *GameState) RestoreSnapshot(snapshot *GameState) {
	gs.CurrentRoom = snapshot.CurrentRoom
	gs.Score = snapshot.Score
	gs.TurnCount = snapshot.TurnCount
	gs.MaxCarryWeight = snapshot.MaxCarryWeight
	gs.MaxInventorySize = snapshot.MaxInventorySize

	gs.Inventory = make([]*Item, len(snapshot.Inventory))
	copy(gs.Inventory, snapshot.Inventory)

	gs.Variables = make(map[string]interface{})
	for k, v := range snapshot.Variables {
		gs.Variables[k] = v
	}

	gs.RoomsVisited = make(map[string]bool)
	for k, v := range snapshot.RoomsVisited {
		gs.RoomsVisited[k] = v
	}

	gs.ItemsExamined = make(map[string]bool)
	for k, v := range snapshot.ItemsExamined {
		gs.ItemsExamined[k] = v
	}

	gs.EquippedItems = make(map[string]string)
	for k, v := range snapshot.EquippedItems {
		gs.EquippedItems[k] = v
	}
}

// ClearInventory empties the player's inventory
func (gs *GameState) ClearInventory() {
	gs.Inventory = make([]*Item, 0)
	gs.ItemLocations = make(map[string]string)
}

// GetItemLocation returns the location of an item
func (gs *GameState) GetItemLocation(itemID string) string {
	if location, ok := gs.ItemLocations[itemID]; ok {
		return location
	}
	return "unknown"
}

// SetItemLocation sets the location of an item
func (gs *GameState) SetItemLocation(itemID, location string) {
	gs.ItemLocations[itemID] = location
}
