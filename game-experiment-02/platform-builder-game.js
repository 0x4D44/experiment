/**
 * Platform Builder - Game Implementation
 * Complete game logic, level editor, and UI management
 */

// ============================================================================
// CONSTANTS & GLOBALS
// ============================================================================

const GRAVITY = 0.6;
const FRICTION = 0.85;
const MAX_FALL_SPEED = 15;
const TILE_COLORS = {
  solid: '#8B4513',
  spike: '#DC143C',
  spring: '#FFD700',
  checkpoint: '#32CD32',
  moving_platform: '#4169E1',
  empty: 'transparent',
};

let gameState = {
  currentLevel: null,
  gameRunning: false,
  gamePaused: false,
  player: null,
  gameStartTime: null,
  editingLevel: null,
  selectedTileType: 'solid',
  drawMode: false,
  tileSize: 40,
  tiles: [],
  enemies: [],
  undoStack: [],
};

const levelLibrary = {};

// ============================================================================
// LEVEL DEFINITIONS - 10 Pre-made Levels
// ============================================================================

function createPreloadedLevels() {
  // Level 1: Tutorial
  levelLibrary['level_1'] = {
    id: 'level_1',
    name: 'Tutorial - Basic Movement',
    width: 800,
    height: 600,
    difficulty: 1,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 500 },
    tiles: [
      // Ground
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // Platform to goal
      { type: 'solid', x: 700, y: 480, width: 100, height: 20, pathIndex: 0 },
      // Checkpoint
      { type: 'checkpoint', x: 200, y: 500, width: 40, height: 10, pathIndex: 0 },
    ],
    enemies: [],
  };

  // Level 2: Jumping Challenge
  levelLibrary['level_2'] = {
    id: 'level_2',
    name: 'Jump Practice',
    width: 800,
    height: 600,
    difficulty: 2,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 300 },
    tiles: [
      // Ground
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // Jumping sequence
      { type: 'solid', x: 100, y: 480, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 250, y: 420, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 400, y: 360, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 550, y: 300, width: 100, height: 20, pathIndex: 0 },
      { type: 'checkpoint', x: 300, y: 420, width: 40, height: 10, pathIndex: 0 },
    ],
    enemies: [],
  };

  // Level 3: Spike Hazard
  levelLibrary['level_3'] = {
    id: 'level_3',
    name: 'Spike Course',
    width: 800,
    height: 600,
    difficulty: 2,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 400 },
    tiles: [
      // Ground
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // Platforms with spikes
      { type: 'solid', x: 100, y: 480, width: 100, height: 20, pathIndex: 0 },
      { type: 'spike', x: 120, y: 460, width: 40, height: 20, pathIndex: 0 },
      { type: 'solid', x: 300, y: 420, width: 100, height: 20, pathIndex: 0 },
      { type: 'spike', x: 320, y: 400, width: 40, height: 20, pathIndex: 0 },
      { type: 'solid', x: 500, y: 360, width: 100, height: 20, pathIndex: 0 },
      { type: 'spike', x: 520, y: 340, width: 40, height: 20, pathIndex: 0 },
      { type: 'solid', x: 700, y: 400, width: 100, height: 20, pathIndex: 0 },
      { type: 'checkpoint', x: 300, y: 420, width: 40, height: 10, pathIndex: 0 },
    ],
    enemies: [],
  };

  // Level 4: Spring Mechanics
  levelLibrary['level_4'] = {
    id: 'level_4',
    name: 'Spring Bounce',
    width: 800,
    height: 600,
    difficulty: 2,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 200 },
    tiles: [
      // Ground
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // Springs to high platform
      { type: 'spring', x: 150, y: 500, width: 40, height: 40, pathIndex: 0 },
      { type: 'spring', x: 300, y: 400, width: 40, height: 40, pathIndex: 0 },
      { type: 'spring', x: 450, y: 300, width: 40, height: 40, pathIndex: 0 },
      { type: 'solid', x: 600, y: 200, width: 200, height: 20, pathIndex: 0 },
      { type: 'checkpoint', x: 150, y: 500, width: 40, height: 10, pathIndex: 0 },
    ],
    enemies: [],
  };

  // Level 5: Moving Platforms
  levelLibrary['level_5'] = {
    id: 'level_5',
    name: 'Moving Platforms',
    width: 800,
    height: 600,
    difficulty: 3,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 300 },
    tiles: [
      // Ground
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // Moving platform 1
      { type: 'moving_platform', x: 200, y: 450, width: 80, height: 20,
        pathX: [200, 400, 200], pathY: [450, 450, 450], pathIndex: 0, speed: 2 },
      // Moving platform 2
      { type: 'moving_platform', x: 500, y: 350, width: 80, height: 20,
        pathX: [500, 650, 500], pathY: [350, 350, 350], pathIndex: 0, speed: 2 },
      // Goal platform
      { type: 'solid', x: 700, y: 300, width: 100, height: 20, pathIndex: 0 },
    ],
    enemies: [],
  };

  // Level 6: Enemy Avoidance
  levelLibrary['level_6'] = {
    id: 'level_6',
    name: 'Avoid the Guard',
    width: 800,
    height: 600,
    difficulty: 3,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 400 },
    tiles: [
      // Ground
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // Platforms
      { type: 'solid', x: 150, y: 480, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 350, y: 420, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 550, y: 360, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 700, y: 400, width: 100, height: 20, pathIndex: 0 },
    ],
    enemies: [{
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
    }],
  };

  // Level 7: Complex Obstacle Course
  levelLibrary['level_7'] = {
    id: 'level_7',
    name: 'Obstacle Course',
    width: 800,
    height: 600,
    difficulty: 4,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 100 },
    tiles: [
      // Ground
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // First section - jumping
      { type: 'solid', x: 100, y: 480, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 250, y: 420, width: 100, height: 20, pathIndex: 0 },
      // Second section - spikes and springs
      { type: 'spike', x: 350, y: 400, width: 40, height: 20, pathIndex: 0 },
      { type: 'spring', x: 420, y: 380, width: 40, height: 40, pathIndex: 0 },
      // Third section - moving platform
      { type: 'moving_platform', x: 550, y: 300, width: 80, height: 20,
        pathX: [550, 650, 550], pathY: [300, 300, 300], pathIndex: 0, speed: 2 },
      // Final platform
      { type: 'solid', x: 700, y: 100, width: 100, height: 20, pathIndex: 0 },
      // Checkpoints
      { type: 'checkpoint', x: 250, y: 420, width: 40, height: 10, pathIndex: 0 },
      { type: 'checkpoint', x: 550, y: 300, width: 40, height: 10, pathIndex: 0 },
    ],
    enemies: [],
  };

  // Level 8: Gauntlet
  levelLibrary['level_8'] = {
    id: 'level_8',
    name: 'The Gauntlet',
    width: 800,
    height: 600,
    difficulty: 4,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 150 },
    tiles: [
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // Platforms with spikes
      { type: 'solid', x: 100, y: 480, width: 100, height: 20, pathIndex: 0 },
      { type: 'spike', x: 150, y: 460, width: 50, height: 20, pathIndex: 0 },
      { type: 'solid', x: 250, y: 420, width: 100, height: 20, pathIndex: 0 },
      { type: 'spike', x: 300, y: 400, width: 50, height: 20, pathIndex: 0 },
      { type: 'spring', x: 400, y: 380, width: 40, height: 40, pathIndex: 0 },
      { type: 'solid', x: 500, y: 250, width: 100, height: 20, pathIndex: 0 },
      { type: 'spike', x: 550, y: 230, width: 50, height: 20, pathIndex: 0 },
      { type: 'spring', x: 650, y: 200, width: 40, height: 40, pathIndex: 0 },
      { type: 'solid', x: 700, y: 150, width: 100, height: 20, pathIndex: 0 },
      { type: 'checkpoint', x: 250, y: 420, width: 40, height: 10, pathIndex: 0 },
      { type: 'checkpoint', x: 500, y: 250, width: 40, height: 10, pathIndex: 0 },
    ],
    enemies: [],
  };

  // Level 9: Multiple Enemies
  levelLibrary['level_9'] = {
    id: 'level_9',
    name: 'Guard Patrol',
    width: 800,
    height: 600,
    difficulty: 4,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 350 },
    tiles: [
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      { type: 'solid', x: 150, y: 480, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 350, y: 420, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 550, y: 350, width: 100, height: 20, pathIndex: 0 },
      { type: 'solid', x: 700, y: 350, width: 100, height: 20, pathIndex: 0 },
    ],
    enemies: [
      {
        id: 'enemy_1',
        x: 200,
        y: 400,
        width: 20,
        height: 20,
        velocityX: 0,
        velocityY: 0,
        grounded: true,
        pathX: [150, 500, 150],
        pathY: [400, 400, 400],
        pathIndex: 0,
        speed: 1.5,
      },
      {
        id: 'enemy_2',
        x: 550,
        y: 300,
        width: 20,
        height: 20,
        velocityX: 0,
        velocityY: 0,
        grounded: true,
        pathX: [450, 700, 450],
        pathY: [300, 300, 300],
        pathIndex: 0,
        speed: 2,
      },
    ],
  };

  // Level 10: Ultimate Challenge
  levelLibrary['level_10'] = {
    id: 'level_10',
    name: 'Ultimate Challenge',
    width: 800,
    height: 600,
    difficulty: 5,
    playerStart: { x: 40, y: 500 },
    playerEnd: { x: 750, y: 100 },
    tiles: [
      { type: 'solid', x: 0, y: 550, width: 800, height: 50, pathIndex: 0 },
      // Section 1: Jumping gauntlet
      { type: 'solid', x: 80, y: 480, width: 80, height: 20, pathIndex: 0 },
      { type: 'solid', x: 180, y: 440, width: 80, height: 20, pathIndex: 0 },
      { type: 'solid', x: 280, y: 400, width: 80, height: 20, pathIndex: 0 },
      // Section 2: Spike maze
      { type: 'spike', x: 340, y: 380, width: 40, height: 20, pathIndex: 0 },
      { type: 'spike', x: 400, y: 360, width: 40, height: 20, pathIndex: 0 },
      // Section 3: Spring boost
      { type: 'spring', x: 470, y: 320, width: 40, height: 40, pathIndex: 0 },
      // Section 4: Moving platforms
      { type: 'moving_platform', x: 550, y: 280, width: 60, height: 20,
        pathX: [550, 650, 550], pathY: [280, 280, 280], pathIndex: 0, speed: 2 },
      // Section 5: Final climb
      { type: 'solid', x: 680, y: 200, width: 80, height: 20, pathIndex: 0 },
      { type: 'spike', x: 700, y: 180, width: 40, height: 20, pathIndex: 0 },
      { type: 'solid', x: 740, y: 100, width: 60, height: 20, pathIndex: 0 },
      // Checkpoints
      { type: 'checkpoint', x: 280, y: 400, width: 40, height: 10, pathIndex: 0 },
      { type: 'checkpoint', x: 550, y: 280, width: 40, height: 10, pathIndex: 0 },
    ],
    enemies: [
      {
        id: 'enemy_1',
        x: 400,
        y: 300,
        width: 20,
        height: 20,
        velocityX: 0,
        velocityY: 0,
        grounded: true,
        pathX: [350, 550, 350],
        pathY: [300, 300, 300],
        pathIndex: 0,
        speed: 2,
      },
    ],
  };
}

