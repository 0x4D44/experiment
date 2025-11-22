/**
 * Conway's Game of Life - Core Logic
 * Implements efficient cellular automaton with toroidal topology
 */

class GameOfLife {
    constructor(width, height) {
        this.width = width;
        this.height = height;
        this.generation = 0;
        this.wrapAround = true;

        // Use two grids for double buffering (current and next state)
        this.grid = this.createGrid();
        this.nextGrid = this.createGrid();

        // Track cell ages for coloring (0 = dead, >0 = generations alive)
        this.ages = this.createGrid();
        this.nextAges = this.createGrid();

        // Active cells optimization - only check cells that might change
        this.activeCells = new Set();
    }

    /**
     * Create an empty grid
     */
    createGrid() {
        return Array(this.height).fill(null).map(() => Array(this.width).fill(0));
    }

    /**
     * Set cell state at (x, y)
     */
    setCell(x, y, alive) {
        if (x < 0 || x >= this.width || y < 0 || y >= this.height) {
            return;
        }

        this.grid[y][x] = alive ? 1 : 0;

        if (alive) {
            this.ages[y][x] = 1;
            this.addActiveCell(x, y);
            // Also mark neighbors as potentially active
            this.addNeighborsToActive(x, y);
        } else {
            this.ages[y][x] = 0;
        }
    }

    /**
     * Get cell state at (x, y)
     */
    getCell(x, y) {
        if (this.wrapAround) {
            x = (x + this.width) % this.width;
            y = (y + this.height) % this.height;
        } else {
            if (x < 0 || x >= this.width || y < 0 || y >= this.height) {
                return 0;
            }
        }
        return this.grid[y][x];
    }

    /**
     * Toggle cell at (x, y)
     */
    toggleCell(x, y) {
        if (x < 0 || x >= this.width || y < 0 || y >= this.height) {
            return;
        }
        this.setCell(x, y, !this.grid[y][x]);
    }

    /**
     * Count live neighbors for cell at (x, y)
     */
    countNeighbors(x, y) {
        let count = 0;

        // Check all 8 neighbors
        for (let dy = -1; dy <= 1; dy++) {
            for (let dx = -1; dx <= 1; dx++) {
                if (dx === 0 && dy === 0) continue; // Skip self
                count += this.getCell(x + dx, y + dy);
            }
        }

        return count;
    }

    /**
     * Add cell and its neighbors to active set
     */
    addActiveCell(x, y) {
        this.activeCells.add(`${x},${y}`);
        this.addNeighborsToActive(x, y);
    }

    /**
     * Add all neighbors to active set
     */
    addNeighborsToActive(x, y) {
        for (let dy = -1; dy <= 1; dy++) {
            for (let dx = -1; dx <= 1; dx++) {
                let nx = x + dx;
                let ny = y + dy;

                if (this.wrapAround) {
                    nx = (nx + this.width) % this.width;
                    ny = (ny + this.height) % this.height;
                } else {
                    if (nx < 0 || nx >= this.width || ny < 0 || ny >= this.height) {
                        continue;
                    }
                }

                this.activeCells.add(`${nx},${ny}`);
            }
        }
    }

    /**
     * Compute next generation using Conway's rules
     */
    step() {
        // Build set of cells to check (active cells only for optimization)
        const cellsToCheck = new Set(this.activeCells);
        const newActiveCells = new Set();

        // Process each active cell
        for (const key of cellsToCheck) {
            const [x, y] = key.split(',').map(Number);

            const alive = this.grid[y][x];
            const neighbors = this.countNeighbors(x, y);

            // Apply Conway's rules
            let nextState = 0;

            if (alive) {
                // Survival: 2 or 3 neighbors
                if (neighbors === 2 || neighbors === 3) {
                    nextState = 1;
                    this.nextAges[y][x] = this.ages[y][x] + 1;
                } else {
                    // Death by underpopulation or overcrowding
                    nextState = 0;
                    this.nextAges[y][x] = 0;
                }
            } else {
                // Birth: exactly 3 neighbors
                if (neighbors === 3) {
                    nextState = 1;
                    this.nextAges[y][x] = 1;
                } else {
                    nextState = 0;
                    this.nextAges[y][x] = 0;
                }
            }

            this.nextGrid[y][x] = nextState;

            // If cell is alive or has live neighbors, keep it active
            if (nextState || neighbors > 0) {
                newActiveCells.add(key);
                this.addNeighborsToActive(x, y);
            }
        }

        // Swap grids (double buffering)
        [this.grid, this.nextGrid] = [this.nextGrid, this.grid];
        [this.ages, this.nextAges] = [this.nextAges, this.ages];

        // Update active cells for next iteration
        this.activeCells = newActiveCells;

        this.generation++;
    }

    /**
     * Clear all cells
     */
    clear() {
        this.grid = this.createGrid();
        this.nextGrid = this.createGrid();
        this.ages = this.createGrid();
        this.nextAges = this.createGrid();
        this.activeCells.clear();
        this.generation = 0;
    }

    /**
     * Randomize grid with given density (0-1)
     */
    randomize(density = 0.3) {
        this.clear();

        for (let y = 0; y < this.height; y++) {
            for (let x = 0; x < this.width; x++) {
                if (Math.random() < density) {
                    this.setCell(x, y, true);
                }
            }
        }
    }

    /**
     * Get population count
     */
    getPopulation() {
        let count = 0;
        for (let y = 0; y < this.height; y++) {
            for (let x = 0; x < this.width; x++) {
                count += this.grid[y][x];
            }
        }
        return count;
    }

    /**
     * Load pattern at center or specified position
     */
    loadPattern(pattern, centerX = null, centerY = null) {
        if (centerX === null) centerX = Math.floor(this.width / 2);
        if (centerY === null) centerY = Math.floor(this.height / 2);

        // Calculate offset to center pattern
        const offsetX = centerX - Math.floor(pattern.width / 2);
        const offsetY = centerY - Math.floor(pattern.height / 2);

        // Place pattern cells
        for (let i = 0; i < pattern.cells.length; i++) {
            const [dx, dy] = pattern.cells[i];
            const x = offsetX + dx;
            const y = offsetY + dy;

            if (x >= 0 && x < this.width && y >= 0 && y < this.height) {
                this.setCell(x, y, true);
            }
        }
    }

    /**
     * Resize grid (clears current state)
     */
    resize(width, height) {
        this.width = width;
        this.height = height;
        this.clear();
    }

    /**
     * Get cell age for coloring
     */
    getCellAge(x, y) {
        if (x < 0 || x >= this.width || y < 0 || y >= this.height) {
            return 0;
        }
        return this.ages[y][x];
    }
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = GameOfLife;
}
