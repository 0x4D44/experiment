/**
 * Platform Builder - Core Game Engine
 * Handles physics, collision detection, and game state management
 */

// ============================================================================
// TYPES & ENUMS
// ============================================================================

export enum TileType {
  EMPTY = 'empty',
  SOLID = 'solid',
  SPIKE = 'spike',
  SPRING = 'spring',
  MOVING_PLATFORM = 'moving_platform',
  CHECKPOINT = 'checkpoint',
}

export interface Vector2 {
  x: number;
  y: number;
}

export interface Tile {
  type: TileType;
  x: number;
  y: number;
  width: number;
  height: number;
  // For moving platforms
  pathX?: number[];
  pathY?: number[];
  pathIndex?: number;
  speed?: number;
}

export interface Entity {
  x: number;
  y: number;
  width: number;
  height: number;
  velocityX: number;
  velocityY: number;
  grounded: boolean;
}

export interface Enemy extends Entity {
  id: string;
  pathX: number[];
  pathY: number[];
  pathIndex: number;
  speed: number;
}

export interface Player extends Entity {
  jumpPower: number;
  maxSpeed: number;
  checkpointX: number;
  checkpointY: number;
}

export interface Level {
  id: string;
  name: string;
  width: number;
  height: number;
  tiles: Tile[];
  enemies: Enemy[];
  checkpoints: Vector2[];
  playerStart: Vector2;
  playerEnd: Vector2;
  difficulty: number; // 1-5
}

export interface GameState {
  level: Level;
  player: Player;
  enemies: Enemy[];
  time: number;
  completed: boolean;
  startTime: number;
}

// ============================================================================
// PHYSICS ENGINE
// ============================================================================

const GRAVITY = 0.6;
const FRICTION = 0.85;
const MAX_FALL_SPEED = 15;
const EPSILON = 0.001;

export class PhysicsEngine {
  update(entity: Entity, deltaTime: number = 1): void {
    // Apply gravity
    if (!entity.grounded) {
      entity.velocityY += GRAVITY;
      if (entity.velocityY > MAX_FALL_SPEED) {
        entity.velocityY = MAX_FALL_SPEED;
      }
    } else {
      entity.velocityY = 0;
    }

    // Apply friction
    entity.velocityX *= FRICTION;

    // Update position
    entity.x += entity.velocityX;
    entity.y += entity.velocityY;

    // Reset grounded state
    entity.grounded = false;
  }

  jump(entity: Player): void {
    if (entity.grounded) {
      entity.velocityY = -entity.jumpPower;
      entity.grounded = false;
    }
  }

  moveLeft(entity: Player): void {
    entity.velocityX = Math.max(entity.velocityX - 1, -entity.maxSpeed);
  }

  moveRight(entity: Player): void {
    entity.velocityX = Math.min(entity.velocityX + 1, entity.maxSpeed);
  }
}

// ============================================================================
// COLLISION DETECTION
// ============================================================================

export class CollisionDetector {
  isColliding(rect1: Entity | Tile, rect2: Entity | Tile): boolean {
    return (
      rect1.x < rect2.x + rect2.width &&
      rect1.x + rect1.width > rect2.x &&
      rect1.y < rect2.y + rect2.height &&
      rect1.y + rect1.height > rect2.y
    );
  }

  getCollisions(entity: Entity, tiles: Tile[]): Tile[] {
    return tiles.filter(tile => this.isColliding(entity, tile));
  }

  resolveCollisions(player: Player, tiles: Tile[]): void {
    const collisions = this.getCollisions(player, tiles);

    for (const tile of collisions) {
      if (tile.type === TileType.EMPTY) continue;

      // Determine collision side
      const overlapLeft = player.x + player.width - tile.x;
      const overlapRight = tile.x + tile.width - player.x;
      const overlapTop = player.y + player.height - tile.y;
      const overlapBottom = tile.y + tile.height - player.y;

      const minOverlapX = Math.min(overlapLeft, overlapRight);
      const minOverlapY = Math.min(overlapTop, overlapBottom);

      // Handle different tile types
      if (tile.type === TileType.SOLID) {
        if (minOverlapX < minOverlapY) {
          // Horizontal collision
          if (overlapLeft < overlapRight) {
            player.x = tile.x - player.width;
          } else {
            player.x = tile.x + tile.width;
          }
          player.velocityX = 0;
        } else {
          // Vertical collision
          if (overlapTop < overlapBottom) {
            player.y = tile.y - player.height;
            player.velocityY = 0;
            player.grounded = true;
          } else {
            player.y = tile.y + tile.height;
            player.velocityY = 0.5;
          }
        }
      } else if (tile.type === TileType.SPIKE) {
        // Spikes kill player - reset to checkpoint
        player.x = player.checkpointX;
        player.y = player.checkpointY;
        player.velocityX = 0;
        player.velocityY = 0;
      } else if (tile.type === TileType.SPRING) {
        // Springs bounce player up
        if (overlapTop < overlapBottom) {
          player.y = tile.y - player.height;
          player.velocityY = -15;
          player.grounded = true;
        }
      } else if (tile.type === TileType.CHECKPOINT) {
        // Update checkpoint position
        player.checkpointX = player.x;
        player.checkpointY = player.y;
      }
    }
  }

  checkLevelCompletion(player: Player, endZone: Vector2, size: number = 40): boolean {
    return (
      player.x < endZone.x + size &&
      player.x + player.width > endZone.x &&
      player.y < endZone.y + size &&
      player.y + player.height > endZone.y
    );
  }
}