// ============================================================================
// PHYSICS ENGINE
// ============================================================================

class PhysicsEngine {
  constructor() {
    this.gravity = GRAVITY;
  }

  update(player) {
    if (!player.grounded) {
      player.velocityY += this.gravity;
      if (player.velocityY > MAX_FALL_SPEED) {
        player.velocityY = MAX_FALL_SPEED;
      }
    } else {
      player.velocityY = 0;
    }

    player.velocityX *= FRICTION;
    player.x += player.velocityX;
    player.y += player.velocityY;
    player.grounded = false;
  }

  jump(player) {
    if (player.grounded) {
      player.velocityY = -12;
      player.grounded = false;
    }
  }

  moveLeft(player) {
    player.velocityX = Math.max(player.velocityX - 1.2, -8);
  }

  moveRight(player) {
    player.velocityX = Math.min(player.velocityX + 1.2, 8);
  }
}

// ============================================================================
// COLLISION DETECTION
// ============================================================================

class CollisionDetector {
  isColliding(rect1, rect2) {
    return (
      rect1.x < rect2.x + rect2.width &&
      rect1.x + rect1.width > rect2.x &&
      rect1.y < rect2.y + rect2.height &&
      rect1.y + rect1.height > rect2.y
    );
  }

  getOverlapSide(player, tile) {
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
  }

