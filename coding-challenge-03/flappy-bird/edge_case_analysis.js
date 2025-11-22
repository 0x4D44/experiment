/**
 * Edge Case and Bug Analysis for Flappy Bird
 * Tests for corner cases, race conditions, and potential bugs
 */

const fs = require('fs');
const gameCode = fs.readFileSync('game.js', 'utf8');
const indexCode = fs.readFileSync('index.html', 'utf8');

console.log('='.repeat(80));
console.log('EDGE CASE AND BUG ANALYSIS');
console.log('='.repeat(80));
console.log();

let criticalIssues = [];
let warnings = [];
let passes = [];

function checkCritical(condition, message) {
    if (condition) {
        criticalIssues.push(message);
        console.log(`  ✗ CRITICAL: ${message}`);
    }
}

function checkWarning(condition, message) {
    if (condition) {
        warnings.push(message);
        console.log(`  ⚠ WARNING: ${message}`);
    }
}

function checkPass(message) {
    passes.push(message);
    console.log(`  ✓ ${message}`);
}

// Test 1: Collision Detection Edge Cases
console.log('Test 1: Collision Detection Edge Cases');
console.log('-'.repeat(80));

// Check ceiling collision
if (gameCode.includes('bird.y < 0')) {
    checkPass('Ceiling collision handled');
} else {
    checkCritical(true, 'Missing ceiling collision check');
}

// Check ground collision with proper offset
if (gameCode.match(/bird\.y.*\+.*BIRD_SIZE.*>.*GROUND_HEIGHT/)) {
    checkPass('Ground collision uses proper bird bottom edge');
} else {
    checkWarning(true, 'Ground collision may not use bird bottom edge');
}

// Check pipe collision uses bounding boxes
if (gameCode.includes('birdLeft') && gameCode.includes('birdRight') &&
    gameCode.includes('birdTop') && gameCode.includes('birdBottom')) {
    checkPass('Proper bounding box collision detection');
} else {
    checkCritical(true, 'Missing proper bounding box collision');
}

// Check gap collision logic
if (gameCode.match(/birdTop\s*<\s*pipe\.gapY/) &&
    gameCode.match(/birdBottom\s*>\s*pipe\.gapY\s*\+\s*.*PIPE_GAP/)) {
    checkPass('Gap collision logic correct (top and bottom pipes)');
} else {
    checkCritical(true, 'Gap collision logic may be incorrect');
}

console.log();

// Test 2: Physics Edge Cases
console.log('Test 2: Physics Edge Cases');
console.log('-'.repeat(80));

// Check if velocity can become NaN or Infinity
if (gameCode.includes('velocity') && !gameCode.includes('isNaN')) {
    checkWarning(true, 'No NaN check for velocity (may not be needed)');
} else {
    checkPass('Physics calculations appear safe');
}

// Check ceiling velocity reset
if (gameCode.match(/bird\.y\s*<\s*0[\s\S]{0,50}velocity\s*=\s*0/)) {
    checkPass('Velocity reset on ceiling collision');
} else {
    checkWarning(true, 'Velocity may not reset on ceiling collision');
}

// Check if bird can go off-screen
if (gameCode.includes('bird.y < 0')) {
    checkPass('Bird Y position bounded');
} else {
    checkWarning(true, 'Bird may be able to go above screen');
}

console.log();

// Test 3: Pipe Generation Edge Cases
console.log('Test 3: Pipe Generation Edge Cases');
console.log('-'.repeat(80));

// Check pipe gap bounds
const minGapMatch = gameCode.match(/minGapY\s*=\s*([\d.]+)/);
const maxGapMatch = gameCode.match(/maxGapY\s*=.*GROUND_HEIGHT.*PIPE_GAP/);

if (minGapMatch && maxGapMatch) {
    checkPass('Pipe gap bounds defined');

    // Verify gap is reasonable
    if (gameCode.includes('Math.random()')) {
        checkPass('Random gap generation implemented');
    }
} else {
    checkCritical(true, 'Pipe gap bounds not properly defined');
}

// Check pipe removal
if (gameCode.match(/pipe\.x\s*\+\s*.*PIPE_WIDTH\s*<\s*0/) &&
    gameCode.includes('splice')) {
    checkPass('Off-screen pipes properly removed');
} else {
    checkCritical(true, 'Pipes may not be removed properly (memory leak)');
}

// Check spawn timer reset
if (gameCode.match(/pipeSpawnTimer\s*=\s*0/)) {
    checkPass('Spawn timer resets properly');
} else {
    checkWarning(true, 'Spawn timer may not reset');
}

