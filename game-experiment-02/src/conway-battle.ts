/**
 * Conway's Battle Arena - A competitive twist on Conway's Game of Life
 * Two players battle for cellular dominance through strategic placement and emergent gameplay
 */

// ============================================================================
// TYPES & ENUMS
// ============================================================================

export enum PlayerColor {
  Empty = 0,
  Player1 = 1,
  Player2 = 2,
}

export enum GamePhase {
  SETUP = 'SETUP',
  DEPLOYMENT = 'DEPLOYMENT',
  BATTLE = 'BATTLE',
  GAME_OVER = 'GAME_OVER',
}

export enum GameResult {
  PLAYER1_WINS = 'PLAYER1_WINS',
  PLAYER2_WINS = 'PLAYER2_WINS',
  DRAW = 'DRAW',
  IN_PROGRESS = 'IN_PROGRESS',
}

export interface Position {
  x: number;
  y: number;
}

export interface GameStats {
  player1Cells: number;
  player2Cells: number;
  emptyCells: number;
  generation: number;
  player1CellsCreated: number;
  player2CellsCreated: number;
  cellsBorn: number;
  cellsDied: number;
}

// ============================================================================
// CELL CLASS
// ============================================================================

export class Cell {
  constructor(public x: number, public y: number, public owner: PlayerColor = PlayerColor.Empty) {}

  isAlive(): boolean {
    return this.owner !== PlayerColor.Empty;
  }

  setOwner(owner: PlayerColor): void {
    this.owner = owner;
  }

  reset(): void {
    this.owner = PlayerColor.Empty;
  }

  clone(): Cell {
    return new Cell(this.x, this.y, this.owner);
  }
}

// ============================================================================
// GAME BOARD CLASS
// ============================================================================

export class GameBoard {
  private grid: Cell[][];
  private width: number;
  private height: number;
  private stats: GameStats;

  constructor(width: number = 40, height: number = 40) {
    this.width = width;
    this.height = height;
    this.grid = [];
    this.stats = {
      player1Cells: 0,
      player2Cells: 0,
      emptyCells: width * height,
      generation: 0,
      player1CellsCreated: 0,
      player2CellsCreated: 0,
      cellsBorn: 0,
      cellsDied: 0,
    };

    this.initializeGrid();
  }

  private initializeGrid(): void {
    for (let y = 0; y < this.height; y++) {
      this.grid[y] = [];
      for (let x = 0; x < this.width; x++) {
        this.grid[y][x] = new Cell(x, y, PlayerColor.Empty);
      }
    }
  }

  getWidth(): number {
    return this.width;
  }

  getHeight(): number {
    return this.height;
  }

  isInBounds(pos: Position): boolean {
    return pos.x >= 0 && pos.x < this.width && pos.y >= 0 && pos.y < this.height;
  }

  getCell(pos: Position): Cell | null {
    if (!this.isInBounds(pos)) return null;
    return this.grid[pos.y][pos.x];
  }

  setCell(pos: Position, owner: PlayerColor): boolean {
    if (!this.isInBounds(pos)) return false;
    const cell = this.grid[pos.y][pos.x];

    const oldOwner = cell.owner;
    cell.setOwner(owner);

    // Update stats
    if (oldOwner === PlayerColor.Empty && owner !== PlayerColor.Empty) {
      this.stats.emptyCells--;
      if (owner === PlayerColor.Player1) {
        this.stats.player1Cells++;
        this.stats.player1CellsCreated++;
      } else {
        this.stats.player2Cells++;
        this.stats.player2CellsCreated++;
      }
    } else if (oldOwner !== PlayerColor.Empty && owner === PlayerColor.Empty) {
      this.stats.emptyCells++;
      if (oldOwner === PlayerColor.Player1) {
        this.stats.player1Cells--;
      } else {
        this.stats.player2Cells--;
      }
    } else if (oldOwner !== PlayerColor.Empty && owner !== PlayerColor.Empty && oldOwner !== owner) {
      // Owner changed
      if (oldOwner === PlayerColor.Player1) {
        this.stats.player1Cells--;
        this.stats.player2Cells++;
      } else {
        this.stats.player2Cells--;
        this.stats.player1Cells++;
      }
    }

    return true;
  }

