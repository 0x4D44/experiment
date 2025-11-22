/**
 * Node.js Test Runner for 2048 Game
 * Run with: node test.js
 */

const Game = require('./game.js');

class TestRunner {
    constructor() {
        this.passed = 0;
        this.failed = 0;
        this.tests = [];
    }

    test(name, fn) {
        this.tests.push({ name, fn });
    }

    assertEquals(actual, expected, message) {
        if (JSON.stringify(actual) !== JSON.stringify(expected)) {
            throw new Error(message || `Expected ${JSON.stringify(expected)} but got ${JSON.stringify(actual)}`);
        }
    }

    assertTrue(condition, message) {
        if (!condition) {
            throw new Error(message || 'Expected condition to be true');
        }
    }

    assertFalse(condition, message) {
        if (condition) {
            throw new Error(message || 'Expected condition to be false');
        }
    }

    run() {
        console.log('\n2048 Game Test Suite\n' + '='.repeat(50) + '\n');

        this.tests.forEach(test => {
            try {
                test.fn.call(this);
                this.passed++;
                console.log(`✓ ${test.name}`);
            } catch (error) {
                this.failed++;
                console.log(`✗ ${test.name}`);
                console.log(`  Error: ${error.message}\n`);
            }
        });

        const total = this.passed + this.failed;
        console.log('\n' + '='.repeat(50));
        console.log(`Results: ${this.passed}/${total} tests passed`);

        if (this.failed === 0) {
            console.log('🎉 All tests passed!\n');
            process.exit(0);
        } else {
            console.log(`❌ ${this.failed} test(s) failed\n`);
            process.exit(1);
        }
    }
}

const runner = new TestRunner();

// Test: Game initialization
runner.test('Game Initialization', function() {
    const game = new Game();
    this.assertEquals(game.size, 4, 'Grid size should be 4');
    this.assertEquals(game.score, 0, 'Initial score should be 0');
    this.assertFalse(game.won, 'Game should not be won initially');
    this.assertFalse(game.over, 'Game should not be over initially');
});

// Test: New game start
runner.test('New Game Start', function() {
    const game = new Game();
    game.startNewGame();

    const tiles = game.getAllTiles();
    this.assertEquals(tiles.length, 2, 'Should have 2 tiles after starting');
});

// Test: Tile movement - Left
runner.test('Tile Movement - Left', function() {
    const game = new Game();
    game.grid = [
        [2, 0, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 2],
        [0, 0, 0, 0]
    ];

    const result = game.move('left');

    this.assertTrue(result.moved, 'Tiles should have moved');
    this.assertEquals(game.grid[0][0], 2, 'First tile should be at position [0][0]');
    this.assertEquals(game.grid[1][0], 2, 'Second tile should be at position [1][0]');
    this.assertEquals(game.grid[2][0], 2, 'Third tile should be at position [2][0]');
});

// Test: Tile movement - Right
runner.test('Tile Movement - Right', function() {
    const game = new Game();
    game.grid = [
        [2, 0, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 2],
        [0, 0, 0, 0]
    ];

    const result = game.move('right');

    this.assertTrue(result.moved, 'Tiles should have moved');
    this.assertEquals(game.grid[0][3], 2, 'First tile should be at position [0][3]');
    this.assertEquals(game.grid[1][3], 2, 'Second tile should be at position [1][3]');
    this.assertEquals(game.grid[2][3], 2, 'Third tile should be at position [2][3]');
});

// Test: Tile movement - Up
runner.test('Tile Movement - Up', function() {
    const game = new Game();
    game.grid = [
        [2, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 2]
    ];

    const result = game.move('up');

    this.assertTrue(result.moved, 'Tiles should have moved');
    this.assertEquals(game.grid[0][0], 2, 'First tile should be at position [0][0]');
    this.assertEquals(game.grid[0][1], 2, 'Second tile should be at position [0][1]');
    this.assertEquals(game.grid[0][2], 2, 'Third tile should be at position [0][2]');
    this.assertEquals(game.grid[0][3], 2, 'Fourth tile should be at position [0][3]');
});

// Test: Tile movement - Down
runner.test('Tile Movement - Down', function() {
    const game = new Game();
    game.grid = [
        [2, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 2]
    ];

    const result = game.move('down');

    this.assertTrue(result.moved, 'Tiles should have moved');
    this.assertEquals(game.grid[3][0], 2, 'First tile should be at position [3][0]');
    this.assertEquals(game.grid[3][1], 2, 'Second tile should be at position [3][1]');
    this.assertEquals(game.grid[3][2], 2, 'Third tile should be at position [3][2]');
    this.assertEquals(game.grid[3][3], 2, 'Fourth tile should be at position [3][3]');
});

// Test: Tile merging
runner.test('Tile Merging', function() {
    const game = new Game();
    game.grid = [
        [2, 2, 0, 0],
        [4, 4, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    const result = game.move('left');

    this.assertTrue(result.moved, 'Tiles should have moved');
    this.assertEquals(game.grid[0][0], 4, 'First row should merge to 4');
    this.assertEquals(game.grid[1][0], 8, 'Second row should merge to 8');
    this.assertEquals(result.scoreGained, 12, 'Score gained should be 4 + 8 = 12');
});

// Test: Multiple merges in one move
runner.test('Multiple Merges in One Move', function() {
    const game = new Game();
    game.grid = [
        [2, 2, 2, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    const result = game.move('left');

    this.assertTrue(result.moved, 'Tiles should have moved');
    this.assertEquals(game.grid[0][0], 4, 'First merge should be 4');
    this.assertEquals(game.grid[0][1], 4, 'Second merge should be 4');
    this.assertEquals(result.scoreGained, 8, 'Score gained should be 4 + 4 = 8');
});

// Test: Win condition
runner.test('Win Condition', function() {
    const game = new Game();
    game.grid = [
        [1024, 1024, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    const result = game.move('left');

    this.assertTrue(result.moved, 'Tiles should have moved');
    this.assertEquals(game.grid[0][0], 2048, 'Should create 2048 tile');
    this.assertTrue(game.hasWon(), 'Game should be won');
});

// Test: Game over detection
runner.test('Game Over Detection', function() {
    const game = new Game();
    game.grid = [
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 4, 2, 4],
        [4, 2, 4, 2]
    ];

    this.assertFalse(game.canMove(), 'No moves should be available');
});

// Test: Undo functionality
runner.test('Undo Functionality', function() {
    const game = new Game();
    game.grid = [
        [2, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    game.move('right');
    this.assertTrue(game.canUndo(), 'Should be able to undo');

    game.restoreState();
    this.assertEquals(game.grid[0][0], 2, 'Grid should be restored to original state');
});

// Run all tests
runner.run();
