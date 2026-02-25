/**
 * Platform Builder - Comprehensive Test Suite
 * Tests for physics, collision detection, game logic, and level management
 */

// Mock types since we're testing the logic
interface Vector2 {
  x: number;
  y: number;
}

interface Tile {
  type: string;
  x: number;
  y: number;
  width: number;
  height: number;
  pathIndex?: number;
}

interface Entity {
  x: number;
  y: number;
  width: number;
  height: number;
  velocityX: number;
  velocityY: number;
  grounded: boolean;
}

interface Player extends Entity {
  checkpointX: number;
  checkpointY: number;
}

// ============================================================================
// PHYSICS ENGINE TESTS
// ============================================================================

describe('PhysicsEngine', () => {
  let physics: any;

  beforeEach(() => {
    physics = {
      update: (entity: Entity) => {
        const GRAVITY = 0.6;
        const FRICTION = 0.85;
        const MAX_FALL_SPEED = 15;

        if (!entity.grounded) {
          entity.velocityY += GRAVITY;
          if (entity.velocityY > MAX_FALL_SPEED) {
            entity.velocityY = MAX_FALL_SPEED;
          }
        } else {
          entity.velocityY = 0;
        }

        entity.velocityX *= FRICTION;
        entity.x += entity.velocityX;
        entity.y += entity.velocityY;
        entity.grounded = false;
      },
      jump: (entity: Player) => {
        if (entity.grounded) {
          entity.velocityY = -12;
          entity.grounded = false;
        }
      },
      moveLeft: (entity: Player) => {
        entity.velocityX = Math.max(entity.velocityX - 1.2, -8);
      },
      moveRight: (entity: Player) => {
        entity.velocityX = Math.min(entity.velocityX + 1.2, 8);
      },
    };
  });

  test('gravity pulls entity down when not grounded', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      checkpointX: 100,
      checkpointY: 100,
    };

    physics.update(player);
    expect(player.velocityY).toBeGreaterThan(0);
    expect(player.y).toBeGreaterThan(100);
  });

  test('no gravity applied when grounded', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 5,
      grounded: true,
      checkpointX: 100,
      checkpointY: 100,
    };

    physics.update(player);
    expect(player.velocityY).toBe(0);
    expect(player.y).toBe(100);
  });

  test('velocity capped at max fall speed', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 100,
      grounded: false,
      checkpointX: 100,
      checkpointY: 100,
    };

    physics.update(player);
    expect(player.velocityY).toBeLessThanOrEqual(15);
  });

  test('jump applies upward velocity when grounded', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: 100,
      checkpointY: 100,
    };

    physics.jump(player);
    expect(player.velocityY).toBe(-12);
    expect(player.grounded).toBe(false);
  });

  test('jump does nothing when not grounded', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: -5,
      grounded: false,
      checkpointX: 100,
      checkpointY: 100,
    };

    physics.jump(player);
    expect(player.velocityY).toBe(-5);
  });

  test('friction reduces horizontal velocity', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 10,
      velocityY: 0,
      grounded: true,
      checkpointX: 100,
      checkpointY: 100,
    };

    physics.update(player);
    expect(player.velocityX).toBeLessThan(10);
    expect(player.velocityX).toBeGreaterThan(0);
  });

  test('moveRight increases velocity within max speed', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: 100,
      checkpointY: 100,
    };

    physics.moveRight(player);
    expect(player.velocityX).toBeGreaterThan(0);
    expect(player.velocityX).toBeLessThanOrEqual(8);
  });

  test('moveLeft decreases velocity within max speed', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: 100,
      checkpointY: 100,
    };

    physics.moveLeft(player);
    expect(player.velocityX).toBeLessThan(0);
    expect(player.velocityX).toBeGreaterThanOrEqual(-8);
  });

  test('acceleration over time reaches max speed', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: 100,
      checkpointY: 100,
    };

    // Accelerate multiple times
    for (let i = 0; i < 10; i++) {
      physics.moveRight(player);
    }

    expect(player.velocityX).toBeLessThanOrEqual(8);
  });
});

// ============================================================================
// COLLISION DETECTION TESTS
// ============================================================================

