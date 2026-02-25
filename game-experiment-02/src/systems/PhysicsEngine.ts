import { Vector2D } from '../utils/Vector2D';
import { GameState, GravityWell, Obstacle, Wormhole, CollisionResult, TrajectoryPoint, PhysicsParams } from '../types/Physics';

/**
 * Core physics engine for Gravity Golf
 */
export class PhysicsEngine {
  private params: PhysicsParams;

  constructor(params: Partial<PhysicsParams> = {}) {
    this.params = {
      timeStep: 0.016, // ~60fps
      damping: 0.98,
      groundFriction: 0.05,
      gravityConstant: 500, // Adjusted for game feel
      maxVelocity: 500,
      stopThreshold: 0.1,
      ...params,
    };
  }

  /**
   * Update physics for one time step
   */
  update(state: GameState): void {
    if (state.inHole) return;

    // Apply gravity from all wells
    this.applyGravity(state);

    // Apply damping and friction
    this.applyDamping(state);

    // Update position based on velocity
    this.updatePosition(state);

    // Check collisions
    this.handleCollisions(state);

    // Check if ball is in hole
    this.checkHoleCollision(state);

    // Update ball motion state
    this.updateMotionState(state);
  }

  /**
   * Calculate and apply gravity from all wells
   */
  private applyGravity(state: GameState): void {
    const ball = state.ball;
    const acceleration = new Vector2D(0, 0);

    for (const well of state.gravityWells) {
      const direction = Vector2D.subtract(well.position, ball.position);
      const distSquared = direction.magnitudeSquared();

      // Only apply if within effect radius and not too close
      if (distSquared < well.radius * well.radius && distSquared > 1) {
        const distance = Math.sqrt(distSquared);
        const force = (this.params.gravityConstant * well.strength) / distSquared;
        const forceVector = Vector2D.normalize(direction);
        forceVector.scale(force);
        acceleration.add(forceVector);
      }
    }

    // Apply acceleration to velocity
    ball.velocity.add(acceleration.scale(this.params.timeStep));

    // Clamp maximum velocity
    ball.velocity.clampMagnitude(this.params.maxVelocity);
  }

  /**
   * Apply damping and friction
   */
  private applyDamping(state: GameState): void {
    // Air damping
    state.ball.velocity.scale(this.params.damping);

    // Ground friction (slow down if moving slowly)
    const speed = state.ball.velocity.magnitude();
    if (speed < 2) {
      state.ball.velocity.scale(1 - this.params.groundFriction);
    }
  }

  /**
   * Update ball position
   */
  private updatePosition(state: GameState): void {
    const displacement = state.ball.velocity.clone().scale(this.params.timeStep);
    state.ball.position.add(displacement);
  }

  /**
   * Handle collisions with obstacles and boundaries
   */
  private handleCollisions(state: GameState): void {
    // Boundary collisions
    this.checkBoundaryCollision(state);

    // Obstacle collisions
    for (const obstacle of state.obstacles) {
      if (this.checkCircleCollision(state.ball.position, state.ball.radius, obstacle.position, obstacle.radius)) {
        if (obstacle.type === 'blackhole') {
          // Black hole destroys the ball - reset it
          state.ball.velocity.x = 0;
          state.ball.velocity.y = 0;
          state.strokes++;
          // Ball gets pulled to black hole center then disappears
          state.ball.position = state.ball.position; // Would reset in full game
        } else if (obstacle.type === 'wormhole') {
          // Will be handled by wormhole teleportation check
          const wormhole = state.wormholes.find(
            (w) => w.entrance.distance(obstacle.position) < 1
          );
          if (wormhole) {
            state.ball.position = wormhole.exit.clone();
          }
        } else if (obstacle.type === 'asteroid') {
          // Bounce off asteroid
          this.bounceOffObstacle(state, obstacle);
        } else if (obstacle.type === 'wall') {
          // Bounce off wall
          this.bounceOffObstacle(state, obstacle);
        }
      }
    }

    // Wormhole check
    for (const wormhole of state.wormholes) {
      if (this.checkCircleCollision(state.ball.position, state.ball.radius, wormhole.entrance, wormhole.radius)) {
        state.ball.position = wormhole.exit.clone();
        // Maintain some velocity through wormhole
        state.ball.velocity.scale(0.8);
      }
    }
  }

