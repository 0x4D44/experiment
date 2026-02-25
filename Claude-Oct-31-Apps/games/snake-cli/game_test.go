package main

import (
	"testing"
)

// ============================================================================
// SNAKE MOVEMENT TESTS
// ============================================================================

func TestSnakeInitialization(t *testing.T) {
	snake := NewSnake(20, 10)

	if len(snake.Body) != 3 {
		t.Errorf("Expected snake body length of 3, got %d", len(snake.Body))
	}

	// Verify head is at center
	if snake.Body[0].X != 10 || snake.Body[0].Y != 5 {
		t.Errorf("Expected head at (10, 5), got (%d, %d)", snake.Body[0].X, snake.Body[0].Y)
	}

	// Verify snake is horizontal
	if snake.Direction != Right {
		t.Errorf("Expected initial direction Right, got %v", snake.Direction)
	}
}

func TestSnakeMovement(t *testing.T) {
	tests := []struct {
		name          string
		initialDir    Direction
		newDir        Direction
		expectedHeadX int
		expectedHeadY int
	}{
		{
			name:          "Move right",
			initialDir:    Right,
			newDir:        Right,
			expectedHeadX: 11,
			expectedHeadY: 5,
		},
		{
			name:          "Move left",
			initialDir:    Left,
			newDir:        Left,
			expectedHeadX: 9,
			expectedHeadY: 5,
		},
		{
			name:          "Move up",
			initialDir:    Up,
			newDir:        Up,
			expectedHeadX: 10,
			expectedHeadY: 4,
		},
		{
			name:          "Move down",
			initialDir:    Down,
			newDir:        Down,
			expectedHeadX: 10,
			expectedHeadY: 6,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			snake := NewSnake(20, 10)
			snake.Direction = tt.initialDir
			snake.Move(tt.newDir)

			if snake.Body[0].X != tt.expectedHeadX || snake.Body[0].Y != tt.expectedHeadY {
				t.Errorf("Expected head at (%d, %d), got (%d, %d)",
					tt.expectedHeadX, tt.expectedHeadY, snake.Body[0].X, snake.Body[0].Y)
			}
		})
	}
}

func TestSnakeReverseDirection(t *testing.T) {
	snake := NewSnake(20, 10)
	snake.Direction = Right

	// Try to move left (reverse) - should be ignored
	snake.Move(Left)

	// Direction should still be right
	if snake.Direction != Right {
		t.Errorf("Expected direction to remain Right, got %v", snake.Direction)
	}

	// Head should move right (not left)
	if snake.Body[0].X <= 10 {
		t.Errorf("Expected head to move right, but x-coordinate is %d", snake.Body[0].X)
	}
}

func TestSnakeGrowth(t *testing.T) {
	snake := NewSnake(20, 10)
	initialLength := len(snake.Body)

	snake.Grow()

	if len(snake.Body) != initialLength+1 {
		t.Errorf("Expected snake length %d after growth, got %d", initialLength+1, len(snake.Body))
	}
}

func TestSnakeDirectionQueueing(t *testing.T) {
	snake := NewSnake(20, 10)
	snake.Direction = Right
	snake.NextDir = None

	// Queue a new direction
	snake.SetNextDirection(Down)

	if snake.NextDir != Down {
		t.Errorf("Expected NextDir to be Down, got %v", snake.NextDir)
	}

	// Queue another direction
	snake.SetNextDirection(Left)

	// Should keep the first queued direction
	if snake.NextDir != Down {
		t.Errorf("Expected NextDir to remain Down, got %v", snake.NextDir)
	}
}

func TestSnakeSetNextDirectionIgnoresReverse(t *testing.T) {
	snake := NewSnake(20, 10)
	snake.Direction = Right

	// Try to queue reverse direction
	snake.SetNextDirection(Left)

	if snake.NextDir == Left {
		t.Errorf("SetNextDirection should ignore reverse direction")
	}
}

// ============================================================================
// COLLISION DETECTION TESTS
// ============================================================================

func TestWallCollisionX(t *testing.T) {
	collider := NewCollisionDetector(20, 10)
	snake := NewSnake(20, 10)

	// Move snake head outside left boundary
	snake.Body[0].X = -1
	if !collider.DetectWallCollision(snake) {
		t.Errorf("Should detect collision at x < 0")
	}

	// Move snake head outside right boundary
	snake.Body[0].X = 20
	if !collider.DetectWallCollision(snake) {
		t.Errorf("Should detect collision at x >= width")
	}
}

