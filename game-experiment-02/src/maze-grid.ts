/**
 * Maze grid management and generation
 */

import { MazeGrid, Cell, CellType, Position } from './maze-types';

export class Maze {
  private grid: Cell[][];
  private width: number;
  private height: number;
  private startPos: Position;
  private goalPos: Position;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    this.grid = this.initializeGrid();
    this.startPos = { x: 0, y: 0 };
    this.goalPos = { x: width - 1, y: height - 1 };
  }

  private initializeGrid(): Cell[][] {
    const grid: Cell[][] = [];
    for (let y = 0; y < this.height; y++) {
      const row: Cell[] = [];
      for (let x = 0; x < this.width; x++) {
        row.push({ type: CellType.Empty });
      }
      grid.push(row);
    }
    return grid;
  }

  /**
   * Set cell type at position
   */
  public setCell(pos: Position, type: CellType): void {
    if (this.isValidPosition(pos)) {
      const cell = this.grid[pos.y][pos.x];
      cell.type |= type; // Add type to bitmask
    }
  }

  /**
   * Clear cell type at position
   */
  public clearCell(pos: Position, type: CellType): void {
    if (this.isValidPosition(pos)) {
      const cell = this.grid[pos.y][pos.x];
      cell.type &= ~type; // Remove type from bitmask
    }
  }

  /**
   * Check if cell has type
   */
  public hasType(pos: Position, type: CellType): boolean {
    if (!this.isValidPosition(pos)) return false;
    return (this.grid[pos.y][pos.x].type & type) !== 0;
  }

  /**
   * Check if position is valid
   */
  public isValidPosition(pos: Position): boolean {
    return pos.x >= 0 && pos.x < this.width && pos.y >= 0 && pos.y < this.height;
  }

  /**
   * Check if cell is walkable
   */
  public isWalkable(pos: Position): boolean {
    if (!this.isValidPosition(pos)) return false;
    const cell = this.grid[pos.y][pos.x];
    // Walkable if not a wall
    return !this.hasType(pos, CellType.Wall);
  }

  /**
   * Set cell as wall
   */
  public setWall(pos: Position): void {
    this.setCell(pos, CellType.Wall);
  }

  /**
   * Check if cell is a wall
   */
  public isWall(pos: Position): boolean {
    return this.hasType(pos, CellType.Wall);
  }

  /**
   * Set goal position
   */
  public setGoal(pos: Position): void {
    this.goalPos = { ...pos };
    this.setCell(pos, CellType.Goal);
  }

  /**
   * Set start position
   */
  public setStart(pos: Position): void {
    this.startPos = { ...pos };
    this.setCell(pos, CellType.StartPosition);
  }

  /**
   * Add a key to the maze
   */
  public addKey(pos: Position, keyId: number): void {
    if (this.isValidPosition(pos)) {
      const cell = this.grid[pos.y][pos.x];
      cell.type |= CellType.Key;
      cell.keyId = keyId;
    }
  }

  /**
   * Add a door to the maze
   */
  public addDoor(pos: Position, keyId: number): void {
    if (this.isValidPosition(pos)) {
      const cell = this.grid[pos.y][pos.x];
      cell.type |= CellType.Door;
      cell.keyId = keyId;
    }
  }

  /**
   * Add a teleporter
   */
  public addTeleporter(pos: Position, target: Position): void {
    if (this.isValidPosition(pos) && this.isValidPosition(target)) {
      const cell = this.grid[pos.y][pos.x];
      cell.type |= CellType.Teleporter;
      cell.teleportTarget = { ...target };
    }
  }

  /**
   * Get cell at position
   */
  public getCell(pos: Position): Cell | null {
    if (!this.isValidPosition(pos)) return null;
    return this.grid[pos.y][pos.x];
  }

  /**
   * Get maze as exportable structure
   */
  public getGrid(): MazeGrid {
    return {
      width: this.width,
      height: this.height,
      cells: this.grid,
      startPos: { ...this.startPos },
      goalPos: { ...this.goalPos },
    };
  }

  /**
   * Get width
   */
  public getWidth(): number {
    return this.width;
  }

  /**
   * Get height
   */
  public getHeight(): number {
    return this.height;
  }

  /**
   * Get start position
   */
  public getStart(): Position {
    return { ...this.startPos };
  }

  /**
   * Get goal position
   */
  public getGoal(): Position {
    return { ...this.goalPos };
  }
}
