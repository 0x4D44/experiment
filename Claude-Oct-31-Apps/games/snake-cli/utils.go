package main

import (
	"fmt"
	"math/rand"
	"time"
)

// Note: 'time' is imported above

// Version information
const (
	AppName    = "Snake CLI"
	AppVersion = "1.0.0"
)

// RandomPoint generates a random point within the specified bounds
func RandomPoint(width, height int) Point {
	rand.Seed(time.Now().UnixNano())
	return Point{
		X: rand.Intn(width),
		Y: rand.Intn(height),
	}
}

// RandomPointExcluding generates a random point that's not in the excluded list
func RandomPointExcluding(width, height int, excluded []Point) Point {
	excludedMap := make(map[Point]bool)
	for _, p := range excluded {
		excludedMap[p] = true
	}

	maxAttempts := width * height
	for attempts := 0; attempts < maxAttempts; attempts++ {
		p := RandomPoint(width, height)
		if !excludedMap[p] {
			return p
		}
	}

	// Fallback: find first available
	for x := 0; x < width; x++ {
		for y := 0; y < height; y++ {
			p := Point{X: x, Y: y}
			if !excludedMap[p] {
				return p
			}
		}
	}

	return Point{X: 0, Y: 0}
}

// Contains checks if a point is in a slice of points
func Contains(points []Point, p Point) bool {
	for _, point := range points {
		if point == p {
			return true
		}
	}
	return false
}

// DistanceManhattan calculates the Manhattan distance between two points
func DistanceManhattan(p1, p2 Point) int {
	dx := p1.X - p2.X
	if dx < 0 {
		dx = -dx
	}
	dy := p1.Y - p2.Y
	if dy < 0 {
		dy = -dy
	}
	return dx + dy
}

// DistanceEuclidean calculates the Euclidean distance between two points
func DistanceEuclidean(p1, p2 Point) float64 {
	dx := float64(p1.X - p2.X)
	dy := float64(p1.Y - p2.Y)
	return (dx*dx + dy*dy) // square root not needed for comparison
}

// FormatScore formats a score with thousands separators
func FormatScore(score int) string {
	if score < 1000 {
		return fmt.Sprintf("%d", score)
	}
	if score < 1000000 {
		return fmt.Sprintf("%d,%03d", score/1000, score%1000)
	}
	return fmt.Sprintf("%d,%03d,%03d", score/1000000, (score/1000)%1000, score%1000)
}

// GameStats tracks overall game statistics
type GameStats struct {
	GamesPlayed    int
	GamesWon       int
	TotalScore     int
	HighScore      int
	TotalFoodEaten int
	AverageScore   float64
}

// UpdateStats updates game statistics after a game ends
func (gs *GameStats) UpdateStats(finalScore int, foodEaten int) {
	gs.GamesPlayed++
	gs.TotalScore += finalScore
	gs.TotalFoodEaten += foodEaten
	gs.AverageScore = float64(gs.TotalScore) / float64(gs.GamesPlayed)

	if finalScore > gs.HighScore {
		gs.HighScore = finalScore
	}
}

// String returns a formatted string of statistics
func (gs *GameStats) String() string {
	return fmt.Sprintf(`
Game Statistics:
  Games Played:     %d
  High Score:       %d
  Total Score:      %d
  Average Score:    %.1f
  Total Food Eaten: %d
  Win Rate:         %.1f%%
`,
		gs.GamesPlayed,
		gs.HighScore,
		gs.TotalScore,
		gs.AverageScore,
		gs.TotalFoodEaten,
		float64(gs.GamesWon)*100/float64(gs.GamesPlayed),
	)
}

// TimeFormat formats a duration as MM:SS
func TimeFormat(duration time.Duration) string {
	totalSeconds := int(duration.Seconds())
	minutes := totalSeconds / 60
	seconds := totalSeconds % 60
	return fmt.Sprintf("%02d:%02d", minutes, seconds)
}

// TicksToSeconds converts ticks to seconds given the FPS
func TicksToSeconds(ticks int, fps int) float64 {
	return float64(ticks) / float64(fps)
}

// SecondsToTicks converts seconds to ticks given the FPS
func SecondsToTicks(seconds float64, fps int) int {
	return int(seconds * float64(fps))
}

// IsValidDirection checks if a direction is valid
func IsValidDirection(d Direction) bool {
	return d >= Up && d <= Right
}

// OppositeDirection returns the opposite direction
func OppositeDirection(d Direction) Direction {
	switch d {
	case Up:
		return Down
	case Down:
		return Up
	case Left:
		return Right
	case Right:
		return Left
	default:
		return None
	}
}

// PerpendicularDirections returns the two directions perpendicular to the given direction
func PerpendicularDirections(d Direction) (Direction, Direction) {
	switch d {
	case Up, Down:
		return Left, Right
	case Left, Right:
		return Up, Down
	default:
		return None, None
	}
}

// DisplayMessage shows a temporary message on screen
type DisplayMessage struct {
	Text      string
	Duration  time.Duration
	CreatedAt time.Time
}

// IsExpired checks if the message should no longer be displayed
func (dm *DisplayMessage) IsExpired() bool {
	return time.Since(dm.CreatedAt) > dm.Duration
}

// NewDisplayMessage creates a new display message with the specified duration
func NewDisplayMessage(text string, duration time.Duration) *DisplayMessage {
	return &DisplayMessage{
		Text:      text,
		Duration:  duration,
		CreatedAt: time.Now(),
	}
}

// Logger provides simple logging functionality
type Logger struct {
	enabled bool
}

// NewLogger creates a new logger
func NewLogger(enabled bool) *Logger {
	return &Logger{enabled: enabled}
}

// Log logs a message if logging is enabled
func (l *Logger) Log(format string, args ...interface{}) {
	if l.enabled {
		fmt.Printf("[GAME] "+format+"\n", args...)
	}
}

// LogError logs an error message
func (l *Logger) LogError(format string, args ...interface{}) {
	if l.enabled {
		fmt.Printf("[ERROR] "+format+"\n", args...)
	}
}

// LogDebug logs a debug message
func (l *Logger) LogDebug(format string, args ...interface{}) {
	if l.enabled {
		fmt.Printf("[DEBUG] "+format+"\n", args...)
	}
}

// ProgramInfo returns information about the program
func ProgramInfo() string {
	return fmt.Sprintf(`
╔════════════════════════════════════════╗
║         SNAKE CLI v%s                ║
║      A Terminal-Based Snake Game      ║
╚════════════════════════════════════════╝

Controls:
  Arrow Keys / WASD - Move
  Space            - Pause
  Q                - Quit
  R                - Restart

Objective:
  Eat food (●) to grow and score points
  Avoid walls and yourself
  Speed increases as you progress!

Good luck!
`, AppVersion)
}