  getAllCells(): Cell[] {
    const cells: Cell[] = [];
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        cells.push(this.grid[y][x]);
      }
    }
    return cells;
  }

  getStats(): GameStats {
    return { ...this.stats };
  }

  incrementGeneration(): void {
    this.stats.generation++;
  }

  getGeneration(): number {
    return this.stats.generation;
  }

  clear(): void {
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        this.grid[y][x].reset();
      }
    }
    this.stats = {
      player1Cells: 0,
      player2Cells: 0,
      emptyCells: this.width * this.height,
      generation: 0,
      player1CellsCreated: 0,
      player2CellsCreated: 0,
      cellsBorn: 0,
      cellsDied: 0,
    };
  }

  private countNeighbors(pos: Position): { total: number; player1: number; player2: number } {
    let total = 0;
    let player1 = 0;
    let player2 = 0;

    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        if (dx === 0 && dy === 0) continue;
        const neighborPos = { x: pos.x + dx, y: pos.y + dy };
        if (this.isInBounds(neighborPos)) {
          const neighbor = this.grid[neighborPos.y][neighborPos.x];
          if (neighbor.isAlive()) {
            total++;
            if (neighbor.owner === PlayerColor.Player1) {
              player1++;
            } else {
              player2++;
            }
          }
        }
      }
    }

    return { total, player1, player2 };
  }

  evolveGeneration(): void {
    const newGrid: Cell[][] = [];
    let cellsBorn = 0;
    let cellsDied = 0;

    // Create new grid state
    for (let y = 0; y < this.height; y++) {
      newGrid[y] = [];
      for (let x = 0; x < this.width; x++) {
        const cell = this.grid[y][x];
        const pos = { x, y };
        const neighbors = this.countNeighbors(pos);

        let newOwner = PlayerColor.Empty;

        if (cell.isAlive()) {
          // Cell is alive: survives with 2-3 neighbors
          if (neighbors.total === 2 || neighbors.total === 3) {
            newOwner = cell.owner;
          } else {
            cellsDied++;
          }
        } else {
          // Cell is dead: births with exactly 3 neighbors
          if (neighbors.total === 3) {
            // Majority rule: most neighbors' color wins
            if (neighbors.player1 > neighbors.player2) {
              newOwner = PlayerColor.Player1;
            } else if (neighbors.player2 > neighbors.player1) {
              newOwner = PlayerColor.Player2;
            } else {
              // Tie: no cell born
              newOwner = PlayerColor.Empty;
            }

            if (newOwner !== PlayerColor.Empty) {
              cellsBorn++;
            }
          }
        }

        newGrid[y][x] = new Cell(x, y, newOwner);
      }
    }

    // Apply new grid
    this.grid = newGrid;
    this.stats.cellsBorn += cellsBorn;
    this.stats.cellsDied += cellsDied;

    // Recalculate stats
    let player1Count = 0;
    let player2Count = 0;
    let emptyCount = 0;

    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        const owner = this.grid[y][x].owner;
        if (owner === PlayerColor.Player1) {
          player1Count++;
        } else if (owner === PlayerColor.Player2) {
          player2Count++;
        } else {
          emptyCount++;
        }
      }
    }

    this.stats.player1Cells = player1Count;
    this.stats.player2Cells = player2Count;
    this.stats.emptyCells = emptyCount;
    this.incrementGeneration();
  }
}

// ============================================================================
// DEPLOYMENT MANAGER
// ============================================================================

export class DeploymentManager {
  private cellsPlaced: { [key in PlayerColor]?: number } = {};
  private maxCellsPerPlayer: number;
  private currentPlayerTurn: PlayerColor;

