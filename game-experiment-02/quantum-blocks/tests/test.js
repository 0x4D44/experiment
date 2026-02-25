import { GameEngine, COLORS, BLOCK_TYPES, GAME_MODES, GRID_WIDTH, GRID_HEIGHT } from '../game.js';

// Simple test framework
class TestRunner {
  constructor() {
    this.tests = [];
    this.passed = 0;
    this.failed = 0;
  }

  test(name, fn) {
    this.tests.push({ name, fn });
  }

  async run() {
    console.log('Starting test suite...\n');

    for (const test of this.tests) {
      try {
        await test.fn();
        this.passed++;
        console.log(`✓ ${test.name}`);
      } catch (error) {
        this.failed++;
        console.error(`✗ ${test.name}`);
        console.error(`  Error: ${error.message}\n`);
      }
    }

    console.log(`\n${'='.repeat(50)}`);
    console.log(`Test Results: ${this.passed} passed, ${this.failed} failed`);
    console.log(`Total: ${this.passed + this.failed}\n`);

    return this.failed === 0;
  }
}

// Assertions
function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertEqual(actual, expected, message) {
  if (actual !== expected) {
    throw new Error(`${message}: expected ${expected}, got ${actual}`);
  }
}

function assertTrue(condition, message) {
  assert(condition, message);
}

function assertFalse(condition, message) {
  assert(!condition, message);
}

// Test suite
const runner = new TestRunner();

// Game Engine Creation Tests
runner.test('GameEngine initializes with correct grid size', () => {
  const engine = new GameEngine();
  assertEqual(engine.grid.length, GRID_HEIGHT, 'Grid height incorrect');
  assertEqual(engine.grid[0].length, GRID_WIDTH, 'Grid width incorrect');
});

runner.test('GameEngine initializes with correct default values', () => {
  const engine = new GameEngine(GAME_MODES.ZEN);
  assertEqual(engine.score, 0, 'Initial score should be 0');
  assertEqual(engine.level, 1, 'Initial level should be 1');
  assertFalse(engine.gameOver, 'Game should not be over initially');
  assertEqual(engine.mode, GAME_MODES.ZEN, 'Mode should be ZEN');
});

runner.test('GameEngine time-attack mode initializes with 180 seconds', () => {
  const engine = new GameEngine(GAME_MODES.TIME_ATTACK);
  assertEqual(engine.mode, GAME_MODES.TIME_ATTACK, 'Mode should be TIME_ATTACK');
  assertEqual(engine.timeRemaining, 180, 'Time should be 180 seconds');
});

// Block Spawning Tests
runner.test('SpawnNewBlock creates a block at the top', () => {
  const engine = new GameEngine();
  const spawned = engine.spawnNewBlock();
  assertTrue(spawned, 'Block should spawn successfully');

  let blockFound = false;
  for (let col = 0; col < GRID_WIDTH; col++) {
    if (engine.grid[0][col] !== null) {
      blockFound = true;
      break;
    }
  }
  assertTrue(blockFound, 'Block should be placed in top row');
});

runner.test('SpawnNewBlock returns false when grid is full', () => {
  const engine = new GameEngine();

  // Fill the top row
  for (let col = 0; col < GRID_WIDTH; col++) {
    engine.grid[0][col] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  }

  const spawned = engine.spawnNewBlock();
  assertFalse(spawned, 'Should not be able to spawn when top row is full');
  assertTrue(engine.gameOver, 'Game should be over');
});

// Block Movement Tests
runner.test('MoveBlockDown moves falling block down', () => {
  const engine = new GameEngine();
  engine.spawnNewBlock();

  // Find the spawned block
  let originalRow = -1;
  for (let row = 0; row < GRID_HEIGHT; row++) {
    if (engine.grid[row][0] !== null) {
      originalRow = row;
      break;
    }
  }

  const moved = engine.moveBlockDown(0);
  assertTrue(moved, 'Block should move down');

  // Check that block moved
  let newRow = -1;
  for (let row = 0; row < GRID_HEIGHT; row++) {
    if (engine.grid[row][0] !== null) {
      newRow = row;
      break;
    }
  }

  assertEqual(newRow, originalRow + 1, 'Block should move exactly one row down');
});

