package engine

// NewWorld creates a new game world
func NewWorld() *World {
	return &World{
		Rooms: make(map[string]*Room),
		Items: make(map[string]*Item),
		NPCs:  make(map[string]*NPC),
	}
}

// AddRoom adds a room to the world
func (w *World) AddRoom(room *Room) error {
	if room == nil {
		return ErrNilRoom
	}
	if room.ID == "" {
		return ErrInvalidRoomID
	}
	if _, exists := w.Rooms[room.ID]; exists {
		return ErrDuplicateRoom
	}

	if room.State == nil {
		room.State = make(map[string]interface{})
	}

	w.Rooms[room.ID] = room
	return nil
}

// GetRoom retrieves a room by ID
func (w *World) GetRoom(id string) *Room {
	return w.Rooms[id]
}

// AddItem adds an item to the world
func (w *World) AddItem(item *Item) error {
	if item == nil {
		return ErrNilItem
	}
	if item.ID == "" {
		return ErrInvalidItemID
	}
	if _, exists := w.Items[item.ID]; exists {
		return ErrDuplicateItem
	}

	if item.State == nil {
		item.State = make(map[string]interface{})
	}

	w.Items[item.ID] = item
	return nil
}

// GetItem retrieves an item by ID
func (w *World) GetItem(id string) *Item {
	return w.Items[id]
}

// AddNPC adds an NPC to the world
func (w *World) AddNPC(npc *NPC) error {
	if npc == nil {
		return ErrNilNPC
	}
	if npc.ID == "" {
		return ErrInvalidNPCID
	}
	if _, exists := w.NPCs[npc.ID]; exists {
		return ErrDuplicateNPC
	}

	if npc.State == nil {
		npc.State = make(map[string]interface{})
	}

	w.NPCs[npc.ID] = npc
	return nil
}

// GetNPC retrieves an NPC by ID
func (w *World) GetNPC(id string) *NPC {
	return w.NPCs[id]
}

// GetRoomItems returns all items in a room
func (w *World) GetRoomItems(roomID string) []*Item {
	room := w.GetRoom(roomID)
	if room == nil {
		return []*Item{}
	}
	return room.Items
}

// GetRoomNPCs returns all NPCs in a room
func (w *World) GetRoomNPCs(roomID string) []*NPC {
	room := w.GetRoom(roomID)
	if room == nil {
		return []*NPC{}
	}
	return room.NPCs
}

// AddItemToRoom adds an item to a specific room
func (w *World) AddItemToRoom(itemID, roomID string) error {
	item := w.GetItem(itemID)
	if item == nil {
		return ErrItemNotFound
	}

	room := w.GetRoom(roomID)
	if room == nil {
		return ErrRoomNotFound
	}

	room.Items = append(room.Items, item)
	return nil
}

// RemoveItemFromRoom removes an item from a room
func (w *World) RemoveItemFromRoom(itemID, roomID string) error {
	room := w.GetRoom(roomID)
	if room == nil {
		return ErrRoomNotFound
	}

	for i, item := range room.Items {
		if item.ID == itemID {
			room.Items = append(room.Items[:i], room.Items[i+1:]...)
			return nil
		}
	}

	return ErrItemNotInRoom
}

// AddNPCToRoom adds an NPC to a specific room
func (w *World) AddNPCToRoom(npcID, roomID string) error {
	npc := w.GetNPC(npcID)
	if npc == nil {
		return ErrNPCNotFound
	}

	room := w.GetRoom(roomID)
	if room == nil {
		return ErrRoomNotFound
	}

	npc.Location = roomID
	room.NPCs = append(room.NPCs, npc)
	return nil
}

// RemoveNPCFromRoom removes an NPC from a room
func (w *World) RemoveNPCFromRoom(npcID, roomID string) error {
	room := w.GetRoom(roomID)
	if room == nil {
		return ErrRoomNotFound
	}

	for i, npc := range room.NPCs {
		if npc.ID == npcID {
			room.NPCs = append(room.NPCs[:i], room.NPCs[i+1:]...)
			return nil
		}
	}

	return ErrNPCNotInRoom
}

