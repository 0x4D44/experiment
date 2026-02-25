import { Ball } from './src/entities/Ball';
import { Paddle } from './src/entities/Paddle';
import { Block } from './src/entities/Block';
import { LevelManager } from './src/systems/LevelManager';
import { ScoringSystem } from './src/utils/scoring';
import { magnitude, rectColliding } from './src/utils/collision';

describe('Breakout Remix Game Tests', () => {
  describe('Ball Tests', () => {
    it('should initialize correctly', () => {
      const ball = new Ball(100, 200, 8);
      expect(ball.x).toBe(100);
      expect(ball.y).toBe(200);
    });
    it('should detect out of bounds', () => {
      const ball = new Ball(100, 700);
      expect(ball.isOutOfBounds(600)).toBe(true);
    });
  });

  describe('Paddle Tests', () => {
    it('should move and stop', () => {
      const paddle = new Paddle(300, 550);
      paddle.moveLeft();
      expect(paddle.velocityX).toBeLessThan(0);
      paddle.stop();
      expect(paddle.velocityX).toBe(0);
    });
  });

  describe('Block Tests', () => {
    it('should take damage', () => {
      const block = new Block({
        x: 0, y: 0, width: 90, height: 20,
        health: 2, points: 100, color: 0xff0000,
      });
      block.takeDamage(1);
      expect(block.health).toBe(1);
    });
  });

  describe('LevelManager Tests', () => {
    it('should generate level', () => {
      const manager = new LevelManager();
      const level = manager.generateLevel(1);
      expect(level.number).toBe(1);
      expect(level.blocks.length).toBeGreaterThan(0);
    });
  });

  describe('ScoringSystem Tests', () => {
    it('should calculate points', () => {
      const scoring = new ScoringSystem();
      const points = scoring.calculateBlockPoints(0, 1, 0);
      expect(points).toBeGreaterThan(0);
    });
  });

  describe('Collision Tests', () => {
    it('should detect collision', () => {
      const r1 = { x: 0, y: 0, width: 100, height: 100 };
      const r2 = { x: 50, y: 50, width: 100, height: 100 };
      expect(rectColliding(r1, r2)).toBe(true);
    });
    it('should calculate magnitude', () => {
      expect(magnitude({ x: 3, y: 4 })).toBe(5);
    });
  });
});
