/**
 * Particle Playground - Physics Engine
 * A particle physics sandbox puzzle game with interactive particle simulation
 */

// Constants for physics simulation
const COULOMB_CONSTANT = 0.5;
const GRAVITY_CONSTANT = 0.1;
const FRICTION = 0.99;
const MIN_DISTANCE = 20;
const MAX_DISTANCE = 500;
const COLLISION_RADIUS = 8;
const PARTICLE_RADIUS = 6;

/**
 * Particle type enum
 */
export enum ParticleType {
  NEUTRAL = 'neutral',
  POSITIVE = 'positive',
  NEGATIVE = 'negative',
}

/**
 * 2D Vector class for physics calculations
 */
export class Vector2D {
  constructor(public x: number = 0, public y: number = 0) {}

  static zero(): Vector2D {
    return new Vector2D(0, 0);
  }

  static fromAngle(angle: number, magnitude: number = 1): Vector2D {
    return new Vector2D(Math.cos(angle) * magnitude, Math.sin(angle) * magnitude);
  }

  clone(): Vector2D {
    return new Vector2D(this.x, this.y);
  }

  add(other: Vector2D): Vector2D {
    this.x += other.x;
    this.y += other.y;
    return this;
  }

  subtract(other: Vector2D): Vector2D {
    this.x -= other.x;
    this.y -= other.y;
    return this;
  }

  multiply(scalar: number): Vector2D {
    this.x *= scalar;
    this.y *= scalar;
    return this;
  }

  divide(scalar: number): Vector2D {
    if (scalar !== 0) {
      this.x /= scalar;
      this.y /= scalar;
    }
    return this;
  }

  distance(other: Vector2D): number {
    const dx = other.x - this.x;
    const dy = other.y - this.y;
    return Math.sqrt(dx * dx + dy * dy);
  }

  distanceSquared(other: Vector2D): number {
    const dx = other.x - this.x;
    const dy = other.y - this.y;
    return dx * dx + dy * dy;
  }

  magnitude(): number {
    return Math.sqrt(this.x * this.x + this.y * this.y);
  }

  normalize(): Vector2D {
    const mag = this.magnitude();
    if (mag > 0) {
      this.divide(mag);
    }
    return this;
  }

  static add(a: Vector2D, b: Vector2D): Vector2D {
    return new Vector2D(a.x + b.x, a.y + b.y);
  }

  static subtract(a: Vector2D, b: Vector2D): Vector2D {
    return new Vector2D(a.x - b.x, a.y - b.y);
  }
}

/**
 * Particle class - represents a single particle in the simulation
 */
export class Particle {
  position: Vector2D;
  velocity: Vector2D;
  acceleration: Vector2D;
  mass: number;
  charge: number;
  type: ParticleType;
  radius: number;
  trail: Vector2D[];
  maxTrailLength: number;

  constructor(
    position: Vector2D,
    velocity: Vector2D = Vector2D.zero(),
    type: ParticleType = ParticleType.NEUTRAL,
    mass: number = 1,
  ) {
    this.position = position.clone();
    this.velocity = velocity.clone();
    this.acceleration = Vector2D.zero();
    this.mass = mass;
    this.radius = PARTICLE_RADIUS;
    this.type = type;
    this.maxTrailLength = 50;
    this.trail = [this.position.clone()];

    // Set charge based on type
    this.charge = type === ParticleType.NEUTRAL ? 0 : type === ParticleType.POSITIVE ? 1 : -1;
  }

  /**
   * Add force to particle (will affect acceleration)
   */
  applyForce(force: Vector2D): void {
    const acceleration = force.clone().divide(this.mass);
    this.acceleration.add(acceleration);
  }

  /**
   * Update particle position and velocity
   */
  update(): void {
    // Apply friction
    this.velocity.multiply(FRICTION);

    // Integrate velocity into position
    this.position.add(this.velocity);

    // Reset acceleration
    this.acceleration = Vector2D.zero();

    // Update trail
    this.trail.push(this.position.clone());
    if (this.trail.length > this.maxTrailLength) {
      this.trail.shift();
    }
  }

  /**
   * Check if particle is within bounds
   */
  isInBounds(width: number, height: number, margin: number = 50): boolean {
    return (
      this.position.x > -margin &&
      this.position.x < width + margin &&
      this.position.y > -margin &&
      this.position.y < height + margin
    );
  }