runner.test('MoveBlockLeft moves falling block left', () => {
  const engine = new GameEngine();
  engine.grid[10][5] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: true };

  const moved = engine.moveBlockLeft(5);
  assertTrue(moved, 'Block should move left');
  assertTrue(engine.grid[10][4] !== null, 'Block should be at column 4');
  assertTrue(engine.grid[10][5] === null, 'Original position should be empty');
});

runner.test('MoveBlockRight moves falling block right', () => {
  const engine = new GameEngine();
  engine.grid[10][5] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: true };

  const moved = engine.moveBlockRight(5);
  assertTrue(moved, 'Block should move right');
  assertTrue(engine.grid[10][6] !== null, 'Block should be at column 6');
  assertTrue(engine.grid[10][5] === null, 'Original position should be empty');
});

runner.test('MoveBlockLeft returns false at left boundary', () => {
  const engine = new GameEngine();
  engine.grid[10][0] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: true };

  const moved = engine.moveBlockLeft(0);
  assertFalse(moved, 'Block should not move at left boundary');
});

runner.test('MoveBlockRight returns false at right boundary', () => {
  const engine = new GameEngine();
  engine.grid[10][GRID_WIDTH - 1] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: true };

  const moved = engine.moveBlockRight(GRID_WIDTH - 1);
  assertFalse(moved, 'Block should not move at right boundary');
});

// Gravity Tests
runner.test('ApplyGravity makes blocks fall', () => {
  const engine = new GameEngine();
  engine.grid[5][0] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };

  engine.applyGravity();

  // Gravity should settle block all the way to the bottom
  assertTrue(engine.grid[GRID_HEIGHT - 1][0] !== null, 'Block should settle to bottom');
  assertTrue(engine.grid[5][0] === null, 'Original position should be empty');
});

runner.test('ApplyGravity stops at bottom', () => {
  const engine = new GameEngine();
  engine.grid[GRID_HEIGHT - 1][0] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };

  engine.applyGravity();

  assertTrue(engine.grid[GRID_HEIGHT - 1][0] !== null, 'Block should remain at bottom');
});

runner.test('ApplyGravity stops at collision', () => {
  const engine = new GameEngine();
  engine.grid[5][0] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  engine.grid[19][0] = { color: COLORS.BLUE, type: BLOCK_TYPES.NORMAL, falling: false }; // Block at bottom

  engine.applyGravity();

  // Block at row 5 should fall until it hits the bottom block
  assertTrue(engine.grid[18][0] !== null, 'Block should settle just above the blocking block');
  assertTrue(engine.grid[19][0] !== null, 'Bottom block should remain in place');
  assertTrue(engine.grid[5][0] === null, 'Original position should be empty');
});

// Match Detection Tests
runner.test('FindMatches detects horizontal matches', () => {
  const engine = new GameEngine();
  // Create a horizontal line of red blocks
  for (let col = 0; col < 3; col++) {
    engine.grid[10][col] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  }

  const matched = engine.findMatches();
  assertTrue(matched[10][0], 'Match should be detected at position 0');
  assertTrue(matched[10][1], 'Match should be detected at position 1');
  assertTrue(matched[10][2], 'Match should be detected at position 2');
});

runner.test('FindMatches detects vertical matches', () => {
  const engine = new GameEngine();
  // Create a vertical line of red blocks
  for (let row = 0; row < 3; row++) {
    engine.grid[row][5] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  }

  const matched = engine.findMatches();
  assertTrue(matched[0][5], 'Match should be detected at row 0');
  assertTrue(matched[1][5], 'Match should be detected at row 1');
  assertTrue(matched[2][5], 'Match should be detected at row 2');
});

