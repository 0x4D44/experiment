#!/usr/bin/env node

class MinesweeperValidator {
    constructor(rows, cols, mines) {
        this.rows = rows;
        this.cols = cols;
        this.mineCount = mines;
        this.grid = Array(rows).fill(null).map(() => Array(cols).fill(0));
        this.revealed = Array(rows).fill(null).map(() => Array(cols).fill(false));
        this.flagged = Array(rows).fill(null).map(() => Array(cols).fill(false));
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

    printGrid() {
        console.log('\nGrid (M=mine, numbers=adjacent mines):');
        for (let row = 0; row < this.rows; row++) {
            let line = '';
            for (let col = 0; col < this.cols; col++) {
                if (this.grid[row][col] === -1) {
                    line += ' M';
                } else {
                    line += ' ' + this.grid[row][col];
                }
            }
            console.log(line);
        }
    }

    printRevealed() {
        console.log('\nRevealed (X=revealed, .=covered):');
        for (let row = 0; row < this.rows; row++) {
            let line = '';
            for (let col = 0; col < this.cols; col++) {
                line += this.revealed[row][col] ? ' X' : ' .';
            }
            console.log(line);
        }
    }
}

// Test the scenario
console.log('Testing: Multiple flood fill operations');
console.log('Creating 10x10 board with mine at (5,5)');

const game = new MinesweeperValidator(10, 10, 0);
game.placeMinesAt([[5,5]]);

game.printGrid();

console.log('\n1. Revealing (0,0) - top-left corner');
game.revealCell(0, 0);
const first = game.countRevealed();
console.log(`Cells revealed: ${first}`);
game.printRevealed();

console.log('\n2. Revealing (9,9) - bottom-right corner');
game.revealCell(9, 9);
const second = game.countRevealed();
console.log(`Cells revealed: ${second}`);
game.printRevealed();

console.log(`\nFirst reveal: ${first} cells`);
console.log(`Second reveal: ${second} cells`);
console.log(`Difference: ${second - first} cells`);

if (second > first) {
    console.log('\n✓ Test PASSED - Additional cells were revealed');
} else {
    console.log('\n✗ Test FAILED - No additional cells revealed');
    console.log('This happens when all cells are already revealed in first flood fill');
    console.log('because the mine at (5,5) is not blocking the flood fill path');
}
