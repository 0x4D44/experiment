/**
 * Quantum Blocks - A puzzle game combining Tetris with Match-3 mechanics
 * Complete game implementation in a single file for simplicity
 */

// Constants
const COLORS = {
  RED: 1,
  BLUE: 2,
  GREEN: 3,
  YELLOW: 4,
  PURPLE: 5,
  ORANGE: 6,
  EMPTY: 0
};

const BLOCK_TYPES = {
  NORMAL: 'normal',
  BOMB: 'bomb',
  COLOR_CHANGER: 'color-changer',
  GRAVITY_REVERSER: 'gravity-reverser'
};

const GAME_MODES = {
  ZEN: 'zen',
  TIME_ATTACK: 'time-attack'
};

const GRID_WIDTH = 10;
const GRID_HEIGHT = 20;
const BLOCK_SIZE = 30;
const MIN_MATCH = 3;

const MATCH_DIRECTIONS = [
  { dx: 1, dy: 0 },  // horizontal
  { dx: 0, dy: 1 },  // vertical
  { dx: 1, dy: 1 },  // diagonal down-right
  { dx: 1, dy: -1 }  // diagonal down-left
];

/**
 * Game Engine - Core game logic
 */
class GameEngine {
  constructor(mode = GAME_MODES.ZEN) {
    this.grid = this.createEmptyGrid();
    this.score = 0;
    this.level = 1;
    this.speed = 1;
    this.mode = mode;
    this.timeRemaining = mode === GAME_MODES.TIME_ATTACK ? 180 : Infinity;
    this.gameOver = false;
    this.nextBlockColor = this.getRandomColor();
    this.chainMultiplier = 1;
    this.gravityReversed = false;
    this.particleEffects = [];
  }

  createEmptyGrid() {
    return Array(GRID_HEIGHT).fill(null).map(() =>
      Array(GRID_WIDTH).fill(null)
    );
  }

  getRandomColor() {
    const colors = [COLORS.RED, COLORS.BLUE, COLORS.GREEN, COLORS.YELLOW, COLORS.PURPLE, COLORS.ORANGE];
    return colors[Math.floor(Math.random() * colors.length)];
  }

  spawnNewBlock() {
    const color = this.nextBlockColor;
    this.nextBlockColor = this.getRandomColor();

    for (let col = 0; col < GRID_WIDTH; col++) {
      if (this.grid[0][col] === null) {
        this.grid[0][col] = {
          color,
          type: BLOCK_TYPES.NORMAL,
          falling: true
        };
        return true;
      }
    }

    this.gameOver = true;
    return false;
  }

  moveBlockDown(col) {
    for (let row = 0; row < GRID_HEIGHT - 1; row++) {
      if (this.grid[row][col] && this.grid[row][col].falling) {
        const block = this.grid[row][col];

        if (this.grid[row + 1][col] === null) {
          this.grid[row + 1][col] = block;
          this.grid[row][col] = null;
          return true;
        } else {
          block.falling = false;
          return false;
        }
      }
    }

    return false;
  }

  moveBlockLeft(col) {
    if (col === 0) return false;

    for (let row = 0; row < GRID_HEIGHT; row++) {
      if (this.grid[row][col] && this.grid[row][col].falling) {
        if (this.grid[row][col - 1] === null) {
          const block = this.grid[row][col];
          this.grid[row][col - 1] = block;
          this.grid[row][col] = null;
          return true;
        }
        return false;
      }
    }

    return false;
  }

  moveBlockRight(col) {
    if (col === GRID_WIDTH - 1) return false;

    for (let row = 0; row < GRID_HEIGHT; row++) {
      if (this.grid[row][col] && this.grid[row][col].falling) {
        if (this.grid[row][col + 1] === null) {
          const block = this.grid[row][col];
          this.grid[row][col + 1] = block;
          this.grid[row][col] = null;
          return true;
        }
        return false;
      }
    }

    return false;
  }

