package main

import (
	"fmt"
	"sync"
	"time"
)

// InputHandler manages keyboard input from the user
type InputHandler struct {
	directionChan chan Direction
	commandChan   chan string
	stopChan      chan bool
	mu            sync.Mutex
	running       bool
}

// NewInputHandler creates a new input handler
func NewInputHandler() *InputHandler {
	return &InputHandler{
		directionChan: make(chan Direction, 10),
		commandChan:   make(chan string, 10),
		stopChan:      make(chan bool, 1),
		running:       false,
	}
}

// Start begins listening for keyboard input
func (ih *InputHandler) Start() error {
	ih.mu.Lock()
	if ih.running {
		ih.mu.Unlock()
		return fmt.Errorf("input handler already running")
	}
	ih.running = true
	ih.mu.Unlock()

	// Enable raw mode
	err := enableRawMode()
	if err != nil {
		return fmt.Errorf("failed to enable raw mode: %w", err)
	}

	go ih.readInput()
	return nil
}

// Stop stops listening for keyboard input
func (ih *InputHandler) Stop() error {
	ih.mu.Lock()
	if !ih.running {
		ih.mu.Unlock()
		return nil
	}
	ih.running = false
	ih.mu.Unlock()

	// Send stop signal
	select {
	case ih.stopChan <- true:
	default:
	}

	// Give it a moment to stop
	time.Sleep(100 * time.Millisecond)

	// Disable raw mode
	return disableRawMode()
}

// GetDirectionChan returns the direction channel
func (ih *InputHandler) GetDirectionChan() <-chan Direction {
	return ih.directionChan
}

// GetCommandChan returns the command channel
func (ih *InputHandler) GetCommandChan() <-chan string {
	return ih.commandChan
}

// readInput reads keyboard input in a separate goroutine
func (ih *InputHandler) readInput() {
	// This is a placeholder that demonstrates the structure
	// In production, this would use tcell or similar library
	// For now, we'll implement a basic version using standard input

	// Note: Full implementation requires handling raw terminal mode
	// which varies by platform (Unix vs Windows)
}

// enableRawMode enables raw input mode on the terminal
func enableRawMode() error {
	// This would typically use syscalls or a library like tcell
	// Placeholder for demonstration
	return nil
}

// disableRawMode disables raw input mode on the terminal
func disableRawMode() error {
	// This would typically use syscalls or a library like tcell
	// Placeholder for demonstration
	return nil
}

// InputProcessor processes raw input and converts to game commands
type InputProcessor struct {
	handler *InputHandler
}

// NewInputProcessor creates a new input processor
func NewInputProcessor(handler *InputHandler) *InputProcessor {
	return &InputProcessor{handler: handler}
}

// ProcessInputLoop runs the main input processing loop
func (ip *InputProcessor) ProcessInputLoop(game *GameState) {
	directionChan := ip.handler.GetDirectionChan()
	commandChan := ip.handler.GetCommandChan()

	for {
		select {
		case dir := <-directionChan:
			game.Snake.SetNextDirection(dir)

		case cmd := <-commandChan:
			switch cmd {
			case "pause":
				game.TogglePause()
			case "quit":
				game.EndGame()
				return
			case "reset":
				game.Reset()
			}
		}
	}
}

// MockInputHandler is used for testing
type MockInputHandler struct {
	directions []Direction
	index      int
	mu         sync.Mutex
}

// NewMockInputHandler creates a mock input handler for testing
func NewMockInputHandler() *MockInputHandler {
	return &MockInputHandler{
		directions: []Direction{},
		index:      0,
	}
}

// QueueDirection adds a direction to be returned
func (mih *MockInputHandler) QueueDirection(dir Direction) {
	mih.mu.Lock()
	defer mih.mu.Unlock()
	mih.directions = append(mih.directions, dir)
}

// GetNextDirection returns the next queued direction
func (mih *MockInputHandler) GetNextDirection() Direction {
	mih.mu.Lock()
	defer mih.mu.Unlock()

	if mih.index >= len(mih.directions) {
		return None
	}

	dir := mih.directions[mih.index]
	mih.index++
	return dir
}

// Reset clears the mock queue
func (mih *MockInputHandler) Reset() {
	mih.mu.Lock()
	defer mih.mu.Unlock()
	mih.directions = []Direction{}
	mih.index = 0
}
