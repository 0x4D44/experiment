import { PhysicsEngine } from './PhysicsEngine';
import { GameState, GravityWell } from '../types/Physics';
import { Vector2D } from '../utils/Vector2D';

describe('PhysicsEngine', () => {
  let engine: PhysicsEngine;
  let gameState: GameState;

  beforeEach(() => {
    engine = new PhysicsEngine({
      timeStep: 0.016,
      damping: 0.98,
      groundFriction: 0.05,
      gravityConstant: 500,
      maxVelocity: 500,
      stopThreshold: 0.1,
    });

    gameState = {
      ball: {
        position: new Vector2D(400, 300),
        velocity: new Vector2D(0, 0),
        mass: 1,
        radius: 5,
      },
      gravityWells: [],
      obstacles: [],
      wormholes: [],
      hole: new Vector2D(400, 500),
      holeRadius: 20,
      groundFriction: 0.05,
      damping: 0.98,
      ballStopped: true,
      strokes: 0,
      gravityModifiersUsed: 0,
      maxGravityModifiers: 3,
      inHole: false,
      ballInMotion: false,
    };
  });

  describe('gravity application', () => {
    test('gravity well pulls ball towards it', () => {
      const well: GravityWell = {
        position: new Vector2D(400, 500),
        strength: 300,
        radius: 300,
      };
      gameState.gravityWells = [well];

      const initialY = gameState.ball.position.y;

      engine.update(gameState);

      expect(gameState.ball.velocity.y).toBeGreaterThan(0); // Should move towards well
    });

    test('repulsive gravity well pushes ball away', () => {
      const well: GravityWell = {
        position: new Vector2D(400, 500),
        strength: -300, // Negative = repulsive
        radius: 300,
      };
      gameState.gravityWells = [well];

      engine.update(gameState);

      expect(gameState.ball.velocity.y).toBeLessThan(0); // Should move away from well
    });

    test('gravity effects stronger at close range', () => {
      const well: GravityWell = {
        position: new Vector2D(400, 500),
        strength: 300,
        radius: 500,
      };
      gameState.gravityWells = [well];

      // Test at far distance first
      gameState.ball.position = new Vector2D(400, 301); // 199 units away
      gameState.ball.velocity = new Vector2D(0, 0);
      engine.update(gameState);
      const accAtFar = gameState.ball.velocity.magnitude();

      // Test at close distance
      gameState.ball.position = new Vector2D(400, 450); // 50 units away
      gameState.ball.velocity = new Vector2D(0, 0);
      engine.update(gameState);
      const accAtClose = gameState.ball.velocity.magnitude();

      // Closer distance should result in higher acceleration (gravity is stronger)
      expect(accAtClose).toBeGreaterThan(accAtFar);
    });

    test('no gravity outside well radius', () => {
      const well: GravityWell = {
        position: new Vector2D(400, 500),
        strength: 300,
        radius: 50,
      };
      gameState.gravityWells = [well];
      gameState.ball.position = new Vector2D(100, 300); // Far outside

      const initialVel = gameState.ball.velocity.clone();
      engine.update(gameState);

      expect(gameState.ball.velocity.equals(initialVel, 0.01)).toBe(true);
    });
  });

  describe('ball movement', () => {
    test('ball moves in direction of velocity', () => {
      gameState.ball.velocity = new Vector2D(10, 0);
      const initialX = gameState.ball.position.x;

      engine.update(gameState);

      expect(gameState.ball.position.x).toBeGreaterThan(initialX);
    });

    test('damping reduces velocity', () => {
      gameState.ball.velocity = new Vector2D(100, 100);
      const initialVel = gameState.ball.velocity.magnitude();

      engine.update(gameState);

      expect(gameState.ball.velocity.magnitude()).toBeLessThan(initialVel);
    });

    test('velocity is clamped to max', () => {
      gameState.ball.velocity = new Vector2D(1000, 0);

      engine.update(gameState);

      expect(gameState.ball.velocity.magnitude()).toBeLessThanOrEqual(500);
    });
  });

  describe('motion state tracking', () => {
    test('stopped ball is marked as stopped', () => {
      gameState.ball.velocity = new Vector2D(0, 0);
      gameState.ballStopped = false;

      engine.update(gameState);

      expect(gameState.ballStopped).toBe(true);
    });

    test('moving ball is marked as in motion', () => {
      gameState.ball.velocity = new Vector2D(10, 10);

      engine.update(gameState);

      expect(gameState.ballInMotion).toBe(true);
    });
  });

  describe('hole collision', () => {
    test('ball in hole is detected', () => {
      gameState.ball.position = gameState.hole.clone();
      gameState.inHole = false;

      engine.update(gameState);

      expect(gameState.inHole).toBe(true);
    });

    test('ball near hole is detected', () => {
      gameState.hole = new Vector2D(400, 500);
      gameState.holeRadius = 20;
      gameState.ball.position = new Vector2D(410, 500); // Within radius
      gameState.inHole = false;

      engine.update(gameState);

      expect(gameState.inHole).toBe(true);
    });

    test('ball outside hole is not detected', () => {
      gameState.hole = new Vector2D(400, 500);
      gameState.holeRadius = 20;
      gameState.ball.position = new Vector2D(450, 500); // Outside radius
      gameState.inHole = false;

      engine.update(gameState);

      expect(gameState.inHole).toBe(false);
    });
  });

  describe('boundary collisions', () => {
    test('ball bounces off left boundary', () => {
      gameState.ball.position = new Vector2D(2, 300);
      gameState.ball.velocity = new Vector2D(-10, 0);

      engine.update(gameState);

      expect(gameState.ball.velocity.x).toBeGreaterThan(0);
    });

    test('ball bounces off right boundary', () => {
      gameState.ball.position = new Vector2D(798, 300);
      gameState.ball.velocity = new Vector2D(10, 0);

      engine.update(gameState);

      expect(gameState.ball.velocity.x).toBeLessThan(0);
    });

    test('ball bounces off top boundary', () => {
      gameState.ball.position = new Vector2D(400, 2);
      gameState.ball.velocity = new Vector2D(0, -10);

      engine.update(gameState);

      expect(gameState.ball.velocity.y).toBeGreaterThan(0);
    });

    test('ball bounces off bottom boundary', () => {
      gameState.ball.position = new Vector2D(400, 598);
      gameState.ball.velocity = new Vector2D(0, 10);

      engine.update(gameState);

      expect(gameState.ball.velocity.y).toBeLessThan(0);
    });
  });

  describe('impulse application', () => {
    test('apply impulse sets ball velocity', () => {
      const direction = new Vector2D(1, 0);
      const power = 100;

      engine.applyImpulse(gameState, direction, power);

      expect(gameState.ball.velocity.magnitude()).toBeCloseTo(power, 5);
      expect(gameState.strokes).toBe(1);
    });

    test('multiple impulses add strokes', () => {
      engine.applyImpulse(gameState, new Vector2D(1, 0), 50);
      engine.applyImpulse(gameState, new Vector2D(0, 1), 50);

      expect(gameState.strokes).toBe(2);
    });
  });

  describe('gravity well placement', () => {
    test('place gravity well succeeds when under limit', () => {
      const result = engine.placeGravityWell(
        gameState,
        new Vector2D(200, 200),
        300,
        150
      );

      expect(result).toBe(true);
      expect(gameState.gravityWells.length).toBe(1);
      expect(gameState.gravityModifiersUsed).toBe(1);
    });

    test('place gravity well fails when at limit', () => {
      gameState.gravityModifiersUsed = 3;
      gameState.maxGravityModifiers = 3;

      const result = engine.placeGravityWell(
        gameState,
        new Vector2D(200, 200),
        300,
        150
      );

      expect(result).toBe(false);
      expect(gameState.gravityWells.length).toBe(0);
    });
  });

  describe('trajectory prediction', () => {
    test('predict trajectory returns array of points', () => {
      gameState.ball.velocity = new Vector2D(100, 0);

      const trajectory = engine.predictTrajectory(gameState, 50);

      expect(trajectory.length).toBeGreaterThan(0);
      expect(trajectory[0]).toHaveProperty('position');
      expect(trajectory[0]).toHaveProperty('velocity');
    });

    test('trajectory prediction stops at hole', () => {
      gameState.ball.position = new Vector2D(400, 300);
      gameState.hole = new Vector2D(500, 300);
      gameState.holeRadius = 20;
      gameState.ball.velocity = new Vector2D(100, 0);

      const trajectory = engine.predictTrajectory(gameState, 1000);

      // Last point should be near hole
      const lastPoint = trajectory[trajectory.length - 1];
      expect(lastPoint.position.distance(gameState.hole)).toBeLessThan(50);
    });

    test('trajectory does not modify game state', () => {
      const originalPos = gameState.ball.position.clone();
      gameState.ball.velocity = new Vector2D(100, 0);
      const originalVel = gameState.ball.velocity.clone();

      engine.predictTrajectory(gameState, 50);

      expect(gameState.ball.position).toEqual(originalPos);
      expect(gameState.ball.velocity).toEqual(originalVel);
    });
  });

  describe('multiple gravity wells', () => {
    test('multiple wells combine forces', () => {
      gameState.gravityWells = [
        {
          position: new Vector2D(200, 300),
          strength: 200,
          radius: 300,
        },
        {
          position: new Vector2D(600, 300),
          strength: 200,
          radius: 300,
        },
      ];

      gameState.ball.position = new Vector2D(400, 300);
      engine.update(gameState);

      // Ball should be pulled in both directions - net force should cancel out horizontally
      expect(Math.abs(gameState.ball.velocity.x)).toBeLessThan(1);
    });

    test('attractive and repulsive wells interact', () => {
      gameState.gravityWells = [
        {
          position: new Vector2D(300, 300),
          strength: 300,
          radius: 300,
        },
        {
          position: new Vector2D(500, 300),
          strength: -200, // Repulsive
          radius: 300,
        },
      ];

      gameState.ball.position = new Vector2D(400, 300);
      engine.update(gameState);

      // Should be pulled towards attractive (left) more than repelled (right)
      expect(gameState.ball.velocity.x).toBeLessThan(0);
    });
  });
});