  /**
   * Check and handle boundary collisions
   */
  private checkBoundaryCollision(state: GameState): void {
    const { x, y } = state.ball.position;
    const r = state.ball.radius;
    const bounds = { minX: 0, maxX: 800, minY: 0, maxY: 600 }; // Default bounds

    if (x - r < bounds.minX) {
      state.ball.position.x = bounds.minX + r;
      state.ball.velocity.x = Math.abs(state.ball.velocity.x) * 0.7;
    }
    if (x + r > bounds.maxX) {
      state.ball.position.x = bounds.maxX - r;
      state.ball.velocity.x = -Math.abs(state.ball.velocity.x) * 0.7;
    }
    if (y - r < bounds.minY) {
      state.ball.position.y = bounds.minY + r;
      state.ball.velocity.y = Math.abs(state.ball.velocity.y) * 0.7;
    }
    if (y + r > bounds.maxY) {
      state.ball.position.y = bounds.maxY - r;
      state.ball.velocity.y = -Math.abs(state.ball.velocity.y) * 0.7;
    }
  }

  /**
   * Check circle-circle collision
   */
  private checkCircleCollision(pos1: Vector2D, radius1: number, pos2: Vector2D, radius2: number): boolean {
    const distance = pos1.distance(pos2);
    return distance < radius1 + radius2;
  }

  /**
   * Bounce ball off obstacle
   */
  private bounceOffObstacle(state: GameState, obstacle: Obstacle): void {
    const normal = Vector2D.subtract(state.ball.position, obstacle.position);
    normal.normalize();

    // Push ball out of collision
    const penetration = state.ball.radius + obstacle.radius - state.ball.position.distance(obstacle.position);
    normal.scale(penetration + 0.1);
    state.ball.position.add(normal);

    // Reflect velocity
    const velocity = state.ball.velocity;
    const dotProduct = Vector2D.dot(velocity, normal);
    const reflection = Vector2D.scale(normal, 2 * dotProduct);
    velocity.subtract(reflection);
    velocity.scale(0.8); // Energy loss on bounce
  }

  /**
   * Check if ball is in hole
   */
  private checkHoleCollision(state: GameState): void {
    const distance = state.ball.position.distance(state.hole);
    if (distance < state.holeRadius) {
      state.inHole = true;
      state.ballInMotion = false;
    }
  }

  /**
   * Update ball motion state
   */
  private updateMotionState(state: GameState): void {
    const speed = state.ball.velocity.magnitude();
    state.ballStopped = speed < this.params.stopThreshold;
    state.ballInMotion = speed > this.params.stopThreshold;
  }

  /**
   * Predict trajectory for a number of steps
   */
  predictTrajectory(state: GameState, steps: number = 100): TrajectoryPoint[] {
    const trajectory: TrajectoryPoint[] = [];
    const testState = this.cloneGameState(state);

    for (let i = 0; i < steps; i++) {
      trajectory.push({
        position: testState.ball.position.clone(),
        velocity: testState.ball.velocity.clone(),
      });

      // Simulate one step
      this.update(testState);

      if (testState.inHole || testState.ballStopped) {
        break;
      }
    }

    return trajectory;
  }

  /**
   * Clone game state for simulation
   */
  private cloneGameState(state: GameState): GameState {
    return {
      ...state,
      ball: {
        ...state.ball,
        position: state.ball.position.clone(),
        velocity: state.ball.velocity.clone(),
      },
      gravityWells: state.gravityWells.map((w) => ({
        ...w,
        position: w.position.clone(),
      })),
      hole: state.hole.clone(),
      obstacles: state.obstacles,
      wormholes: state.wormholes.map((w) => ({
        ...w,
        entrance: w.entrance.clone(),
        exit: w.exit.clone(),
      })),
    };
  }

  /**
   * Apply impulse to ball (for hitting it)
   */
  applyImpulse(state: GameState, direction: Vector2D, power: number): void {
    state.ball.velocity = direction.clone().normalize().scale(power);
    state.strokes++;
    state.ballInMotion = true;
  }

  /**
   * Place gravity well modifier
   */
  placeGravityWell(state: GameState, position: Vector2D, strength: number, radius: number): boolean {
    if (state.gravityModifiersUsed >= state.maxGravityModifiers) {
      return false;
    }

    state.gravityWells.push({
      position: position.clone(),
      strength,
      radius,
    });

    state.gravityModifiersUsed++;
    return true;
  }
}
