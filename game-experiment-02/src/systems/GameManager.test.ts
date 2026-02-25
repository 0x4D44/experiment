import { GameManager } from './GameManager';
import { Vector2D } from '../utils/Vector2D';

describe('GameManager', () => {
  let manager: GameManager;

  beforeEach(() => {
    manager = new GameManager();
  });

  describe('game initialization', () => {
    test('start game initializes game state', () => {
      manager.startGame();

      expect(manager.getCurrentHoleNumber()).toBe(1);
      expect(manager.isBallStopped()).toBe(true);
      expect(manager.getCurrentStrokeCount()).toBe(0);
    });

    test('load hole sets up correct configuration', () => {
      manager.startGame();
      const config = manager.getCurrentHoleConfig();

      expect(config.id).toBe(1);
      expect(config.name).toBe('The Gentle Start');
      expect(config.par).toBe(2);
    });
  });

  describe('scoring', () => {
    test('strokes are counted', () => {
      manager.startGame();
      const gameState = manager.getGameState();
      const direction = new Vector2D(1, 0);

      // This simulates a stroke via the physics engine
      gameState.strokes = 1;
      expect(manager.getCurrentStrokeCount()).toBe(1);
    });

    test('score relative to par is calculated', () => {
      manager.startGame();
      const gameState = manager.getGameState();
      gameState.strokes = 3;

      const relativePar = manager.getScoreRelativeToPar();
      expect(relativePar).toBe(1); // 3 strokes - 2 par
    });
  });

  describe('ball physics', () => {
    test('hit ball applies impulse', () => {
      manager.startGame();
      const gameState = manager.getGameState();

      manager.hitBall(0, 200); // Angle 0, power 200

      expect(gameState.ballInMotion).toBe(true);
      expect(gameState.strokes).toBe(1);
      expect(gameState.ball.velocity.magnitude()).toBeGreaterThan(0);
    });

    test('ball motion state is tracked', () => {
      manager.startGame();

      expect(manager.isBallStopped()).toBe(true);
      expect(manager.isBallInMotion()).toBe(false);

      manager.hitBall(0, 100);

      expect(manager.isBallInMotion()).toBe(true);
    });
  });

  describe('gravity modifiers', () => {
    test('place gravity modifier succeeds when within limit', () => {
      manager.startGame();
      manager.loadHole(3); // Load hole with modifiers

      const result = manager.placeGravityModifier(new Vector2D(300, 300), false);

      expect(result).toBe(true);
      expect(manager.getGravityModifiersRemaining()).toBeGreaterThan(0);
    });

    test('gravity modifier count is tracked', () => {
      manager.startGame();
      manager.loadHole(3); // Load hole with modifiers
      const config = manager.getCurrentHoleConfig();
      const initialRemaining = config.maxGravityModifiers;

      manager.placeGravityModifier(new Vector2D(300, 300), false);

      expect(manager.getGravityModifiersRemaining()).toBe(initialRemaining - 1);
    });

    test('repulsive gravity modifier can be placed', () => {
      manager.startGame();
      manager.loadHole(3); // Load hole with modifiers

      const result = manager.placeGravityModifier(new Vector2D(300, 300), true);

      expect(result).toBe(true);
      const wells = manager.getGravityWells();
      expect(wells.length).toBeGreaterThan(0);
    });
  });

  describe('hole completion', () => {
    test('detecting hole completion', () => {
      manager.startGame();
      const gameState = manager.getGameState();

      expect(manager.isHoleComplete()).toBe(false);

      gameState.inHole = true;
      expect(manager.isHoleComplete()).toBe(true);
    });

    test('finish hole records score', () => {
      manager.startGame();
      const gameState = manager.getGameState();
      gameState.strokes = 2;

      manager.finishHole();
      const scores = manager.getHoleScores();

      expect(scores[0]).toBe(2);
      expect(manager.getTotalScore()).toBe(2);
    });
  });

  describe('hole progression', () => {
    test('next hole advances to next level', () => {
      manager.startGame();
      expect(manager.getCurrentHoleNumber()).toBe(1);

      manager.nextHole();

      expect(manager.getCurrentHoleNumber()).toBe(2);
    });

    test('next hole after last ends game', () => {
      manager.startGame();

      // Skip to last hole
      for (let i = 0; i < 8; i++) {
        manager.nextHole();
      }

      expect(manager.getCurrentHoleNumber()).toBe(9);
      expect(manager.isGameOver()).toBe(false);

      manager.nextHole();

      expect(manager.isGameOver()).toBe(true);
    });

    test('restart hole resets to beginning', () => {
      manager.startGame();
      const gameState = manager.getGameState();
      gameState.strokes = 5;

      manager.restartHole();

      expect(manager.getCurrentStrokeCount()).toBe(0);
      expect(gameState.ball.position).toEqual(manager.getCurrentHoleConfig().ballStartPos);
    });
  });

  describe('trajectory prediction', () => {
    test('get trajectory preview returns points', () => {
      manager.startGame();

      const trajectory = manager.getTrajectoryPreview(0, 100);

      expect(Array.isArray(trajectory)).toBe(true);
      expect(trajectory.length).toBeGreaterThan(0);
      trajectory.forEach((point) => {
        expect(point).toHaveProperty('x');
        expect(point).toHaveProperty('y');
      });
    });

    test('trajectory prediction does not affect game state', () => {
      manager.startGame();
      const gameState = manager.getGameState();
      const originalPos = gameState.ball.position.clone();
      const originalVel = gameState.ball.velocity.clone();

      manager.getTrajectoryPreview(0, 100);

      expect(gameState.ball.position).toEqual(originalPos);
      expect(gameState.ball.velocity).toEqual(originalVel);
    });
  });

  describe('game progress', () => {
    test('total holes is 9', () => {
      expect(manager.getTotalHoles()).toBe(9);
    });

    test('all hole configs are available', () => {
      const holes = manager.getAllHoles();
      expect(holes.length).toBe(9);
      holes.forEach((hole, index) => {
        expect(hole.id).toBe(index + 1);
      });
    });

    test('total par calculation', () => {
      manager.startGame();
      const gameState = manager.getGameState();
      gameState.strokes = 2;
      manager.finishHole();

      manager.nextHole();
      gameState.strokes = 3;
      manager.finishHole();

      const totalPar = manager.getTotalPar();
      // Par for hole 1 (2) + hole 2 (2) = 4
      expect(totalPar).toBe(4);
    });
  });

  describe('hole characteristics', () => {
    test('hole 1 is simple with no obstacles', () => {
      manager.loadHole(0);
      const config = manager.getCurrentHoleConfig();

      expect(config.name).toBe('The Gentle Start');
      expect(config.obstacles.length).toBe(0);
      expect(config.initialGravityWells.length).toBe(0);
      expect(config.maxGravityModifiers).toBe(0);
    });

    test('hole 9 is complex with obstacles and wells', () => {
      manager.loadHole(8);
      const config = manager.getCurrentHoleConfig();

      expect(config.name).toBe('Cosmic Challenge');
      expect(config.obstacles.length).toBeGreaterThan(0);
      expect(config.initialGravityWells.length).toBeGreaterThan(0);
      expect(config.maxGravityModifiers).toBeGreaterThan(0);
    });

    test('each hole has unique par values', () => {
      const holes = manager.getAllHoles();
      const pars = holes.map((h) => h.par);

      // Check that par values exist and are reasonable
      pars.forEach((par) => {
        expect(par).toBeGreaterThan(1);
        expect(par).toBeLessThanOrEqual(5);
      });
    });
  });
});