  constructor(maxCellsPerPlayer: number = 30) {
    this.maxCellsPerPlayer = maxCellsPerPlayer;
    this.cellsPlaced[PlayerColor.Player1] = 0;
    this.cellsPlaced[PlayerColor.Player2] = 0;
    this.currentPlayerTurn = PlayerColor.Player1;
  }

  getCurrentPlayer(): PlayerColor {
    return this.currentPlayerTurn;
  }

  getCurrentPlayerNumber(): number {
    return this.currentPlayerTurn === PlayerColor.Player1 ? 1 : 2;
  }

  canPlaceCell(player: PlayerColor): boolean {
    return (this.cellsPlaced[player] || 0) < this.maxCellsPerPlayer;
  }

  placeCell(player: PlayerColor): boolean {
    if (player !== this.currentPlayerTurn) {
      return false; // Not this player's turn
    }

    if (!this.canPlaceCell(player)) {
      return false; // Player has placed max cells
    }

    this.cellsPlaced[player] = (this.cellsPlaced[player] || 0) + 1;
    return true;
  }

  switchTurn(): void {
    this.currentPlayerTurn = this.currentPlayerTurn === PlayerColor.Player1 ? PlayerColor.Player2 : PlayerColor.Player1;
  }

  getCellsPlaced(player: PlayerColor): number {
    return this.cellsPlaced[player] || 0;
  }

  getCellsRemaining(player: PlayerColor): number {
    return this.maxCellsPerPlayer - (this.cellsPlaced[player] || 0);
  }

  isDeploymentComplete(): boolean {
    return (
      (this.cellsPlaced[PlayerColor.Player1] || 0) === this.maxCellsPerPlayer &&
      (this.cellsPlaced[PlayerColor.Player2] || 0) === this.maxCellsPerPlayer
    );
  }

  reset(): void {
    this.cellsPlaced = {
      [PlayerColor.Player1]: 0,
      [PlayerColor.Player2]: 0,
    };
    this.currentPlayerTurn = PlayerColor.Player1;
  }
}

// ============================================================================
// VICTORY CONDITION CHECKER
// ============================================================================

export class VictoryChecker {
  private readonly dominationThreshold = 0.7; // 70% of all cells
  private readonly maxTurns = 500;

  checkVictory(board: GameBoard, generation: number): GameResult {
    const stats = board.getStats();
    const totalAlive = stats.player1Cells + stats.player2Cells;

    // Check if a player has no cells (eliminated)
    if (stats.player1Cells === 0 && stats.player2Cells > 0) {
      return GameResult.PLAYER2_WINS;
    }
    if (stats.player2Cells === 0 && stats.player1Cells > 0) {
      return GameResult.PLAYER1_WINS;
    }

    // Check domination victory (70% of alive cells)
    if (totalAlive > 0) {
      const p1Dominance = stats.player1Cells / totalAlive;
      const p2Dominance = stats.player2Cells / totalAlive;

      if (p1Dominance >= this.dominationThreshold) {
        return GameResult.PLAYER1_WINS;
      }
      if (p2Dominance >= this.dominationThreshold) {
        return GameResult.PLAYER2_WINS;
      }
    }

    // Check max turns reached
    if (generation >= this.maxTurns) {
      if (stats.player1Cells > stats.player2Cells) {
        return GameResult.PLAYER1_WINS;
      } else if (stats.player2Cells > stats.player1Cells) {
        return GameResult.PLAYER2_WINS;
      } else {
        return GameResult.DRAW;
      }
    }

    return GameResult.IN_PROGRESS;
  }

  isGameOver(result: GameResult): boolean {
    return result !== GameResult.IN_PROGRESS;
  }
}

// ============================================================================
// AI PLAYER
// ============================================================================

export enum AIDifficulty {
  EASY = 'EASY',
  NORMAL = 'NORMAL',
  HARD = 'HARD',
}

export class AIPlayer {
  private difficulty: AIDifficulty;
  private seed: number;