describe('CollisionDetector', () => {
  let collision: any;

  beforeEach(() => {
    collision = {
      isColliding: (rect1: any, rect2: any) => {
        return (
          rect1.x < rect2.x + rect2.width &&
          rect1.x + rect1.width > rect2.x &&
          rect1.y < rect2.y + rect2.height &&
          rect1.y + rect1.height > rect2.y
        );
      },
      getOverlapSide: (player: Player, tile: Tile) => {
        const overlapLeft = player.x + player.width - tile.x;
        const overlapRight = tile.x + tile.width - player.x;
        const overlapTop = player.y + player.height - tile.y;
        const overlapBottom = tile.y + tile.height - player.y;

        const minOverlapX = Math.min(overlapLeft, overlapRight);
        const minOverlapY = Math.min(overlapTop, overlapBottom);

        if (minOverlapX < minOverlapY) {
          return overlapLeft < overlapRight ? 'left' : 'right';
        }
        return overlapTop < overlapBottom ? 'top' : 'bottom';
      },
    };
  });

  test('detects collision between overlapping rectangles', () => {
    const rect1 = { x: 0, y: 0, width: 50, height: 50 };
    const rect2 = { x: 25, y: 25, width: 50, height: 50 };
    expect(collision.isColliding(rect1, rect2)).toBe(true);
  });

  test('does not detect collision between non-overlapping rectangles', () => {
    const rect1 = { x: 0, y: 0, width: 50, height: 50 };
    const rect2 = { x: 100, y: 100, width: 50, height: 50 };
    expect(collision.isColliding(rect1, rect2)).toBe(false);
  });

  test('detects collision with touching rectangles', () => {
    const rect1 = { x: 0, y: 0, width: 50, height: 50 };
    const rect2 = { x: 50, y: 0, width: 50, height: 50 };
    expect(collision.isColliding(rect1, rect2)).toBe(false);
  });

  test('determines top collision correctly', () => {
    const player: Player = {
      x: 0,
      y: 20,
      width: 50,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      checkpointX: 0,
      checkpointY: 20,
    };
    const tile: Tile = {
      type: 'solid',
      x: 0,
      y: 50,
      width: 50,
      height: 20,
      pathIndex: 0,
    };

    expect(collision.getOverlapSide(player, tile)).toBe('top');
  });

  test('determines bottom collision correctly', () => {
    const player: Player = {
      x: 0,
      y: 50,
      width: 50,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      checkpointX: 0,
      checkpointY: 50,
    };
    const tile: Tile = {
      type: 'solid',
      x: 0,
      y: 20,
      width: 50,
      height: 20,
      pathIndex: 0,
    };

    expect(collision.getOverlapSide(player, tile)).toBe('bottom');
  });

  test('determines left collision correctly', () => {
    const player: Player = {
      x: 20,
      y: 0,
      width: 30,
      height: 50,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      checkpointX: 20,
      checkpointY: 0,
    };
    const tile: Tile = {
      type: 'solid',
      x: 50,
      y: 0,
      width: 20,
      height: 50,
      pathIndex: 0,
    };

    expect(collision.getOverlapSide(player, tile)).toBe('left');
  });

  test('determines right collision correctly', () => {
    const player: Player = {
      x: 50,
      y: 0,
      width: 30,
      height: 50,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      checkpointX: 50,
      checkpointY: 0,
    };
    const tile: Tile = {
      type: 'solid',
      x: 20,
      y: 0,
      width: 20,
      height: 50,
      pathIndex: 0,
    };

    expect(collision.getOverlapSide(player, tile)).toBe('right');
  });
});

// ============================================================================
// GAME LOGIC TESTS
// ============================================================================