  resolveCollisions(player, tiles) {
    const collisions = tiles.filter(t => this.isColliding(player, t));

    for (const tile of collisions) {
      if (tile.type === 'empty') continue;

      const side = this.getOverlapSide(player, tile);

      if (tile.type === 'solid' || tile.type === 'moving_platform') {
        if (side === 'left') {
          player.x = tile.x - player.width;
          player.velocityX = 0;
        } else if (side === 'right') {
          player.x = tile.x + tile.width;
          player.velocityX = 0;
        } else if (side === 'top') {
          player.y = tile.y - player.height;
          player.velocityY = 0;
          player.grounded = true;
        } else {
          player.y = tile.y + tile.height;
          player.velocityY = 0.5;
        }
      } else if (tile.type === 'spike') {
        player.x = player.checkpointX;
        player.y = player.checkpointY;
        player.velocityX = 0;
        player.velocityY = 0;
      } else if (tile.type === 'spring') {
        if (side === 'top') {
          player.y = tile.y - player.height;
          player.velocityY = -15;
          player.grounded = true;
        }
      } else if (tile.type === 'checkpoint') {
        player.checkpointX = Math.round(player.x);
        player.checkpointY = Math.round(player.y);
      }
    }
  }
}

// ============================================================================
// GAME ENGINE
// ============================================================================

