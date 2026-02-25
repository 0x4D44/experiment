package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"
	"time"
)

// GameConfig holds game configuration
type GameConfig struct {
	Width  int
	Height int
	Speed  int
}

func main() {
	// Parse command-line flags
	width := flag.Int("width", 40, "Board width")
	height := flag.Int("height", 20, "Board height")
	speed := flag.Int("speed", 10, "Initial ticks per move")
	flag.Parse()

	config := GameConfig{
		Width:  *width,
		Height: *height,
		Speed:  *speed,
	}

	// Validate configuration
	if config.Width < 10 || config.Width > 200 {
		fmt.Println("Error: Width must be between 10 and 200")
		os.Exit(1)
	}
	if config.Height < 5 || config.Height > 50 {
		fmt.Println("Error: Height must be between 5 and 50")
		os.Exit(1)
	}
	if config.Speed < 1 {
		fmt.Println("Error: Speed must be at least 1")
		os.Exit(1)
	}

	// Run game
	err := runGame(config)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

// runGame is the main game loop
func runGame(config GameConfig) error {
	// Initialize game state
	game := NewGameState(config.Width, config.Height)
	game.TicksPerMove = config.Speed

	// Create renderer
	renderer := NewRenderer(config.Width, config.Height)

	// Hide cursor
	err := renderer.HideCursor()
	if err != nil {
		return fmt.Errorf("failed to hide cursor: %w", err)
	}
	defer renderer.ShowCursor()

	// Setup terminal raw mode
	err = setupTerminal()
	if err != nil {
		return fmt.Errorf("failed to setup terminal: %w", err)
	}
	defer restoreTerminal()

	// Handle Ctrl+C gracefully
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)

	// Input channel
	inputChan := make(chan Direction, 10)
	commandChan := make(chan string, 10)

	// Start input reader goroutine
	go readInput(inputChan, commandChan)

	// Game loop
	ticker := time.NewTicker(100 * time.Millisecond) // 10 FPS
	defer ticker.Stop()

	currentDirection := Right

	for {
		select {
		case <-sigChan:
			return nil

		case dir := <-inputChan:
			currentDirection = dir

		case cmd := <-commandChan:
			switch cmd {
			case "pause":
				game.TogglePause()
			case "quit":
				return nil
			case "reset":
				game.Reset()
			}

		case <-ticker.C:
			// Update game state
			game.ProcessTick(currentDirection)

			// Render game
			err := renderer.RenderGame(game)
			if err != nil {
				return fmt.Errorf("failed to render: %w", err)
			}

			// Check if game is over
			if game.GetGameOver() {
				// Handle game over
				renderer.ShowCursor()
				err = renderer.RenderGameOver(game, game.Score > game.HighScore)
				if err != nil {
					return err
				}

				// Update high score if needed
				if game.Score > game.HighScore {
					game.HighScore = game.Score
				}

				// Wait for user to press R or Q
				for {
					cmd := <-commandChan
					if cmd == "reset" {
						game.Reset()
						break
					} else if cmd == "quit" {
						return nil
					}
				}
			}
		}
	}
}

// setupTerminal sets up the terminal for the game
func setupTerminal() error {
	// Get the current terminal settings
	// This is a simplified version - real implementation would use tcell or similar
	return nil
}

// restoreTerminal restores the original terminal settings
func restoreTerminal() error {
	// Restore original terminal settings
	return nil
}

// readInput reads keyboard input and sends to channels
func readInput(directionChan chan<- Direction, commandChan chan<- string) {
	reader := bufio.NewReader(os.Stdin)

	for {
		// Read character
		ch, _, err := reader.ReadRune()
		if err != nil {
			close(directionChan)
			close(commandChan)
			return
		}

		// Process input
		switch ch {
		// Arrow keys and WASD
		case 'w', 'W', 'A':
			directionChan <- Up
		case 's', 'S':
			directionChan <- Down
		case 'a', 'A':
			directionChan <- Left
		case 'd', 'D':
			directionChan <- Right

		// Space for pause
		case ' ':
			commandChan <- "pause"

		// Q for quit
		case 'q', 'Q':
			commandChan <- "quit"

		// R for reset
		case 'r', 'R':
			commandChan <- "reset"

		// Escape for special keys
		case '\x1b':
			// Read next characters for escape sequences
			next1, _, _ := reader.ReadRune()
			if next1 == '[' {
				next2, _, _ := reader.ReadRune()
				switch next2 {
				case 'A': // Up arrow
					directionChan <- Up
				case 'B': // Down arrow
					directionChan <- Down
				case 'C': // Right arrow
					directionChan <- Right
				case 'D': // Left arrow
					directionChan <- Left
				}
			}
		}
	}
}

// GameController manages the overall game flow
type GameController struct {
	game     *GameState
	renderer *Renderer
	config   GameConfig
}

// NewGameController creates a new game controller
func NewGameController(config GameConfig) *GameController {
	return &GameController{
		game:     NewGameState(config.Width, config.Height),
		renderer: NewRenderer(config.Width, config.Height),
		config:   config,
	}
}

// Run starts the game
func (gc *GameController) Run() error {
	// Hide cursor at start
	err := gc.renderer.HideCursor()
	if err != nil {
		return err
	}
	defer gc.renderer.ShowCursor()

	// Main game loop
	return runGame(gc.config)
}
