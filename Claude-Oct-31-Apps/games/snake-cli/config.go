package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// Config holds the game configuration
type Config struct {
	BoardWidth     int
	BoardHeight    int
	InitialSpeed   int
	EnableColors   bool
	HighScorePath  string
	MaxSpeedLevel  int
	MinTicksPerMove int
}

// DefaultConfig returns a configuration with default values
func DefaultConfig() *Config {
	homeDir, _ := os.UserHomeDir()
	highScorePath := filepath.Join(homeDir, ".snake-cli-highscore")

	return &Config{
		BoardWidth:      40,
		BoardHeight:     20,
		InitialSpeed:    10,
		EnableColors:    true,
		HighScorePath:   highScorePath,
		MaxSpeedLevel:   20,
		MinTicksPerMove: 3,
	}
}

// Validate checks if the configuration is valid
func (c *Config) Validate() error {
	if c.BoardWidth < 10 || c.BoardWidth > 200 {
		return fmt.Errorf("board width must be between 10 and 200, got %d", c.BoardWidth)
	}

	if c.BoardHeight < 5 || c.BoardHeight > 50 {
		return fmt.Errorf("board height must be between 5 and 50, got %d", c.BoardHeight)
	}

	if c.InitialSpeed < 1 {
		return fmt.Errorf("initial speed must be at least 1, got %d", c.InitialSpeed)
	}

	if c.MinTicksPerMove < 1 {
		return fmt.Errorf("min ticks per move must be at least 1, got %d", c.MinTicksPerMove)
	}

	return nil
}

// String returns a string representation of the configuration
func (c *Config) String() string {
	var sb strings.Builder
	sb.WriteString("Game Configuration:\n")
	sb.WriteString(fmt.Sprintf("  Board Size: %dx%d\n", c.BoardWidth, c.BoardHeight))
	sb.WriteString(fmt.Sprintf("  Initial Speed: %d ticks per move\n", c.InitialSpeed))
	sb.WriteString(fmt.Sprintf("  Colors: %v\n", c.EnableColors))
	sb.WriteString(fmt.Sprintf("  High Score File: %s\n", c.HighScorePath))
	return sb.String()
}

// SaveHighScore writes the high score to the configured file
func (c *Config) SaveHighScore(score int) error {
	dir := filepath.Dir(c.HighScorePath)

	// Create directory if it doesn't exist
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		if err := os.MkdirAll(dir, 0755); err != nil {
			return fmt.Errorf("failed to create high score directory: %w", err)
		}
	}

	// Write score to file
	scoreStr := strconv.Itoa(score)
	err := os.WriteFile(c.HighScorePath, []byte(scoreStr), 0644)
	if err != nil {
		return fmt.Errorf("failed to write high score: %w", err)
	}

	return nil
}

// LoadHighScore reads the high score from the configured file
func (c *Config) LoadHighScore() int {
	data, err := os.ReadFile(c.HighScorePath)
	if err != nil {
		// File doesn't exist or can't be read, return 0
		return 0
	}

	score, err := strconv.Atoi(strings.TrimSpace(string(data)))
	if err != nil {
		// Invalid format, return 0
		return 0
	}

	return score
}

// GameDifficulty represents a difficulty level
type GameDifficulty int

const (
	Easy GameDifficulty = iota
	Normal
	Hard
	Extreme
)

// DifficultyConfig returns configuration for a specific difficulty
func DifficultyConfig(difficulty GameDifficulty) *Config {
	config := DefaultConfig()

	switch difficulty {
	case Easy:
		config.InitialSpeed = 15 // Slower
		config.BoardWidth = 30
		config.BoardHeight = 15

	case Normal:
		// Use defaults

	case Hard:
		config.InitialSpeed = 8 // Faster
		config.BoardWidth = 50
		config.BoardHeight = 25

	case Extreme:
		config.InitialSpeed = 5 // Much faster
		config.BoardWidth = 60
		config.BoardHeight = 30
	}

	return config
}

// SpeedProgression calculates the ticks per move for a given speed level
func SpeedProgression(baseTicksPerMove int, speedLevel int, minTicks int) int {
	// Each level reduces speed by 5%
	current := baseTicksPerMove
	for i := 0; i < speedLevel; i++ {
		current = int(float64(current) * 0.95)
		if current < minTicks {
			return minTicks
		}
	}
	return current
}

// ScoreCalculation calculates points for eating food at a given speed level
func ScoreCalculation(basePoints int, speedLevel int) int {
	return basePoints + (speedLevel * 5)
}

// TerminalCapabilities represents the terminal's capabilities
type TerminalCapabilities struct {
	SupportsColor     bool
	SupportsUnicode   bool
	Width             int
	Height            int
	SupportsRawMode   bool
}

// DetectTerminalCapabilities attempts to detect terminal capabilities
func DetectTerminalCapabilities() *TerminalCapabilities {
	// In a real implementation, would detect actual capabilities
	// For now, assume most modern terminals support these features
	return &TerminalCapabilities{
		SupportsColor:   true,
		SupportsUnicode: true,
		Width:           80,  // Default terminal width
		Height:          24,  // Default terminal height
		SupportsRawMode: true,
	}
}

// ThemeConfig defines the visual theme
type ThemeConfig struct {
	SnakeHead     string
	SnakeBody     string
	Food          string
	BorderH       string
	BorderV       string
	CornerTL      string
	CornerTR      string
	CornerBL      string
	CornerBR      string
	EmptyCell     string
	BackgroundColor string
	SnakeColor    string
	FoodColor     string
}

// DefaultTheme returns the default visual theme
func DefaultTheme() *ThemeConfig {
	return &ThemeConfig{
		SnakeHead:   "@",
		SnakeBody:   "█",
		Food:        "●",
		BorderH:     "═",
		BorderV:     "║",
		CornerTL:    "╔",
		CornerTR:    "╗",
		CornerBL:    "╚",
		CornerBR:    "╝",
		EmptyCell:   " ",
	}
}

// AsciiTheme returns an ASCII-only theme for compatibility
func AsciiTheme() *ThemeConfig {
	return &ThemeConfig{
		SnakeHead:   "@",
		SnakeBody:   "#",
		Food:        "*",
		BorderH:     "-",
		BorderV:     "|",
		CornerTL:    "+",
		CornerTR:    "+",
		CornerBL:    "+",
		CornerBR:    "+",
		EmptyCell:   " ",
	}
}