  applyGravity() {
    // Normal gravity: blocks fall down (positive direction)
    // Reversed gravity: blocks fall up (negative direction)
    const direction = this.gravityReversed ? -1 : 1;

    let moved = true;
    while (moved) {
      moved = false;

      // Iterate from the direction blocks are moving AWAY FROM
      // If normal gravity: iterate from bottom up (blocks fall down, so process bottom first)
      // If reversed: iterate from top down (blocks fall up, so process top first)
      if (this.gravityReversed) {
        // Gravity reversed: process from top to bottom
        for (let row = 0; row < GRID_HEIGHT; row++) {
          for (let col = 0; col < GRID_WIDTH; col++) {
            const block = this.grid[row][col];

            if (block && !block.falling) {
              const nextRow = row + direction; // direction is -1, so nextRow = row - 1

              if (nextRow < 0 || nextRow >= GRID_HEIGHT) {
                continue;
              }

              if (this.grid[nextRow][col] === null) {
                this.grid[nextRow][col] = block;
                this.grid[row][col] = null;
                moved = true;
              }
            }
          }
        }
      } else {
        // Normal gravity: process from bottom to top
        for (let row = GRID_HEIGHT - 1; row >= 0; row--) {
          for (let col = 0; col < GRID_WIDTH; col++) {
            const block = this.grid[row][col];

            if (block && !block.falling) {
              const nextRow = row + direction; // direction is 1, so nextRow = row + 1

              if (nextRow < 0 || nextRow >= GRID_HEIGHT) {
                continue;
              }

              if (this.grid[nextRow][col] === null) {
                this.grid[nextRow][col] = block;
                this.grid[row][col] = null;
                moved = true;
              }
            }
          }
        }
      }
    }
  }

  findMatches() {
    const matched = Array(GRID_HEIGHT).fill(null).map(() => Array(GRID_WIDTH).fill(false));

    for (let row = 0; row < GRID_HEIGHT; row++) {
      for (let col = 0; col < GRID_WIDTH; col++) {
        if (!this.grid[row][col] || this.grid[row][col].color === COLORS.EMPTY) {
          continue;
        }

        const color = this.grid[row][col].color;

        for (const direction of MATCH_DIRECTIONS) {
          const matches = this.getMatchesInDirection(row, col, color, direction);

          if (matches.length >= MIN_MATCH) {
            matches.forEach(([r, c]) => {
              matched[r][c] = true;
            });
          }
        }
      }
    }

    return matched;
  }

  getMatchesInDirection(startRow, startCol, color, direction) {
    const matches = [[startRow, startCol]];

    // Forward
    let row = startRow + direction.dy;
    let col = startCol + direction.dx;

    while (row >= 0 && row < GRID_HEIGHT && col >= 0 && col < GRID_WIDTH) {
      const block = this.grid[row][col];
      if (block && block.color === color) {
        matches.push([row, col]);
        row += direction.dy;
        col += direction.dx;
      } else {
        break;
      }
    }

    // Backward
    row = startRow - direction.dy;
    col = startCol - direction.dx;

    while (row >= 0 && row < GRID_HEIGHT && col >= 0 && col < GRID_WIDTH) {
      const block = this.grid[row][col];
      if (block && block.color === color) {
        matches.unshift([row, col]);
        row -= direction.dy;
        col -= direction.dx;
      } else {
        break;
      }
    }

    return matches;
  }

  clearMatches(matched) {
    let blockCount = 0;

    for (let row = 0; row < GRID_HEIGHT; row++) {
      for (let col = 0; col < GRID_WIDTH; col++) {
        if (matched[row][col]) {
          const block = this.grid[row][col];

          if (block.type === BLOCK_TYPES.BOMB) {
            this.explodeBomb(row, col);
          } else if (block.type === BLOCK_TYPES.GRAVITY_REVERSER) {
            this.toggleGravity();
          }

          this.grid[row][col] = null;
          blockCount++;

          this.particleEffects.push({
            row,
            col,
            age: 0,
            duration: 0.5
          });
        }
      }
    }

    const pointsGained = blockCount * 10 * this.chainMultiplier;
    this.score += pointsGained;

    if (blockCount > 0) {
      this.chainMultiplier++;
    }

    return blockCount;
  }

