/**
 * 2048 Game Logic
 * Core game mechanics including grid management, tile movement, and merging
 */

class Game {
    constructor() {
        this.size = 4;
        this.grid = this.createEmptyGrid();
        this.score = 0;
        this.bestScore = this.loadBestScore();
        this.won = false;
        this.over = false;
        this.keepPlaying = false;
        this.history = [];
        this.maxHistorySize = 10;
    }

    /**
     * Create an empty 4x4 grid
     */
    createEmptyGrid() {
        return Array(this.size).fill(null).map(() => Array(this.size).fill(0));
    }

    /**
     * Save current state to history for undo
     */
    saveState() {
        const state = {
            grid: this.grid.map(row => [...row]),
            score: this.score,
            won: this.won,
            over: this.over
        };
        this.history.push(state);

        // Limit history size to prevent memory issues
        if (this.history.length > this.maxHistorySize) {
            this.history.shift();
        }
    }

    /**
     * Restore previous state (undo)
     */
    restoreState() {
        if (this.history.length === 0) {
            return false;
        }

        const state = this.history.pop();
        this.grid = state.grid.map(row => [...row]);
        this.score = state.score;
        this.won = state.won;
        this.over = state.over;

        return true;
    }

    /**
     * Check if undo is available
     */
    canUndo() {
        return this.history.length > 0;
    }

    /**
     * Start a new game
     */
    startNewGame() {
        this.grid = this.createEmptyGrid();
        this.score = 0;
        this.won = false;
        this.over = false;
        this.keepPlaying = false;
        this.history = [];

        this.addRandomTile();
        this.addRandomTile();
    }

    /**
     * Continue playing after winning
     */
    continueGame() {
        this.keepPlaying = true;
        this.won = false;
    }

    /**
     * Add a random tile (2 or 4) to an empty cell
     */
    addRandomTile() {
        const emptyCells = this.getEmptyCells();

        if (emptyCells.length === 0) {
            return null;
        }

        const randomCell = emptyCells[Math.floor(Math.random() * emptyCells.length)];
        const value = Math.random() < 0.9 ? 2 : 4; // 90% chance of 2, 10% chance of 4

        this.grid[randomCell.row][randomCell.col] = value;

        return { row: randomCell.row, col: randomCell.col, value };
    }

    /**
     * Get all empty cells in the grid
     */
    getEmptyCells() {
        const cells = [];

        for (let row = 0; row < this.size; row++) {
            for (let col = 0; col < this.size; col++) {
                if (this.grid[row][col] === 0) {
                    cells.push({ row, col });
                }
            }
        }

        return cells;
    }

    /**
     * Move tiles in the specified direction
     * @param {string} direction - 'up', 'down', 'left', 'right'
     * @returns {object} Move result with moved tiles and score gained
     */
    move(direction) {
        if (this.over && !this.keepPlaying) {
            return { moved: false, tiles: [], scoreGained: 0 };
        }

        // Save state before move
        this.saveState();

        const moveResult = this.performMove(direction);

        if (!moveResult.moved) {
            // No tiles moved, remove the saved state
            this.history.pop();
            return moveResult;
        }

        // Add score from merged tiles
        this.score += moveResult.scoreGained;

        // Update best score
        if (this.score > this.bestScore) {
            this.bestScore = this.score;
            this.saveBestScore();
        }

        // Add new random tile
        const newTile = this.addRandomTile();
        if (newTile) {
            moveResult.newTile = newTile;
        }

        // Check for win condition (2048 tile)
        if (!this.won && !this.keepPlaying && this.hasWon()) {
            this.won = true;
        }

        // Check for game over
        if (!this.canMove()) {
            this.over = true;
        }

        return moveResult;
    }

    /**
     * Perform the actual move operation
     */
    performMove(direction) {
        let moved = false;
        const tiles = [];
        let scoreGained = 0;
        const merged = this.createEmptyGrid(); // Track merged tiles

        // Transform grid based on direction
        const { grid, transform, reverse } = this.prepareGrid(direction);

        // Process each row
        for (let row = 0; row < this.size; row++) {
            const rowResult = this.processRow(grid[row], merged[row]);

            if (rowResult.moved) {
                moved = true;
                grid[row] = rowResult.row;
                scoreGained += rowResult.scoreGained;

                // Track tile movements
                rowResult.tiles.forEach(tile => {
                    const pos = reverse(row, tile.col);
                    tiles.push({
                        ...tile,
                        from: tile.from ? reverse(row, tile.from) : null,
                        to: pos,
                        row: pos.row,
                        col: pos.col
                    });
                });
            }
        }

        // Apply transformed grid back
        this.applyGrid(grid, direction);

        return { moved, tiles, scoreGained };
    }