  /**
   * Handle collision with boundary
   */
  bounceOffWalls(width: number, height: number): void {
    const bounce = 0.8;

    if (this.position.x - this.radius < 0) {
      this.position.x = this.radius;
      this.velocity.x *= -bounce;
    } else if (this.position.x + this.radius > width) {
      this.position.x = width - this.radius;
      this.velocity.x *= -bounce;
    }

    if (this.position.y - this.radius < 0) {
      this.position.y = this.radius;
      this.velocity.y *= -bounce;
    } else if (this.position.y + this.radius > height) {
      this.position.y = height - this.radius;
      this.velocity.y *= -bounce;
    }
  }
}

/**
 * Attractor/Repulsor object
 */
export class Attractor {
  position: Vector2D;
  strength: number;
  radius: number;
  isAttractor: boolean;

  constructor(position: Vector2D, strength: number = 2, isAttractor: boolean = true, radius: number = 100) {
    this.position = position.clone();
    this.strength = strength;
    this.isAttractor = isAttractor;
    this.radius = radius;
  }

  /**
   * Calculate force on particle
   */
  calculateForce(particle: Particle): Vector2D {
    const direction = Vector2D.subtract(this.position, particle.position);
    const distance = direction.magnitude();

    if (distance < 1) return Vector2D.zero();
    if (distance > this.radius) return Vector2D.zero();

    direction.normalize();

    // Force decreases with distance (inverse square)
    const forceMagnitude = (this.strength * (1 - distance / this.radius)) / (distance + 1);

    const force = direction.multiply(forceMagnitude);
    if (!this.isAttractor) {
      force.multiply(-1);
    }

    return force;
  }
}

/**
 * Goal zone for puzzle mode
 */
export class GoalZone {
  position: Vector2D;
  radius: number;
  requiredParticles: number;
  particlesInZone: Particle[];

  constructor(position: Vector2D, radius: number = 30, requiredParticles: number = 1) {
    this.position = position.clone();
    this.radius = radius;
    this.requiredParticles = requiredParticles;
    this.particlesInZone = [];
  }

  /**
   * Check if particle is in goal zone
   */
  contains(particle: Particle): boolean {
    return this.position.distance(particle.position) < this.radius;
  }

  /**
   * Update particles in zone
   */
  update(particles: Particle[]): void {
    this.particlesInZone = particles.filter((p) => this.contains(p));
  }

  /**
   * Check if goal is completed
   */
  isComplete(): boolean {
    return this.particlesInZone.length >= this.requiredParticles;
  }
}

/**
 * Barrier for blocking particles
 */
export class Barrier {
  start: Vector2D;
  end: Vector2D;
  thickness: number;

  constructor(start: Vector2D, end: Vector2D, thickness: number = 10) {
    this.start = start.clone();
    this.end = end.clone();
    this.thickness = thickness;
  }

  /**
   * Check if particle collides with barrier
   */
  collidesWith(particle: Particle): boolean {
    return this.distanceToPoint(particle.position) < particle.radius + this.thickness / 2;
  }

  /**
   * Calculate distance from point to line segment
   */
  distanceToPoint(point: Vector2D): number {
    const line = Vector2D.subtract(this.end, this.start);
    const lineLen = line.magnitude();

    if (lineLen === 0) {
      return this.start.distance(point);
    }

    let t = ((point.x - this.start.x) * line.x + (point.y - this.start.y) * line.y) / (lineLen * lineLen);
    t = Math.max(0, Math.min(1, t));

    const closest = new Vector2D(this.start.x + t * line.x, this.start.y + t * line.y);
    return point.distance(closest);
  }
}

/**
 * Portal for teleporting particles
 */
export class Portal {
  position: Vector2D;
  radius: number;
  destinationPosition: Vector2D;
  color: string;

  constructor(position: Vector2D, destinationPosition: Vector2D, radius: number = 15, color: string = '#00FF00') {
    this.position = position.clone();
    this.destinationPosition = destinationPosition.clone();
    this.radius = radius;
    this.color = color;
  }

  /**
   * Check if particle enters portal
   */
  contains(particle: Particle): boolean {
    return this.position.distance(particle.position) < this.radius;
  }

  /**
   * Teleport particle
   */
  teleport(particle: Particle): void {
    particle.position = this.destinationPosition.clone();
  }
}

/**
 * Physics engine - manages all particles and forces
 */
export class PhysicsEngine {
  particles: Particle[];
  attractors: Attractor[];
  barriers: Barrier[];
  portals: Portal[];
  width: number;
  height: number;
  gravity: Vector2D;
  isPaused: boolean;

  constructor(width: number, height: number) {
    this.particles = [];
    this.attractors = [];
    this.barriers = [];
    this.portals = [];
    this.width = width;
    this.height = height;
    this.gravity = new Vector2D(0, 0.05);
    this.isPaused = false;
  }

