/**
 * Particle Playground - Game Engine
 * Handles game state, levels, and game flow
 */

import {
  Particle,
  ParticleType,
  Vector2D,
  PhysicsEngine,
  Attractor,
  GoalZone,
  Barrier,
  Portal,
} from './particle-physics';

export enum GameMode {
  PUZZLE = 'puzzle',
  SANDBOX = 'sandbox',
}

export interface LevelDefinition {
  id: number;
  name: string;
  description: string;
  particles: Array<{
    position: Vector2D;
    velocity: Vector2D;
    type: ParticleType;
  }>;
  attractors: Array<{
    position: Vector2D;
    strength: number;
    isAttractor: boolean;
    radius: number;
  }>;
  barriers: Array<{
    start: Vector2D;
    end: Vector2D;
    thickness: number;
  }>;
  portals: Array<{
    position: Vector2D;
    destination: Vector2D;
    radius: number;
    color: string;
  }>;
  goals: Array<{
    position: Vector2D;
    radius: number;
    requiredParticles: number;
  }>;
  gravity: { x: number; y: number };
  timeLimit: number; // seconds, 0 = no limit
}

/**
 * Particle game engine
 */
export class ParticleGame {
  physics: PhysicsEngine;
  goals: GoalZone[];
  mode: GameMode;
  currentLevel: number;
  levels: LevelDefinition[];
  isRunning: boolean;
  time: number;
  timeLimit: number;
  isLevelComplete: boolean;
  isFailed: boolean;

  constructor(width: number, height: number) {
    this.physics = new PhysicsEngine(width, height);
    this.goals = [];
    this.mode = GameMode.PUZZLE;
    this.currentLevel = 0;
    this.levels = this.generateLevels();
    this.isRunning = false;
    this.time = 0;
    this.timeLimit = 0;
    this.isLevelComplete = false;
    this.isFailed = false;
  }

  /**
   * Start the game
   */
  start(): void {
    this.isRunning = true;
    this.loadLevel(this.currentLevel);
  }

  /**
   * Load a specific level
   */
  loadLevel(levelIndex: number): void {
    if (levelIndex >= this.levels.length) {
      levelIndex = this.levels.length - 1;
    }
    if (levelIndex < 0) {
      levelIndex = 0;
    }

    this.currentLevel = levelIndex;
    const level = this.levels[levelIndex];

    // Clear physics engine
    this.physics.clearAll();
    this.goals = [];

    // Set gravity
    this.physics.setGravity(level.gravity.x, level.gravity.y);

    // Add particles
    for (const particleData of level.particles) {
      const particle = new Particle(particleData.position, particleData.velocity, particleData.type);
      this.physics.addParticle(particle);
    }

    // Add attractors
    for (const attractorData of level.attractors) {
      const attractor = new Attractor(
        attractorData.position,
        attractorData.strength,
        attractorData.isAttractor,
        attractorData.radius,
      );
      this.physics.addAttractor(attractor);
    }

    // Add barriers
    for (const barrierData of level.barriers) {
      const barrier = new Barrier(barrierData.start, barrierData.end, barrierData.thickness);
      this.physics.addBarrier(barrier);
    }

    // Add portals
    for (const portalData of level.portals) {
      const portal = new Portal(portalData.position, portalData.destination, portalData.radius, portalData.color);
      this.physics.addPortal(portal);
    }

    // Add goals
    for (const goalData of level.goals) {
      const goal = new GoalZone(goalData.position, goalData.radius, goalData.requiredParticles);
      this.goals.push(goal);
    }

    this.time = 0;
    this.timeLimit = level.timeLimit;
    this.isLevelComplete = false;
    this.isFailed = false;
  }

  /**
   * Update game state
   */
  update(): void {
    if (!this.isRunning || this.isLevelComplete || this.isFailed) return;

    // Update physics
    this.physics.update();

    // Update goals
    for (const goal of this.goals) {
      goal.update(this.physics.particles);
    }

    // Check win condition
    if (this.goals.length > 0 && this.goals.every((g) => g.isComplete())) {
      this.isLevelComplete = true;
    }

    // Check time limit
    this.time += 1 / 60; // Assume 60 FPS
    if (this.timeLimit > 0 && this.time > this.timeLimit) {
      this.isFailed = true;
    }
  }