    /**
     * Prepare grid for movement in specified direction
     */
    prepareGrid(direction) {
        const grid = this.createEmptyGrid();
        let transform, reverse;

        switch (direction) {
            case 'left':
                // Copy grid as-is
                for (let row = 0; row < this.size; row++) {
                    grid[row] = [...this.grid[row]];
                }
                transform = (row, col) => ({ row, col });
                reverse = (row, col) => ({ row, col });
                break;

            case 'right':
                // Reverse each row
                for (let row = 0; row < this.size; row++) {
                    grid[row] = [...this.grid[row]].reverse();
                }
                transform = (row, col) => ({ row, col: this.size - 1 - col });
                reverse = (row, col) => ({ row, col: this.size - 1 - col });
                break;

            case 'up':
                // Transpose grid
                for (let row = 0; row < this.size; row++) {
                    for (let col = 0; col < this.size; col++) {
                        grid[row][col] = this.grid[col][row];
                    }
                }
                transform = (row, col) => ({ row: col, col: row });
                reverse = (row, col) => ({ row: col, col: row });
                break;

            case 'down':
                // For each column, reverse it and put it as a row
                for (let col = 0; col < this.size; col++) {
                    for (let row = 0; row < this.size; row++) {
                        grid[col][row] = this.grid[row][col];
                    }
                    grid[col].reverse();
                }
                transform = (row, col) => ({ row: col, col: row });
                reverse = (row, col) => ({ row: col, col: row });
                break;
        }

        return { grid, transform, reverse };
    }

    /**
     * Apply transformed grid back to the game grid
     */
    applyGrid(grid, direction) {
        switch (direction) {
            case 'left':
                this.grid = grid.map(row => [...row]);
                break;

            case 'right':
                this.grid = grid.map(row => [...row].reverse());
                break;

            case 'up':
                for (let row = 0; row < this.size; row++) {
                    for (let col = 0; col < this.size; col++) {
                        this.grid[row][col] = grid[col][row];
                    }
                }
                break;

            case 'down':
                // Reverse the transformation: each row becomes a reversed column
                for (let col = 0; col < this.size; col++) {
                    const reversedRow = [...grid[col]].reverse();
                    for (let row = 0; row < this.size; row++) {
                        this.grid[row][col] = reversedRow[row];
                    }
                }
                break;
        }
    }

    /**
     * Process a single row: move tiles left and merge
     */
    processRow(row, mergedRow) {
        const original = [...row];
        const tiles = [];
        let scoreGained = 0;
        let moved = false;

        // Compress: move all non-zero tiles to the left
        const compressed = row.filter(cell => cell !== 0);

        // Merge: combine adjacent equal tiles
        const merged = [];
        let skip = false;

        for (let i = 0; i < compressed.length; i++) {
            if (skip) {
                skip = false;
                continue;
            }

            if (i < compressed.length - 1 && compressed[i] === compressed[i + 1]) {
                // Merge tiles
                const mergedValue = compressed[i] * 2;
                merged.push(mergedValue);
                scoreGained += mergedValue;
                skip = true;

                tiles.push({
                    value: mergedValue,
                    col: merged.length - 1,
                    merged: true,
                    from: null // Will be filled by caller
                });
            } else {
                merged.push(compressed[i]);

                if (merged.length - 1 !== row.indexOf(compressed[i])) {
                    tiles.push({
                        value: compressed[i],
                        col: merged.length - 1,
                        merged: false,
                        from: row.indexOf(compressed[i])
                    });
                }
            }
        }

        // Pad with zeros
        while (merged.length < this.size) {
            merged.push(0);
        }

        // Check if anything changed
        moved = !original.every((val, idx) => val === merged[idx]);

        return { row: merged, moved, tiles, scoreGained };
    }

    /**
     * Check if the player has won (reached 2048)
     */
    hasWon() {
        for (let row = 0; row < this.size; row++) {
            for (let col = 0; col < this.size; col++) {
                if (this.grid[row][col] === 2048) {
                    return true;
                }
            }
        }
        return false;
    }

    /**
     * Check if any moves are possible
     */
    canMove() {
        // Check for empty cells
        if (this.getEmptyCells().length > 0) {
            return true;
        }

        // Check for adjacent matching tiles
        for (let row = 0; row < this.size; row++) {
            for (let col = 0; col < this.size; col++) {
                const value = this.grid[row][col];

                // Check right
                if (col < this.size - 1 && value === this.grid[row][col + 1]) {
                    return true;
                }

                // Check down
                if (row < this.size - 1 && value === this.grid[row + 1][col]) {
                    return true;
                }
            }
        }

        return false;
    }

    /**
     * Get current grid state
     */
    getGrid() {
        return this.grid.map(row => [...row]);
    }

    /**
     * Get current score
     */
    getScore() {
        return this.score;
    }

    /**
     * Get best score
     */
    getBestScore() {
        return this.bestScore;
    }

    /**
     * Check if game is won
     */
    isWon() {
        return this.won;
    }

    /**
     * Check if game is over
     */
    isOver() {
        return this.over;
    }

    /**
     * Save best score to localStorage
     */
    saveBestScore() {
        try {
            localStorage.setItem('bestScore', this.bestScore.toString());
        } catch (e) {
            // localStorage might not be available
            console.error('Failed to save best score:', e);
        }
    }

    /**
     * Load best score from localStorage
     */
    loadBestScore() {
        try {
            const saved = localStorage.getItem('bestScore');
            return saved ? parseInt(saved, 10) : 0;
        } catch (e) {
            // localStorage might not be available
            console.error('Failed to load best score:', e);
            return 0;
        }
    }

    /**
     * Get all tiles with their positions
     */
    getAllTiles() {
        const tiles = [];

        for (let row = 0; row < this.size; row++) {
            for (let col = 0; col < this.size; col++) {
                if (this.grid[row][col] !== 0) {
                    tiles.push({
                        row,
                        col,
                        value: this.grid[row][col]
                    });
                }
            }
        }

        return tiles;
    }
}

// Export for testing
if (typeof module !== 'undefined' && module.exports) {
    module.exports = Game;
}
