/**
 * Unit tests for Maze grid system
 */

import { Maze } from './maze-grid';
import { CellType } from './maze-types';

describe('Maze', () => {
  let maze: Maze;

  beforeEach(() => {
    maze = new Maze(8, 8);
  });

  describe('initialization', () => {
    it('should create maze with correct dimensions', () => {
      expect(maze.getWidth()).toBe(8);
      expect(maze.getHeight()).toBe(8);
    });

    it('should start with all empty cells', () => {
      expect(maze.isWall({ x: 0, y: 0 })).toBe(false);
    });
  });

  describe('position validation', () => {
    it('should validate positions within bounds', () => {
      expect(maze.isValidPosition({ x: 0, y: 0 })).toBe(true);
      expect(maze.isValidPosition({ x: 7, y: 7 })).toBe(true);
    });

    it('should reject positions outside bounds', () => {
      expect(maze.isValidPosition({ x: -1, y: 0 })).toBe(false);
      expect(maze.isValidPosition({ x: 8, y: 0 })).toBe(false);
      expect(maze.isValidPosition({ x: 0, y: 8 })).toBe(false);
      expect(maze.isValidPosition({ x: 0, y: -1 })).toBe(false);
    });
  });

  describe('wall management', () => {
    it('should set walls correctly', () => {
      maze.setWall({ x: 2, y: 3 });
      expect(maze.isWall({ x: 2, y: 3 })).toBe(true);
    });

    it('should not be walkable through walls', () => {
      maze.setWall({ x: 2, y: 3 });
      expect(maze.isWalkable({ x: 2, y: 3 })).toBe(false);
    });

    it('should be walkable on empty cells', () => {
      expect(maze.isWalkable({ x: 2, y: 3 })).toBe(true);
    });

    it('should clear walls', () => {
      maze.setWall({ x: 2, y: 3 });
      maze.clearCell({ x: 2, y: 3 }, CellType.Wall);
      expect(maze.isWall({ x: 2, y: 3 })).toBe(false);
    });
  });

  describe('start and goal positions', () => {
    it('should set start position', () => {
      maze.setStart({ x: 1, y: 1 });
      const start = maze.getStart();
      expect(start.x).toBe(1);
      expect(start.y).toBe(1);
    });

    it('should set goal position', () => {
      maze.setGoal({ x: 7, y: 7 });
      const goal = maze.getGoal();
      expect(goal.x).toBe(7);
      expect(goal.y).toBe(7);
    });

    it('should mark start position as non-wall', () => {
      maze.setStart({ x: 1, y: 1 });
      expect(maze.isWalkable({ x: 1, y: 1 })).toBe(true);
    });
  });

  describe('keys and doors', () => {
    it('should add keys', () => {
      maze.addKey({ x: 2, y: 2 }, 1);
      const cell = maze.getCell({ x: 2, y: 2 });
      expect(cell?.keyId).toBe(1);
      expect(maze.hasType({ x: 2, y: 2 }, CellType.Key)).toBe(true);
    });

    it('should add doors', () => {
      maze.addDoor({ x: 3, y: 3 }, 1);
      const cell = maze.getCell({ x: 3, y: 3 });
      expect(cell?.keyId).toBe(1);
      expect(maze.hasType({ x: 3, y: 3 }, CellType.Door)).toBe(true);
    });

    it('should track different key IDs', () => {
      maze.addKey({ x: 2, y: 2 }, 1);
      maze.addKey({ x: 3, y: 3 }, 2);
      const cell1 = maze.getCell({ x: 2, y: 2 });
      const cell2 = maze.getCell({ x: 3, y: 3 });
      expect(cell1?.keyId).toBe(1);
      expect(cell2?.keyId).toBe(2);
    });
  });

  describe('teleporters', () => {
    it('should add teleporters', () => {
      const target = { x: 5, y: 5 };
      maze.addTeleporter({ x: 2, y: 2 }, target);
      const cell = maze.getCell({ x: 2, y: 2 });
      expect(cell?.teleportTarget?.x).toBe(5);
      expect(cell?.teleportTarget?.y).toBe(5);
      expect(maze.hasType({ x: 2, y: 2 }, CellType.Teleporter)).toBe(true);
    });

    it('should store correct teleport destinations', () => {
      maze.addTeleporter({ x: 1, y: 1 }, { x: 7, y: 7 });
      maze.addTeleporter({ x: 7, y: 7 }, { x: 1, y: 1 });
      const cell1 = maze.getCell({ x: 1, y: 1 });
      const cell2 = maze.getCell({ x: 7, y: 7 });
      expect(cell1?.teleportTarget?.x).toBe(7);
      expect(cell2?.teleportTarget?.x).toBe(1);
    });
  });

  describe('cell type bitmask', () => {
    it('should combine multiple types on same cell', () => {
      maze.setCell({ x: 2, y: 2 }, CellType.Key);
      maze.setCell({ x: 2, y: 2 }, CellType.Goal);
      expect(maze.hasType({ x: 2, y: 2 }, CellType.Key)).toBe(true);
      expect(maze.hasType({ x: 2, y: 2 }, CellType.Goal)).toBe(true);
    });
  });

  describe('grid export', () => {
    it('should export grid correctly', () => {
      maze.setStart({ x: 0, y: 0 });
      maze.setGoal({ x: 7, y: 7 });
      maze.setWall({ x: 2, y: 2 });
      const grid = maze.getGrid();

      expect(grid.width).toBe(8);
      expect(grid.height).toBe(8);
      expect(grid.startPos.x).toBe(0);
      expect(grid.goalPos.x).toBe(7);
    });
  });
});
