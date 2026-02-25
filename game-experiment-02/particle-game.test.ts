/**
 * Particle Playground - Game Engine Tests
 * Tests for game logic, levels, and state management
 */

import { ParticleGame, GameMode } from './particle-game';
import { ParticleType, Vector2D } from './particle-physics';

describe('ParticleGame', () => {
  test('should create game with correct dimensions', () => {
    const game = new ParticleGame(800, 600);
    expect(game.physics.width).toBe(800);
    expect(game.physics.height).toBe(600);
  });

  test('should have multiple levels loaded', () => {
    const game = new ParticleGame(800, 600);
    expect(game.levels.length).toBeGreaterThanOrEqual(16);
  });

  test('should start in puzzle mode', () => {
    const game = new ParticleGame(800, 600);
    expect(game.mode).toBe(GameMode.PUZZLE);
  });

  test('should load first level on start', () => {
    const game = new ParticleGame(800, 600);
    game.start();
    expect(game.isRunning).toBe(true);
    expect(game.currentLevel).toBe(0);
  });

  test('should load specific level', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(5);
    expect(game.currentLevel).toBe(5);
    expect(game.goals.length).toBeGreaterThan(0);
  });

  test('should navigate to next level', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(0);
    const initialLevel = game.currentLevel;
    game.nextLevel();
    expect(game.currentLevel).toBe(initialLevel + 1);
  });

  test('should navigate to previous level', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(5);
    const initialLevel = game.currentLevel;
    game.previousLevel();
    expect(game.currentLevel).toBe(initialLevel - 1);
  });

  test('should not go below level 0', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(0);
    game.previousLevel();
    expect(game.currentLevel).toBe(0);
  });

  test('should not go above max level', () => {
    const game = new ParticleGame(800, 600);
    const maxLevel = game.levels.length - 1;
    game.loadLevel(maxLevel);
    game.nextLevel();
    expect(game.currentLevel).toBe(maxLevel);
  });

  test('should reset level', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(2);
    const particleCount = game.physics.particles.length;

    // Modify state
    game.physics.particles.pop();
    expect(game.physics.particles.length).toBeLessThan(particleCount);

    // Reset
    game.resetLevel();
    expect(game.physics.particles.length).toBe(particleCount);
  });

  test('should toggle pause', () => {
    const game = new ParticleGame(800, 600);
    game.start();

    expect(game.physics.isPaused).toBe(false);
    game.togglePause();
    expect(game.physics.isPaused).toBe(true);
    game.togglePause();
    expect(game.physics.isPaused).toBe(false);
  });

  test('should track time during game', () => {
    const game = new ParticleGame(800, 600);
    game.start();

    const initialTime = game.time;
    game.update();
    game.update();
    game.update();

    expect(game.time).toBeGreaterThan(initialTime);
  });

  test('should spawn particle in sandbox mode', () => {
    const game = new ParticleGame(800, 600);
    game.mode = GameMode.SANDBOX;

    const initialCount = game.physics.particles.length;
    game.spawnParticle(new Vector2D(100, 100), ParticleType.NEUTRAL);

    expect(game.physics.particles.length).toBe(initialCount + 1);
  });

  test('should add attractor in sandbox mode', () => {
    const game = new ParticleGame(800, 600);
    game.mode = GameMode.SANDBOX;

    const initialCount = game.physics.attractors.length;
    game.addAttractorInSandbox(new Vector2D(400, 300), true);

    expect(game.physics.attractors.length).toBe(initialCount + 1);
  });

  test('should add repulsor in sandbox mode', () => {
    const game = new ParticleGame(800, 600);
    game.mode = GameMode.SANDBOX;

    const initialCount = game.physics.attractors.length;
    game.addAttractorInSandbox(new Vector2D(400, 300), false);

    expect(game.physics.attractors.length).toBe(initialCount + 1);
  });

  test('level 1 should have gravity and one goal', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(0);

    expect(game.physics.gravity.y).toBeGreaterThan(0);
    expect(game.goals.length).toBeGreaterThan(0);
    expect(game.physics.particles.length).toBeGreaterThan(0);
  });

  test('level 2 should have attractor', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(1);

    expect(game.physics.attractors.length).toBeGreaterThan(0);
    expect(game.goals.length).toBeGreaterThan(0);
  });

  test('level 3 should have positive particle and repulsor', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(2);

    // Should have at least one positive particle
    const hasPositive = game.physics.particles.some((p) => p.type === ParticleType.POSITIVE);
    expect(hasPositive).toBe(true);

    // Should have at least one repulsor
    const hasRepulsor = game.physics.attractors.some((a) => !a.isAttractor);
    expect(hasRepulsor).toBe(true);
  });

  test('level 5 should have barriers', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(4);

    expect(game.physics.barriers.length).toBeGreaterThan(0);
  });

  test('level 6 should have portals', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(5);

    expect(game.physics.portals.length).toBeGreaterThan(0);
  });

  test('should detect goal completion when particles in zone', () => {
    const game = new ParticleGame(800, 600);
    game.loadLevel(0);

    // Manually place particle in goal
    if (game.goals.length > 0 && game.physics.particles.length > 0) {
      game.physics.particles[0].position = game.goals[0].position.clone();
      game.goals[0].update(game.physics.particles);

      expect(game.goals[0].isComplete()).toBe(true);
    }
  });

  test('should update game state', () => {
    const game = new ParticleGame(800, 600);
    game.start();

    const initialParticleCount = game.physics.particles.length;

    // Run several updates
    for (let i = 0; i < 10; i++) {
      game.update();
    }

    // Particles may be removed if they go out of bounds, so just check it doesn't error
    expect(game.physics.particles.length).toBeLessThanOrEqual(initialParticleCount);
  });

  test('all levels should have valid structure', () => {
    const game = new ParticleGame(800, 600);

    for (let i = 0; i < game.levels.length; i++) {
      const level = game.levels[i];
      expect(level.id).toBe(i + 1);
      expect(level.name).toBeTruthy();
      expect(level.description).toBeTruthy();
      expect(Array.isArray(level.particles)).toBe(true);
      expect(Array.isArray(level.goals)).toBe(true);
      expect(typeof level.gravity).toBe('object');
    }
  });
});
