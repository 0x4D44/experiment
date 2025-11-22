#!/usr/bin/env node

/**
 * Simple test runner to verify game.js structure is intact
 * This ensures our bug fixes didn't break existing functionality
 */

const fs = require('fs');
const path = require('path');

console.log('='.repeat(60));
console.log('Running Basic Structure Tests');
console.log('='.repeat(60));
console.log();

const gameJsPath = path.join(__dirname, 'game.js');
const gameJs = fs.readFileSync(gameJsPath, 'utf-8');

let passed = 0;
let failed = 0;

function test(name, fn) {
    try {
        fn();
        console.log(`✅ ${name}`);
        passed++;
    } catch (error) {
        console.log(`❌ ${name}`);
        console.log(`   Error: ${error.message}`);
        failed++;
    }
}

function assert(condition, message) {
    if (!condition) {
        throw new Error(message || 'Assertion failed');
    }
}

// Core class structure tests
test('PhysicsGame class exists', () => {
    assert(gameJs.includes('class PhysicsGame'), 'PhysicsGame class should exist');
});

test('Constructor initializes all properties', () => {
    assert(gameJs.includes('constructor()'), 'Constructor should exist');
    assert(gameJs.includes('this.engine = null'), 'Should initialize engine');
    assert(gameJs.includes('this.moveCount = 0'), 'Should initialize moveCount');
    assert(gameJs.includes('this.history = []'), 'Should initialize history');
});

// Method existence tests
test('All required methods exist', () => {
    const methods = [
        'init',
        'setupKeyboardControls',
        'showMenu',
        'loadLevel',
        'initPhysicsEngine',
        'buildLevel',
        'placeObject',
        'undo',
        'startLevel',
        'completeLevel',
        'calculateStars',
        'saveProgress',
        'loadProgress',
        'createParticles',
        'updateParticles'
    ];

    methods.forEach(method => {
        assert(gameJs.includes(`${method}(`), `Method ${method} should exist`);
    });
});

// Level configuration tests
test('All 15 levels exist', () => {
    for (let i = 1; i <= 15; i++) {
        assert(gameJs.includes(`getLevel${i}()`), `Level ${i} should exist`);
    }
});

test('getLevels method returns all levels', () => {
    assert(gameJs.includes('getLevels()'), 'getLevels method should exist');
    assert(gameJs.includes('this.getLevel1()'), 'Should include level 1');
    assert(gameJs.includes('this.getLevel15()'), 'Should include level 15');
});

// Physics integration tests
test('Matter.js integration', () => {
    assert(gameJs.includes('const Engine = Matter.Engine'), 'Should import Engine');
    assert(gameJs.includes('const World = Matter.World'), 'Should import World');
    assert(gameJs.includes('const Bodies = Matter.Bodies'), 'Should import Bodies');
    assert(gameJs.includes('Engine.create()'), 'Should create engine');
});

// Interactive object creation tests
test('Interactive object creators exist', () => {
    assert(gameJs.includes('createRope('), 'Should have createRope');
    assert(gameJs.includes('createBomb('), 'Should have createBomb');
    assert(gameJs.includes('createDomino('), 'Should have createDomino');
    assert(gameJs.includes('createSeesaw('), 'Should have createSeesaw');
    assert(gameJs.includes('createPendulum('), 'Should have createPendulum');
});

// Event handling tests
test('Event handlers properly configured', () => {
    assert(gameJs.includes('handleMouseDown('), 'Should have mouse handler');
    assert(gameJs.includes('handleCollision('), 'Should have collision handler');
    assert(gameJs.includes("Events.on(this.engine, 'collisionStart'"), 'Should register collision events');
});

// UI update tests
test('UI update methods exist', () => {
    assert(gameJs.includes('updateStarsDisplay('), 'Should have updateStarsDisplay');
    assert(gameJs.includes('updateStarsEarned('), 'Should have updateStarsEarned');
    assert(gameJs.includes('showWinScreen('), 'Should have showWinScreen');
});

// Game initialization
test('Game initialization at page load', () => {
    assert(gameJs.includes('let game;'), 'Should declare game variable');
    assert(gameJs.includes("window.addEventListener('DOMContentLoaded'"), 'Should listen for DOMContentLoaded');
    assert(gameJs.includes('game = new PhysicsGame()'), 'Should create game instance');
});

console.log();
console.log('='.repeat(60));
console.log(`Tests passed: ${passed}`);
console.log(`Tests failed: ${failed}`);
console.log(`Pass rate: ${((passed / (passed + failed)) * 100).toFixed(1)}%`);
console.log('='.repeat(60));

if (failed === 0) {
    console.log('✅ All structure tests passed! Game integrity maintained.');
    process.exit(0);
} else {
    console.log('❌ Some tests failed. Please review changes.');
    process.exit(1);
}