  explodeBomb(row, col) {
    const radius = 2;

    for (let r = row - radius; r <= row + radius; r++) {
      for (let c = col - radius; c <= col + radius; c++) {
        if (r >= 0 && r < GRID_HEIGHT && c >= 0 && c < GRID_WIDTH) {
          this.grid[r][c] = null;
        }
      }
    }
  }

  toggleGravity() {
    this.gravityReversed = !this.gravityReversed;
  }

  tick(deltaTime) {
    if (this.gameOver) return;

    if (this.mode === GAME_MODES.TIME_ATTACK) {
      this.timeRemaining -= deltaTime;
      if (this.timeRemaining <= 0) {
        this.gameOver = true;
      }
    }

    this.particleEffects = this.particleEffects
      .map(p => ({ ...p, age: p.age + deltaTime }))
      .filter(p => p.age < p.duration);

    if (this.particleEffects.length === 0) {
      this.chainMultiplier = 1;
    }
  }

  async stabilizeGrid() {
    let iterations = 0;
    const maxIterations = 100;

    while (iterations < maxIterations) {
      this.applyGravity();

      const matched = this.findMatches();
      const matchCount = matched.flat().filter(m => m).length;

      if (matchCount === 0) {
        break;
      }

      this.clearMatches(matched);
      iterations++;

      await new Promise(resolve => setTimeout(resolve, 50));
    }
  }

  getGameState() {
    return {
      grid: this.grid,
      score: this.score,
      level: this.level,
      speed: this.speed,
      mode: this.mode,
      timeRemaining: this.timeRemaining,
      gameOver: this.gameOver,
      nextBlockColor: this.nextBlockColor,
      chainMultiplier: this.chainMultiplier,
      gravityReversed: this.gravityReversed,
      particleEffects: this.particleEffects
    };
  }

  reset() {
    this.grid = this.createEmptyGrid();
    this.score = 0;
    this.level = 1;
    this.speed = 1;
    this.gameOver = false;
    this.nextBlockColor = this.getRandomColor();
    this.chainMultiplier = 1;
    this.gravityReversed = false;
    this.particleEffects = [];

    if (this.mode === GAME_MODES.TIME_ATTACK) {
      this.timeRemaining = 180;
    }
  }
}

/**
 * Game Controller - Game loop and input handling
 */
class GameController {
  constructor(mode = GAME_MODES.ZEN) {
    this.engine = new GameEngine(mode);
    this.lastBlockSpawnTime = 0;
    this.blockFallInterval = 1000;
    this.currentFallingCol = Math.floor(GRID_WIDTH / 2);
    this.isRunning = false;
    this.isPaused = false;
    this.animationFrameId = null;
  }

  start() {
    this.isRunning = true;
    this.isPaused = false;
    this.engine.reset();
    this.engine.spawnNewBlock();
    this.gameLoop(performance.now());
  }

  pause() {
    this.isPaused = true;
  }

  resume() {
    this.isPaused = false;
  }