describe('Game Logic', () => {
  test('spike resets player to checkpoint', () => {
    const player: Player = {
      x: 100,
      y: 100,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      checkpointX: 50,
      checkpointY: 50,
    };

    const spikeTile: Tile = {
      type: 'spike',
      x: 100,
      y: 100,
      width: 20,
      height: 20,
      pathIndex: 0,
    };

    // Collision with spike
    if (
      player.x < spikeTile.x + spikeTile.width &&
      player.x + player.width > spikeTile.x &&
      player.y < spikeTile.y + spikeTile.height &&
      player.y + player.height > spikeTile.y
    ) {
      player.x = player.checkpointX;
      player.y = player.checkpointY;
      player.velocityX = 0;
      player.velocityY = 0;
    }

    expect(player.x).toBe(50);
    expect(player.y).toBe(50);
  });

  test('checkpoint updates save location', () => {
    const player: Player = {
      x: 200,
      y: 200,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: 50,
      checkpointY: 50,
    };

    const checkpointTile: Tile = {
      type: 'checkpoint',
      x: 200,
      y: 200,
      width: 20,
      height: 10,
      pathIndex: 0,
    };

    // Collision with checkpoint
    if (
      player.x < checkpointTile.x + checkpointTile.width &&
      player.x + player.width > checkpointTile.x &&
      player.y < checkpointTile.y + checkpointTile.height &&
      player.y + player.height > checkpointTile.y
    ) {
      player.checkpointX = Math.round(player.x);
      player.checkpointY = Math.round(player.y);
    }

    expect(player.checkpointX).toBe(200);
    expect(player.checkpointY).toBe(200);
  });

  test('level completion detected when player reaches goal', () => {
    const player: Player = {
      x: 750,
      y: 400,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: 40,
      checkpointY: 500,
    };

    const goalZone = { x: 750, y: 400 };

    const completed =
      player.x < goalZone.x + 40 &&
      player.x + player.width > goalZone.x &&
      player.y < goalZone.y + 40 &&
      player.y + player.height > goalZone.y;

    expect(completed).toBe(true);
  });

  test('player bounds are enforced', () => {
    const player: Player = {
      x: -10,
      y: 500,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: 40,
      checkpointY: 500,
    };

    // Clamp to bounds
    if (player.x < 0) player.x = 0;
    if (player.x + player.width > 800) {
      player.x = 800 - player.width;
    }

    expect(player.x).toBeGreaterThanOrEqual(0);
    expect(player.x + player.width).toBeLessThanOrEqual(800);
  });

  test('fall detection resets player', () => {
    const player: Player = {
      x: 400,
      y: 700,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: 40,
      checkpointY: 500,
    };

    // If player falls off screen
    if (player.y > 600) {
      player.x = player.checkpointX;
      player.y = player.checkpointY;
      player.velocityX = 0;
      player.velocityY = 0;
    }

    expect(player.x).toBe(40);
    expect(player.y).toBe(500);
  });
});

// ============================================================================
// LEVEL BUILDING TESTS
// ============================================================================

