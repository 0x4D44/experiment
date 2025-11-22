/**
 * 2048 Game - Core Game Logic
 * A complete implementation of the popular 2048 puzzle game
 */

class Tile {
  constructor(value, row, col) {
    this.value = value;
    this.row = row;
    this.col = col;
    this.previousPosition = null;
    this.mergedFrom = null;
  }

  savePosition() {
    this.previousPosition = { row: this.row, col: this.col };
  }

  updatePosition(position) {
    this.row = position.row;
    this.col = position.col;
  }
}

class Grid {
  constructor(size = 4) {
    this.size = size;
    this.cells = this.empty();
  }

  empty() {
    const cells = [];
    for (let row = 0; row < this.size; row++) {
      cells[row] = [];
      for (let col = 0; col < this.size; col++) {
        cells[row][col] = null;
      }
    }
    return cells;
  }

  randomAvailableCell() {
    const cells = this.availableCells();
    if (cells.length > 0) {
      return cells[Math.floor(Math.random() * cells.length)];
    }
  }

  availableCells() {
    const cells = [];
    for (let row = 0; row < this.size; row++) {
      for (let col = 0; col < this.size; col++) {
        if (!this.cells[row][col]) {
          cells.push({ row, col });
        }
      }
    }
    return cells;
  }

  cellsAvailable() {
    return this.availableCells().length > 0;
  }

  cellAvailable(position) {
    return !this.cellOccupied(position);
  }

  cellOccupied(position) {
    return !!this.cellContent(position);
  }

  cellContent(position) {
    if (this.withinBounds(position)) {
      return this.cells[position.row][position.col];
    }
    return null;
  }

  insertTile(tile) {
    this.cells[tile.row][tile.col] = tile;
  }

  removeTile(tile) {
    this.cells[tile.row][tile.col] = null;
  }

  withinBounds(position) {
    return (
      position.row >= 0 &&
      position.row < this.size &&
      position.col >= 0 &&
      position.col < this.size
    );
  }

  clone() {
    const clonedGrid = new Grid(this.size);
    for (let row = 0; row < this.size; row++) {
      for (let col = 0; col < this.size; col++) {
        const cell = this.cells[row][col];
        if (cell) {
          clonedGrid.cells[row][col] = new Tile(cell.value, row, col);
        }
      }
    }
    return clonedGrid;
  }
}

class GameManager {
  constructor(size = 4) {
    this.size = size;
    this.startTiles = 2;
    this.grid = new Grid(this.size);
    this.score = 0;
    this.over = false;
    this.won = false;
    this.keepPlaying = false;

    // Direction vectors
    this.vectors = {
      0: { row: -1, col: 0 }, // Up
      1: { row: 0, col: 1 },  // Right
      2: { row: 1, col: 0 },  // Down
      3: { row: 0, col: -1 }  // Left
    };

    this.setup();
  }

  setup() {
    this.grid = new Grid(this.size);
    this.score = 0;
    this.over = false;
    this.won = false;
    this.keepPlaying = false;

    // Add the initial tiles
    this.addStartTiles();
  }

  restart() {
    this.setup();
  }

  keepPlayingAfterWin() {
    this.keepPlaying = true;
  }

  isGameTerminated() {
    return this.over || (this.won && !this.keepPlaying);
  }

  addStartTiles() {
    for (let i = 0; i < this.startTiles; i++) {
      this.addRandomTile();
    }
  }

  addRandomTile() {
    if (this.grid.cellsAvailable()) {
      const value = Math.random() < 0.9 ? 2 : 4;
      const cell = this.grid.randomAvailableCell();
      const tile = new Tile(value, cell.row, cell.col);
      this.grid.insertTile(tile);
      return tile;
    }
    return null;
  }

  prepareTiles() {
    for (let row = 0; row < this.size; row++) {
      for (let col = 0; col < this.size; col++) {
        const tile = this.grid.cells[row][col];
        if (tile) {
          tile.mergedFrom = null;
          tile.savePosition();
        }
      }
    }
  }

  moveTile(tile, cell) {
    this.grid.cells[tile.row][tile.col] = null;
    this.grid.cells[cell.row][cell.col] = tile;
    tile.updatePosition(cell);
  }

  move(direction) {
    if (this.isGameTerminated()) return false;

    let moved = false;
    const vector = this.vectors[direction];
    const traversals = this.buildTraversals(vector);

    this.prepareTiles();

    traversals.row.forEach((row) => {
      traversals.col.forEach((col) => {
        const cell = { row, col };
        const tile = this.grid.cellContent(cell);

        if (tile) {
          const positions = this.findFarthestPosition(cell, vector);
          const next = this.grid.cellContent(positions.next);

          // Check if tiles can merge
          if (next && next.value === tile.value && !next.mergedFrom) {
            const merged = new Tile(tile.value * 2, positions.next.row, positions.next.col);
            merged.mergedFrom = [tile, next];

            this.grid.insertTile(merged);
            this.grid.removeTile(tile);

            // Update position for animation
            tile.updatePosition(positions.next);

            // Update score
            this.score += merged.value;

            // Check for win condition
            if (merged.value === 2048) {
              this.won = true;
            }

            moved = true;
          } else {
            this.moveTile(tile, positions.farthest);
          }

          if (!this.positionsEqual(cell, tile)) {
            moved = true;
          }
        }
      });
    });

    if (moved) {
      this.addRandomTile();

      if (!this.movesAvailable()) {
        this.over = true;
      }
    }

    return moved;
  }

  buildTraversals(vector) {
    const traversals = { row: [], col: [] };

    for (let pos = 0; pos < this.size; pos++) {
      traversals.row.push(pos);
      traversals.col.push(pos);
    }

    // Always traverse from the farthest cell in the chosen direction
    if (vector.row === 1) traversals.row = traversals.row.reverse();
    if (vector.col === 1) traversals.col = traversals.col.reverse();

    return traversals;
  }

  findFarthestPosition(cell, vector) {
    let previous;

    do {
      previous = cell;
      cell = { row: previous.row + vector.row, col: previous.col + vector.col };
    } while (this.grid.withinBounds(cell) && this.grid.cellAvailable(cell));

    return {
      farthest: previous,
      next: cell
    };
  }

  movesAvailable() {
    return this.grid.cellsAvailable() || this.tileMatchesAvailable();
  }

  tileMatchesAvailable() {
    for (let row = 0; row < this.size; row++) {
      for (let col = 0; col < this.size; col++) {
        const tile = this.grid.cellContent({ row, col });

        if (tile) {
          for (let direction = 0; direction < 4; direction++) {
            const vector = this.vectors[direction];
            const cell = { row: row + vector.row, col: col + vector.col };
            const other = this.grid.cellContent(cell);

            if (other && other.value === tile.value) {
              return true;
            }
          }
        }
      }
    }
    return false;
  }

  positionsEqual(first, second) {
    return first.row === second.row && first.col === second.col;
  }

  getState() {
    return {
      grid: this.grid.clone(),
      score: this.score,
      over: this.over,
      won: this.won,
      keepPlaying: this.keepPlaying
    };
  }
}

// Export for use in tests
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { Tile, Grid, GameManager };
}
