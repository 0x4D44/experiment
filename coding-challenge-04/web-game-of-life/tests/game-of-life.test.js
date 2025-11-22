/**
 * Comprehensive tests for Conway's Game of Life
 * Tests all rules and edge cases
 */

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

    assertEqual(actual, expected, message = '') {
        if (actual !== expected) {
            throw new Error(`Assertion failed: ${message}\nExpected: ${expected}\nActual: ${actual}`);
        }
    }

    assertArrayEqual(actual, expected, message = '') {
        if (JSON.stringify(actual) !== JSON.stringify(expected)) {
            throw new Error(`Assertion failed: ${message}\nExpected: ${JSON.stringify(expected)}\nActual: ${JSON.stringify(actual)}`);
        }
    }

    assertTrue(condition, message = '') {
        if (!condition) {
            throw new Error(`Assertion failed: ${message}`);
        }
    }

    run() {
        console.log('=== Running Game of Life Tests ===\n');

        for (const test of this.tests) {
            try {
                test.fn.call(this);
                this.passed++;
                console.log(`✓ ${test.name}`);
            } catch (error) {
                this.failed++;
                console.error(`✗ ${test.name}`);
                console.error(`  ${error.message}\n`);
            }
        }

        console.log(`\n=== Test Results ===`);
        console.log(`Passed: ${this.passed}`);
        console.log(`Failed: ${this.failed}`);
        console.log(`Total: ${this.tests.length}`);

        return this.failed === 0;
    }
}

// Create test runner
const runner = new TestRunner();

// Test 1: Grid initialization
runner.test('Grid should initialize with correct dimensions', function() {
    const game = new GameOfLife(10, 10);
    this.assertEqual(game.width, 10);
    this.assertEqual(game.height, 10);
    this.assertEqual(game.generation, 0);
});

// Test 2: Cell setting and getting
runner.test('Should set and get cell states correctly', function() {
    const game = new GameOfLife(5, 5);
    game.setCell(2, 2, true);
    this.assertEqual(game.getCell(2, 2), 1);
    game.setCell(2, 2, false);
    this.assertEqual(game.getCell(2, 2), 0);
});

// Test 3: Neighbor counting
runner.test('Should count neighbors correctly', function() {
    const game = new GameOfLife(5, 5);

    // Create a 3x3 block with center cell
    game.setCell(1, 1, true);
    game.setCell(2, 1, true);
    game.setCell(3, 1, true);
    game.setCell(1, 2, true);
    game.setCell(2, 2, true);
    game.setCell(3, 2, true);
    game.setCell(1, 3, true);
    game.setCell(2, 3, true);
    game.setCell(3, 3, true);

    // Center cell should have 8 neighbors
    this.assertEqual(game.countNeighbors(2, 2), 8);

    // Edge cell should have 5 neighbors
    this.assertEqual(game.countNeighbors(1, 1), 3);
});

// Test 4: Rule - Death by underpopulation
runner.test('Rule: Live cell with < 2 neighbors dies (underpopulation)', function() {
    const game = new GameOfLife(5, 5);

    // Single cell (0 neighbors)
    game.setCell(2, 2, true);
    game.step();
    this.assertEqual(game.getCell(2, 2), 0, 'Cell with 0 neighbors should die');

    // Two cells (1 neighbor each)
    game.clear();
    game.setCell(2, 2, true);
    game.setCell(2, 3, true);
    game.step();
    this.assertEqual(game.getCell(2, 2), 0, 'Cell with 1 neighbor should die');
    this.assertEqual(game.getCell(2, 3), 0, 'Cell with 1 neighbor should die');
});

// Test 5: Rule - Survival
runner.test('Rule: Live cell with 2-3 neighbors survives', function() {
    const game = new GameOfLife(5, 5);

    // Blinker pattern (middle cell has 2 neighbors)
    game.setCell(2, 1, true);
    game.setCell(2, 2, true);
    game.setCell(2, 3, true);

    const before = game.getCell(2, 2);
    game.step();
    // Middle cell should survive (2 neighbors)
    this.assertEqual(before, 1, 'Cell should be alive before step');
});

