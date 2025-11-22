#!/usr/bin/env node

/**
 * Test script to validate Tetris game logic
 */

// Test 1: Verify all tetromino shapes
console.log('=== Testing Tetromino Shapes ===');

const SHAPES = {
    I: [
        [[0,0,0,0],[1,1,1,1],[0,0,0,0],[0,0,0,0]],
        [[0,0,1,0],[0,0,1,0],[0,0,1,0],[0,0,1,0]],
        [[0,0,0,0],[0,0,0,0],[1,1,1,1],[0,0,0,0]],
        [[0,1,0,0],[0,1,0,0],[0,1,0,0],[0,1,0,0]]
    ],
    O: [
        [[0,1,1,0],[0,1,1,0],[0,0,0,0],[0,0,0,0]]
    ],
    T: [
        [[0,1,0],[1,1,1],[0,0,0]],
        [[0,1,0],[0,1,1],[0,1,0]],
        [[0,0,0],[1,1,1],[0,1,0]],
        [[0,1,0],[1,1,0],[0,1,0]]
    ],
    S: [
        [[0,1,1],[1,1,0],[0,0,0]],
        [[0,1,0],[0,1,1],[0,0,1]]
    ],
    Z: [
        [[1,1,0],[0,1,1],[0,0,0]],
        [[0,0,1],[0,1,1],[0,1,0]]
    ],
    J: [
        [[1,0,0],[1,1,1],[0,0,0]],
        [[0,1,1],[0,1,0],[0,1,0]],
        [[0,0,0],[1,1,1],[0,0,1]],
        [[0,1,0],[0,1,0],[1,1,0]]
    ],
    L: [
        [[0,0,1],[1,1,1],[0,0,0]],
        [[0,1,0],[0,1,0],[0,1,1]],
        [[0,0,0],[1,1,1],[1,0,0]],
        [[1,1,0],[0,1,0],[0,1,0]]
    ]
};

function countBlocks(shape) {
    return shape.flat().filter(x => x === 1).length;
}

let allTestsPassed = true;

// Test each piece has 4 blocks
for (const [type, rotations] of Object.entries(SHAPES)) {
    for (let i = 0; i < rotations.length; i++) {
        const blocks = countBlocks(rotations[i]);
        if (blocks !== 4) {
            console.error(`❌ ${type} rotation ${i} has ${blocks} blocks (expected 4)`);
            allTestsPassed = false;
        } else {
            console.log(`✓ ${type} rotation ${i}: 4 blocks`);
        }
    }
}

console.log('\n=== Testing Scoring System ===');

const SCORES = {
    SINGLE: 100,
    DOUBLE: 300,
    TRIPLE: 500,
    TETRIS: 800,
    SOFT_DROP: 1,
    HARD_DROP: 2
};

function testScoring() {
    const tests = [
        { lines: 1, level: 1, expected: 100, name: 'Single line at level 1' },
        { lines: 2, level: 1, expected: 300, name: 'Double line at level 1' },
        { lines: 3, level: 1, expected: 500, name: 'Triple line at level 1' },
        { lines: 4, level: 1, expected: 800, name: 'Tetris at level 1' },
        { lines: 1, level: 5, expected: 500, name: 'Single line at level 5' },
    ];

    for (const test of tests) {
        let score = 0;
        switch(test.lines) {
            case 1: score = SCORES.SINGLE * test.level; break;
            case 2: score = SCORES.DOUBLE * test.level; break;
            case 3: score = SCORES.TRIPLE * test.level; break;
            case 4: score = SCORES.TETRIS * test.level; break;
        }

        if (score === test.expected) {
            console.log(`✓ ${test.name}: ${score} points`);
        } else {
            console.error(`❌ ${test.name}: got ${score}, expected ${test.expected}`);
            allTestsPassed = false;
        }
    }
}

testScoring();

console.log('\n=== Testing Level Progression ===');