  /**
   * Next level
   */
  nextLevel(): void {
    if (this.currentLevel < this.levels.length - 1) {
      this.loadLevel(this.currentLevel + 1);
    }
  }

  /**
   * Previous level
   */
  previousLevel(): void {
    if (this.currentLevel > 0) {
      this.loadLevel(this.currentLevel - 1);
    }
  }

  /**
   * Reset current level
   */
  resetLevel(): void {
    this.loadLevel(this.currentLevel);
  }

  /**
   * Toggle pause
   */
  togglePause(): void {
    this.physics.togglePause();
  }

  /**
   * Spawn particle in sandbox mode
   */
  spawnParticle(position: Vector2D, type: ParticleType): void {
    const velocity = Vector2D.fromAngle(Math.random() * Math.PI * 2, Math.random() * 5);
    const particle = new Particle(position, velocity, type);
    this.physics.addParticle(particle);
  }

  /**
   * Add attractor in sandbox mode
   */
  addAttractorInSandbox(position: Vector2D, isAttractor: boolean = true): void {
    const attractor = new Attractor(position, 2, isAttractor, 150);
    this.physics.addAttractor(attractor);
  }

  /**
   * Generate all puzzle levels
   */
  private generateLevels(): LevelDefinition[] {
    const levels: LevelDefinition[] = [];

    // Level 1: Simple gravity
    levels.push({
      id: 1,
      name: 'Gravity Basics',
      description: 'Guide particles into the goal using gravity',
      particles: [
        {
          position: new Vector2D(400, 100),
          velocity: new Vector2D(0, 0),
          type: ParticleType.NEUTRAL,
        },
      ],
      attractors: [],
      barriers: [],
      portals: [],
      goals: [
        {
          position: new Vector2D(400, 500),
          radius: 40,
          requiredParticles: 1,
        },
      ],
      gravity: { x: 0, y: 0.1 },
      timeLimit: 0,
    });

    // Level 2: Simple attractor
    levels.push({
      id: 2,
      name: 'Attractor Force',
      description: 'Use an attractor to pull particles to the goal',
      particles: [
        {
          position: new Vector2D(200, 300),
          velocity: new Vector2D(0, 0),
          type: ParticleType.NEUTRAL,
        },
      ],
      attractors: [
        {
          position: new Vector2D(600, 300),
          strength: 3,
          isAttractor: true,
          radius: 200,
        },
      ],
      barriers: [],
      portals: [],
      goals: [
        {
          position: new Vector2D(600, 300),
          radius: 40,
          requiredParticles: 1,
        },
      ],
      gravity: { x: 0, y: 0 },
      timeLimit: 0,
    });

    // Level 3: Charge interaction
    levels.push({
      id: 3,
      name: 'Charge Repulsion',
      description: 'Guide positive particles away from positive attractor to goal',
      particles: [
        {
          position: new Vector2D(400, 100),
          velocity: new Vector2D(0, 0),
          type: ParticleType.POSITIVE,
        },
      ],
      attractors: [
        {
          position: new Vector2D(400, 300),
          strength: 2,
          isAttractor: false, // Repulsor
          radius: 250,
        },
      ],
      barriers: [],
      portals: [],
      goals: [
        {
          position: new Vector2D(400, 500),
          radius: 40,
          requiredParticles: 1,
        },
      ],
      gravity: { x: 0, y: 0.05 },
      timeLimit: 0,
    });

    // Level 4: Mixed charges
    levels.push({
      id: 4,
      name: 'Opposite Attraction',
      description: 'Positive particles attract to negative, negative to positive',
      particles: [
        {
          position: new Vector2D(100, 300),
          velocity: new Vector2D(0, 0),
          type: ParticleType.POSITIVE,
        },
        {
          position: new Vector2D(700, 300),
          velocity: new Vector2D(0, 0),
          type: ParticleType.NEGATIVE,
        },
      ],
      attractors: [],
      barriers: [],
      portals: [],
      goals: [
        {
          position: new Vector2D(400, 300),
          radius: 50,
          requiredParticles: 2,
        },
      ],
      gravity: { x: 0, y: 0 },
      timeLimit: 0,
    });

    // Level 5: Barrier navigation
    levels.push({
      id: 5,
      name: 'Navigate Barriers',
      description: 'Guide particle through gap in barriers to goal',
      particles: [
        {
          position: new Vector2D(100, 100),
          velocity: new Vector2D(5, 3),
          type: ParticleType.NEUTRAL,
        },
      ],
      attractors: [
        {
          position: new Vector2D(700, 500),
          strength: 2,
          isAttractor: true,
          radius: 300,
        },
      ],
      barriers: [
        {
          start: new Vector2D(300, 150),
          end: new Vector2D(300, 350),
          thickness: 15,
        },
        {
          start: new Vector2D(500, 350),
          end: new Vector2D(500, 550),
          thickness: 15,
        },
      ],
      portals: [],
      goals: [
        {
          position: new Vector2D(700, 500),
          radius: 40,
          requiredParticles: 1,
        },
      ],
      gravity: { x: 0, y: 0.02 },
      timeLimit: 0,
    });

    // Level 6: Portal puzzle
    levels.push({
      id: 6,
      name: 'Portal Jump',
      description: 'Use portal to reach the goal',
      particles: [
        {
          position: new Vector2D(100, 300),
          velocity: new Vector2D(5, 0),
          type: ParticleType.NEUTRAL,
        },
      ],
      attractors: [
        {
          position: new Vector2D(250, 300),
          strength: 2,
          isAttractor: true,
          radius: 150,
        },
      ],
      barriers: [],
      portals: [
        {
          position: new Vector2D(250, 300),
          destination: new Vector2D(650, 300),
          radius: 20,
          color: '#00FF00',
        },
      ],
      goals: [
        {
          position: new Vector2D(700, 300),
          radius: 40,
          requiredParticles: 1,
        },
      ],
      gravity: { x: 0, y: 0 },
      timeLimit: 0,
    });

    // Level 7: Three particle coordination
    levels.push({
      id: 7,
      name: 'Triforce',
      description: 'Get all three particles to the goal',
      particles: [
        {
          position: new Vector2D(200, 200),
          velocity: new Vector2D(0, 0),
          type: ParticleType.NEUTRAL,
        },
        {
          position: new Vector2D(600, 200),
          velocity: new Vector2D(0, 0),
          type: ParticleType.NEUTRAL,
        },
        {
          position: new Vector2D(400, 150),
          velocity: new Vector2D(0, 0),
          type: ParticleType.NEUTRAL,
        },
      ],
      attractors: [
        {
          position: new Vector2D(400, 450),
          strength: 2,
          isAttractor: true,
          radius: 250,
        },
      ],
      barriers: [],
      portals: [],
      goals: [
        {
          position: new Vector2D(400, 450),
          radius: 50,
          requiredParticles: 3,
        },
      ],
      gravity: { x: 0, y: 0.08 },
      timeLimit: 0,
    });

    // Level 8: Charge separation
    levels.push({
      id: 8,
      name: 'Charge Separation',
      description: 'Separate positive and negative particles to different goals',
      particles: [
        {
          position: new Vector2D(400, 300),
          velocity: new Vector2D(0, 0),
          type: ParticleType.POSITIVE,
        },
        {
          position: new Vector2D(400, 300),
          velocity: new Vector2D(2, 0),
          type: ParticleType.NEGATIVE,
        },
      ],
      attractors: [
        {
          position: new Vector2D(200, 300),
          strength: 2,
          isAttractor: true,
          radius: 200,
        },
        {
          position: new Vector2D(600, 300),
          strength: 2,
          isAttractor: true,
          radius: 200,
        },
      ],
      barriers: [],
      portals: [],
      goals: [
        {
          position: new Vector2D(200, 300),
          radius: 40,
          requiredParticles: 1,
        },
        {
          position: new Vector2D(600, 300),
          radius: 40,
          requiredParticles: 1,
        },
      ],
      gravity: { x: 0, y: 0 },
      timeLimit: 0,
    });

    // Level 9: Complex maze
    levels.push({
      id: 9,
      name: 'Magnetic Maze',
      description: 'Navigate through barriers using magnetic forces',
      particles: [
        {
          position: new Vector2D(50, 300),
          velocity: new Vector2D(0, 0),
          type: ParticleType.POSITIVE,
        },
      ],
      attractors: [
        {
          position: new Vector2D(750, 300),
          strength: 3,
          isAttractor: true,
          radius: 300,
        },
      ],
      barriers: [
        {
          start: new Vector2D(150, 100),
          end: new Vector2D(150, 400),
          thickness: 12,
        },
        {
          start: new Vector2D(300, 300),
          end: new Vector2D(300, 600),
          thickness: 12,
        },
        {
          start: new Vector2D(450, 100),
          end: new Vector2D(450, 400),
          thickness: 12,
        },
        {
          start: new Vector2D(600, 300),
          end: new Vector2D(600, 600),
          thickness: 12,
        },
      ],
      portals: [],
      goals: [
        {
          position: new Vector2D(750, 300),
          radius: 40,
          requiredParticles: 1,
        },
      ],
      gravity: { x: 0, y: 0 },
      timeLimit: 0,
    });

    // Level 10: Gravity well with counter-force
    levels.push({
      id: 10,
      name: 'Gravity Well',
      description: 'Escape the gravity well with the repulsive force',
      particles: [
        {
          position: new Vector2D(400, 300),
          velocity: new Vector2D(0, 0),
          type: ParticleType.NEUTRAL,
        },
      ],
      attractors: [
        {
          position: new Vector2D(400, 300),
          strength: 5,
          isAttractor: true,
          radius: 300,
        },
        {
          position: new Vector2D(400, 100),
          strength: 3,
          isAttractor: false,
          radius: 200,
        },
      ],
      barriers: [],
      portals: [],
      goals: [
        {
          position: new Vector2D(400, 50),
          radius: 40,
          requiredParticles: 1,
        },
      ],
      gravity: { x: 0, y: 0 },
      timeLimit: 0,
    });

    // Add more levels for variety
    for (let i = 0; i < 6; i++) {
      levels.push(this.generateRandomLevel(i + 11));
    }

    return levels;
  }