class GameEngine {
  constructor(level) {
    this.level = level;
    this.physics = new PhysicsEngine();
    this.collision = new CollisionDetector();
    this.startTime = Date.now();
    this.paused = false;

    this.player = {
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

    this.enemies = level.enemies.map(e => ({ ...e }));
  }

  update(input) {
    if (this.paused) return;

    const { player, level, physics, collision } = this;

    // Input
    if (input.left) physics.moveLeft(player);
    if (input.right) physics.moveRight(player);
    if (input.jump) physics.jump(player);

    // Physics
    physics.update(player);

    // Collisions
    collision.resolveCollisions(player, level.tiles);

    // Update moving platforms
    for (const tile of level.tiles) {
      if (tile.type === 'moving_platform' && tile.pathX) {
        tile.pathIndex = (tile.pathIndex + 1) % tile.pathX.length;
        tile.x = tile.pathX[tile.pathIndex];
        tile.y = tile.pathY[tile.pathIndex];

        if (collision.isColliding(player, tile)) {
          player.grounded = true;
          player.y = tile.y - player.height;
        }
      }
    }

    // Enemy updates
    for (const enemy of this.enemies) {
      if (enemy.pathX) {
        enemy.pathIndex = (enemy.pathIndex + 1) % enemy.pathX.length;
        enemy.x = enemy.pathX[enemy.pathIndex];
        enemy.y = enemy.pathY[enemy.pathIndex];
      }

      if (collision.isColliding(player, enemy)) {
        player.x = player.checkpointX;
        player.y = player.checkpointY;
        player.velocityX = 0;
        player.velocityY = 0;
      }
    }

    // Bounds checking
    if (player.x < 0) player.x = 0;
    if (player.x + player.width > level.width) {
      player.x = level.width - player.width;
    }

    if (player.y > level.height) {
      player.x = player.checkpointX;
      player.y = player.checkpointY;
      player.velocityX = 0;
      player.velocityY = 0;
    }
  }

  isCompleted() {
    const { player, level } = this;
    const end = level.playerEnd;
    return (
      player.x < end.x + 40 &&
      player.x + player.width > end.x &&
      player.y < end.y + 40 &&
      player.y + player.height > end.y
    );
  }

  getTime() {
    return ((Date.now() - this.startTime) / 1000).toFixed(1);
  }

  setPaused(paused) {
    this.paused = paused;
  }
}

// ============================================================================
// RENDERING
// ============================================================================

function drawGame(canvas, engine) {
  const ctx = canvas.getContext('2d');
  const { level, player, enemies } = engine;

  // Clear
  ctx.fillStyle = '#87CEEB';
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  // Draw tiles
  for (const tile of level.tiles) {
    ctx.fillStyle = TILE_COLORS[tile.type];
    ctx.fillRect(tile.x, tile.y, tile.width, tile.height);

    // Add visual distinctions
    if (tile.type === 'spike') {
      ctx.fillStyle = '#8B0000';
      ctx.font = 'bold 24px Arial';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText('⚠', tile.x + tile.width / 2, tile.y + tile.height / 2);
    } else if (tile.type === 'spring') {
      ctx.fillStyle = '#FFA500';
      ctx.font = 'bold 20px Arial';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText('S', tile.x + tile.width / 2, tile.y + tile.height / 2);
    } else if (tile.type === 'checkpoint') {
      ctx.fillStyle = '#006400';
      ctx.font = 'bold 20px Arial';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText('P', tile.x + tile.width / 2, tile.y + tile.height / 2);
    }
  }

  // Draw goal
  ctx.fillStyle = '#FF69B4';
  ctx.fillRect(level.playerEnd.x, level.playerEnd.y, 40, 40);
  ctx.fillStyle = '#FFFFFF';
  ctx.font = 'bold 24px Arial';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText('🚪', level.playerEnd.x + 20, level.playerEnd.y + 20);

  // Draw player
  ctx.fillStyle = '#00FF00';
  ctx.fillRect(player.x, player.y, player.width, player.height);
  ctx.fillStyle = '#000000';
  ctx.font = '12px Arial';
  ctx.textAlign = 'center';
  ctx.fillText('P', player.x + player.width / 2, player.y + 15);

  // Draw enemies
  for (const enemy of enemies) {
    ctx.fillStyle = '#FF0000';
    ctx.fillRect(enemy.x, enemy.y, enemy.width, enemy.height);
    ctx.fillStyle = '#FFFFFF';
    ctx.font = 'bold 14px Arial';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText('E', enemy.x + enemy.width / 2, enemy.y + enemy.height / 2);
  }
}

function drawEditor(canvas, tiles, tileSize) {
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = '#FFFFFF';
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  // Grid
  ctx.strokeStyle = '#E0E0E0';
  ctx.lineWidth = 1;
  for (let x = 0; x < canvas.width; x += tileSize) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
    ctx.stroke();
  }
  for (let y = 0; y < canvas.height; y += tileSize) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
    ctx.stroke();
  }

  // Tiles
  for (const tile of tiles) {
    ctx.fillStyle = TILE_COLORS[tile.type];
    ctx.fillRect(tile.x, tile.y, tile.width, tile.height);
    ctx.strokeStyle = '#333333';
    ctx.lineWidth = 2;
    ctx.strokeRect(tile.x, tile.y, tile.width, tile.height);
  }

  // Player start
  if (gameState.editingLevel && gameState.editingLevel.playerStart) {
    const ps = gameState.editingLevel.playerStart;
    ctx.fillStyle = '#00FF00';
    ctx.fillRect(ps.x, ps.y, 20, 30);
  }

  // Player end
  if (gameState.editingLevel && gameState.editingLevel.playerEnd) {
    const pe = gameState.editingLevel.playerEnd;
    ctx.fillStyle = '#FF69B4';
    ctx.fillRect(pe.x, pe.y, 40, 40);
  }
}