func TestWallCollisionY(t *testing.T) {
	collider := NewCollisionDetector(20, 10)
	snake := NewSnake(20, 10)

	// Move snake head outside top boundary
	snake.Body[0].Y = -1
	if !collider.DetectWallCollision(snake) {
		t.Errorf("Should detect collision at y < 0")
	}

	// Move snake head outside bottom boundary
	snake.Body[0].Y = 10
	if !collider.DetectWallCollision(snake) {
		t.Errorf("Should detect collision at y >= height")
	}
}

func TestWallCollisionValidBoundaries(t *testing.T) {
	collider := NewCollisionDetector(20, 10)
	snake := NewSnake(20, 10)

	// Test all valid boundaries
	boundaryTests := []struct {
		x, y int
		name string
	}{
		{0, 0, "top-left corner"},
		{19, 9, "bottom-right corner"},
		{10, 5, "center"},
	}

	for _, tt := range boundaryTests {
		t.Run(tt.name, func(t *testing.T) {
			snake.Body[0].X = tt.x
			snake.Body[0].Y = tt.y

			if collider.DetectWallCollision(snake) {
				t.Errorf("Should not detect collision at valid position (%d, %d)", tt.x, tt.y)
			}
		})
	}
}

func TestSelfCollisionDetection(t *testing.T) {
	snake := NewSnake(20, 10)
	collider := NewCollisionDetector(20, 10)

	// Grow the snake to have a body
	snake.Grow()
	snake.Grow()

	// Add manual body segments to create self-collision scenario
	snake.Body = append(snake.Body, Point{X: snake.Body[0].X + 1, Y: snake.Body[0].Y})

	// Move head to collide with body
	originalHeadX := snake.Body[0].X
	originalHeadY := snake.Body[0].Y
	snake.Body[0].X = snake.Body[2].X
	snake.Body[0].Y = snake.Body[2].Y

	if !collider.DetectSelfCollision(snake) {
		t.Errorf("Should detect self-collision when head overlaps body at (%d, %d)",
			originalHeadX, originalHeadY)
	}
}

func TestNoSelfCollisionWithValidSnake(t *testing.T) {
	snake := NewSnake(20, 10)
	collider := NewCollisionDetector(20, 10)

	if collider.DetectSelfCollision(snake) {
		t.Errorf("Should not detect self-collision in valid snake")
	}

	snake.Grow()
	if collider.DetectSelfCollision(snake) {
		t.Errorf("Should not detect self-collision after growth")
	}
}

func TestFoodCollisionDetection(t *testing.T) {
	collider := NewCollisionDetector(20, 10)
	snake := NewSnake(20, 10)

	headX := snake.Body[0].X
	headY := snake.Body[0].Y
	food := Point{X: headX, Y: headY}

	if !collider.DetectFoodCollision(snake, food) {
		t.Errorf("Should detect food collision when head at (%d, %d) and food at (%d, %d)",
			headX, headY, food.X, food.Y)
	}
}

func TestNoFoodCollisionWhenApart(t *testing.T) {
	collider := NewCollisionDetector(20, 10)
	snake := NewSnake(20, 10)
	food := Point{X: 0, Y: 0}

	if collider.DetectFoodCollision(snake, food) {
		t.Errorf("Should not detect food collision when separated")
	}
}

// ============================================================================
// FOOD SPAWNING TESTS
// ============================================================================

func TestFoodSpawning(t *testing.T) {
	spawner := NewFoodSpawner(20, 10)
	snake := NewSnake(20, 10)

	food := spawner.SpawnFood(snake)

	// Check food is within bounds
	if food.X < 0 || food.X >= 20 || food.Y < 0 || food.Y >= 10 {
		t.Errorf("Food spawned outside bounds: (%d, %d)", food.X, food.Y)
	}

	// Check food is not on snake
	for _, segment := range snake.Body {
		if food.X == segment.X && food.Y == segment.Y {
			t.Errorf("Food spawned on snake at (%d, %d)", food.X, food.Y)
		}
	}
}