function testLevelProgression() {
    const tests = [
        { lines: 0, expectedLevel: 1 },
        { lines: 10, expectedLevel: 2 },
        { lines: 20, expectedLevel: 3 },
        { lines: 40, expectedLevel: 5 },
        { lines: 100, expectedLevel: 11 },
    ];

    for (const test of tests) {
        const level = Math.floor(test.lines / 10) + 1;
        if (level === test.expectedLevel) {
            console.log(`✓ ${test.lines} lines = level ${level}`);
        } else {
            console.error(`❌ ${test.lines} lines: got level ${level}, expected ${test.expectedLevel}`);
            allTestsPassed = false;
        }
    }
}

testLevelProgression();

console.log('\n=== Testing Drop Speed ===');

function testDropSpeed() {
    const tests = [
        { level: 1, expectedSpeed: 1000 },
        { level: 2, expectedSpeed: 900 },
        { level: 5, expectedSpeed: 600 },
        { level: 10, expectedSpeed: 100 },
        { level: 15, expectedSpeed: 100 }, // Should cap at 100
    ];

    for (const test of tests) {
        const speed = Math.max(100, 1000 - (test.level - 1) * 100);
        if (speed === test.expectedSpeed) {
            console.log(`✓ Level ${test.level}: ${speed}ms drop interval`);
        } else {
            console.error(`❌ Level ${test.level}: got ${speed}ms, expected ${test.expectedSpeed}ms`);
            allTestsPassed = false;
        }
    }
}

testDropSpeed();

console.log('\n=== Testing Grid Boundaries ===');

const COLS = 10;
const ROWS = 20;

function testBoundaries() {
    const tests = [
        { x: -1, y: 0, valid: false, name: 'Left boundary' },
        { x: 10, y: 0, valid: false, name: 'Right boundary' },
        { x: 0, y: 20, valid: false, name: 'Bottom boundary' },
        { x: 0, y: 0, valid: true, name: 'Top-left corner' },
        { x: 9, y: 19, valid: true, name: 'Bottom-right corner' },
    ];

    for (const test of tests) {
        const isValid = test.x >= 0 && test.x < COLS && test.y >= 0 && test.y < ROWS;
        if (isValid === test.valid) {
            console.log(`✓ ${test.name}: correctly ${isValid ? 'valid' : 'invalid'}`);
        } else {
            console.error(`❌ ${test.name}: got ${isValid}, expected ${test.valid}`);
            allTestsPassed = false;
        }
    }
}

testBoundaries();

console.log('\n=== Testing Line Clearing ===');

function testLineClearing() {
    // Create a grid
    const grid = Array.from({ length: ROWS }, () => Array(COLS).fill(0));

    // Fill bottom 3 rows completely
    grid[17] = Array(COLS).fill('#f00');
    grid[18] = Array(COLS).fill('#f00');
    grid[19] = Array(COLS).fill('#f00');

    // Count full lines
    let fullLines = 0;
    for (let y = 0; y < ROWS; y++) {
        if (grid[y].every(cell => cell !== 0)) {
            fullLines++;
        }
    }

    if (fullLines === 3) {
        console.log(`✓ Correctly detected ${fullLines} full lines`);
    } else {
        console.error(`❌ Detected ${fullLines} full lines, expected 3`);
        allTestsPassed = false;
    }

    // Test partial line
    const partialLine = Array(COLS).fill(0);
    partialLine[0] = '#f00';
    partialLine[1] = '#f00';
    const isPartialFull = partialLine.every(cell => cell !== 0);

    if (!isPartialFull) {
        console.log('✓ Correctly identified partial line as not full');
    } else {
        console.error('❌ Incorrectly identified partial line as full');
        allTestsPassed = false;
    }
}

testLineClearing();

console.log('\n' + '='.repeat(50));
if (allTestsPassed) {
    console.log('✅ ALL TESTS PASSED!');
    process.exit(0);
} else {
    console.log('❌ SOME TESTS FAILED!');
    process.exit(1);
}