  /**
   * Generate a random level
   */
  private generateRandomLevel(id: number): LevelDefinition {
    const randomType = (): ParticleType => {
      const types = [ParticleType.NEUTRAL, ParticleType.POSITIVE, ParticleType.NEGATIVE];
      return types[Math.floor(Math.random() * types.length)];
    };

    const particleCount = 1 + Math.floor(Math.random() * 3);
    const particles: LevelDefinition['particles'] = [];

    for (let i = 0; i < particleCount; i++) {
      particles.push({
        position: new Vector2D(Math.random() * 600 + 100, Math.random() * 300 + 100),
        velocity: new Vector2D((Math.random() - 0.5) * 4, (Math.random() - 0.5) * 4),
        type: randomType(),
      });
    }

    const attractorCount = Math.floor(Math.random() * 3);
    const attractors: LevelDefinition['attractors'] = [];

    for (let i = 0; i < attractorCount; i++) {
      attractors.push({
        position: new Vector2D(Math.random() * 600 + 100, Math.random() * 400 + 100),
        strength: 2 + Math.random() * 2,
        isAttractor: Math.random() > 0.3,
        radius: 150 + Math.random() * 100,
      });
    }

    return {
      id,
      name: `Challenge ${id}`,
      description: 'A procedurally generated challenge level',
      particles,
      attractors,
      barriers: [],
      portals: [],
      goals: [
        {
          position: new Vector2D(Math.random() * 600 + 100, Math.random() * 400 + 100),
          radius: 35,
          requiredParticles: particleCount,
        },
      ],
      gravity: { x: 0, y: 0.05 },
      timeLimit: 0,
    };
  }
}