// GetRoomDescription returns a room's description, using dynamic function if available
func (w *World) GetRoomDescription(roomID string, state *GameState) string {
	room := w.GetRoom(roomID)
	if room == nil {
		return ""
	}

	if room.DescriptionFn != nil {
		return room.DescriptionFn(state)
	}

	return room.BaseDescription
}

// HasExit checks if a room has an exit in a given direction
func (w *World) HasExit(roomID, direction string) bool {
	room := w.GetRoom(roomID)
	if room == nil {
		return false
	}

	_, exists := room.Exits[direction]
	return exists
}

// GetExit returns the room ID that an exit leads to
func (w *World) GetExit(roomID, direction string) string {
	room := w.GetRoom(roomID)
	if room == nil {
		return ""
	}

	return room.Exits[direction]
}

// GetAvailableExits returns all available exits from a room
func (w *World) GetAvailableExits(roomID string) []string {
	room := w.GetRoom(roomID)
	if room == nil {
		return []string{}
	}

	exits := make([]string, 0, len(room.Exits))
	for direction := range room.Exits {
		exits = append(exits, direction)
	}

	return exits
}

// ItemExists checks if an item exists in the world
func (w *World) ItemExists(itemID string) bool {
	_, exists := w.Items[itemID]
	return exists
}

// RoomExists checks if a room exists in the world
func (w *World) RoomExists(roomID string) bool {
	_, exists := w.Rooms[roomID]
	return exists
}

// NPCExists checks if an NPC exists in the world
func (w *World) NPCExists(npcID string) bool {
	_, exists := w.NPCs[npcID]
	return exists
}

// FindItemInRoom searches for an item in a room by name (fuzzy match)
func (w *World) FindItemInRoom(roomID, itemName string) *Item {
	room := w.GetRoom(roomID)
	if room == nil {
		return nil
	}

	// Exact match first
	for _, item := range room.Items {
		if item.ID == itemName || item.Name == itemName {
			return item
		}
	}

	// Fuzzy match
	for _, item := range room.Items {
		distance := LevenshteinDistance(itemName, item.ID)
		if distance <= 2 {
			return item
		}
	}

	return nil
}

// FindNPCInRoom searches for an NPC in a room by name
func (w *World) FindNPCInRoom(roomID, npcName string) *NPC {
	room := w.GetRoom(roomID)
	if room == nil {
		return nil
	}

	for _, npc := range room.NPCs {
		if npc.ID == npcName || npc.Name == npcName {
			return npc
		}
	}

	return nil
}

// GetItemByName returns an item by its display name
func (w *World) GetItemByName(name string) *Item {
	for _, item := range w.Items {
		if item.Name == name {
			return item
		}
	}
	return nil
}

// GetNPCByName returns an NPC by its display name
func (w *World) GetNPCByName(name string) *NPC {
	for _, npc := range w.NPCs {
		if npc.Name == name {
			return npc
		}
	}
	return nil
}

// ValidateWorld checks if the world structure is valid
func (w *World) ValidateWorld() []string {
	errors := []string{}

	// Check for dangling room references
	for roomID, room := range w.Rooms {
		for direction, targetID := range room.Exits {
			if _, exists := w.Rooms[targetID]; !exists {
				errors = append(errors, "Room "+roomID+" has exit "+direction+" to non-existent room "+targetID)
			}
		}
	}

	// Check for missing items
	for roomID, room := range w.Rooms {
		for _, item := range room.Items {
			if _, exists := w.Items[item.ID]; !exists {
				errors = append(errors, "Room "+roomID+" references non-existent item "+item.ID)
			}
		}
	}

	// Check for missing NPCs
	for roomID, room := range w.Rooms {
		for _, npc := range room.NPCs {
			if _, exists := w.NPCs[npc.ID]; !exists {
				errors = append(errors, "Room "+roomID+" references non-existent NPC "+npc.ID)
			}
		}
	}

	return errors
}

// GetWorldStats returns statistics about the world
func (w *World) GetWorldStats() map[string]int {
	stats := make(map[string]int)
	stats["rooms"] = len(w.Rooms)
	stats["items"] = len(w.Items)
	stats["npcs"] = len(w.NPCs)

	totalConnections := 0
	for _, room := range w.Rooms {
		totalConnections += len(room.Exits)
	}
	stats["connections"] = totalConnections

	return stats
}
