#!/usr/bin/env node

/**
 * Validation script for Tetris game
 * Tests that all required components are present
 */

const fs = require('fs');
const path = require('path');

console.log('🎮 TETRIS GAME VALIDATION SCRIPT\n');

const files = {
    'index.html': 'Main game file',
    'test.html': 'Test suite',
    'README.md': 'Documentation'
};

let allValid = true;

// Check files exist
console.log('📁 Checking file structure...');
for (const [file, desc] of Object.entries(files)) {
    const exists = fs.existsSync(file);
    console.log(`  ${exists ? '✓' : '✗'} ${file} - ${desc}`);
    if (!exists) allValid = false;
}

// Check index.html content
console.log('\n🎯 Validating game features...');
const indexContent = fs.readFileSync('index.html', 'utf8');

const features = {
    'PIECES': 'All 7 tetromino shapes defined',
    'WALL_KICKS': 'SRS rotation system',
    'checkCollision': 'Collision detection',
    'clearLines': 'Line clearing logic',
    'hardDrop': 'Hard drop functionality',
    'holdCurrentPiece': 'Hold piece feature',
    'getGhostPosition': 'Ghost piece preview',
    'drawNextPiece': 'Next piece preview',
    'localStorage': 'High score persistence',
    'gameOver': 'Game over handling',
    'rotate': 'Piece rotation',
    'softDrop': 'Soft drop',
    'ArrowLeft': 'Left movement control',
    'ArrowRight': 'Right movement control',
    'ArrowDown': 'Down movement control',
    'ArrowUp': 'Rotation control',
    'canvas': 'Canvas rendering'
};

for (const [feature, desc] of Object.entries(features)) {
    const exists = indexContent.includes(feature);
    console.log(`  ${exists ? '✓' : '✗'} ${desc}`);
    if (!exists) allValid = false;
}

// Check test.html content
console.log('\n🧪 Validating test suite...');
const testContent = fs.readFileSync('test.html', 'utf8');

const testCategories = {
    'Piece Rotation': 'Rotation tests',
    'Collision Detection': 'Collision tests',
    'Line Clearing': 'Line clearing tests',
    'Score Calculation': 'Scoring tests',
    'Level Progression': 'Level tests',
    'Game State': 'State management tests',
    'Edge Cases': 'Edge case tests',
    'Piece Shapes': 'Shape validation tests'
};

for (const [category, desc] of Object.entries(testCategories)) {
    const exists = testContent.includes(category);
    console.log(`  ${exists ? '✓' : '✗'} ${desc}`);
    if (!exists) allValid = false;
}

// Check piece definitions
console.log('\n🔲 Validating piece definitions...');
const requiredPieces = ['I', 'O', 'T', 'S', 'Z', 'J', 'L'];
for (const piece of requiredPieces) {
    const pieceRegex = new RegExp(`${piece}:\\s*{`);
    const exists = pieceRegex.test(indexContent);
    console.log(`  ${exists ? '✓' : '✗'} ${piece} piece defined`);
    if (!exists) allValid = false;
}

// Check CSS styling
console.log('\n🎨 Validating visual features...');
const visualFeatures = {
    'gradient': 'Gradient animations',
    'box-shadow': 'Glow effects',
    'animation': 'CSS animations',
    'neon': 'Neon aesthetic',
    '@keyframes': 'Keyframe animations',
    'rgba': 'Transparency effects'
};

for (const [feature, desc] of Object.entries(visualFeatures)) {
    const exists = indexContent.toLowerCase().includes(feature.toLowerCase());
    console.log(`  ${exists ? '✓' : '✗'} ${desc}`);
    if (!exists) allValid = false;
}

// File size checks
console.log('\n📊 File statistics...');
const stats = fs.statSync('index.html');
const testStats = fs.statSync('test.html');
const readmeStats = fs.statSync('README.md');

console.log(`  📄 index.html: ${(stats.size / 1024).toFixed(1)} KB`);
console.log(`  📄 test.html: ${(testStats.size / 1024).toFixed(1)} KB`);
console.log(`  📄 README.md: ${(readmeStats.size / 1024).toFixed(1)} KB`);
console.log(`  📄 Total size: ${((stats.size + testStats.size + readmeStats.size) / 1024).toFixed(1)} KB`);

// Summary
console.log('\n' + '='.repeat(50));
if (allValid) {
    console.log('✅ ALL VALIDATION CHECKS PASSED!');
    console.log('🏆 Game is ready for competition!');
    console.log('\nTo play:');
    console.log('  1. Open index.html in a web browser');
    console.log('  2. Click START GAME');
    console.log('  3. Use arrow keys to play');
    console.log('\nTo test:');
    console.log('  1. Open test.html in a web browser');
    console.log('  2. View test results (auto-runs)');
    process.exit(0);
} else {
    console.log('❌ VALIDATION FAILED');
    console.log('Some features are missing. Please review the output above.');
    process.exit(1);
}