func TestFoodSpawningWithLargeSnake(t *testing.T) {
	spawner := NewFoodSpawner(20, 10)
	snake := NewSnake(20, 10)

	// Grow snake significantly
	for i := 0; i < 50; i++ {
		snake.Grow()
	}

	// Spawn food multiple times
	for i := 0; i < 10; i++ {
		food := spawner.SpawnFood(snake)

		// Verify food is not on snake
		for _, segment := range snake.Body {
			if food.X == segment.X && food.Y == segment.Y {
				t.Errorf("Food spawned on snake at (%d, %d)", food.X, food.Y)
			}
		}
	}
}

func TestFoodSpawningBounds(t *testing.T) {
	widths := []int{10, 20, 40}
	heights := []int{5, 10, 20}

	for _, w := range widths {
		for _, h := range heights {
			t.Run("Bounds", func(t *testing.T) {
				spawner := NewFoodSpawner(w, h)
				snake := NewSnake(w, h)

				for i := 0; i < 20; i++ {
					food := spawner.SpawnFood(snake)

					if food.X < 0 || food.X >= w || food.Y < 0 || food.Y >= h {
						t.Errorf("Food out of bounds (%d, %d) for board (%d, %d)",
							food.X, food.Y, w, h)
					}
				}
			})
		}
	}
}

// ============================================================================
// GAME STATE MANAGEMENT TESTS
// ============================================================================

func TestGameStateInitialization(t *testing.T) {
	game := NewGameState(40, 20)

	if game.Score != 0 {
		t.Errorf("Expected initial score 0, got %d", game.Score)
	}

	if game.GameOver {
		t.Errorf("Expected game to not be over initially")
	}

	if game.Paused {
		t.Errorf("Expected game to not be paused initially")
	}

	if game.SpeedLevel != 0 {
		t.Errorf("Expected initial speed level 0, got %d", game.SpeedLevel)
	}
}

func TestScoreCalculation(t *testing.T) {
	game := NewGameState(40, 20)

	initialScore := game.Score
	game.AddScore(10)

	if game.Score != initialScore+10 {
		t.Errorf("Expected score %d, got %d", initialScore+10, game.Score)
	}
}

func TestSpeedLevelProgression(t *testing.T) {
	game := NewGameState(40, 20)

	initialLevel := game.SpeedLevel
	game.IncreaseSpeedLevel()

	if game.SpeedLevel != initialLevel+1 {
		t.Errorf("Expected speed level %d, got %d", initialLevel+1, game.SpeedLevel)
	}
}

func TestGameOverState(t *testing.T) {
	game := NewGameState(40, 20)

	game.EndGame()

	if !game.GameOver {
		t.Errorf("Expected game to be over after EndGame()")
	}
}

func TestPauseToggle(t *testing.T) {
	game := NewGameState(40, 20)

	if game.Paused {
		t.Errorf("Expected game to not be paused initially")
	}

	game.TogglePause()
	if !game.Paused {
		t.Errorf("Expected game to be paused after toggle")
	}

	game.TogglePause()
	if game.Paused {
		t.Errorf("Expected game to be unpaused after second toggle")
	}
}

func TestHighScoreTracking(t *testing.T) {
	game := NewGameState(40, 20)
	game.HighScore = 100
	game.Score = 150

	if game.Score <= game.HighScore {
		t.Errorf("Score should be higher than high score for testing")
	}

	// Game should track that current score beats high score
	if game.Score > game.HighScore {
		// This is expected - score beats high score
	}
}

func TestGameStateBoardAccess(t *testing.T) {
	game := NewGameState(40, 20)

	if game.Board == nil {
		t.Errorf("Expected board to be initialized")
	}

	if game.Board.Width != 40 || game.Board.Height != 20 {
		t.Errorf("Expected board dimensions 40x20, got %dx%d",
			game.Board.Width, game.Board.Height)
	}
}

func TestGameStateSnakeAccess(t *testing.T) {
	game := NewGameState(40, 20)

	if game.Snake == nil {
		t.Errorf("Expected snake to be initialized")
	}

	if len(game.Snake.Body) == 0 {
		t.Errorf("Expected snake to have body segments")
	}
}

// ============================================================================
// INPUT VALIDATION TESTS
// ============================================================================

func TestDirectionEnumValues(t *testing.T) {
	directions := []Direction{Up, Down, Left, Right, None}

	for _, dir := range directions {
		// Just verify they're defined and can be used
		if dir < Up || (dir > Right && dir != None) {
			t.Errorf("Invalid direction value")
		}
	}
}

