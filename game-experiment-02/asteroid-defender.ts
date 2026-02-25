/**
 * Asteroid Defender - A Space Shooter Game
 * Complete implementation in TypeScript
 */

// ============================================================================
// VECTOR2D MATH
// ============================================================================

export interface Vector2D {
  x: number;
  y: number;
}

export class Vector {
  static zero(): Vector2D {
    return { x: 0, y: 0 };
  }

  static create(x: number, y: number): Vector2D {
    return { x, y };
  }

  static add(a: Vector2D, b: Vector2D): Vector2D {
    return { x: a.x + b.x, y: a.y + b.y };
  }

  static subtract(a: Vector2D, b: Vector2D): Vector2D {
    return { x: a.x - b.x, y: a.y - b.y };
  }

  static scale(v: Vector2D, scalar: number): Vector2D {
    return { x: v.x * scalar, y: v.y * scalar };
  }

  static magnitude(v: Vector2D): number {
    return Math.sqrt(v.x * v.x + v.y * v.y);
  }

  static normalize(v: Vector2D): Vector2D {
    const mag = Vector.magnitude(v);
    if (mag === 0) return { x: 0, y: 0 };
    return { x: v.x / mag, y: v.y / mag };
  }

  static distance(a: Vector2D, b: Vector2D): number {
    return Vector.magnitude(Vector.subtract(b, a));
  }

  static dot(a: Vector2D, b: Vector2D): number {
    return a.x * b.x + a.y * b.y;
  }

  static fromAngle(angleRad: number, length: number = 1): Vector2D {
    return { x: Math.cos(angleRad) * length, y: Math.sin(angleRad) * length };
  }
}

// ============================================================================
// BASE ENTITY
// ============================================================================

export interface EntityConfig {
  id: string;
  position: Vector2D;
  velocity: Vector2D;
  rotation: number;
  isActive: boolean;
}

export class Entity implements EntityConfig {
  id: string;
  position: Vector2D;
  velocity: Vector2D;
  rotation: number;
  isActive: boolean;

  constructor(config: EntityConfig) {
    this.id = config.id;
    this.position = { ...config.position };
    this.velocity = { ...config.velocity };
    this.rotation = config.rotation;
    this.isActive = config.isActive;
  }

  update(deltaTime: number): void {
    this.position.x += this.velocity.x * deltaTime;
    this.position.y += this.velocity.y * deltaTime;
  }

  deactivate(): void {
    this.isActive = false;
  }
}

// ============================================================================
// COLLISION
// ============================================================================

export interface Circle {
  position: Vector2D;
  radius: number;
}

export class Collision {
  static circleToCircle(a: Circle, b: Circle): boolean {
    const distance = Vector.distance(a.position, b.position);
    return distance < a.radius + b.radius;
  }

  static getContactPoint(a: Circle, b: Circle): Vector2D | null {
    const distance = Vector.distance(a.position, b.position);
    if (distance >= a.radius + b.radius) {
      return null;
    }

    const direction = Vector.subtract(b.position, a.position);
    const normalized = Vector.normalize(direction);
    return {
      x: a.position.x + normalized.x * a.radius,
      y: a.position.y + normalized.y * a.radius,
    };
  }

  static getOverlap(a: Circle, b: Circle): number {
    const distance = Vector.distance(a.position, b.position);
    const minDistance = a.radius + b.radius;
    return Math.max(0, minDistance - distance);
  }

  static pointInCircle(point: Vector2D, circle: Circle): boolean {
    const distance = Vector.distance(point, circle.position);
    return distance <= circle.radius;
  }

  static wrapPosition(
    position: Vector2D,
    worldWidth: number,
    worldHeight: number,
    padding: number = 0
  ): Vector2D {
    let x = position.x;
    let y = position.y;

    if (x < -padding) {
      x = worldWidth + padding;
    } else if (x > worldWidth + padding) {
      x = 0; // Use 0 instead of -padding for right boundary
    }

    if (y < -padding) {
      y = worldHeight + padding;
    } else if (y > worldHeight + padding) {
      y = 0; // Use 0 instead of -padding for bottom boundary
    }

    return { x, y };
  }

  static clampPosition(
    position: Vector2D,
    radius: number,
    worldWidth: number,
    worldHeight: number
  ): Vector2D {
    return {
      x: Math.max(radius, Math.min(worldWidth - radius, position.x)),
      y: Math.max(radius, Math.min(worldHeight - radius, position.y)),
    };
  }
}

// ============================================================================
// PLAYER
// ============================================================================

export interface PlayerConfig extends EntityConfig {
  maxHealth: number;
  speed: number;
  radius: number;
  shootCooldown: number;
}

export class Player extends Entity {
  maxHealth: number;
  health: number;
  speed: number;
  radius: number;
  shootCooldown: number;
  currentShootCooldown: number;
  direction: Vector2D;

  constructor(config: PlayerConfig) {
    super(config);
    this.maxHealth = config.maxHealth;
    this.health = config.maxHealth;
    this.speed = config.speed;
    this.radius = config.radius;
    this.shootCooldown = config.shootCooldown;
    this.currentShootCooldown = 0;
    this.direction = Vector.create(0, -1);
  }