  constructor(difficulty: AIDifficulty = AIDifficulty.NORMAL, seed: number = Date.now()) {
    this.difficulty = difficulty;
    this.seed = seed;
  }

  getRandomPosition(board: GameBoard): Position | null {
    let attempts = 0;
    const maxAttempts = 100;

    while (attempts < maxAttempts) {
      const x = this.seededRandom() * board.getWidth() | 0;
      const y = this.seededRandom() * board.getHeight() | 0;
      const pos = { x, y };

      const cell = board.getCell(pos);
      if (cell && !cell.isAlive()) {
        return pos;
      }
      attempts++;
    }

    return null;
  }

  getStrategicPosition(board: GameBoard, playerColor: PlayerColor): Position | null {
    switch (this.difficulty) {
      case AIDifficulty.EASY:
        return this.getRandomPosition(board);

      case AIDifficulty.NORMAL:
        return this.getNormalStrategy(board, playerColor);

      case AIDifficulty.HARD:
        return this.getHardStrategy(board, playerColor);

      default:
        return this.getRandomPosition(board);
    }
  }

  private getNormalStrategy(board: GameBoard, playerColor: PlayerColor): Position | null {
    // Normal strategy: try to cluster near other cells
    const cells = board.getAllCells().filter((c) => c.owner === playerColor);

    if (cells.length === 0) {
      // No cells yet, start from edge
      return this.getEdgePosition(board);
    }

    // Try to place near existing cells
    const center = cells[this.seededRandom() * cells.length | 0];
    const offsets = [
      { x: 1, y: 0 },
      { x: -1, y: 0 },
      { x: 0, y: 1 },
      { x: 0, y: -1 },
      { x: 1, y: 1 },
      { x: -1, y: -1 },
    ];

    for (const offset of offsets) {
      const pos = { x: center.x + offset.x, y: center.y + offset.y };
      const cell = board.getCell(pos);
      if (cell && !cell.isAlive()) {
        return pos;
      }
    }

    return this.getRandomPosition(board);
  }

  private getHardStrategy(board: GameBoard, playerColor: PlayerColor): Position | null {
    // Hard strategy: control edges and create patterns
    const cells = board.getAllCells().filter((c) => c.owner === playerColor);

    if (cells.length < 5) {
      return this.getEdgePosition(board);
    }

    // Try to create stable patterns or control center
    const centerX = board.getWidth() / 2;
    const centerY = board.getHeight() / 2;

    // Look for position close to center
    let bestPos: Position | null = null;
    let bestDistance = Infinity;

    for (let attempts = 0; attempts < 20; attempts++) {
      const pos = this.getRandomPosition(board);
      if (!pos) continue;

      const distance = Math.sqrt(Math.pow(pos.x - centerX, 2) + Math.pow(pos.y - centerY, 2));
      if (distance < bestDistance) {
        bestDistance = distance;
        bestPos = pos;
      }
    }

    return bestPos || this.getRandomPosition(board);
  }

  private getEdgePosition(board: GameBoard): Position | null {
    const edge = this.seededRandom() * 4 | 0;
    let pos: Position;

    switch (edge) {
      case 0: // Top edge
        pos = { x: this.seededRandom() * board.getWidth() | 0, y: 0 };
        break;
      case 1: // Bottom edge
        pos = { x: this.seededRandom() * board.getWidth() | 0, y: board.getHeight() - 1 };
        break;
      case 2: // Left edge
        pos = { x: 0, y: this.seededRandom() * board.getHeight() | 0 };
        break;
      case 3: // Right edge
        pos = { x: board.getWidth() - 1, y: this.seededRandom() * board.getHeight() | 0 };
        break;
      default:
        return null;
    }

    const cell = board.getCell(pos);
    if (cell && !cell.isAlive()) {
      return pos;
    }

    return this.getRandomPosition(board);
  }

  private seededRandom(): number {
    this.seed = (this.seed * 9301 + 49297) % 233280;
    return this.seed / 233280;
  }
}

// ============================================================================
// MAIN GAME ENGINE
// ============================================================================