// ============================================================================
// UI & EVENT HANDLERS
// ============================================================================

let keyState = {};

window.addEventListener('keydown', (e) => {
  keyState[e.key.toLowerCase()] = true;

  if (e.key === 'r' || e.key === 'R') {
    if (gameState.gameRunning && gameState.currentLevel) {
      startGame(gameState.currentLevel);
    }
  }
});

window.addEventListener('keyup', (e) => {
  keyState[e.key.toLowerCase()] = false;
});

function getInput() {
  return {
    left: keyState['arrowleft'] || keyState['a'],
    right: keyState['arrowright'] || keyState['d'],
    jump: keyState['arrowup'] || keyState['w'],
  };
}

function startGame(level) {
  gameState.currentLevel = level;
  gameState.gameEngine = new GameEngine(level);
  gameState.gameRunning = true;
  gameState.gamePaused = false;

  const canvas = document.getElementById('gameCanvas');
  const gameLoop = () => {
    const input = getInput();
    gameState.gameEngine.update(input);

    drawGame(canvas, gameState.gameEngine);
    updateGameUI();

    if (gameState.gameEngine.isCompleted()) {
      gameState.gameRunning = false;
      showCompletionMessage();
    } else if (gameState.gameRunning) {
      requestAnimationFrame(gameLoop);
    }
  };

  gameLoop();
}