  /**
   * Add particle to simulation
   */
  addParticle(particle: Particle): void {
    this.particles.push(particle);
  }

  /**
   * Add attractor to simulation
   */
  addAttractor(attractor: Attractor): void {
    this.attractors.push(attractor);
  }

  /**
   * Add barrier to simulation
   */
  addBarrier(barrier: Barrier): void {
    this.barriers.push(barrier);
  }

  /**
   * Add portal to simulation
   */
  addPortal(portal: Portal): void {
    this.portals.push(portal);
  }

  /**
   * Clear all particles
   */
  clearParticles(): void {
    this.particles = [];
  }

  /**
   * Clear all objects
   */
  clearAll(): void {
    this.particles = [];
    this.attractors = [];
    this.barriers = [];
    this.portals = [];
  }

  /**
   * Calculate force between two particles
   */
  private calculateInteraction(p1: Particle, p2: Particle): Vector2D {
    const direction = Vector2D.subtract(p2.position, p1.position);
    let distance = direction.magnitude();

    // Clamp distance to avoid extreme forces
    distance = Math.max(MIN_DISTANCE, Math.min(MAX_DISTANCE, distance));

    if (distance === 0) return Vector2D.zero();

    direction.normalize();

    let force = 0;

    // Coulomb force (charge interaction)
    if (p1.charge !== 0 && p2.charge !== 0) {
      const coulombForce = (COULOMB_CONSTANT * p1.charge * p2.charge) / (distance * distance);
      force += coulombForce;
    }

    // Gravity (opposite to charge - attract all mass)
    const gravityForce = (GRAVITY_CONSTANT * p1.mass * p2.mass) / (distance * distance);
    force -= gravityForce; // Negative for attraction

    direction.multiply(force);
    return direction;
  }

  /**
   * Check collision between two particles
   */
  private checkCollision(p1: Particle, p2: Particle): void {
    const distance = p1.position.distance(p2.position);
    const minDistance = p1.radius + p2.radius;

    if (distance < minDistance && distance > 0) {
      // Simple elastic collision response
      const direction = Vector2D.subtract(p1.position, p2.position).normalize();

      // Separate particles
      const overlap = minDistance - distance;
      p1.position.add(direction.clone().multiply(overlap / 2));
      direction.multiply(-1);
      p2.position.add(direction.clone().multiply(overlap / 2));

      // Exchange velocities (simplified)
      const tempVel = p1.velocity.clone();
      p1.velocity = p2.velocity.clone();
      p2.velocity = tempVel;
    }
  }

  /**
   * Update physics simulation
   */
  update(): void {
    if (this.isPaused) return;

    // Apply forces to all particles
    for (const particle of this.particles) {
      // Apply gravity
      particle.applyForce(this.gravity.clone().multiply(particle.mass));

      // Apply attractor forces
      for (const attractor of this.attractors) {
        const force = attractor.calculateForce(particle);
        particle.applyForce(force);
      }

      // Apply particle-particle interactions
      for (const other of this.particles) {
        if (particle !== other) {
          const force = this.calculateInteraction(particle, other);
          particle.applyForce(force);
        }
      }
    }

    // Update all particles
    for (const particle of this.particles) {
      particle.update();
    }

    // Handle collisions
    for (let i = 0; i < this.particles.length; i++) {
      for (let j = i + 1; j < this.particles.length; j++) {
        this.checkCollision(this.particles[i], this.particles[j]);
      }
    }

    // Handle barriers
    for (const particle of this.particles) {
      for (const barrier of this.barriers) {
        if (barrier.collidesWith(particle)) {
          // Simple bounce: reflect velocity
          const normal = Vector2D.subtract(particle.position, barrier.start).normalize();
          const dot = particle.velocity.x * normal.x + particle.velocity.y * normal.y;
          if (dot < 0) {
            particle.velocity.subtract(normal.clone().multiply(2 * dot));
          }
        }
      }
    }

    // Handle portals
    for (const particle of this.particles) {
      for (const portal of this.portals) {
        if (portal.contains(particle)) {
          portal.teleport(particle);
        }
      }
    }

    // Bounce off walls
    for (const particle of this.particles) {
      particle.bounceOffWalls(this.width, this.height);
    }

    // Remove particles that are too far away
    this.particles = this.particles.filter((p) => p.isInBounds(this.width, this.height));
  }

  /**
   * Set gravity
   */
  setGravity(gx: number, gy: number): void {
    this.gravity = new Vector2D(gx, gy);
  }

  /**
   * Toggle pause
   */
  togglePause(): void {
    this.isPaused = !this.isPaused;
  }
}