  update(deltaTime: number): void {
    super.update(deltaTime);
    if (this.currentShootCooldown > 0) {
      this.currentShootCooldown -= deltaTime;
    }
  }

  moveUp(): void {
    this.velocity.y = -this.speed;
  }

  moveDown(): void {
    this.velocity.y = this.speed;
  }

  moveLeft(): void {
    this.velocity.x = -this.speed;
  }

  moveRight(): void {
    this.velocity.x = this.speed;
  }

  stopVertical(): void {
    this.velocity.y = 0;
  }

  stopHorizontal(): void {
    this.velocity.x = 0;
  }

  setDirection(angle: number): void {
    this.direction = Vector.fromAngle(angle);
    this.rotation = angle;
  }

  canShoot(): boolean {
    return this.currentShootCooldown <= 0;
  }

  shoot(): void {
    if (this.canShoot()) {
      this.currentShootCooldown = this.shootCooldown;
    }
  }

  takeDamage(amount: number): void {
    this.health = Math.max(0, this.health - amount);
    if (this.health === 0) {
      this.deactivate();
    }
  }

  heal(amount: number): void {
    this.health = Math.min(this.maxHealth, this.health + amount);
  }

  isAlive(): boolean {
    return this.health > 0;
  }
}

// ============================================================================
// PROJECTILE
// ============================================================================

export interface ProjectileConfig extends EntityConfig {
  damage: number;
  lifetime: number;
  radius: number;
  owner: string;
}

export class Projectile extends Entity {
  damage: number;
  lifetime: number;
  timeAlive: number;
  radius: number;
  owner: string;

  constructor(config: ProjectileConfig) {
    super(config);
    this.damage = config.damage;
    this.lifetime = config.lifetime;
    this.timeAlive = 0;
    this.radius = config.radius;
    this.owner = config.owner;
  }

  update(deltaTime: number): void {
    super.update(deltaTime);
    this.timeAlive += deltaTime;

    if (this.timeAlive >= this.lifetime) {
      this.deactivate();
    }
  }

  hasExpired(): boolean {
    return this.timeAlive >= this.lifetime;
  }
}

// ============================================================================
// ASTEROID
// ============================================================================

export enum AsteroidSize {
  Large = 'large',
  Medium = 'medium',
  Small = 'small',
}

export interface AsteroidConfig extends EntityConfig {
  size: AsteroidSize;
  health: number;
  radius: number;
}

export class Asteroid extends Entity {
  size: AsteroidSize;
  health: number;
  maxHealth: number;
  radius: number;
  spinRate: number;

  constructor(config: AsteroidConfig) {
    super(config);
    this.size = config.size;
    this.health = config.health;
    this.maxHealth = config.health;
    this.radius = config.radius;
    this.spinRate = (Math.random() - 0.5) * 4;
  }

  update(deltaTime: number): void {
    super.update(deltaTime);
    this.rotation += this.spinRate * deltaTime;
  }

  takeDamage(amount: number): void {
    this.health = Math.max(0, this.health - amount);
    if (this.health === 0) {
      this.deactivate();
    }
  }

  isDestroyed(): boolean {
    return this.health <= 0;
  }

  canFragment(): boolean {
    return this.size === AsteroidSize.Large || this.size === AsteroidSize.Medium;
  }

  getFragmentSize(): AsteroidSize {
    switch (this.size) {
      case AsteroidSize.Large:
        return AsteroidSize.Medium;
      case AsteroidSize.Medium:
        return AsteroidSize.Small;
      default:
        return AsteroidSize.Small;
    }
  }
}

// ============================================================================
// ENTITY MANAGER
// ============================================================================

export class EntityManager {
  private entities: Map<string, Entity> = new Map();
  private activeIds: Set<string> = new Set();

  add(entity: Entity): void {
    this.entities.set(entity.id, entity);
    if (entity.isActive) {
      this.activeIds.add(entity.id);
    }
  }

  remove(id: string): void {
    this.entities.delete(id);
    this.activeIds.delete(id);
  }

  get(id: string): Entity | undefined {
    return this.entities.get(id);
  }

  getActive(): Entity[] {
    return Array.from(this.activeIds)
      .map((id) => this.entities.get(id)!)
      .filter((e) => e.isActive);
  }

  getAll(): Entity[] {
    return Array.from(this.entities.values());
  }

  getByType<T extends Entity>(type: new (...args: unknown[]) => T): T[] {
    return this.getActive().filter((e) => e instanceof type) as T[];
  }

  update(deltaTime: number): void {
    for (const entity of this.getActive()) {
      entity.update(deltaTime);
    }

    const entriesToRemove: string[] = [];
    this.entities.forEach((entity, id) => {
      if (!entity.isActive) {
        entriesToRemove.push(id);
      }
    });

    for (const id of entriesToRemove) {
      this.activeIds.delete(id);
    }
  }

  clear(): void {
    this.entities.clear();
    this.activeIds.clear();
  }

  getActiveCount(): number {
    return this.getActive().length;
  }

  getTotalCount(): number {
    return this.entities.size;
  }
}