function updateGameUI() {
  const engine = gameState.gameEngine;
  if (!engine) return;

  document.getElementById('gameTime').textContent = engine.getTime() + 's';
  document.getElementById('gameDifficulty').textContent = 'Difficulty ' + engine.level.difficulty;
  document.getElementById('gameStatus').textContent = gameState.gamePaused ? 'Paused' : 'Playing';
}

function showCompletionMessage() {
  const time = gameState.gameEngine.getTime();
  document.getElementById('completionTime').textContent = `Time: ${time} seconds`;
  document.getElementById('completionMessage').style.display = 'block';
}

function populateLevelSelect() {
  const select = document.getElementById('levelSelect');
  const option = select.querySelector('option[value=""]');

  for (const [id, level] of Object.entries(levelLibrary)) {
    const opt = document.createElement('option');
    opt.value = id;
    opt.textContent = level.name + ' (Difficulty ' + level.difficulty + ')';
    select.appendChild(opt);
  }

  select.addEventListener('change', (e) => {
    if (e.target.value) {
      startGame(levelLibrary[e.target.value]);
    }
  });
}

function updateLevelsList() {
  const list = document.getElementById('levelsList');
  list.innerHTML = '';

  for (const [id, level] of Object.entries(levelLibrary)) {
    const li = document.createElement('li');
    li.style.cssText = 'padding: 10px; margin: 5px 0; background: white; border-radius: 4px; border-left: 4px solid #667eea;';

    const name = document.createElement('strong');
    name.textContent = level.name;

    const stats = document.createElement('div');
    stats.style.fontSize = '12px';
    stats.style.color = '#666';
    stats.style.marginTop = '5px';
    stats.textContent = `Difficulty: ${level.difficulty} | Tiles: ${level.tiles.length} | Enemies: ${level.enemies.length}`;

    li.appendChild(name);
    li.appendChild(stats);
    list.appendChild(li);
  }
}

// ============================================================================
// EDITOR FUNCTIONALITY
// ============================================================================