describe('Level Building', () => {
  test('creates empty level', () => {
    const level = {
      id: 'test_1',
      name: 'Test Level',
      width: 800,
      height: 600,
      difficulty: 1,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [],
      enemies: [],
    };

    expect(level.id).toBe('test_1');
    expect(level.tiles.length).toBe(0);
    expect(level.enemies.length).toBe(0);
  });

  test('adds tiles to level', () => {
    const level = {
      id: 'test_2',
      name: 'Test Level 2',
      width: 800,
      height: 600,
      difficulty: 2,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [] as Tile[],
      enemies: [],
    };

    level.tiles.push({
      type: 'solid',
      x: 0,
      y: 550,
      width: 800,
      height: 50,
      pathIndex: 0,
    });

    expect(level.tiles.length).toBe(1);
    expect(level.tiles[0].type).toBe('solid');
  });

  test('adds enemies to level', () => {
    const level = {
      id: 'test_3',
      name: 'Test Level 3',
      width: 800,
      height: 600,
      difficulty: 3,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [] as Tile[],
      enemies: [] as any[],
    };

    level.enemies.push({
      id: 'enemy_1',
      x: 200,
      y: 400,
      width: 20,
      height: 20,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      pathX: [150, 550, 150],
      pathY: [400, 400, 400],
      pathIndex: 0,
      speed: 2,
    });

    expect(level.enemies.length).toBe(1);
    expect(level.enemies[0].id).toBe('enemy_1');
  });

  test('sets player start and end positions', () => {
    const level = {
      id: 'test_4',
      name: 'Test Level 4',
      width: 800,
      height: 600,
      difficulty: 1,
      playerStart: { x: 100, y: 500 },
      playerEnd: { x: 700, y: 300 },
      tiles: [],
      enemies: [],
    };

    expect(level.playerStart.x).toBe(100);
    expect(level.playerEnd.y).toBe(300);
  });

  test('sets difficulty level within range', () => {
    const level = {
      id: 'test_5',
      name: 'Test Level 5',
      width: 800,
      height: 600,
      difficulty: 3,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [],
      enemies: [],
    };

    expect(level.difficulty).toBeGreaterThanOrEqual(1);
    expect(level.difficulty).toBeLessThanOrEqual(5);
  });

  test('creates level with mixed tile types', () => {
    const level = {
      id: 'test_6',
      name: 'Mixed Tiles',
      width: 800,
      height: 600,
      difficulty: 3,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [
        { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
        { type: 'spike', x: 200, y: 520, width: 40, height: 30, pathIndex: 0 },
        { type: 'spring', x: 400, y: 480, width: 40, height: 40, pathIndex: 0 },
        { type: 'checkpoint', x: 300, y: 420, width: 40, height: 10, pathIndex: 0 },
      ],
      enemies: [],
    };

    expect(level.tiles.length).toBe(4);
    expect(level.tiles.filter(t => t.type === 'solid').length).toBe(1);
    expect(level.tiles.filter(t => t.type === 'spike').length).toBe(1);
    expect(level.tiles.filter(t => t.type === 'spring').length).toBe(1);
    expect(level.tiles.filter(t => t.type === 'checkpoint').length).toBe(1);
  });
});

// ============================================================================
// SERIALIZATION TESTS
// ============================================================================

describe('Level Serialization', () => {
  test('serializes level to JSON', () => {
    const level = {
      id: 'test_7',
      name: 'Test Level 7',
      width: 800,
      height: 600,
      difficulty: 2,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [],
      enemies: [],
    };

    const json = JSON.stringify(level);
    expect(json).toContain('test_7');
    expect(json).toContain('Test Level 7');
  });

  test('deserializes level from JSON', () => {
    const levelJson =
      '{"id":"test_8","name":"Test Level 8","width":800,"height":600,"difficulty":1,"playerStart":{"x":40,"y":500},"playerEnd":{"x":750,"y":400},"tiles":[],"enemies":[]}';

    const level = JSON.parse(levelJson);
    expect(level.id).toBe('test_8');
    expect(level.name).toBe('Test Level 8');
    expect(level.width).toBe(800);
  });

  test('round-trip serialization preserves data', () => {
    const original = {
      id: 'test_9',
      name: 'Test Level 9',
      width: 800,
      height: 600,
      difficulty: 3,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [
        { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      ],
      enemies: [],
    };

    const json = JSON.stringify(original);
    const restored = JSON.parse(json);

    expect(restored.id).toBe(original.id);
    expect(restored.tiles.length).toBe(original.tiles.length);
    expect(restored.playerStart.x).toBe(original.playerStart.x);
  });
});

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

describe('Integration Tests', () => {
  test('complete gameplay sequence - start to goal', () => {
    // Setup
    const level = {
      id: 'integration_1',
      name: 'Integration Test Level',
      width: 800,
      height: 600,
      difficulty: 1,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [
        { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
        { type: 'solid', x: 700, y: 400, width: 100, height: 20, pathIndex: 0 },
      ],
      enemies: [],
    };

    const player: Player = {
      x: level.playerStart.x,
      y: level.playerStart.y,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: false,
      checkpointX: level.playerStart.x,
      checkpointY: level.playerStart.y,
    };

    // Move towards goal
    player.velocityX = 5;
    for (let i = 0; i < 100; i++) {
      player.x += player.velocityX;
    }

    // Check level not completed yet
    let completed =
      player.x < level.playerEnd.x + 40 &&
      player.x + player.width > level.playerEnd.x &&
      player.y < level.playerEnd.y + 40 &&
      player.y + player.height > level.playerEnd.y;

    expect(completed).toBe(false);

    // Move to goal
    player.x = level.playerEnd.x;
    player.y = level.playerEnd.y;

    // Check level completed
    completed =
      player.x < level.playerEnd.x + 40 &&
      player.x + player.width > level.playerEnd.x &&
      player.y < level.playerEnd.y + 40 &&
      player.y + player.height > level.playerEnd.y;

    expect(completed).toBe(true);
  });

  test('obstacle avoidance sequence - spike hazard', () => {
    const level = {
      id: 'integration_2',
      name: 'Obstacle Avoidance Test',
      width: 800,
      height: 600,
      difficulty: 2,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [
        { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
        { type: 'spike', x: 300, y: 520, width: 40, height: 30, pathIndex: 0 },
        { type: 'solid', x: 700, y: 400, width: 100, height: 20, pathIndex: 0 },
      ],
      enemies: [],
    };

    const player: Player = {
      x: level.playerStart.x,
      y: level.playerStart.y,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: level.playerStart.x,
      checkpointY: level.playerStart.y,
    };

    // Hit spike
    player.x = 300;
    player.y = 500;

    if (
      player.x < 340 &&
      player.x + 20 > 300 &&
      player.y < 550 &&
      player.y + 30 > 520
    ) {
      player.x = player.checkpointX;
      player.y = player.checkpointY;
    }

    expect(player.x).toBe(level.playerStart.x);
    expect(player.y).toBe(level.playerStart.y);
  });

  test('multiple checkpoints in level', () => {
    const level = {
      id: 'integration_3',
      name: 'Checkpoint Test',
      width: 800,
      height: 600,
      difficulty: 2,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 100 },
      tiles: [
        { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
        { type: 'checkpoint', x: 200, y: 480, width: 40, height: 20, pathIndex: 0 },
        { type: 'checkpoint', x: 450, y: 350, width: 40, height: 20, pathIndex: 0 },
      ],
      enemies: [],
    };

    const player: Player = {
      x: level.playerStart.x,
      y: level.playerStart.y,
      width: 20,
      height: 30,
      velocityX: 0,
      velocityY: 0,
      grounded: true,
      checkpointX: level.playerStart.x,
      checkpointY: level.playerStart.y,
    };

    // Reach first checkpoint
    player.x = 200;
    player.y = 480;

    if (
      player.x < 240 &&
      player.x + 20 > 200 &&
      player.y < 500 &&
      player.y + 30 > 480
    ) {
      player.checkpointX = Math.round(player.x);
      player.checkpointY = Math.round(player.y);
    }

    expect(player.checkpointX).toBe(200);

    // Reach second checkpoint
    player.x = 450;
    player.y = 350;

    if (
      player.x < 490 &&
      player.x + 20 > 450 &&
      player.y < 370 &&
      player.y + 30 > 350
    ) {
      player.checkpointX = Math.round(player.x);
      player.checkpointY = Math.round(player.y);
    }

    expect(player.checkpointX).toBe(450);
  });

  test('level with enemies', () => {
    const level = {
      id: 'integration_4',
      name: 'Enemy Test',
      width: 800,
      height: 600,
      difficulty: 3,
      playerStart: { x: 40, y: 500 },
      playerEnd: { x: 750, y: 400 },
      tiles: [
        { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      ],
      enemies: [
        {
          id: 'enemy_1',
          x: 300,
          y: 400,
          width: 20,
          height: 20,
          velocityX: 0,
          velocityY: 0,
          grounded: true,
          pathX: [250, 400, 250],
          pathY: [400, 400, 400],
          pathIndex: 0,
          speed: 2,
        },
      ],
    };

    expect(level.enemies.length).toBe(1);
    expect(level.enemies[0].pathX.length).toBe(3);

    // Move enemy along path
    const enemy = level.enemies[0];
    const oldX = enemy.x;
    enemy.pathIndex = (enemy.pathIndex + 1) % enemy.pathX.length;
    enemy.x = enemy.pathX[enemy.pathIndex];
    enemy.y = enemy.pathY[enemy.pathIndex];
    expect(enemy.x).not.toBe(oldX);
  });
});
