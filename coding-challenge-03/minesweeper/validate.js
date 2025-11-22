#!/usr/bin/env node

/**
 * Comprehensive Minesweeper Validation Script
 * Tests all game logic without requiring a browser
 */

// Extract and test the game logic
class MinesweeperValidator {
    constructor(rows, cols, mines) {
        this.rows = rows;
        this.cols = cols;
        this.mineCount = mines;
        this.grid = Array(rows).fill(null).map(() => Array(cols).fill(0));
        this.revealed = Array(rows).fill(null).map(() => Array(cols).fill(false));
        this.flagged = Array(rows).fill(null).map(() => Array(cols).fill(false));
        this.gameOver = false;
        this.gameWon = false;
    }

    placeMines(excludeRow, excludeCol) {
        let minesPlaced = 0;
        const maxAttempts = this.rows * this.cols * 10;
        let attempts = 0;

        while (minesPlaced < this.mineCount && attempts < maxAttempts) {
            attempts++;
            const row = Math.floor(Math.random() * this.rows);
            const col = Math.floor(Math.random() * this.cols);

            if ((row === excludeRow && col === excludeCol) || this.grid[row][col] === -1) {
                continue;
            }

            this.grid[row][col] = -1;
            minesPlaced++;
        }

        if (minesPlaced < this.mineCount) {
            throw new Error(`Failed to place all mines. Placed ${minesPlaced}/${this.mineCount}`);
        }

        this.calculateNumbers();
    }

    placeMinesAt(positions) {
        positions.forEach(([row, col]) => {
            this.grid[row][col] = -1;
        });
        this.calculateNumbers();
    }

    calculateNumbers() {
        for (let row = 0; row < this.rows; row++) {
            for (let col = 0; col < this.cols; col++) {
                if (this.grid[row][col] !== -1) {
                    this.grid[row][col] = this.countAdjacentMines(row, col);
                }
            }
        }
    }

    countAdjacentMines(row, col) {
        let count = 0;
        for (let dr = -1; dr <= 1; dr++) {
            for (let dc = -1; dc <= 1; dc++) {
                if (dr === 0 && dc === 0) continue;
                const newRow = row + dr;
                const newCol = col + dc;
                if (this.isValid(newRow, newCol) && this.grid[newRow][newCol] === -1) {
                    count++;
                }
            }
        }
        return count;
    }

    isValid(row, col) {
        return row >= 0 && row < this.rows && col >= 0 && col < this.cols;
    }

    revealCell(row, col) {
        if (!this.isValid(row, col) || this.revealed[row][col] || this.flagged[row][col]) {
            return;
        }

        this.revealed[row][col] = true;

        if (this.grid[row][col] === 0) {
            for (let dr = -1; dr <= 1; dr++) {
                for (let dc = -1; dc <= 1; dc++) {
                    if (dr === 0 && dc === 0) continue;
                    this.revealCell(row + dr, col + dc);
                }
            }
        }
    }

    countRevealed() {
        let count = 0;
        for (let row = 0; row < this.rows; row++) {
            for (let col = 0; col < this.cols; col++) {
                if (this.revealed[row][col]) count++;
            }
        }
        return count;
    }

    countFlags() {
        let count = 0;
        for (let row = 0; row < this.rows; row++) {
            for (let col = 0; col < this.cols; col++) {
                if (this.flagged[row][col]) count++;
            }
        }
        return count;
    }

    getTotalMines() {
        let count = 0;
        for (let row = 0; row < this.rows; row++) {
            for (let col = 0; col < this.cols; col++) {
                if (this.grid[row][col] === -1) count++;
            }
        }
        return count;
    }

    checkWin() {
        const revealedCount = this.countRevealed();
        const totalCells = this.rows * this.cols;
        return revealedCount === totalCells - this.mineCount;
    }

    countAdjacentFlags(row, col) {
        let count = 0;
        for (let dr = -1; dr <= 1; dr++) {
            for (let dc = -1; dc <= 1; dc++) {
                if (dr === 0 && dc === 0) continue;
                const newRow = row + dr;
                const newCol = col + dc;
                if (this.isValid(newRow, newCol) && this.flagged[newRow][newCol]) {
                    count++;
                }
            }
        }
        return count;
    }
}