// ============================================================================
// GAME MANAGER
// ============================================================================

export class GameManager {
  physics: PhysicsEngine;
  collision: CollisionDetector;
  gameState: GameState;
  level: Level;

  constructor(level: Level) {
    this.physics = new PhysicsEngine();
    this.collision = new CollisionDetector();
    this.level = level;

    const player: Player = {
      x: level.playerStart.x,
      y: level.playerStart.y,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      jumpPower: 12,
      maxSpeed: 8,
      checkpointX: level.playerStart.x,
      checkpointY: level.playerStart.y,
    };

    this.gameState = {
      level,
      player,
      enemies: level.enemies.map(e => ({ ...e })),
      time: 0,
      completed: false,
      startTime: Date.now(),
    };
  }

  update(input: { left: boolean; right: boolean; jump: boolean }): void {
    const { player, level, enemies } = this.gameState;

    // Handle input
    if (input.left) this.physics.moveLeft(player);
    if (input.right) this.physics.moveRight(player);
    if (input.jump) this.physics.jump(player);

    // Update player physics
    this.physics.update(player);

    // Resolve collisions with tiles
    this.collision.resolveCollisions(player, level.tiles);

    // Update moving platforms
    for (const tile of level.tiles) {
      if (tile.type === TileType.MOVING_PLATFORM && tile.pathX && tile.pathY) {
        tile.pathIndex = (tile.pathIndex || 0) + 1;
        if (tile.pathIndex >= tile.pathX.length) {
          tile.pathIndex = 0;
        }
        tile.x = tile.pathX[tile.pathIndex];
        tile.y = tile.pathY[tile.pathIndex];

        // Check if player is on moving platform
        if (this.collision.isColliding(player, tile)) {
          player.grounded = true;
        }
      }
    }

    // Update enemies
    for (const enemy of enemies) {
      if (enemy.pathX && enemy.pathY) {
        enemy.pathIndex = (enemy.pathIndex || 0) + 1;
        if (enemy.pathIndex >= enemy.pathX.length) {
          enemy.pathIndex = 0;
        }
        enemy.x = enemy.pathX[enemy.pathIndex];
        enemy.y = enemy.pathY[enemy.pathIndex];
      }

      // Check collision with player
      if (this.collision.isColliding(player, enemy)) {
        player.x = player.checkpointX;
        player.y = player.checkpointY;
        player.velocityX = 0;
        player.velocityY = 0;
      }
    }

    // Check level completion
    if (this.collision.checkLevelCompletion(player, level.playerEnd)) {
      this.gameState.completed = true;
    }

    // Update time
    this.gameState.time = (Date.now() - this.gameState.startTime) / 1000;

    // Keep player in bounds
    if (player.x < 0) player.x = 0;
    if (player.x + player.width > level.width) {
      player.x = level.width - player.width;
    }

    // Kill player if falls off screen
    if (player.y > level.height) {
      player.x = player.checkpointX;
      player.y = player.checkpointY;
      player.velocityX = 0;
      player.velocityY = 0;
    }
  }

  isGameOver(): boolean {
    return this.gameState.completed;
  }

  getGameState(): GameState {
    return this.gameState;
  }
}

// ============================================================================
// LEVEL BUILDER
// ============================================================================

export class LevelBuilder {
  private level: Level;

  constructor(
    id: string,
    name: string,
    width: number = 800,
    height: number = 600
  ) {
    this.level = {
      id,
      name,
      width,
      height,
      tiles: [],
      enemies: [],
      checkpoints: [],
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 500 },
      difficulty: 1,
    };
  }

  addTile(
    type: TileType,
    x: number,
    y: number,
    width: number = 40,
    height: number = 40
  ): LevelBuilder {
    this.level.tiles.push({ type, x, y, width, height });
    return this;
  }

  addMovingPlatform(
    x: number,
    y: number,
    pathX: number[],
    pathY: number[],
    width: number = 60,
    height: number = 20,
    speed: number = 2
  ): LevelBuilder {
    this.level.tiles.push({
      type: TileType.MOVING_PLATFORM,
      x,
      y,
      width,
      height,
      pathX,
      pathY,
      pathIndex: 0,
      speed,
    });
    return this;
  }

  addEnemy(
    id: string,
    x: number,
    y: number,
    pathX: number[],
    pathY: number[],
    speed: number = 2,
    width: number = 20,
    height: number = 20
  ): LevelBuilder {
    this.level.enemies.push({
      id,
      x,
      y,
      width,
      height,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      pathX,
      pathY,
      pathIndex: 0,
      speed,
    });
    return this;
  }

  setPlayerStart(x: number, y: number): LevelBuilder {
    this.level.playerStart = { x, y };
    return this;
  }

  setPlayerEnd(x: number, y: number): LevelBuilder {
    this.level.playerEnd = { x, y };
    return this;
  }

  setDifficulty(difficulty: number): LevelBuilder {
    this.level.difficulty = Math.max(1, Math.min(5, difficulty));
    return this;
  }

  build(): Level {
    return JSON.parse(JSON.stringify(this.level));
  }
}

// ============================================================================
// LEVEL SERIALIZATION
// ============================================================================

export function serializeLevel(level: Level): string {
  return JSON.stringify(level, null, 2);
}

export function deserializeLevel(json: string): Level {
  return JSON.parse(json);
}

export function shareCode(level: Level): string {
  const json = JSON.stringify(level);
  return btoa(json).slice(0, 100);
}

export function decodeShareCode(code: string): Level {
  const json = atob(code);
  return JSON.parse(json);
}