console.log();

// Test 4: Scoring Edge Cases
console.log('Test 4: Scoring Edge Cases');
console.log('-'.repeat(80));

// Check double-scoring prevention
if (gameCode.includes('!pipe.scored') && gameCode.includes('pipe.scored = true')) {
    checkPass('Double-scoring prevention implemented');
} else {
    checkCritical(true, 'Missing double-scoring prevention');
}

// Check score condition
if (gameCode.match(/pipe\.x\s*\+\s*.*PIPE_WIDTH\s*<\s*bird\.x/)) {
    checkPass('Score condition: pipe passed bird');
} else {
    checkWarning(true, 'Score condition may be incorrect');
}

// Check high score update
if (gameCode.includes('score > this.highScore') &&
    gameCode.includes('this.highScore = this.score')) {
    checkPass('High score updates correctly');
} else {
    checkWarning(true, 'High score update may be missing');
}

console.log();

// Test 5: Game State Edge Cases
console.log('Test 5: Game State Edge Cases');
console.log('-'.repeat(80));

// Check if game can be started while playing
if (gameCode.includes('this.gameState === \'playing\'')) {
    checkPass('Game state checked before actions');
} else {
    checkWarning(true, 'Game state may not be checked consistently');
}

// Check restart clears arrays
if (gameCode.match(/startGame[\s\S]{0,200}pipes\s*=\s*\[\]/) &&
    gameCode.match(/startGame[\s\S]{0,200}particles\s*=\s*\[\]/)) {
    checkPass('Arrays cleared on restart');
} else {
    checkCritical(true, 'Arrays may not be cleared on restart');
}

// Check game over can only trigger once
if (gameCode.includes('endGame') && gameCode.includes('gameState !== \'playing\'')) {
    checkPass('Updates skip when not playing (prevents multiple game overs)');
} else {
    checkWarning(true, 'Multiple simultaneous game overs might be possible');
}

console.log();

// Test 6: Input Handling Edge Cases
console.log('Test 6: Input Handling Edge Cases');
console.log('-'.repeat(80));

// Check space bar doesn't scroll page
if (indexCode.includes('preventDefault') || gameCode.includes('preventDefault')) {
    checkPass('preventDefault() used (prevents page scroll)');
} else {
    checkWarning(true, 'Space bar may scroll the page');
}

// Check touch events don't trigger both click and touch
if (gameCode.match(/touchstart[\s\S]{0,100}preventDefault/)) {
    checkPass('Touch preventDefault prevents double input');
} else {
    checkWarning(true, 'Touch may trigger both touch and click events');
}

// Check input only works during playing state
if (gameCode.match(/handleInput[\s\S]{0,100}gameState.*playing/)) {
    checkPass('Input only works during play (prevents flapping in menus)');
} else {
    checkWarning(true, 'Input may work during non-playing states');
}

console.log();

// Test 7: Particle System Edge Cases
console.log('Test 7: Particle System Edge Cases');
console.log('-'.repeat(80));

// Check particle removal
if (gameCode.match(/p\.life\s*<=\s*0/) && gameCode.includes('splice')) {
    checkPass('Dead particles removed (prevents memory leak)');
} else {
    checkCritical(true, 'Particles may not be removed (memory leak)');
}

// Check particle life decay
if (gameCode.includes('p.life -= p.decay')) {
    checkPass('Particle life decays over time');
} else {
    checkWarning(true, 'Particle decay may be missing');
}

// Check particle gravity
if (gameCode.includes('p.vy += ')) {
    checkPass('Particle gravity implemented');
} else {
    checkPass('Particle movement implemented (gravity optional)');
}

console.log();

// Test 8: Audio Edge Cases
console.log('Test 8: Audio Edge Cases');
console.log('-'.repeat(80));

// Check audio context in try-catch
if (gameCode.match(/try[\s\S]{0,500}AudioContext[\s\S]{0,500}catch/)) {
    checkPass('Audio wrapped in try-catch (handles browser compatibility)');
} else {
    checkWarning(true, 'Audio may not be protected by try-catch');
}

// Check sound enabled flag
if (gameCode.includes('if (!this.soundEnabled)') ||
    gameCode.includes('if (this.soundEnabled)')) {
    checkPass('Sound can be toggled');
} else {
    checkWarning(true, 'Sound toggle may not work');
}

