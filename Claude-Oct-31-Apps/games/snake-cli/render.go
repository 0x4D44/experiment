package main

import (
	"bytes"
	"fmt"
	"os"
	"strings"
)

// Renderer handles terminal output and game rendering
type Renderer struct {
	width  int
	height int
	buffer *bytes.Buffer
}

// NewRenderer creates a new game renderer
func NewRenderer(boardWidth, boardHeight int) *Renderer {
	return &Renderer{
		width:  boardWidth,
		height: boardHeight,
		buffer: &bytes.Buffer{},
	}
}

// RenderGame renders the complete game state to the terminal
func (r *Renderer) RenderGame(game *GameState) error {
	r.buffer.Reset()

	// Clear screen
	r.clearScreen()

	// Draw game board
	r.drawBoard(game)

	// Draw HUD (score, high score, instructions)
	r.drawHUD(game)

	// Write to stdout
	_, err := r.buffer.WriteTo(os.Stdout)
	return err
}

// RenderGameOver renders the game over screen
func (r *Renderer) RenderGameOver(game *GameState, newHighScore bool) error {
	r.buffer.Reset()

	// Clear screen
	r.clearScreen()

	// Center text on screen
	cols := 40
	rows := 15

	padding := (r.width - cols) / 2
	topPadding := (r.height - rows) / 2

	// Top padding
	for i := 0; i < topPadding; i++ {
		fmt.Fprint(r.buffer, "\n")
	}

	// Game Over title
	title := "    GAME OVER    "
	centerLine := centerText(title, padding)
	fmt.Fprint(r.buffer, centerLine, "\n")
	fmt.Fprint(r.buffer, "\n")

	// Score
	scoreLine := fmt.Sprintf("Final Score: %d", game.GetScore())
	fmt.Fprint(r.buffer, centerText(scoreLine, padding), "\n")

	// High score
	highScoreLine := fmt.Sprintf("High Score: %d", game.HighScore)
	fmt.Fprint(r.buffer, centerText(highScoreLine, padding), "\n")

	if newHighScore {
		fmt.Fprint(r.buffer, centerText("NEW HIGH SCORE!", padding), "\n")
	}

	fmt.Fprint(r.buffer, "\n\n")

	// Instructions
	fmt.Fprint(r.buffer, centerText("Press R to Restart or Q to Quit", padding), "\n")

	// Write to stdout
	_, err := r.buffer.WriteTo(os.Stdout)
	return err
}

// drawBoard draws the game board with snake and food
func (r *Renderer) drawBoard(game *GameState) {
	// Create board representation
	board := make([][]rune, game.Board.Height)
	for i := range board {
		board[i] = make([]rune, game.Board.Width)
		for j := range board[i] {
			board[i][j] = ' '
		}
	}

	// Draw snake
	snake := game.Snake.GetBody()
	for i, segment := range snake {
		if segment.X >= 0 && segment.X < game.Board.Width &&
			segment.Y >= 0 && segment.Y < game.Board.Height {
			if i == 0 {
				// Head
				board[segment.Y][segment.X] = '@'
			} else {
				// Body
				board[segment.Y][segment.X] = '█'
			}
		}
	}

	// Draw food
	food := game.GetFood()
	if food.X >= 0 && food.X < game.Board.Width &&
		food.Y >= 0 && food.Y < game.Board.Height {
		board[food.Y][food.X] = '●'
	}

	// Draw border and board
	fmt.Fprint(r.buffer, "╔")
	for i := 0; i < game.Board.Width; i++ {
		fmt.Fprint(r.buffer, "═")
	}
	fmt.Fprint(r.buffer, "╗\n")

	for y := 0; y < game.Board.Height; y++ {
		fmt.Fprint(r.buffer, "║")
		for x := 0; x < game.Board.Width; x++ {
			fmt.Fprint(r.buffer, string(board[y][x]))
		}
		fmt.Fprint(r.buffer, "║\n")
	}

	fmt.Fprint(r.buffer, "╚")
	for i := 0; i < game.Board.Width; i++ {
		fmt.Fprint(r.buffer, "═")
	}
	fmt.Fprint(r.buffer, "╝\n")
}