  stop() {
    this.isRunning = false;
    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId);
    }
  }

  gameLoop(timestamp) {
    if (!this.isRunning) return;

    const deltaTime = timestamp - (this.lastTimestamp || timestamp);
    this.lastTimestamp = timestamp;

    if (!this.isPaused) {
      this.engine.tick(deltaTime / 1000);

      this.lastBlockSpawnTime += deltaTime;

      const fallIntervalMs = 1000 / this.engine.speed;
      if (this.lastBlockSpawnTime > fallIntervalMs) {
        const moved = this.engine.moveBlockDown(this.currentFallingCol);
        this.lastBlockSpawnTime = 0;

        if (!moved) {
          this.engine.stabilizeGrid().then(() => {
            if (!this.engine.gameOver) {
              this.engine.spawnNewBlock();
            }
          });
        }
      }
    }

    let hasFallingBlock = false;
    for (let row = 0; row < GRID_HEIGHT; row++) {
      for (let col = 0; col < GRID_WIDTH; col++) {
        if (this.engine.grid[row][col]?.falling) {
          hasFallingBlock = true;
          break;
        }
      }
    }

    if (!hasFallingBlock && !this.engine.gameOver && this.lastBlockSpawnTime === 0) {
      this.engine.spawnNewBlock();
    }

    this.animationFrameId = requestAnimationFrame((t) => this.gameLoop(t));
  }

  moveLeft() {
    if (this.currentFallingCol > 0) {
      this.engine.moveBlockLeft(this.currentFallingCol);
      this.currentFallingCol--;
    }
  }

  moveRight() {
    if (this.currentFallingCol < GRID_WIDTH - 1) {
      this.engine.moveBlockRight(this.currentFallingCol);
      this.currentFallingCol++;
    }
  }

  speedUp() {
    this.engine.speed = Math.min(this.engine.speed + 1, 5);
  }

  getGameState() {
    return this.engine.getGameState();
  }

  isGameOver() {
    return this.engine.gameOver;
  }
}

/**
 * Renderer - Canvas rendering
 */
class Renderer {
  constructor(canvasId) {
    this.canvas = document.getElementById(canvasId);
    this.ctx = this.canvas.getContext('2d');

    this.canvas.width = GRID_WIDTH * BLOCK_SIZE;
    this.canvas.height = GRID_HEIGHT * BLOCK_SIZE;

    this.colorMap = {
      [COLORS.RED]: '#FF4444',
      [COLORS.BLUE]: '#4444FF',
      [COLORS.GREEN]: '#44FF44',
      [COLORS.YELLOW]: '#FFFF44',
      [COLORS.PURPLE]: '#FF44FF',
      [COLORS.ORANGE]: '#FF8844',
      [COLORS.EMPTY]: '#222222'
    };
  }

  render(gameState) {
    this.ctx.fillStyle = '#000000';
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

    this.drawGridBackground();
    this.drawBlocks(gameState.grid, gameState.particleEffects);
    this.drawUI(gameState);
  }

  drawGridBackground() {
    this.ctx.strokeStyle = '#333333';
    this.ctx.lineWidth = 1;

    for (let col = 0; col <= GRID_WIDTH; col++) {
      this.ctx.beginPath();
      this.ctx.moveTo(col * BLOCK_SIZE, 0);
      this.ctx.lineTo(col * BLOCK_SIZE, this.canvas.height);
      this.ctx.stroke();
    }

    for (let row = 0; row <= GRID_HEIGHT; row++) {
      this.ctx.beginPath();
      this.ctx.moveTo(0, row * BLOCK_SIZE);
      this.ctx.lineTo(this.canvas.width, row * BLOCK_SIZE);
      this.ctx.stroke();
    }
  }

  drawBlocks(grid, particleEffects) {
    for (let row = 0; row < GRID_HEIGHT; row++) {
      for (let col = 0; col < GRID_WIDTH; col++) {
        const block = grid[row][col];

        if (block) {
          this.drawBlock(block, col, row);
        }
      }
    }

    for (const particle of particleEffects) {
      this.drawParticle(particle);
    }
  }

