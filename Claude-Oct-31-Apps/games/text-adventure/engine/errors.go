package engine

import "errors"

var (
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
	ErrItemNotInRoom   = errors.New("item not in this room")
	ErrItemNotTakeable = errors.New("item cannot be taken")
	ErrItemNotUseable  = errors.New("item cannot be used")
	ErrInventoryFull   = errors.New("inventory is full")
	ErrCarryWeightExceeded = errors.New("carrying too much weight")

	// NPC errors
	ErrNilNPC         = errors.New("NPC cannot be nil")
	ErrInvalidNPCID   = errors.New("NPC ID cannot be empty")
	ErrDuplicateNPC   = errors.New("NPC with this ID already exists")
	ErrNPCNotFound    = errors.New("NPC not found")
	ErrNPCNotInRoom   = errors.New("NPC not in this room")

	// Command errors
	ErrInvalidCommand = errors.New("invalid command")
	ErrUnknownCommand = errors.New("unknown command")
	ErrNoTargetSpecified = errors.New("no target specified")

	// Game state errors
	ErrInvalidGameState = errors.New("invalid game state")
	ErrGameOver = errors.New("game is over")

	// File errors
	ErrFileNotFound = errors.New("file not found")
	ErrInvalidFileFormat = errors.New("invalid file format")
	ErrCorruptSaveFile = errors.New("save file is corrupted")

	// Script errors
	ErrScriptNotFound = errors.New("script not found")
	ErrScriptError = errors.New("script execution error")
)
