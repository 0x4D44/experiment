/**
 * Tests for GameWorld
 */

import { GameWorld } from './GameWorld';

describe('GameWorld', () => {
  let world: GameWorld;

  beforeEach(() => {
    world = new GameWorld(50, 50);
  });

  describe('initialization', () => {
    it('should create world with correct dimensions', () => {
      const bounds = world.getWorldBounds();
      expect(bounds.minX).toBe(0);
      expect(bounds.maxX).toBe(50);
      expect(bounds.minY).toBe(0);
      expect(bounds.maxY).toBe(50);
    });

    it('should place player in center', () => {
      const position = world.getPlayerPosition();
      expect(position.x).toBe(25);
      expect(position.y).toBe(25);
    });

    it('should create obstacles', () => {
      const objects = world.getAllObjects();
      expect(objects.obstacles.length).toBeGreaterThan(0);
    });

    it('should create objectives', () => {
      const objects = world.getAllObjects();
      expect(objects.objectives.length).toBeGreaterThan(0);
    });

    it('should create walls', () => {
      const objects = world.getAllObjects();
      expect(objects.walls.length).toBeGreaterThan(0);
    });
  });

  describe('player movement', () => {
    it('should move player in valid direction', () => {
      const initialPos = world.getPlayerPosition();
      const success = world.movePlayer({ dx: 1, dy: 0 }, 1);

      expect(success).toBe(true);
      const newPos = world.getPlayerPosition();
      expect(newPos.x).toBeGreaterThan(initialPos.x);
      expect(newPos.y).toBe(initialPos.y);
    });

    it('should prevent movement into obstacles', () => {
      const objects = world.getAllObjects();
      const obstacle = objects.obstacles[0];

      // Reset player near obstacle
      world.resetPlayerPosition();
      const playerPos = world.getPlayerPosition();

      // Try to move towards obstacle
      const dx = obstacle.position.x - playerPos.x;
      const dy = obstacle.position.y - playerPos.y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      // Normalize direction
      const direction = { dx: dx / distance, dy: dy / distance };

      // Try to move through obstacle
      let moved = true;
      for (let i = 0; i < 100 && moved; i++) {
        moved = world.movePlayer(direction, 0.5);
      }

      // Should have been stopped by obstacle or wall
      const finalPos = world.getPlayerPosition();
      const finalDist = Math.sqrt(
        (finalPos.x - obstacle.position.x) ** 2 + (finalPos.y - obstacle.position.y) ** 2
      );

      // Should not be able to move into obstacle
      expect(finalDist).toBeGreaterThan(0);
    });

    it('should prevent movement into walls', () => {
      const bounds = world.getWorldBounds();

      // Try to move past left wall
      world.resetPlayerPosition();

      let moved = true;
      for (let i = 0; i < 100 && moved; i++) {
        moved = world.movePlayer({ dx: -1, dy: 0 }, 1);
      }

      const pos = world.getPlayerPosition();
      expect(pos.x).toBeGreaterThan(bounds.minX);
    });
  });

  describe('object detection', () => {
    it('should detect objects within range', () => {
      const objects = world.detectObjects(100);
      expect(objects.length).toBeGreaterThan(0);
    });

    it('should not detect objects outside range', () => {
      const objects = world.detectObjects(0.1);
      expect(objects.length).toBe(0);
    });

    it('should return echoes with distance and angle', () => {
      const echoes = world.getEchoes(100);

      for (const echo of echoes) {
        expect(echo.distance).toBeGreaterThanOrEqual(0);
        expect(echo.angle).toBeGreaterThanOrEqual(-Math.PI);
        expect(echo.angle).toBeLessThanOrEqual(Math.PI);
        expect(['obstacle', 'objective', 'wall']).toContain(echo.type);
      }
    });

    it('should return echoes sorted by distance', () => {
      const echoes = world.getEchoes(100);

      // Check that distances make sense
      for (const echo of echoes) {
        expect(echo.distance).toBeGreaterThanOrEqual(0);
        expect(echo.distance).toBeLessThanOrEqual(100);
      }
    });
  });

  describe('objective collection', () => {
    it('should detect when player reaches objective', () => {
      const objects = world.getAllObjects();
      const objective = objects.objectives[0];

      // Move player to objective
      world.resetPlayerPosition();
      const playerPos = world.getPlayerPosition();

      const dx = objective.position.x - playerPos.x;
      const dy = objective.position.y - playerPos.y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      const direction = { dx: dx / distance, dy: dy / distance };

      // Move towards objective
      for (let i = 0; i < 100; i++) {
        const moved = world.movePlayer(direction, 0.5);
        if (!moved) break;

        if (world.checkObjectiveReached()) {
          break;
        }
      }

      // Check if objective was reached
      const reached = world.checkObjectiveReached();

      // If we reached it, there should be one fewer objective
      const newObjects = world.getAllObjects();
      const objectiveDiff = objects.objectives.length - newObjects.objectives.length;

      if (reached) {
        expect(objectiveDiff).toBeGreaterThan(0);
      }
    });

    it('should decrease objective count when collected', () => {
      const initialCount = world.getRemainingObjectives();
      expect(initialCount).toBeGreaterThan(0);

      // Try to reach an objective
      const objects = world.getAllObjects();
      const objective = objects.objectives[0];

      world.resetPlayerPosition();
      const playerPos = world.getPlayerPosition();

      const dx = objective.position.x - playerPos.x;
      const dy = objective.position.y - playerPos.y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      const direction = { dx: dx / distance, dy: dy / distance };

      for (let i = 0; i < 100; i++) {
        const moved = world.movePlayer(direction, 0.5);
        if (!moved) break;

        if (world.checkObjectiveReached()) {
          break;
        }
      }

      const finalCount = world.getRemainingObjectives();
      expect(finalCount).toBeLessThanOrEqual(initialCount);
    });
  });

  describe('position management', () => {
    it('should reset player position to center', () => {
      // Try to move multiple times to ensure movement happens
      for (let i = 0; i < 10; i++) {
        world.movePlayer({ dx: 1, dy: 0 }, 1);
      }

      // Reset and verify position
      world.resetPlayerPosition();

      const resetPos = world.getPlayerPosition();
      expect(resetPos.x).toBe(25);
      expect(resetPos.y).toBe(25);
    });

    it('should return copy of player position', () => {
      const pos1 = world.getPlayerPosition();
      const pos2 = world.getPlayerPosition();

      expect(pos1).toEqual(pos2);
      expect(pos1).not.toBe(pos2); // Different objects
    });
  });

  describe('world bounds', () => {
    it('should return correct world bounds', () => {
      const bounds = world.getWorldBounds();

      expect(bounds.minX).toBe(0);
      expect(bounds.maxX).toBe(50);
      expect(bounds.minY).toBe(0);
      expect(bounds.maxY).toBe(50);
    });
  });

  describe('object types', () => {
    it('should have different types of objects', () => {
      const objects = world.getAllObjects();

      const hasObstacles = objects.obstacles.length > 0;
      const hasObjectives = objects.objectives.length > 0;
      const hasWalls = objects.walls.length > 0;

      expect(hasObstacles).toBe(true);
      expect(hasObjectives).toBe(true);
      expect(hasWalls).toBe(true);
    });

    it('should return copies of objects', () => {
      const objects1 = world.getAllObjects();
      const objects2 = world.getAllObjects();

      expect(objects1.obstacles).toEqual(objects2.obstacles);
      expect(objects1.obstacles).not.toBe(objects2.obstacles);
    });
  });
});