export class ConwayBattleGame {
  private board: GameBoard;
  private deploymentManager: DeploymentManager;
  private victoryChecker: VictoryChecker;
  private aiPlayer: AIPlayer | null;
  private gamePhase: GamePhase;
  private gameResult: GameResult;
  private frameCount: number;
  private battleFrameCounter: number;
  private battleFrameInterval: number; // Evolve every N frames

  constructor(
    aiPlayer: AIPlayer | null = null,
    gridWidth: number = 40,
    gridHeight: number = 40,
    cellsPerPlayer: number = 30
  ) {
    this.board = new GameBoard(gridWidth, gridHeight);
    this.deploymentManager = new DeploymentManager(cellsPerPlayer);
    this.victoryChecker = new VictoryChecker();
    this.aiPlayer = aiPlayer;
    this.gamePhase = GamePhase.SETUP;
    this.gameResult = GameResult.IN_PROGRESS;
    this.frameCount = 0;
    this.battleFrameCounter = 0;
    this.battleFrameInterval = 30; // Evolve every 30 frames (~2 per second at 60 FPS)
  }

  startDeployment(): void {
    this.gamePhase = GamePhase.DEPLOYMENT;
    this.deploymentManager.reset();
  }

  getCurrentPhase(): GamePhase {
    return this.gamePhase;
  }

  startBattle(): void {
    if (this.gamePhase === GamePhase.DEPLOYMENT) {
      this.gamePhase = GamePhase.BATTLE;
      this.frameCount = 0;
    }
  }

  placeCell(pos: Position, player: PlayerColor): boolean {
    if (this.gamePhase !== GamePhase.DEPLOYMENT) {
      return false;
    }

    // Check if it's this player's turn
    if (this.deploymentManager.getCurrentPlayer() !== player) {
      return false;
    }

    // Check if position is valid
    const cell = this.board.getCell(pos);
    if (!cell || cell.isAlive()) {
      return false;
    }

    // Place cell
    this.board.setCell(pos, player);

    // Record placement
    if (!this.deploymentManager.placeCell(player)) {
      return false;
    }

    // Switch turn
    this.deploymentManager.switchTurn();

    return true;
  }

  aiTakeTurn(): boolean {
    if (!this.aiPlayer || this.gamePhase !== GamePhase.DEPLOYMENT) {
      return false;
    }

    const currentPlayer = this.deploymentManager.getCurrentPlayer();
    if (this.deploymentManager.getCellsRemaining(currentPlayer) <= 0) {
      return false;
    }

    const pos = this.aiPlayer.getStrategicPosition(this.board, currentPlayer);
    if (!pos) {
      return false;
    }

    return this.placeCell(pos, currentPlayer);
  }

  update(): void {
    this.frameCount++;

    if (this.gamePhase === GamePhase.BATTLE) {
      this.battleFrameCounter++;

      // Evolve game every N frames
      if (this.battleFrameCounter >= this.battleFrameInterval) {
        this.battleFrameCounter = 0;
        this.board.evolveGeneration();

        // Check victory conditions
        this.gameResult = this.victoryChecker.checkVictory(this.board, this.board.getGeneration());

        if (this.victoryChecker.isGameOver(this.gameResult)) {
          this.gamePhase = GamePhase.GAME_OVER;
        }
      }
    }
  }

  getBoard(): GameBoard {
    return this.board;
  }

  getGameResult(): GameResult {
    return this.gameResult;
  }

  getDeploymentManager(): DeploymentManager {
    return this.deploymentManager;
  }

  isDeploymentComplete(): boolean {
    return this.deploymentManager.isDeploymentComplete();
  }

  getStats(): GameStats {
    return this.board.getStats();
  }

  reset(): void {
    this.board.clear();
    this.deploymentManager.reset();
    this.gamePhase = GamePhase.SETUP;
    this.gameResult = GameResult.IN_PROGRESS;
    this.frameCount = 0;
    this.battleFrameCounter = 0;
  }
}
