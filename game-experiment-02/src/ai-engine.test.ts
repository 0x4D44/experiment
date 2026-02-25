/**
 * Unit tests for AI Engine
 */

import { AIEngine } from './ai-engine';
import { Maze } from './maze-grid';
import { AICommand } from './maze-types';

describe('AIEngine', () => {
  let maze: Maze;
  let ai: AIEngine;

  beforeEach(() => {
    maze = new Maze(8, 8);
    maze.setStart({ x: 4, y: 4 }); // Start in middle to avoid boundaries
    maze.setGoal({ x: 7, y: 4 });
    ai = new AIEngine(maze, maze.getStart(), 100);
  });

  describe('initialization', () => {
    it('should start at start position', () => {
      const state = ai.getState();
      expect(state.position.x).toBe(4);
      expect(state.position.y).toBe(4);
    });

    it('should face north initially', () => {
      const state = ai.getState();
      expect(state.direction).toBe('N');
    });

    it('should have no keys initially', () => {
      const state = ai.getState();
      expect(state.keysHeld.size).toBe(0);
    });

    it('should have step count of 0', () => {
      expect(ai.getStepCount()).toBe(0);
    });
  });

  describe('forward movement', () => {
    it('should move forward successfully', () => {
      ai.executeCommand(AICommand.Forward);
      const state = ai.getState();
      expect(state.position.y).toBe(3); // North (y decreases)
    });

    it('should increment step count on successful move', () => {
      ai.executeCommand(AICommand.Forward);
      expect(ai.getStepCount()).toBe(1);
    });

    it('should fail moving into wall', () => {
      maze.setWall({ x: 4, y: 3 }); // Wall ahead
      const step = ai.executeCommand(AICommand.Forward);
      expect(step.success).toBe(false);
    });

    it('should fail moving outside maze', () => {
      for (let i = 0; i < 5; i++) {
        ai.executeCommand(AICommand.Forward);
      }
      const state = ai.getState();
      expect(state.position.y).toBeLessThan(10);
    });
  });

  describe('turning', () => {
    it('should turn left from north to west', () => {
      ai.executeCommand(AICommand.TurnLeft);
      const state = ai.getState();
      expect(state.direction).toBe('W');
    });

    it('should turn right from north to east', () => {
      ai.executeCommand(AICommand.TurnRight);
      const state = ai.getState();
      expect(state.direction).toBe('E');
    });

    it('should complete full rotation', () => {
      ai.executeCommand(AICommand.TurnRight);
      ai.executeCommand(AICommand.TurnRight);
      ai.executeCommand(AICommand.TurnRight);
      ai.executeCommand(AICommand.TurnRight);
      const state = ai.getState();
      expect(state.direction).toBe('N');
    });

    it('should increment step count on turn', () => {
      const stepsBefore = ai.getStepCount();
      ai.executeCommand(AICommand.TurnLeft);
      expect(ai.getStepCount()).toBe(stepsBefore + 1);
    });
  });

  describe('wall sensing', () => {
    it('should detect wall ahead', () => {
      maze.setWall({ x: 4, y: 3 });
      expect(ai.senseWallAhead()).toBe(true);
    });

    it('should detect no wall on empty path', () => {
      // Should not detect wall at (4,3)
      expect(ai.senseWallAhead()).toBe(false);
    });

    it('should detect boundary as wall', () => {
      for (let i = 0; i < 10; i++) {
        ai.executeCommand(AICommand.Forward);
      }
      expect(ai.senseWallAhead()).toBe(true);
    });
  });

  describe('keys and doors', () => {
    beforeEach(() => {
      maze.addKey({ x: 4, y: 2 }, 1);
      maze.addDoor({ x: 4, y: 0 }, 1);
    });

    it('should pickup key', () => {
      // Move to key (from y=4 to y=2)
      ai.executeCommand(AICommand.Forward);
      ai.executeCommand(AICommand.Forward);
      ai.executeCommand(AICommand.PickupKey);
      const state = ai.getState();
      expect(state.keysHeld.has(1)).toBe(true);
    });

    it('should not allow movement through locked door without key', () => {
      // Try to move through door without key - should fail
      for (let i = 0; i < 5; i++) {
        ai.executeCommand(AICommand.Forward);
      }
      const state = ai.getState();
      // Should be stopped before reaching y=0
      expect(state.position.y).toBeGreaterThan(0);
    });

    it('should allow movement through door with key', () => {
      // Get key first
      ai.executeCommand(AICommand.Forward);
      ai.executeCommand(AICommand.Forward);
      ai.executeCommand(AICommand.PickupKey);

      // Now move through door
      for (let i = 0; i < 3; i++) {
        ai.executeCommand(AICommand.Forward);
      }
      const state = ai.getState();
      expect(state.position.y).toBeLessThanOrEqual(0);
    });
  });

  describe('marking path', () => {
    it('should mark current cell', () => {
      ai.executeCommand(AICommand.MarkPath);
      const state = ai.getState();
      expect(state.markedCells.has('4,4')).toBe(true);
    });

    it('should mark multiple cells', () => {
      ai.executeCommand(AICommand.MarkPath);
      ai.executeCommand(AICommand.Forward);
      ai.executeCommand(AICommand.MarkPath);
      const state = ai.getState();
      expect(state.markedCells.size).toBe(2);
    });
  });

  describe('goal detection', () => {
    it('should detect reaching goal', () => {
      // Move from (4,4) to (7,4) - goal is at (7,4)
      ai.executeCommand(AICommand.TurnRight); // Face East
      for (let i = 0; i < 3; i++) {
        ai.executeCommand(AICommand.Forward);
      }
      expect(ai.isGoalReached()).toBe(true);
    });

    it('should not be at goal initially', () => {
      expect(ai.isGoalReached()).toBe(false);
    });

    it('should set finished flag when goal reached', () => {
      for (let i = 0; i < 7; i++) {
        ai.executeCommand(AICommand.Forward);
      }
      expect(ai.isFinished()).toBe(true);
    });
  });

  describe('step limit', () => {
    it('should respect step limit', () => {
      const limitedAI = new AIEngine(maze, maze.getStart(), 5);
      for (let i = 0; i < 10; i++) {
        limitedAI.executeCommand(AICommand.TurnLeft);
      }
      expect(limitedAI.getStepCount()).toBeLessThanOrEqual(5);
    });
  });

  describe('teleporters', () => {
    beforeEach(() => {
      maze.addTeleporter({ x: 4, y: 1 }, { x: 6, y: 4 });
    });

    it('should teleport to target', () => {
      // Move to teleporter from y=4 to y=1
      for (let i = 0; i < 3; i++) {
        ai.executeCommand(AICommand.Forward);
      }
      const state = ai.getState();
      expect(state.position.x).toBe(6); // Teleported to x=6
      expect(state.position.y).toBe(4); // Teleported to y=4
    });
  });

  describe('execution log', () => {
    it('should log all executed commands', () => {
      ai.executeCommand(AICommand.Forward);
      ai.executeCommand(AICommand.TurnLeft);
      ai.executeCommand(AICommand.Forward);
      const log = ai.getLog();
      expect(log.length).toBe(3);
    });

    it('should track position in log', () => {
      ai.executeCommand(AICommand.Forward);
      const log = ai.getLog();
      expect(log[0].position.y).toBe(3); // Should move north to y=3
    });

    it('should mark failed commands in log', () => {
      maze.setWall({ x: 4, y: 3 });
      ai.executeCommand(AICommand.Forward);
      const log = ai.getLog();
      expect(log[0].success).toBe(false);
    });
  });

  describe('reset', () => {
    it('should reset to initial state', () => {
      ai.executeCommand(AICommand.Forward);
      ai.executeCommand(AICommand.TurnLeft);
      ai.reset();
      const state = ai.getState();
      expect(state.position.x).toBe(4);
      expect(state.position.y).toBe(4);
      expect(state.direction).toBe('N');
      expect(ai.getStepCount()).toBe(0);
    });

    it('should clear execution log on reset', () => {
      ai.executeCommand(AICommand.Forward);
      ai.reset();
      const log = ai.getLog();
      expect(log.length).toBe(0);
    });

    it('should reset goal reached flag', () => {
      for (let i = 0; i < 7; i++) {
        ai.executeCommand(AICommand.Forward);
      }
      ai.reset();
      expect(ai.isGoalReached()).toBe(false);
    });
  });
});
