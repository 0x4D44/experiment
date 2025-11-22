#!/usr/bin/env node

/**
 * Verification script for bug fixes
 * Tests all fixes without requiring a browser
 */

console.log('='.repeat(60));
console.log('Physics Puzzle Game - Bug Fix Verification');
console.log('='.repeat(60));
console.log();

let allTestsPassed = true;
let testCount = 0;
let passCount = 0;

function test(name, fn) {
    testCount++;
    try {
        fn();
        passCount++;
        console.log(`✅ PASS: ${name}`);
        return true;
    } catch (error) {
        allTestsPassed = false;
        console.log(`❌ FAIL: ${name}`);
        console.log(`   Error: ${error.message}`);
        return false;
    }
}

function assert(condition, message) {
    if (!condition) {
        throw new Error(message || 'Assertion failed');
    }
}

// Read the game.js file
const fs = require('fs');
const path = require('path');
const gameJsPath = path.join(__dirname, 'game.js');
const gameJs = fs.readFileSync(gameJsPath, 'utf-8');

console.log('HIGH BUG #1: Undo Functionality');
console.log('-'.repeat(60));

test('History stores button reference in placeObject', () => {
    const match = gameJs.match(/this\.history\.push\(\{[^}]*type:\s*['"]place['"][^}]*button:\s*btn[^}]*\}\)/s);
    assert(match, 'placeObject should store button reference in history');
});

test('Undo uses stored button reference (not selectedObject)', () => {
    const undoMatch = gameJs.match(/case\s+['"]place['"]:[\s\S]*?if\s*\(\s*lastAction\.button\s*\)/);
    assert(undoMatch, 'undo should check lastAction.button instead of selectedObject');

    const noSelectedObject = !gameJs.includes('if (this.selectedObject)') ||
                              gameJs.indexOf('if (lastAction.button)') > gameJs.indexOf('if (this.selectedObject)');
    assert(noSelectedObject || gameJs.match(/if\s*\(\s*lastAction\.button\s*\)[\s\S]{0,500}btn\.dataset\.remaining\+\+/),
           'undo should use lastAction.button for restoration');
});

console.log();
console.log('MEDIUM BUG #2: localStorage Crash Risk');
console.log('-'.repeat(60));

test('loadProgress wrapped in try-catch', () => {
    const loadProgressMatch = gameJs.match(/loadProgress\s*\(\s*\)\s*\{[\s\S]*?try\s*\{[\s\S]*?JSON\.parse[\s\S]*?\}\s*catch/);
    assert(loadProgressMatch, 'loadProgress should have try-catch around JSON.parse');
});

test('loadProgress has error handler with defaults', () => {
    const catchMatch = gameJs.match(/catch[\s\S]{0,300}this\.maxUnlockedLevel\s*=\s*1/);
    assert(catchMatch, 'loadProgress catch block should set default maxUnlockedLevel');

    const starsMatch = gameJs.match(/catch[\s\S]{0,300}this\.levelStars\s*=\s*\{\}/);
    assert(starsMatch, 'loadProgress catch block should set default levelStars');
});

test('saveProgress wrapped in try-catch', () => {
    const saveProgressMatch = gameJs.match(/saveProgress\s*\(\s*\)\s*\{[\s\S]*?try\s*\{[\s\S]*?localStorage\.setItem[\s\S]*?\}\s*catch/);
    assert(saveProgressMatch, 'saveProgress should have try-catch around localStorage.setItem');
});

console.log();
console.log('MEDIUM BUG #3: Event Listener Memory Leak');
console.log('-'.repeat(60));

test('keyboardHandler property exists', () => {
    const handlerMatch = gameJs.match(/this\.keyboardHandler\s*=\s*null/);
    assert(handlerMatch, 'Constructor should initialize keyboardHandler property');
});

test('setupKeyboardControls stores listener reference', () => {
    const setupMatch = gameJs.match(/this\.keyboardHandler\s*=\s*\([^)]*\)\s*=>\s*\{/);
    assert(setupMatch, 'setupKeyboardControls should store listener as arrow function');
});

test('setupKeyboardControls removes old listener', () => {
    const removeMatch = gameJs.match(/setupKeyboardControls[\s\S]{0,300}removeEventListener\s*\(\s*['"]keydown['"]\s*,\s*this\.keyboardHandler\s*\)/);
    assert(removeMatch, 'setupKeyboardControls should remove previous listener');
});

test('showMenu removes keyboard listener', () => {
    const menuMatch = gameJs.match(/showMenu[\s\S]{0,600}removeEventListener\s*\(\s*['"]keydown['"]\s*,\s*this\.keyboardHandler\s*\)/);
    assert(menuMatch, 'showMenu should remove keyboard listener');
});

console.log();
console.log('MEDIUM BUG #4: Timer Not Cleared');
console.log('-'.repeat(60));

test('showMenu clears timerInterval', () => {
    const clearMatch = gameJs.match(/showMenu[\s\S]{0,600}clearInterval\s*\(\s*this\.timerInterval\s*\)/);
    assert(clearMatch, 'showMenu should call clearInterval on timerInterval');
});

test('showMenu sets timerInterval to null', () => {
    const nullMatch = gameJs.match(/showMenu[\s\S]{0,600}clearInterval[\s\S]{0,100}this\.timerInterval\s*=\s*null/);
    assert(nullMatch, 'showMenu should set timerInterval to null after clearing');
});

console.log();
console.log('LOW BUG #5: Particle Count Limit');
console.log('-'.repeat(60));

test('MAX_PARTICLES constant defined', () => {
    const constantMatch = gameJs.match(/const\s+MAX_PARTICLES\s*=\s*200/);
    assert(constantMatch, 'MAX_PARTICLES constant should be defined as 200');
});

test('createParticles enforces limit', () => {
    const limitMatch = gameJs.match(/createParticles[\s\S]{0,600}if\s*\(\s*this\.particles\.length\s*>=\s*MAX_PARTICLES\s*\)/);
    assert(limitMatch, 'createParticles should check particle count against MAX_PARTICLES');
});

test('createParticles breaks loop when limit reached', () => {
    const breakMatch = gameJs.match(/if\s*\(\s*this\.particles\.length\s*>=\s*MAX_PARTICLES\s*\)\s*\{\s*break/);
    assert(breakMatch, 'createParticles should break loop when limit is reached');
});

console.log();
console.log('='.repeat(60));
console.log('SUMMARY');
console.log('='.repeat(60));
console.log(`Total tests: ${testCount}`);
console.log(`Passed: ${passCount}`);
console.log(`Failed: ${testCount - passCount}`);
console.log(`Pass rate: ${((passCount / testCount) * 100).toFixed(1)}%`);
console.log();

if (allTestsPassed) {
    console.log('✅ ALL BUG FIXES VERIFIED SUCCESSFULLY');
    process.exit(0);
} else {
    console.log('❌ SOME FIXES NEED ATTENTION');
    process.exit(1);
}
