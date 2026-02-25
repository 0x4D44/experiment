#!/usr/bin/env node
/**
 * Node.js test runner for Conway's Game of Life
 */

// Load the patterns
const fs = require('fs');
const path = require('path');

// Make GameOfLife and PATTERNS available globally for the tests
global.PATTERNS = require(path.join(__dirname, 'js/patterns.js'));
global.GameOfLife = require(path.join(__dirname, 'js/game-of-life.js'));

// Read and evaluate the test file - it exports runner via module.exports
const testRunner = require(path.join(__dirname, 'tests/game-of-life.test.js'));

// Run the tests
const success = testRunner.run();

// Exit with appropriate code
process.exit(success ? 0 : 1);