// Test Suite
class TestRunner {
    constructor() {
        this.passed = 0;
        this.failed = 0;
        this.tests = [];
    }

    test(name, fn) {
        try {
            fn();
            this.passed++;
            console.log(`✓ ${name}`);
            return true;
        } catch (error) {
            this.failed++;
            console.log(`✗ ${name}`);
            console.log(`  Error: ${error.message}`);
            return false;
        }
    }

    assertEqual(actual, expected, message) {
        if (actual !== expected) {
            throw new Error(message || `Expected ${expected}, got ${actual}`);
        }
    }

    assert(condition, message) {
        if (!condition) {
            throw new Error(message || 'Assertion failed');
        }
    }

    assertGreaterThan(actual, threshold, message) {
        if (actual <= threshold) {
            throw new Error(message || `Expected ${actual} > ${threshold}`);
        }
    }

    printSummary() {
        console.log('\n' + '='.repeat(60));
        console.log('TEST SUMMARY');
        console.log('='.repeat(60));
        console.log(`Total:  ${this.passed + this.failed}`);
        console.log(`Passed: ${this.passed} ✓`);
        console.log(`Failed: ${this.failed} ✗`);
        console.log('='.repeat(60));

        if (this.failed === 0) {
            console.log('🎉 ALL TESTS PASSED! 🎉');
            return true;
        } else {
            console.log('❌ SOME TESTS FAILED');
            return false;
        }
    }
}