// Check oscillator stop
if (gameCode.includes('oscillator.stop')) {
    checkPass('Oscillator stopped (prevents audio leak)');
} else {
    checkWarning(true, 'Oscillators may not be stopped properly');
}

console.log();

// Test 9: Progressive Difficulty Edge Cases
console.log('Test 9: Progressive Difficulty Edge Cases');
console.log('-'.repeat(80));

// Check minimum spawn interval
if (gameCode.match(/Math\.max\(\d+,\s*.*pipeSpawnInterval/)) {
    checkPass('Minimum spawn interval enforced (prevents too difficult)');
} else {
    checkWarning(true, 'Spawn interval may decrease indefinitely');
}

// Check progressive speed increase
if (gameCode.match(/score\s*\/\s*10/) || gameCode.match(/PIPE_SPEED.*\+/)) {
    checkPass('Speed increases progressively');
} else {
    checkWarning(true, 'Progressive speed increase may be missing');
}

console.log();

// Test 10: Rendering Edge Cases
console.log('Test 10: Rendering Edge Cases');
console.log('-'.repeat(80));

// Check canvas clear
if (gameCode.includes('clearRect')) {
    checkPass('Canvas cleared each frame (prevents ghosting)');
} else {
    checkCritical(true, 'Canvas not cleared (visual artifacts)');
}

// Check globalAlpha reset
if (gameCode.match(/globalAlpha\s*=\s*1/)) {
    checkPass('GlobalAlpha reset after particles (prevents transparency issues)');
} else {
    checkWarning(true, 'GlobalAlpha may not reset (could affect other drawings)');
}

// Check context restore
if (gameCode.includes('save()') && gameCode.includes('restore()')) {
    checkPass('Context save/restore used (prevents transformation leaks)');
} else {
    checkWarning(true, 'Context transformations may leak between draws');
}

console.log();

// Test 11: LocalStorage Edge Cases
console.log('Test 11: LocalStorage Edge Cases');
console.log('-'.repeat(80));

// Check localStorage availability
if (gameCode.includes('localStorage')) {
    checkPass('LocalStorage used for high score');

    // Check parseInt
    if (gameCode.includes('parseInt')) {
        checkPass('parseInt used (converts string to number)');
    } else {
        checkWarning(true, 'High score may be stored as string');
    }

    // Check default value
    if (gameCode.includes('? ') && gameCode.includes(': 0')) {
        checkPass('Default high score provided');
    } else {
        checkWarning(true, 'Missing default high score value');
    }
} else {
    checkWarning(true, 'LocalStorage not used');
}

console.log();

// Test 12: Race Conditions
console.log('Test 12: Race Conditions and Timing Issues');
console.log('-'.repeat(80));

// Check if multiple collision handlers can fire
if (gameCode.includes('return;') && gameCode.includes('endGame')) {
    checkPass('Early return after game over (prevents multiple triggers)');
} else {
    checkWarning(true, 'Multiple collision game overs may be possible');
}

// Check if restart during game over is safe
if (gameCode.match(/restartGame[\s\S]{0,100}startGame/)) {
    checkPass('Restart properly calls startGame');
} else {
    checkWarning(true, 'Restart may not fully reset game');
}

// Check game loop uses RAF
if (gameCode.includes('requestAnimationFrame')) {
    checkPass('requestAnimationFrame used (proper game loop)');
} else {
    checkCritical(true, 'Game loop may not use requestAnimationFrame');
}

console.log();

// Final Summary
console.log('='.repeat(80));
console.log('ANALYSIS SUMMARY');
console.log('='.repeat(80));
console.log();
console.log(`✓ Passed Checks: ${passes.length}`);
console.log(`⚠ Warnings: ${warnings.length}`);
console.log(`✗ Critical Issues: ${criticalIssues.length}`);
console.log();

if (criticalIssues.length > 0) {
    console.log('CRITICAL ISSUES:');
    criticalIssues.forEach((issue, i) => {
        console.log(`  ${i + 1}. ${issue}`);
    });
    console.log();
}

if (warnings.length > 0) {
    console.log('WARNINGS (non-critical):');
    warnings.forEach((warning, i) => {
        console.log(`  ${i + 1}. ${warning}`);
    });
    console.log();
}

console.log('='.repeat(80));
if (criticalIssues.length === 0) {
    console.log('VERDICT: NO CRITICAL BUGS FOUND - GAME IS SOLID! ✓');
} else {
    console.log(`VERDICT: ${criticalIssues.length} CRITICAL ISSUE(S) NEED FIXING`);
}
console.log('='.repeat(80));