runner.test('FindMatches detects diagonal matches', () => {
  const engine = new GameEngine();
  // Create a diagonal line of red blocks
  for (let i = 0; i < 3; i++) {
    engine.grid[i][i] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  }

  const matched = engine.findMatches();
  assertTrue(matched[0][0], 'Match should be detected at 0,0');
  assertTrue(matched[1][1], 'Match should be detected at 1,1');
  assertTrue(matched[2][2], 'Match should be detected at 2,2');
});

runner.test('FindMatches ignores matches of less than 3', () => {
  const engine = new GameEngine();
  // Create a line of only 2 blocks
  engine.grid[10][0] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  engine.grid[10][1] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };

  const matched = engine.findMatches();
  assertFalse(matched[10][0], 'Should not match less than 3 blocks');
  assertFalse(matched[10][1], 'Should not match less than 3 blocks');
});

// Clear Matches Tests
runner.test('ClearMatches removes matched blocks', () => {
  const engine = new GameEngine();
  // Create blocks and matches
  for (let col = 0; col < 3; col++) {
    engine.grid[10][col] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  }

  const matched = engine.findMatches();
  const cleared = engine.clearMatches(matched);

  assertEqual(cleared, 3, 'Should clear 3 blocks');
  assertTrue(engine.grid[10][0] === null, 'Block should be cleared');
  assertTrue(engine.grid[10][1] === null, 'Block should be cleared');
  assertTrue(engine.grid[10][2] === null, 'Block should be cleared');
});

runner.test('ClearMatches increases score', () => {
  const engine = new GameEngine();
  for (let col = 0; col < 3; col++) {
    engine.grid[10][col] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  }

  const startScore = engine.score;
  const matched = engine.findMatches();
  engine.clearMatches(matched);

  assertTrue(engine.score > startScore, 'Score should increase after clearing');
});

runner.test('ClearMatches applies chain multiplier', () => {
  const engine = new GameEngine();
  engine.chainMultiplier = 3;

  for (let col = 0; col < 3; col++) {
    engine.grid[10][col] = { color: COLORS.RED, type: BLOCK_TYPES.NORMAL, falling: false };
  }

  const matched = engine.findMatches();
  engine.clearMatches(matched);

  // Score should be: 3 blocks * 10 points * 3 multiplier = 90
  assertEqual(engine.score, 90, 'Chain multiplier should be applied');
});

// Bomb Tests
runner.test('ExplodeBomb clears surrounding blocks', () => {
  const engine = new GameEngine();
  engine.grid[10][5] = { color: COLORS.RED, type: BLOCK_TYPES.BOMB, falling: false };

  // Fill surrounding area
  for (let r = 8; r < 13; r++) {
    for (let c = 3; c < 8; c++) {
      if (!(r === 10 && c === 5)) {
        engine.grid[r][c] = { color: COLORS.BLUE, type: BLOCK_TYPES.NORMAL, falling: false };
      }
    }
  }

  engine.explodeBomb(10, 5);

  // Check that blocks in radius 2 are cleared
  for (let r = 8; r < 13; r++) {
    for (let c = 3; c < 8; c++) {
      assertTrue(engine.grid[r][c] === null, `Block at ${r},${c} should be cleared`);
    }
  }
});

// Game State Tests
runner.test('GetGameState returns valid state object', () => {
  const engine = new GameEngine();
  const state = engine.getGameState();

  assertTrue(state.grid !== undefined, 'State should have grid');
  assertTrue(state.score !== undefined, 'State should have score');
  assertTrue(state.level !== undefined, 'State should have level');
  assertTrue(state.gameOver !== undefined, 'State should have gameOver');
});

runner.test('Reset clears game state', () => {
  const engine = new GameEngine();
  engine.score = 1000;
  engine.gameOver = true;

  engine.reset();

  assertEqual(engine.score, 0, 'Score should reset to 0');
  assertFalse(engine.gameOver, 'GameOver should reset to false');
  assertEqual(engine.level, 1, 'Level should reset to 1');
});

// Run all tests
runner.run().then(success => {
  process.exit(success ? 0 : 1);
});