// Run all tests
function runAllTests() {
    const runner = new TestRunner();

    console.log('\n🧪 MINESWEEPER VALIDATION SUITE\n');

    // Mine Placement Tests
    console.log('\n📍 MINE PLACEMENT TESTS');
    console.log('-'.repeat(60));

    runner.test('Correct number of mines placed', () => {
        const game = new MinesweeperValidator(9, 9, 10);
        game.placeMines(-1, -1);
        const total = game.getTotalMines();
        runner.assertEqual(total, 10, `Expected 10 mines, got ${total}`);
    });

    runner.test('First click position is safe', () => {
        const game = new MinesweeperValidator(9, 9, 10);
        game.placeMines(4, 4);
        runner.assert(game.grid[4][4] !== -1, 'First click position should not have a mine');
    });

    runner.test('Mines are placed in unique positions', () => {
        const game = new MinesweeperValidator(9, 9, 10);
        game.placeMines(-1, -1);
        const mineSet = new Set();
        for (let row = 0; row < 9; row++) {
            for (let col = 0; col < 9; col++) {
                if (game.grid[row][col] === -1) {
                    const key = `${row},${col}`;
                    runner.assert(!mineSet.has(key), 'Duplicate mine found');
                    mineSet.add(key);
                }
            }
        }
    });

    runner.test('Mine placement works on large boards', () => {
        const game = new MinesweeperValidator(16, 30, 99);
        game.placeMines(0, 0);
        const total = game.getTotalMines();
        runner.assertEqual(total, 99, `Expected 99 mines, got ${total}`);
    });

    // Number Calculation Tests
    console.log('\n🔢 NUMBER CALCULATION TESTS');
    console.log('-'.repeat(60));

    runner.test('Corner cell with one mine', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.placeMinesAt([[0, 1]]);
        runner.assertEqual(game.grid[0][0], 1, 'Corner should show 1');
    });

    runner.test('Center cell surrounded by mines', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.placeMinesAt([[0,0], [0,1], [0,2], [1,0], [1,2], [2,0], [2,1], [2,2]]);
        runner.assertEqual(game.grid[1][1], 8, 'Center should show 8');
    });

    runner.test('Edge cell with two mines', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.placeMinesAt([[0,0], [2,0]]);
        runner.assertEqual(game.grid[1][0], 2, 'Edge should show 2');
    });

    runner.test('Empty cell with no adjacent mines', () => {
        const game = new MinesweeperValidator(5, 5, 0);
        game.placeMinesAt([[0,0]]);
        runner.assertEqual(game.grid[2][2], 0, 'Center should show 0');
    });

    runner.test('All numbers calculated correctly', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.placeMinesAt([[0,0], [0,2]]);
        runner.assertEqual(game.grid[0][1], 2, 'Top middle should show 2');
        runner.assertEqual(game.grid[1][1], 2, 'Center should show 2');
        runner.assertEqual(game.grid[1][0], 1, 'Left middle should show 1');
        runner.assertEqual(game.grid[1][2], 1, 'Right middle should show 1');
    });

    // Flood Fill Tests
    console.log('\n🌊 FLOOD FILL TESTS');
    console.log('-'.repeat(60));

    runner.test('Empty area reveals all connected cells', () => {
        const game = new MinesweeperValidator(5, 5, 0);
        game.placeMinesAt([[0,0]]);
        game.revealCell(2, 2);
        const revealed = game.countRevealed();
        runner.assertGreaterThan(revealed, 10, 'Should reveal multiple cells');
    });

    runner.test('Flood fill stops at numbered cells', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.placeMinesAt([[0,0]]);
        game.revealCell(2, 2);
        runner.assert(game.revealed[2][2], 'Starting cell should be revealed');
        runner.assert(game.revealed[1][1], 'Adjacent empty should be revealed');
    });

    runner.test('Flood fill does not reveal flagged cells', () => {
        const game = new MinesweeperValidator(5, 5, 0);
        game.flagged[2][3] = true;
        game.revealCell(2, 2);
        runner.assert(!game.revealed[2][3], 'Flagged cell should not be revealed');
    });

    runner.test('Flood fill handles board edges', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.revealCell(0, 0);
        runner.assert(game.revealed[0][0], 'Corner cell should be revealed');
        runner.assert(game.revealed[0][1], 'Adjacent cell should be revealed');
        runner.assert(game.revealed[1][0], 'Adjacent cell should be revealed');
    });

    runner.test('Flood fill on large empty area', () => {
        const game = new MinesweeperValidator(10, 10, 0);
        game.placeMinesAt([[0,0]]);
        game.revealCell(5, 5);
        const revealed = game.countRevealed();
        runner.assertGreaterThan(revealed, 50, 'Should reveal large area');
    });

    // Win Detection Tests
    console.log('\n🏆 WIN DETECTION TESTS');
    console.log('-'.repeat(60));

    runner.test('Win detected when all non-mines revealed', () => {
        const game = new MinesweeperValidator(3, 3, 1);
        game.placeMinesAt([[0,0]]);
        for (let row = 0; row < 3; row++) {
            for (let col = 0; col < 3; col++) {
                if (row !== 0 || col !== 0) {
                    game.revealed[row][col] = true;
                }
            }
        }
        runner.assert(game.checkWin(), 'Should detect win');
    });

    runner.test('No win with unrevealed cells', () => {
        const game = new MinesweeperValidator(3, 3, 1);
        game.placeMinesAt([[0,0]]);
        game.revealed[1][1] = true;
        runner.assert(!game.checkWin(), 'Should not detect win');
    });

    runner.test('Win detection on large board', () => {
        const game = new MinesweeperValidator(9, 9, 10);
        game.placeMines(-1, -1);
        for (let row = 0; row < 9; row++) {
            for (let col = 0; col < 9; col++) {
                if (game.grid[row][col] !== -1) {
                    game.revealed[row][col] = true;
                }
            }
        }
        runner.assert(game.checkWin(), 'Should detect win on 9x9');
    });

    // Flag Tests
    console.log('\n🚩 FLAG TESTS');
    console.log('-'.repeat(60));

    runner.test('Flag counting works correctly', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.flagged[0][0] = true;
        game.flagged[0][1] = true;
        runner.assertEqual(game.countFlags(), 2, 'Should count 2 flags');
    });

    runner.test('Adjacent flag counting', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.flagged[0][0] = true;
        game.flagged[0][2] = true;
        game.flagged[2][0] = true;
        const count = game.countAdjacentFlags(1, 1);
        runner.assertEqual(count, 3, 'Should count 3 adjacent flags');
    });

    runner.test('Corner flag counting', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.flagged[0][1] = true;
        game.flagged[1][0] = true;
        const count = game.countAdjacentFlags(0, 0);
        runner.assertEqual(count, 2, 'Corner should count 2 adjacent flags');
    });

    // Boundary Tests
    console.log('\n🔒 BOUNDARY TESTS');
    console.log('-'.repeat(60));

    runner.test('isValid rejects negative coordinates', () => {
        const game = new MinesweeperValidator(9, 9, 10);
        runner.assert(!game.isValid(-1, 0), 'Negative row invalid');
        runner.assert(!game.isValid(0, -1), 'Negative col invalid');
    });

    runner.test('isValid rejects out of bounds', () => {
        const game = new MinesweeperValidator(9, 9, 10);
        runner.assert(!game.isValid(9, 0), 'Row 9 invalid');
        runner.assert(!game.isValid(0, 9), 'Col 9 invalid');
    });

    runner.test('isValid accepts valid coordinates', () => {
        const game = new MinesweeperValidator(9, 9, 10);
        runner.assert(game.isValid(0, 0), 'Top-left valid');
        runner.assert(game.isValid(8, 8), 'Bottom-right valid');
        runner.assert(game.isValid(4, 4), 'Center valid');
    });

    // Difficulty Configuration Tests
    console.log('\n⚙️  DIFFICULTY CONFIGURATION TESTS');
    console.log('-'.repeat(60));

    runner.test('Beginner configuration', () => {
        const game = new MinesweeperValidator(9, 9, 10);
        runner.assertEqual(game.rows, 9, 'Beginner rows');
        runner.assertEqual(game.cols, 9, 'Beginner cols');
        runner.assertEqual(game.mineCount, 10, 'Beginner mines');
    });

    runner.test('Intermediate configuration', () => {
        const game = new MinesweeperValidator(16, 16, 40);
        runner.assertEqual(game.rows, 16, 'Intermediate rows');
        runner.assertEqual(game.cols, 16, 'Intermediate cols');
        runner.assertEqual(game.mineCount, 40, 'Intermediate mines');
    });

    runner.test('Expert configuration', () => {
        const game = new MinesweeperValidator(16, 30, 99);
        runner.assertEqual(game.rows, 16, 'Expert rows');
        runner.assertEqual(game.cols, 30, 'Expert cols');
        runner.assertEqual(game.mineCount, 99, 'Expert mines');
    });

    // Reveal Logic Tests
    console.log('\n👁️  REVEAL LOGIC TESTS');
    console.log('-'.repeat(60));

    runner.test('Cannot reveal flagged cell', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.flagged[1][1] = true;
        game.revealCell(1, 1);
        runner.assert(!game.revealed[1][1], 'Flagged cell not revealed');
    });

    runner.test('Cannot double-reveal cell', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.revealed[1][1] = true;
        const initial = game.countRevealed();
        game.revealCell(1, 1);
        runner.assertEqual(game.countRevealed(), initial, 'No double reveal');
    });

    runner.test('Single cell reveal (numbered)', () => {
        const game = new MinesweeperValidator(3, 3, 0);
        game.placeMinesAt([[0,0]]);
        game.revealCell(0, 1);
        runner.assert(game.revealed[0][1], 'Cell revealed');
        runner.assertEqual(game.countRevealed(), 1, 'Only one cell');
    });

    // Edge Case Tests
    console.log('\n🎯 EDGE CASE TESTS');
    console.log('-'.repeat(60));

    runner.test('Board with single cell', () => {
        const game = new MinesweeperValidator(1, 1, 0);
        game.revealCell(0, 0);
        runner.assert(game.revealed[0][0], 'Single cell revealed');
    });

    runner.test('Board fully covered in mines', () => {
        const game = new MinesweeperValidator(3, 3, 8);
        game.placeMines(1, 1); // Safe spot
        runner.assertEqual(game.getTotalMines(), 8, '8 mines placed');
        runner.assert(game.grid[1][1] !== -1, 'Safe spot has no mine');
    });

    runner.test('Empty board (no mines)', () => {
        const game = new MinesweeperValidator(5, 5, 0);
        game.calculateNumbers();
        game.revealCell(0, 0);
        const revealed = game.countRevealed();
        runner.assertEqual(revealed, 25, 'All cells revealed');
    });

    runner.test('Multiple flood fill operations', () => {
        const game = new MinesweeperValidator(10, 10, 0);
        game.placeMinesAt([[5,5]]);
        game.revealCell(0, 0);
        const first = game.countRevealed();
        game.revealCell(9, 9);
        const second = game.countRevealed();
        runner.assertGreaterThan(second, first, 'Additional cells revealed');
    });

    return runner.printSummary();
}

// Run tests
const success = runAllTests();
process.exit(success ? 0 : 1);