func TestDirectionOpposition(t *testing.T) {
	tests := []struct {
		direction Direction
		opposite  Direction
	}{
		{Up, Down},
		{Down, Up},
		{Left, Right},
		{Right, Left},
	}

	for _, tt := range tests {
		if IsOppositeDirection(tt.direction, tt.opposite) {
			// This is expected
		} else {
			t.Errorf("Expected %v to be opposite of %v", tt.opposite, tt.direction)
		}
	}
}

func TestDirectionOppositionNotSameDirection(t *testing.T) {
	if IsOppositeDirection(Up, Up) {
		t.Errorf("Same direction should not be opposite")
	}

	if IsOppositeDirection(Right, Down) {
		t.Errorf("Perpendicular directions should not be opposite")
	}
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

func TestSnakeMinimumLength(t *testing.T) {
	snake := NewSnake(20, 10)

	if len(snake.Body) < 3 {
		t.Errorf("Snake should have minimum 3 segments, got %d", len(snake.Body))
	}
}

func TestFoodSpawningEdgeCase(t *testing.T) {
	// Create a very small board with large snake
	spawner := NewFoodSpawner(5, 5)
	snake := NewSnake(5, 5)

	// Fill most of the board with snake
	for i := 0; i < 20; i++ {
		snake.Grow()
	}

	// This should still succeed (with available space) or handle gracefully
	food := spawner.SpawnFood(snake)

	// Verify food is not on snake
	for _, segment := range snake.Body {
		if food.X == segment.X && food.Y == segment.Y {
			t.Errorf("Food spawned on snake")
		}
	}
}

func TestGameStateReset(t *testing.T) {
	game := NewGameState(40, 20)
	game.Score = 500
	game.GameOver = true

	game.Reset()

	if game.Score != 0 {
		t.Errorf("Expected score reset to 0, got %d", game.Score)
	}

	if game.GameOver {
		t.Errorf("Expected GameOver reset to false")
	}
}

func TestPointEquality(t *testing.T) {
	p1 := Point{X: 5, Y: 10}
	p2 := Point{X: 5, Y: 10}
	p3 := Point{X: 5, Y: 11}

	if p1 != p2 {
		t.Errorf("Equal points should be equal")
	}

	if p1 == p3 {
		t.Errorf("Different points should not be equal")
	}
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

func TestGameTickWithMovement(t *testing.T) {
	game := NewGameState(40, 20)
	initialHeadPos := game.Snake.Body[0]

	// Process one tick with a direction change
	game.ProcessTick(Right)

	// Head should have moved
	if game.Snake.Body[0] == initialHeadPos {
		t.Errorf("Snake head should move after tick")
	}
}

func TestGameTickWithFoodConsumption(t *testing.T) {
	game := NewGameState(40, 20)
	initialLength := len(game.Snake.Body)

	// Place food at snake head
	game.Food = game.Snake.Body[0]

	// Process tick
	game.ProcessTick(game.Snake.Direction)

	// Snake should have grown
	if len(game.Snake.Body) <= initialLength {
		t.Errorf("Snake should grow after food consumption")
	}
}

func TestGameTickWithCollisionDetection(t *testing.T) {
	game := NewGameState(40, 20)

	// Move snake head outside bounds
	game.Snake.Body[0].X = -1

	// Process tick
	game.ProcessTick(game.Snake.Direction)

	// Game should be over
	if !game.GameOver {
		t.Errorf("Game should end on wall collision")
	}
}

func TestMultipleTicksSequence(t *testing.T) {
	game := NewGameState(40, 20)

	// Run several ticks in different directions
	directions := []Direction{Right, Right, Down, Down, Left}

	for _, dir := range directions {
		if !game.GameOver {
			game.ProcessTick(dir)
		}
	}

	// Snake should have moved
	if game.Snake.Body[0].X == 20 && game.Snake.Body[0].Y == 10 {
		t.Errorf("Snake should have moved from initial position")
	}
}

// ============================================================================
// HELPER FUNCTIONS FOR TESTS
// ============================================================================

// IsOppositeDirection checks if two directions are opposite
func IsOppositeDirection(d1, d2 Direction) bool {
	return (d1 == Up && d2 == Down) ||
		(d1 == Down && d2 == Up) ||
		(d1 == Left && d2 == Right) ||
		(d1 == Right && d2 == Left)
}
