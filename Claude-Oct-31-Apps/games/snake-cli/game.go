package main

import (
	"sync"
	"time"
)

// Point represents a coordinate on the game board
type Point struct {
	X, Y int
}

// Direction represents the direction of snake movement
type Direction int

const (
	None  Direction = -1
	Up    Direction = 0
	Down  Direction = 1
	Left  Direction = 2
	Right Direction = 3
)

// Board represents the game playing area
type Board struct {
	Width  int
	Height int
}

// Snake represents the snake in the game
type Snake struct {
	Body      []Point   // Body[0] is head, Body[len-1] is tail
	Direction Direction // Current direction
	NextDir   Direction // Buffered next direction
	mu        sync.Mutex
}

// GameState represents the overall game state
type GameState struct {
	Board        *Board
	Snake        *Snake
	Food         Point
	Score        int
	HighScore    int
	GameOver     bool
	Paused       bool
	TickCount    int
	SpeedLevel   int
	TicksPerMove int
	Collider     *CollisionDetector
	Spawner      *FoodSpawner
	mu           sync.RWMutex
}

// CollisionDetector handles collision detection logic
type CollisionDetector struct {
	Width  int
	Height int
}

// FoodSpawner handles food generation
type FoodSpawner struct {
	Width  int
	Height int
}

// NewSnake creates a new snake with initial size of 3 segments
func NewSnake(width, height int) *Snake {
	centerX := width / 2
	centerY := height / 2

	return &Snake{
		Body: []Point{
			{X: centerX, Y: centerY},
			{X: centerX - 1, Y: centerY},
			{X: centerX - 2, Y: centerY},
		},
		Direction: Right,
		NextDir:   None,
	}
}

// Move updates the snake's direction and moves it forward
func (s *Snake) Move(nextDir Direction) {
	s.mu.Lock()
	defer s.mu.Unlock()

	// Use buffered next direction if available
	if s.NextDir != None {
		nextDir = s.NextDir
		s.NextDir = None
	}

	// Prevent reverse movement
	if !isOpposite(s.Direction, nextDir) && nextDir != None {
		s.Direction = nextDir
	}

	// Calculate new head position
	newHead := s.Body[0]
	switch s.Direction {
	case Up:
		newHead.Y--
	case Down:
		newHead.Y++
	case Left:
		newHead.X--
	case Right:
		newHead.X++
	}

	// Remove tail and add new head
	s.Body = append([]Point{newHead}, s.Body[:len(s.Body)-1]...)
}

// Grow increases the snake's length by one segment
func (s *Snake) Grow() {
	s.mu.Lock()
	defer s.mu.Unlock()

	// Add a new segment at the tail
	tail := s.Body[len(s.Body)-1]
	s.Body = append(s.Body, tail)
}

// SetNextDirection sets the buffered next direction
func (s *Snake) SetNextDirection(dir Direction) {
	s.mu.Lock()
	defer s.mu.Unlock()

	// Only set if no direction is already buffered and it's not opposite to current
	if s.NextDir == None && !isOpposite(s.Direction, dir) && dir != None {
		s.NextDir = dir
	}
}

// GetBody returns a copy of the snake's body for safe reading
func (s *Snake) GetBody() []Point {
	s.mu.Lock()
	defer s.mu.Unlock()

	bodyCopy := make([]Point, len(s.Body))
	copy(bodyCopy, s.Body)
	return bodyCopy
}

// GetHead returns the head position
func (s *Snake) GetHead() Point {
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.Body[0]
}

// GetDirection returns the current direction
func (s *Snake) GetDirection() Direction {
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.Direction
}

// NewCollisionDetector creates a new collision detector
func NewCollisionDetector(width, height int) *CollisionDetector {
	return &CollisionDetector{Width: width, Height: height}
}

// DetectWallCollision checks if the snake has hit a wall
func (cd *CollisionDetector) DetectWallCollision(snake *Snake) bool {
	head := snake.GetHead()
	return head.X < 0 || head.X >= cd.Width ||
		head.Y < 0 || head.Y >= cd.Height
}

// DetectSelfCollision checks if the snake has collided with itself
func (cd *CollisionDetector) DetectSelfCollision(snake *Snake) bool {
	body := snake.GetBody()
	head := body[0]

	// Check if head overlaps with any body segment (starting from segment 1, not head)
	for i := 1; i < len(body); i++ {
		if body[i].X == head.X && body[i].Y == head.Y {
			return true
		}
	}
	return false
}

// DetectFoodCollision checks if the snake has eaten food
func (cd *CollisionDetector) DetectFoodCollision(snake *Snake, food Point) bool {
	head := snake.GetHead()
	return head.X == food.X && head.Y == food.Y
}

// NewFoodSpawner creates a new food spawner
func NewFoodSpawner(width, height int) *FoodSpawner {
	return &FoodSpawner{Width: width, Height: height}
}