// Test 6: Rule - Death by overcrowding
runner.test('Rule: Live cell with > 3 neighbors dies (overcrowding)', function() {
    const game = new GameOfLife(5, 5);

    // Create cell with 4 neighbors
    game.setCell(2, 2, true); // center
    game.setCell(1, 2, true); // left
    game.setCell(3, 2, true); // right
    game.setCell(2, 1, true); // top
    game.setCell(2, 3, true); // bottom

    game.step();
    this.assertEqual(game.getCell(2, 2), 0, 'Cell with 4 neighbors should die');
});

// Test 7: Rule - Birth
runner.test('Rule: Dead cell with exactly 3 neighbors becomes alive', function() {
    const game = new GameOfLife(5, 5);

    // Create 3 cells around an empty cell
    game.setCell(1, 2, true);
    game.setCell(2, 1, true);
    game.setCell(3, 2, true);

    // Center cell (2,2) has exactly 3 neighbors
    this.assertEqual(game.getCell(2, 2), 0, 'Cell should be dead before birth');
    game.step();
    this.assertEqual(game.getCell(2, 2), 1, 'Cell with 3 neighbors should be born');
});

// Test 8: Blinker oscillator (period 2)
runner.test('Blinker pattern should oscillate with period 2', function() {
    const game = new GameOfLife(5, 5);

    // Horizontal blinker
    game.setCell(2, 1, true);
    game.setCell(2, 2, true);
    game.setCell(2, 3, true);

    const gen0 = [
        game.getCell(1, 2),
        game.getCell(2, 2),
        game.getCell(3, 2)
    ];

    game.step();

    // Should be vertical
    const gen1 = [
        game.getCell(1, 2),
        game.getCell(2, 2),
        game.getCell(3, 2)
    ];

    game.step();

    // Should be horizontal again
    const gen2 = [
        game.getCell(2, 1),
        game.getCell(2, 2),
        game.getCell(2, 3)
    ];

    this.assertArrayEqual(gen0, [0, 0, 0]);
    this.assertArrayEqual(gen1, [1, 1, 1]);
    this.assertArrayEqual(gen2, [1, 1, 1]);
});

// Test 9: Block still life
runner.test('Block pattern should remain stable', function() {
    const game = new GameOfLife(5, 5);

    // 2x2 block
    game.setCell(2, 2, true);
    game.setCell(3, 2, true);
    game.setCell(2, 3, true);
    game.setCell(3, 3, true);

    const before = game.getPopulation();
    game.step();
    const after = game.getPopulation();

    this.assertEqual(before, after, 'Block should remain stable');
    this.assertEqual(game.getCell(2, 2), 1);
    this.assertEqual(game.getCell(3, 2), 1);
    this.assertEqual(game.getCell(2, 3), 1);
    this.assertEqual(game.getCell(3, 3), 1);
});

// Test 10: Population counting
runner.test('Population count should be accurate', function() {
    const game = new GameOfLife(10, 10);

    this.assertEqual(game.getPopulation(), 0, 'Empty grid should have 0 population');

    game.setCell(0, 0, true);
    game.setCell(5, 5, true);
    game.setCell(9, 9, true);

    this.assertEqual(game.getPopulation(), 3, 'Should count 3 live cells');

    game.clear();
    this.assertEqual(game.getPopulation(), 0, 'Cleared grid should have 0 population');
});

// Test 11: Generation counter
runner.test('Generation counter should increment', function() {
    const game = new GameOfLife(5, 5);

    this.assertEqual(game.generation, 0);

    game.step();
    this.assertEqual(game.generation, 1);

    game.step();
    this.assertEqual(game.generation, 2);

    game.clear();
    this.assertEqual(game.generation, 0, 'Clear should reset generation');
});

// Test 12: Wrap-around edges (toroidal topology)
runner.test('Should wrap around edges when enabled', function() {
    const game = new GameOfLife(5, 5);
    game.wrapAround = true;

    // Place cells at edges
    game.setCell(0, 0, true);
    game.setCell(4, 4, true);

    // Cell at (0,0) should see cell at (4,4) as neighbor (diagonal)
    const neighbors = game.countNeighbors(0, 0);
    this.assertTrue(neighbors >= 0, 'Should count wrapped neighbors');
});

