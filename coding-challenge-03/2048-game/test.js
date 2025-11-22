/**
 * Node.js Test Runner for 2048 Game Logic
 * Run with: node test.js
 */

const { Tile, Grid, GameManager } = require('./game.js');

let passed = 0;
let failed = 0;

function assert(condition, message) {
  if (!condition) {
    throw new Error(message || 'Assertion failed');
  }
}

function assertEqual(actual, expected, message) {
  if (actual !== expected) {
    throw new Error(message || `Expected ${expected}, but got ${actual}`);
  }
}

function test(name, fn) {
  try {
    fn();
    console.log(`✓ ${name}`);
    passed++;
  } catch (error) {
    console.log(`✗ ${name}`);
    console.log(`  Error: ${error.message}`);
    failed++;
  }
}

console.log('Running 2048 Game Tests...\n');

// Tile Tests
test('Tile: should create tile with correct properties', () => {
  const tile = new Tile(2, 1, 1);
  assertEqual(tile.value, 2);
  assertEqual(tile.row, 1);
  assertEqual(tile.col, 1);
});

test('Tile: should save and update position', () => {
  const tile = new Tile(2, 1, 1);
  tile.savePosition();
  assertEqual(tile.previousPosition.row, 1);
  tile.updatePosition({ row: 2, col: 2 });
  assertEqual(tile.row, 2);
});

// Grid Tests
test('Grid: should create empty grid', () => {
  const grid = new Grid(4);
  assertEqual(grid.size, 4);
  assertEqual(grid.cells.length, 4);
  assert(grid.cells[0][0] === null);
});

test('Grid: should insert and remove tiles', () => {
  const grid = new Grid(4);
  const tile = new Tile(2, 1, 1);
  grid.insertTile(tile);
  assert(grid.cellOccupied({ row: 1, col: 1 }));
  grid.removeTile(tile);
  assert(!grid.cellOccupied({ row: 1, col: 1 }));
});

test('Grid: should find available cells', () => {
  const grid = new Grid(4);
  let available = grid.availableCells();
  assertEqual(available.length, 16);
  grid.insertTile(new Tile(2, 0, 0));
  available = grid.availableCells();
  assertEqual(available.length, 15);
});

// GameManager Tests
test('GameManager: should initialize correctly', () => {
  const game = new GameManager(4);
  assertEqual(game.size, 4);
  assertEqual(game.score, 0);
  assert(!game.over);
  assert(!game.won);
});

test('GameManager: should add start tiles', () => {
  const game = new GameManager(4);
  let tileCount = 0;
  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 4; col++) {
      if (game.grid.cells[row][col]) tileCount++;
    }
  }
  assertEqual(tileCount, 2);
});

test('GameManager: should restart correctly', () => {
  const game = new GameManager(4);
  game.score = 100;
  game.over = true;
  game.restart();
  assertEqual(game.score, 0);
  assert(!game.over);
});

test('GameManager: tiles should merge when moving', () => {
  const game = new GameManager(4);
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 0, 0));
  game.grid.insertTile(new Tile(2, 0, 1));
  const initialScore = game.score;
  game.move(1);
  assert(game.score > initialScore);
});

test('GameManager: should detect win condition', () => {
  const game = new GameManager(4);
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(1024, 0, 0));
  game.grid.insertTile(new Tile(1024, 0, 1));
  game.move(1);
  assert(game.won);
});

test('GameManager: should handle all four directions', () => {
  const game = new GameManager(4);

  // Up
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 3, 0));
  game.move(0);
  assert(game.grid.cellOccupied({ row: 0, col: 0 }));

  // Right
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 0, 0));
  game.move(1);
  assert(game.grid.cellOccupied({ row: 0, col: 3 }));

  // Down
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 0, 0));
  game.move(2);
  assert(game.grid.cellOccupied({ row: 3, col: 0 }));

  // Left
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 0, 3));
  game.move(3);
  assert(game.grid.cellOccupied({ row: 0, col: 0 }));
});

test('GameManager: should not move when no movement possible', () => {
  const game = new GameManager(4);
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 0, 0));
  const moved = game.move(3);
  assert(!moved);
});

test('GameManager: multiple tiles should merge in one move', () => {
  const game = new GameManager(4);
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 0, 0));
  game.grid.insertTile(new Tile(2, 0, 1));
  game.grid.insertTile(new Tile(4, 0, 2));
  game.grid.insertTile(new Tile(4, 0, 3));
  const initialScore = game.score;
  game.move(1);
  assertEqual(game.score, initialScore + 12);
});

test('GameManager: should add new tile after successful move', () => {
  const game = new GameManager(4);
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 0, 0));

  let tileCount = 0;
  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 4; col++) {
      if (game.grid.cells[row][col]) tileCount++;
    }
  }
  assertEqual(tileCount, 1);

  game.move(1);

  tileCount = 0;
  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 4; col++) {
      if (game.grid.cells[row][col]) tileCount++;
    }
  }
  assertEqual(tileCount, 2);
});

test('GameManager: should detect when moves are available', () => {
  const game = new GameManager(4);
  game.grid = new Grid(4);
  game.grid.insertTile(new Tile(2, 0, 0));
  assert(game.movesAvailable());
});

test('GameManager: should detect when no moves available', () => {
  const game = new GameManager(4);
  game.grid = new Grid(4);

  // Create a checkerboard pattern with larger gaps to ensure no matches
  const pattern = [
    [2, 4, 8, 16],
    [16, 8, 4, 2],
    [2, 4, 8, 16],
    [16, 8, 4, 2]
  ];

  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 4; col++) {
      game.grid.insertTile(new Tile(pattern[row][col], row, col));
    }
  }

  assert(!game.movesAvailable());
});

// Summary
console.log('\n' + '='.repeat(50));
console.log(`Tests passed: ${passed}`);
console.log(`Tests failed: ${failed}`);
console.log(`Total tests: ${passed + failed}`);
console.log(`Pass rate: ${((passed / (passed + failed)) * 100).toFixed(1)}%`);
console.log('='.repeat(50));

process.exit(failed > 0 ? 1 : 0);