// SpawnFood generates a new food position that doesn't overlap with snake
func (fs *FoodSpawner) SpawnFood(snake *Snake) Point {
	body := snake.GetBody()
	bodyMap := make(map[Point]bool)

	for _, segment := range body {
		bodyMap[segment] = true
	}

	// Try to find a valid position
	maxAttempts := fs.Width * fs.Height
	for attempts := 0; attempts < maxAttempts; attempts++ {
		// Simple pseudo-random based on time and attempts
		seed := (time.Now().UnixNano() + int64(attempts)) % int64(fs.Width*fs.Height)
		x := int(seed) % fs.Width
		y := int(seed/int64(fs.Width)) % fs.Height

		food := Point{X: x, Y: y}
		if !bodyMap[food] {
			return food
		}
	}

	// Fallback: return first available position
	for x := 0; x < fs.Width; x++ {
		for y := 0; y < fs.Height; y++ {
			pos := Point{X: x, Y: y}
			if !bodyMap[pos] {
				return pos
			}
		}
	}

	// If no space available, return center (shouldn't happen in normal gameplay)
	return Point{X: fs.Width / 2, Y: fs.Height / 2}
}

// NewGameState creates a new game instance
func NewGameState(width, height int) *GameState {
	return &GameState{
		Board:        &Board{Width: width, Height: height},
		Snake:        NewSnake(width, height),
		Food:         Point{X: width / 4, Y: height / 2},
		Score:        0,
		HighScore:    0,
		GameOver:     false,
		Paused:       false,
		TickCount:    0,
		SpeedLevel:   0,
		TicksPerMove: 10, // 100ms per move at 10 FPS
		Collider:     NewCollisionDetector(width, height),
		Spawner:      NewFoodSpawner(width, height),
	}
}

// ProcessTick updates the game state for one tick
func (g *GameState) ProcessTick(nextDir Direction) {
	g.mu.Lock()
	defer g.mu.Unlock()

	if g.GameOver || g.Paused {
		return
	}

	g.TickCount++

	// Move snake only at tick intervals based on speed
	if g.TickCount%g.TicksPerMove == 0 {
		g.Snake.Move(nextDir)

		// Check collisions
		if g.Collider.DetectWallCollision(g.Snake) ||
			g.Collider.DetectSelfCollision(g.Snake) {
			g.GameOver = true
			return
		}

		// Check food consumption
		if g.Collider.DetectFoodCollision(g.Snake, g.Food) {
			g.Snake.Grow()
			g.AddScoreLocked(10 + g.SpeedLevel*5)

			// Increase speed every 5 food items
			if g.Score%(5*10) == 0 && g.Score > 0 {
				g.IncreaseSpeedLevelLocked()
			}

			// Spawn new food
			g.Food = g.Spawner.SpawnFood(g.Snake)
		}
	}
}

// AddScore adds points to the score (thread-safe)
func (g *GameState) AddScore(points int) {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.AddScoreLocked(points)
}

// AddScoreLocked adds score without locking (use when already locked)
func (g *GameState) AddScoreLocked(points int) {
	g.Score += points
}

// IncreaseSpeedLevel increases difficulty
func (g *GameState) IncreaseSpeedLevel() {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.IncreaseSpeedLevelLocked()
}

// IncreaseSpeedLevelLocked increases speed without locking
func (g *GameState) IncreaseSpeedLevelLocked() {
	g.SpeedLevel++
	// Reduce ticks per move, but not below 3
	newTicks := int(float64(g.TicksPerMove) * 0.95)
	if newTicks < 3 {
		newTicks = 3
	}
	g.TicksPerMove = newTicks
}

// TogglePause pauses/unpauses the game
func (g *GameState) TogglePause() {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.Paused = !g.Paused
}

// EndGame marks the game as over
func (g *GameState) EndGame() {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.GameOver = true
}

// Reset resets the game state for a new game
func (g *GameState) Reset() {
	g.mu.Lock()
	defer g.mu.Unlock()

	g.Snake = NewSnake(g.Board.Width, g.Board.Height)
	g.Food = g.Spawner.SpawnFood(g.Snake)
	g.Score = 0
	g.GameOver = false
	g.Paused = false
	g.TickCount = 0
	g.SpeedLevel = 0
	g.TicksPerMove = 10
}

// GetScore returns current score
func (g *GameState) GetScore() int {
	g.mu.RLock()
	defer g.mu.RUnlock()
	return g.Score
}

// GetGameOver returns game over status
func (g *GameState) GetGameOver() bool {
	g.mu.RLock()
	defer g.mu.RUnlock()
	return g.GameOver
}

// GetPaused returns pause status
func (g *GameState) GetPaused() bool {
	g.mu.RLock()
	defer g.mu.RUnlock()
	return g.Paused
}

// GetFood returns food position
func (g *GameState) GetFood() Point {
	g.mu.RLock()
	defer g.mu.RUnlock()
	return g.Food
}

// isOpposite checks if two directions are opposite
func isOpposite(d1, d2 Direction) bool {
	return (d1 == Up && d2 == Down) ||
		(d1 == Down && d2 == Up) ||
		(d1 == Left && d2 == Right) ||
		(d1 == Right && d2 == Left)
}