document.addEventListener('DOMContentLoaded', () => {
  createPreloadedLevels();
  populateLevelSelect();
  updateLevelsList();

  // Tab switching
  document.querySelectorAll('.tab-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));

      btn.classList.add('active');
      document.getElementById(btn.dataset.tab).classList.add('active');
    });
  });

  // Tile selection
  document.querySelectorAll('.tile-option').forEach(opt => {
    opt.addEventListener('click', () => {
      document.querySelectorAll('.tile-option').forEach(o => o.classList.remove('selected'));
      opt.classList.add('selected');
      gameState.selectedTileType = opt.dataset.tile;
    });
  });

  // Editor canvas
  const editorCanvas = document.getElementById('editorCanvas');
  editorCanvas.addEventListener('click', (e) => {
    const rect = editorCanvas.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left) / gameState.tileSize) * gameState.tileSize;
    const y = Math.floor((e.clientY - rect.top) / gameState.tileSize) * gameState.tileSize;

    if (!gameState.editingLevel) {
      gameState.editingLevel = {
        id: 'custom_' + Date.now(),
        name: 'Custom Level',
        width: editorCanvas.width,
        height: editorCanvas.height,
        difficulty: 1,
        playerStart: { x: 40, y: 500 },
        playerEnd: { x: 750, y: 400 },
        tiles: [],
        enemies: [],
      };
    }

    // Save undo state
    gameState.undoStack.push(JSON.stringify(gameState.editingLevel.tiles));

    const tile = {
      type: gameState.selectedTileType,
      x: x,
      y: y,
      width: gameState.tileSize,
      height: gameState.tileSize,
      pathIndex: 0,
    };
    gameState.editingLevel.tiles.push(tile);
    drawEditor(editorCanvas, gameState.editingLevel.tiles, gameState.tileSize);
  });

  document.getElementById('undoBtn').addEventListener('click', () => {
    if (gameState.undoStack.length > 0 && gameState.editingLevel) {
      gameState.editingLevel.tiles = JSON.parse(gameState.undoStack.pop());
      drawEditor(editorCanvas, gameState.editingLevel.tiles, gameState.tileSize);
    }
  });

  document.getElementById('clearBtn').addEventListener('click', () => {
    if (gameState.editingLevel) {
      gameState.undoStack.push(JSON.stringify(gameState.editingLevel.tiles));
      gameState.editingLevel.tiles = [];
      drawEditor(editorCanvas, [], gameState.tileSize);
    }
  });

  document.getElementById('tileSize').addEventListener('change', (e) => {
    gameState.tileSize = parseInt(e.target.value);
  });

  document.getElementById('saveEditorBtn').addEventListener('click', () => {
    if (gameState.editingLevel) {
      gameState.editingLevel.name = document.getElementById('editorLevelName').value || 'Custom Level';
      gameState.editingLevel.difficulty = parseInt(document.getElementById('editorDifficulty').value);
      levelLibrary[gameState.editingLevel.id] = gameState.editingLevel;
      updateLevelsList();
      showStatus('editorStatus', 'Level saved successfully!', 'success');
    }
  });

  document.getElementById('playtestBtn').addEventListener('click', () => {
    if (gameState.editingLevel) {
      document.querySelector('[data-tab="play"]').click();
      startGame(gameState.editingLevel);
    }
  });

  // Game controls
  document.getElementById('pauseBtn').addEventListener('click', () => {
    if (gameState.gameEngine) {
      gameState.gamePaused = !gameState.gamePaused;
      gameState.gameEngine.setPaused(gameState.gamePaused);
      document.getElementById('pauseBtn').textContent = gameState.gamePaused ? 'Resume' : 'Pause';
    }
  });

  document.getElementById('restartBtn').addEventListener('click', () => {
    if (gameState.currentLevel) {
      startGame(gameState.currentLevel);
      document.getElementById('completionMessage').style.display = 'none';
      document.getElementById('pauseBtn').textContent = 'Pause';
      gameState.gamePaused = false;
    }
  });

  // Export/Import
  document.getElementById('exportBtn').addEventListener('click', () => {
    const level = levelLibrary[Object.keys(levelLibrary)[0]] || gameState.editingLevel;
    if (level) {
      document.getElementById('exportText').value = JSON.stringify(level, null, 2);
      document.getElementById('exportModal').classList.add('active');
    }
  });

  document.getElementById('importBtn').addEventListener('click', () => {
    document.getElementById('importModal').classList.add('active');
  });

  // Initial draw
  drawEditor(editorCanvas, [], gameState.tileSize);
});

function showStatus(elementId, message, type) {
  const el = document.getElementById(elementId);
  el.textContent = message;
  el.className = 'status-message ' + type;
  setTimeout(() => {
    el.className = 'status-message';
  }, 3000);
}

function copyToClipboard(elementId) {
  const text = document.getElementById(elementId);
  text.select();
  document.execCommand('copy');
  showStatus('editorStatus', 'Copied to clipboard!', 'success');
}

function importLevelFromText() {
  const text = document.getElementById('importText').value;
  try {
    const level = JSON.parse(text);
    level.id = 'imported_' + Date.now();
    levelLibrary[level.id] = level;
    updateLevelsList();
    document.getElementById('importModal').classList.remove('active');
    showStatus('editorStatus', 'Level imported successfully!', 'success');
  } catch (e) {
    showStatus('editorStatus', 'Invalid JSON format', 'error');
  }
}

document.getElementById('downloadBtn').addEventListener('click', () => {
  const data = JSON.stringify(levelLibrary, null, 2);
  const blob = new Blob([data], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'platform-builder-levels.json';
  a.click();
});