  drawBlock(block, col, row) {
    const x = col * BLOCK_SIZE;
    const y = row * BLOCK_SIZE;
    const padding = 2;

    this.ctx.fillStyle = this.colorMap[block.color] || '#FFFFFF';
    this.ctx.fillRect(x + padding, y + padding, BLOCK_SIZE - 2 * padding, BLOCK_SIZE - 2 * padding);

    this.ctx.strokeStyle = '#FFFFFF';
    this.ctx.lineWidth = 2;
    this.ctx.strokeRect(x + padding, y + padding, BLOCK_SIZE - 2 * padding, BLOCK_SIZE - 2 * padding);

    if (block.type === 'bomb') {
      this.drawBombIndicator(x, y);
    }

    if (block.falling) {
      this.ctx.strokeStyle = '#FFFF00';
      this.ctx.lineWidth = 3;
      this.ctx.strokeRect(x + 1, y + 1, BLOCK_SIZE - 2, BLOCK_SIZE - 2);
    }
  }

  drawBombIndicator(x, y) {
    const centerX = x + BLOCK_SIZE / 2;
    const centerY = y + BLOCK_SIZE / 2;

    this.ctx.fillStyle = '#000000';
    this.ctx.beginPath();
    this.ctx.arc(centerX, centerY, 4, 0, Math.PI * 2);
    this.ctx.fill();

    this.ctx.strokeStyle = '#FF8844';
    this.ctx.lineWidth = 1;
    this.ctx.beginPath();
    this.ctx.moveTo(centerX, centerY - 4);
    this.ctx.lineTo(centerX, centerY - 8);
    this.ctx.stroke();
  }

  drawParticle(particle) {
    const x = (particle.col + 0.5) * BLOCK_SIZE;
    const y = (particle.row + 0.5) * BLOCK_SIZE;
    const progress = particle.age / particle.duration;
    const opacity = 1 - progress;
    const size = 8 * (1 - progress);

    this.ctx.fillStyle = `rgba(255, 255, 100, ${opacity})`;
    this.ctx.beginPath();
    this.ctx.arc(x, y, size, 0, Math.PI * 2);
    this.ctx.fill();
  }

  drawUI(gameState) {
    const uiX = this.canvas.width + 20;
    const uiY = 20;
    const lineHeight = 25;

    this.ctx.fillStyle = '#FFFFFF';
    this.ctx.font = 'bold 16px Arial';

    let y = uiY;

    this.ctx.fillText(`Score: ${gameState.score}`, uiX, y);
    y += lineHeight;

    this.ctx.fillText(`Level: ${gameState.level}`, uiX, y);
    y += lineHeight;

    this.ctx.fillText(`Speed: ${gameState.speed.toFixed(1)}x`, uiX, y);
    y += lineHeight;

    if (gameState.chainMultiplier > 1) {
      this.ctx.fillStyle = '#FFFF44';
      this.ctx.fillText(`Chain: x${gameState.chainMultiplier}`, uiX, y);
      this.ctx.fillStyle = '#FFFFFF';
    }
    y += lineHeight;

    if (gameState.mode === 'time-attack') {
      const minutes = Math.floor(gameState.timeRemaining / 60);
      const seconds = Math.floor(gameState.timeRemaining % 60);
      this.ctx.fillText(`Time: ${minutes}:${seconds.toString().padStart(2, '0')}`, uiX, y);
    }

    if (gameState.gameOver) {
      this.drawGameOverScreen();
    }
  }

  drawGameOverScreen() {
    this.ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

    this.ctx.fillStyle = '#FF4444';
    this.ctx.font = 'bold 48px Arial';
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'middle';
    this.ctx.fillText('GAME OVER', this.canvas.width / 2, this.canvas.height / 2 - 30);

    this.ctx.fillStyle = '#FFFFFF';
    this.ctx.font = '20px Arial';
    this.ctx.fillText('Press SPACE to restart', this.canvas.width / 2, this.canvas.height / 2 + 30);
  }
}

// Export for ES modules
export { GameEngine, GameController, Renderer, COLORS, BLOCK_TYPES, GAME_MODES, GRID_WIDTH, GRID_HEIGHT };