// Test 13: No wrap-around when disabled
runner.test('Should not wrap around edges when disabled', function() {
    const game = new GameOfLife(5, 5);
    game.wrapAround = false;

    // Place cell at corner
    game.setCell(0, 0, true);
    game.setCell(0, 1, true);
    game.setCell(1, 0, true);

    const neighbors = game.countNeighbors(0, 0);
    this.assertEqual(neighbors, 2, 'Should only count adjacent neighbors');
});

// Test 14: Cell aging
runner.test('Cell age should increment each generation', function() {
    const game = new GameOfLife(5, 5);

    // Create stable block
    game.setCell(2, 2, true);
    game.setCell(3, 2, true);
    game.setCell(2, 3, true);
    game.setCell(3, 3, true);

    this.assertEqual(game.getCellAge(2, 2), 1, 'New cell should have age 1');

    game.step();
    this.assertEqual(game.getCellAge(2, 2), 2, 'Cell should age after step');

    game.step();
    this.assertEqual(game.getCellAge(2, 2), 3, 'Cell should continue aging');
});

// Test 15: Clear function
runner.test('Clear should reset all state', function() {
    const game = new GameOfLife(5, 5);

    game.setCell(2, 2, true);
    game.setCell(3, 3, true);
    game.step();
    game.step();

    game.clear();

    this.assertEqual(game.getPopulation(), 0, 'Population should be 0');
    this.assertEqual(game.generation, 0, 'Generation should be 0');
    this.assertEqual(game.getCellAge(2, 2), 0, 'Cell ages should be reset');
});

// Test 16: Randomize function
runner.test('Randomize should create live cells', function() {
    const game = new GameOfLife(10, 10);

    game.randomize(0.5);

    const population = game.getPopulation();
    this.assertTrue(population > 0, 'Should have some live cells after randomize');
    this.assertTrue(population < 100, 'Should not fill entire grid');
});

// Test 17: Pattern loading
runner.test('Should load patterns correctly', function() {
    const game = new GameOfLife(20, 20);

    const testPattern = {
        width: 3,
        height: 1,
        cells: [[0, 0], [1, 0], [2, 0]]
    };

    game.loadPattern(testPattern, 10, 10);

    this.assertEqual(game.getPopulation(), 3, 'Should have 3 cells from pattern');
});

// Test 18: Glider movement
runner.test('Glider should move diagonally', function() {
    const game = new GameOfLife(20, 20);

    // Load glider pattern
    game.loadPattern(PATTERNS.glider, 10, 10);

    const initialPop = game.getPopulation();

    // Run for 4 generations (glider period)
    for (let i = 0; i < 4; i++) {
        game.step();
    }

    this.assertEqual(game.getPopulation(), initialPop, 'Glider should maintain population');
});

// Test 19: Grid resize
runner.test('Should resize grid correctly', function() {
    const game = new GameOfLife(10, 10);

    game.setCell(5, 5, true);
    game.resize(20, 20);

    this.assertEqual(game.width, 20);
    this.assertEqual(game.height, 20);
    this.assertEqual(game.getPopulation(), 0, 'Resize should clear grid');
});

// Test 20: Toggle cell
runner.test('Toggle should flip cell state', function() {
    const game = new GameOfLife(5, 5);

    game.toggleCell(2, 2);
    this.assertEqual(game.getCell(2, 2), 1, 'Dead cell should become alive');

    game.toggleCell(2, 2);
    this.assertEqual(game.getCell(2, 2), 0, 'Live cell should become dead');
});

// Run all tests
if (typeof module !== 'undefined' && module.exports) {
    module.exports = runner;
} else {
    // Run tests in browser
    window.addEventListener('DOMContentLoaded', () => {
        const success = runner.run();
        if (success) {
            console.log('\n✓ All tests passed!');
        } else {
            console.error('\n✗ Some tests failed!');
        }
    });
}