// drawHUD draws the heads-up display with score and instructions
func (r *Renderer) drawHUD(game *GameState) {
	fmt.Fprint(r.buffer, "\n")

	// Score info
	scoreStr := fmt.Sprintf("Score: %d | High Score: %d | Level: %d",
		game.GetScore(), game.HighScore, game.SpeedLevel)
	fmt.Fprint(r.buffer, scoreStr, "\n")

	// Game status
	var status string
	if game.GetGameOver() {
		status = "GAME OVER"
	} else if game.GetPaused() {
		status = "PAUSED"
	} else {
		status = "RUNNING"
	}
	fmt.Fprint(r.buffer, "Status: ", status, "\n")

	// Controls
	fmt.Fprint(r.buffer, "Controls: Arrow Keys/WASD - Move | Space - Pause | Q - Quit\n")
}

// RenderPaused renders a simple paused message overlay
func (r *Renderer) RenderPaused() error {
	r.buffer.Reset()
	r.moveCursor(r.height/2, r.width/2-5)
	fmt.Fprint(r.buffer, "PAUSED - Press Space to Resume")
	_, err := r.buffer.WriteTo(os.Stdout)
	return err
}

// clearScreen clears the terminal screen
func (r *Renderer) clearScreen() {
	fmt.Fprint(r.buffer, "\033[2J\033[H")
}

// moveCursor moves the cursor to the specified position
func (r *Renderer) moveCursor(row, col int) {
	fmt.Fprintf(r.buffer, "\033[%d;%dH", row, col)
}

// hideCursor hides the terminal cursor
func (r *Renderer) HideCursor() error {
	_, err := fmt.Fprint(os.Stdout, "\033[?25l")
	return err
}

// showCursor shows the terminal cursor
func (r *Renderer) ShowCursor() error {
	_, err := fmt.Fprint(os.Stdout, "\033[?25h")
	return err
}

// PrintStartScreen prints the starting screen
func PrintStartScreen() {
	fmt.Print("\033[2J\033[H") // Clear screen
	fmt.Println()
	fmt.Println("╔════════════════════════════════════════╗")
	fmt.Println("║                                        ║")
	fmt.Println("║            SNAKE CLI GAME              ║")
	fmt.Println("║                                        ║")
	fmt.Println("╚════════════════════════════════════════╝")
	fmt.Println()
	fmt.Println("Controls:")
	fmt.Println("  Arrow Keys or WASD - Move the snake")
	fmt.Println("  Space              - Pause/Resume game")
	fmt.Println("  Q                  - Quit game")
	fmt.Println()
	fmt.Println("Objective:")
	fmt.Println("  Eat food (●) to grow and gain points")
	fmt.Println("  Avoid walls and hitting yourself")
	fmt.Println("  Speed increases as you progress")
	fmt.Println()
	fmt.Print("Press any key to start...")
	// In real implementation, would wait for input here
}

// centerText centers text with padding
func centerText(text string, padding int) string {
	return strings.Repeat(" ", padding) + text
}

// HighScoreManager manages high score persistence
type HighScoreManager struct {
	filePath string
}

// NewHighScoreManager creates a new high score manager
func NewHighScoreManager(filePath string) *HighScoreManager {
	return &HighScoreManager{filePath: filePath}
}

// LoadHighScore loads the high score from file
func (hsm *HighScoreManager) LoadHighScore() int {
	// In a real implementation, would read from file
	// For now, return 0
	return 0
}

// SaveHighScore saves the high score to file
func (hsm *HighScoreManager) SaveHighScore(score int) error {
	// In a real implementation, would write to file
	// For now, just return nil
	return nil
}
