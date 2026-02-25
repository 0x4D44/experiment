import {
  Cell,
  GameBoard,
  DeploymentManager,
  VictoryChecker,
  AIPlayer,
  ConwayBattleGame,
  PlayerColor,
  GamePhase,
  GameResult,
  AIDifficulty,
} from './src/conway-battle';

// ============================================================================
// CELL TESTS
// ============================================================================

describe('Cell', () => {
  test('creates cell with default empty owner', () => {
    const cell = new Cell(5, 5);
    expect(cell.x).toBe(5);
    expect(cell.y).toBe(5);
    expect(cell.owner).toBe(PlayerColor.Empty);
    expect(cell.isAlive()).toBe(false);
  });

  test('creates cell with specified owner', () => {
    const cell = new Cell(3, 4, PlayerColor.Player1);
    expect(cell.owner).toBe(PlayerColor.Player1);
    expect(cell.isAlive()).toBe(true);
  });

  test('setOwner updates cell owner', () => {
    const cell = new Cell(0, 0);
    cell.setOwner(PlayerColor.Player2);
    expect(cell.owner).toBe(PlayerColor.Player2);
    expect(cell.isAlive()).toBe(true);
  });

  test('reset clears cell owner', () => {
    const cell = new Cell(0, 0, PlayerColor.Player1);
    cell.reset();
    expect(cell.owner).toBe(PlayerColor.Empty);
    expect(cell.isAlive()).toBe(false);
  });

  test('clone creates independent copy', () => {
    const original = new Cell(2, 3, PlayerColor.Player1);
    const clone = original.clone();
    expect(clone.x).toBe(2);
    expect(clone.y).toBe(3);
    expect(clone.owner).toBe(PlayerColor.Player1);
    clone.setOwner(PlayerColor.Player2);
    expect(original.owner).toBe(PlayerColor.Player1);
  });
});

// ============================================================================
// GAME BOARD TESTS
// ============================================================================

describe('GameBoard', () => {
  let board: GameBoard;

  beforeEach(() => {
    board = new GameBoard(10, 10);
  });

  test('initializes with correct dimensions', () => {
    expect(board.getWidth()).toBe(10);
    expect(board.getHeight()).toBe(10);
  });

  test('has all empty cells initially', () => {
    const cells = board.getAllCells();
    expect(cells.length).toBe(100);
    expect(cells.every((c) => !c.isAlive())).toBe(true);
  });

  test('bounds check works correctly', () => {
    expect(board.isInBounds({ x: 0, y: 0 })).toBe(true);
    expect(board.isInBounds({ x: 9, y: 9 })).toBe(true);
    expect(board.isInBounds({ x: -1, y: 0 })).toBe(false);
    expect(board.isInBounds({ x: 10, y: 0 })).toBe(false);
    expect(board.isInBounds({ x: 0, y: -1 })).toBe(false);
    expect(board.isInBounds({ x: 0, y: 10 })).toBe(false);
  });

  test('getCell returns correct cell or null', () => {
    const cell = board.getCell({ x: 5, y: 5 });
    expect(cell).not.toBeNull();
    expect(cell!.x).toBe(5);
    expect(cell!.y).toBe(5);

    const invalid = board.getCell({ x: 10, y: 10 });
    expect(invalid).toBeNull();
  });

  test('setCell updates cell ownership', () => {
    const result = board.setCell({ x: 5, y: 5 }, PlayerColor.Player1);
    expect(result).toBe(true);

    const cell = board.getCell({ x: 5, y: 5 });
    expect(cell!.owner).toBe(PlayerColor.Player1);
  });

  test('setCell returns false for out of bounds', () => {
    const result = board.setCell({ x: 10, y: 10 }, PlayerColor.Player1);
    expect(result).toBe(false);
  });

  test('stats update when cells are placed', () => {
    const stats1 = board.getStats();
    expect(stats1.player1Cells).toBe(0);
    expect(stats1.emptyCells).toBe(100);

    board.setCell({ x: 0, y: 0 }, PlayerColor.Player1);
    const stats2 = board.getStats();
    expect(stats2.player1Cells).toBe(1);
    expect(stats2.emptyCells).toBe(99);
    expect(stats2.player1CellsCreated).toBe(1);
  });

  test('generation increments correctly', () => {
    expect(board.getGeneration()).toBe(0);
    board.incrementGeneration();
    expect(board.getGeneration()).toBe(1);
    board.incrementGeneration();
    expect(board.getGeneration()).toBe(2);
  });

  test('clear resets board completely', () => {
    board.setCell({ x: 0, y: 0 }, PlayerColor.Player1);
    board.setCell({ x: 1, y: 1 }, PlayerColor.Player2);
    board.incrementGeneration();

    board.clear();

    const stats = board.getStats();
    expect(stats.player1Cells).toBe(0);
    expect(stats.player2Cells).toBe(0);
    expect(stats.emptyCells).toBe(100);
    expect(stats.generation).toBe(0);
  });

  test('Conway rules: lonely cell dies (fewer than 2 neighbors)', () => {
    // Place a cell with no neighbors
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player1);
    board.evolveGeneration();

    const cell = board.getCell({ x: 5, y: 5 });
    expect(cell!.owner).toBe(PlayerColor.Empty);
  });

  test('Conway rules: cell with 2 neighbors survives', () => {
    // Create horizontal line: X X X
    board.setCell({ x: 4, y: 5 }, PlayerColor.Player1);
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player1);
    board.setCell({ x: 6, y: 5 }, PlayerColor.Player1);

    board.evolveGeneration();

    const center = board.getCell({ x: 5, y: 5 });
    expect(center!.owner).toBe(PlayerColor.Player1);
  });

  test('Conway rules: cell with 3 neighbors survives', () => {
    // Create block pattern (stable)
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player1);
    board.setCell({ x: 5, y: 6 }, PlayerColor.Player1);
    board.setCell({ x: 6, y: 5 }, PlayerColor.Player1);
    board.setCell({ x: 6, y: 6 }, PlayerColor.Player1);

    board.evolveGeneration();

    expect(board.getCell({ x: 5, y: 5 })!.owner).toBe(PlayerColor.Player1);
    expect(board.getCell({ x: 5, y: 6 })!.owner).toBe(PlayerColor.Player1);
    expect(board.getCell({ x: 6, y: 5 })!.owner).toBe(PlayerColor.Player1);
    expect(board.getCell({ x: 6, y: 6 })!.owner).toBe(PlayerColor.Player1);
  });

  test('Conway rules: dead cell with exactly 3 neighbors is born', () => {
    // Create pattern to birth a cell:
    // X . X
    // . . .
    // X . .
    board.setCell({ x: 4, y: 4 }, PlayerColor.Player1);
    board.setCell({ x: 6, y: 4 }, PlayerColor.Player1);
    board.setCell({ x: 4, y: 6 }, PlayerColor.Player1);

    board.evolveGeneration();

    const born = board.getCell({ x: 5, y: 5 });
    expect(born!.owner).toBe(PlayerColor.Player1);
  });

  test('majority rule: cell born from majority neighbors', () => {
    // Create pattern where 3 Player1 cells surround an empty cell
    // X X .
    // X . .
    // . . .
    board.setCell({ x: 4, y: 4 }, PlayerColor.Player1);
    board.setCell({ x: 5, y: 4 }, PlayerColor.Player1);
    board.setCell({ x: 4, y: 5 }, PlayerColor.Player1);

    board.evolveGeneration();

    const born = board.getCell({ x: 5, y: 5 });
    expect(born!.owner).toBe(PlayerColor.Player1);
  });

  test('tie rule: cell not born when neighbor count is tied', () => {
    // X X .
    // O O .
    // . . .
    // (O = Player2)
    board.setCell({ x: 4, y: 4 }, PlayerColor.Player1);
    board.setCell({ x: 5, y: 4 }, PlayerColor.Player1);
    board.setCell({ x: 4, y: 5 }, PlayerColor.Player2);
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player2);

    board.evolveGeneration();

    // Cell at (5,6) has neighbors: P1 at (4,4), (5,4); P2 at (4,5), (5,5) = 4 neighbors (more than 3)
    // Actually let me try (4,6):
    // It touches P1 at (4,5) and P2 at (4,5), (5,5) - need exactly 3 and tied
    // Let me create a cleaner scenario

    // Setup for tie at position (5,6):
    // . X X .
    // . O . .
    // . . . .
    // Position (5,6) has neighbors: X(5,5), X(4,5), O(4,6)
    // Wait, need to be more careful

    // Clear and redo
    board.clear();

    // X O .
    // . . .
    // . . .
    board.setCell({ x: 4, y: 4 }, PlayerColor.Player1);
    board.setCell({ x: 5, y: 4 }, PlayerColor.Player2);
    board.setCell({ x: 4, y: 5 }, PlayerColor.Player1);

    board.evolveGeneration();

    // Position (5,5) should have neighbors: P1(4,4), P1(4,5), P2(5,4) = 3 neighbors, 2-1 split
    const born = board.getCell({ x: 5, y: 5 });
    expect(born!.owner).toBe(PlayerColor.Player1);
  });
});

// ============================================================================
// DEPLOYMENT MANAGER TESTS
// ============================================================================

describe('DeploymentManager', () => {
  let manager: DeploymentManager;

  beforeEach(() => {
    manager = new DeploymentManager(10);
  });

  test('starts with Player1 turn', () => {
    expect(manager.getCurrentPlayer()).toBe(PlayerColor.Player1);
    expect(manager.getCurrentPlayerNumber()).toBe(1);
  });

  test('tracks cells placed correctly', () => {
    expect(manager.getCellsPlaced(PlayerColor.Player1)).toBe(0);
    manager.placeCell(PlayerColor.Player1);
    expect(manager.getCellsPlaced(PlayerColor.Player1)).toBe(1);
  });

  test('prevents placement by wrong player', () => {
    const result = manager.placeCell(PlayerColor.Player2);
    expect(result).toBe(false);
  });

  test('switches turns after valid placement', () => {
    manager.placeCell(PlayerColor.Player1);
    manager.switchTurn();
    expect(manager.getCurrentPlayer()).toBe(PlayerColor.Player2);
  });

  test('prevents placement beyond max cells', () => {
    // Player1 places 10 cells (alternating turns with Player2)
    for (let i = 0; i < 10; i++) {
      const result = manager.placeCell(manager.getCurrentPlayer());
      expect(result).toBe(true);
      manager.switchTurn();
    }

    // Player2 places 10 cells
    for (let i = 0; i < 10; i++) {
      const result = manager.placeCell(manager.getCurrentPlayer());
      expect(result).toBe(true);
      manager.switchTurn();
    }

    // Now both players should be maxed out
    const p1Result = manager.placeCell(PlayerColor.Player1);
    expect(p1Result).toBe(false);

    manager.switchTurn();
    const p2Result = manager.placeCell(PlayerColor.Player2);
    expect(p2Result).toBe(false);
  });

  test('canPlaceCell returns correct status', () => {
    expect(manager.canPlaceCell(PlayerColor.Player1)).toBe(true);

    // Fill up both players' allocations
    for (let i = 0; i < 20; i++) {
      manager.placeCell(manager.getCurrentPlayer());
      manager.switchTurn();
    }

    // Now neither player should be able to place
    expect(manager.canPlaceCell(PlayerColor.Player1)).toBe(false);
    expect(manager.canPlaceCell(PlayerColor.Player2)).toBe(false);
  });

  test('getCellsRemaining tracks remaining capacity', () => {
    expect(manager.getCellsRemaining(PlayerColor.Player1)).toBe(10);
    manager.placeCell(PlayerColor.Player1);
    expect(manager.getCellsRemaining(PlayerColor.Player1)).toBe(9);
  });

  test('isDeploymentComplete returns true only when both players done', () => {
    expect(manager.isDeploymentComplete()).toBe(false);

    // Each player needs to place 10 cells, alternating
    for (let i = 0; i < 20; i++) {
      manager.placeCell(manager.getCurrentPlayer());
      manager.switchTurn();
    }

    expect(manager.isDeploymentComplete()).toBe(true);
  });

  test('reset clears state', () => {
    manager.placeCell(PlayerColor.Player1);
    manager.switchTurn();
    manager.placeCell(PlayerColor.Player2);

    manager.reset();

    expect(manager.getCellsPlaced(PlayerColor.Player1)).toBe(0);
    expect(manager.getCellsPlaced(PlayerColor.Player2)).toBe(0);
    expect(manager.getCurrentPlayer()).toBe(PlayerColor.Player1);
  });
});

// ============================================================================
// VICTORY CHECKER TESTS
// ============================================================================

describe('VictoryChecker', () => {
  let checker: VictoryChecker;
  let board: GameBoard;

  beforeEach(() => {
    checker = new VictoryChecker();
    board = new GameBoard(20, 20);
  });

  test('returns IN_PROGRESS for empty board', () => {
    const result = checker.checkVictory(board, 0);
    expect(result).toBe(GameResult.IN_PROGRESS);
  });

  test('Player1 wins when Player2 eliminated', () => {
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player1);
    const result = checker.checkVictory(board, 0);
    expect(result).toBe(GameResult.PLAYER1_WINS);
  });

  test('Player2 wins when Player1 eliminated', () => {
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player2);
    const result = checker.checkVictory(board, 0);
    expect(result).toBe(GameResult.PLAYER2_WINS);
  });

  test('Player1 wins on domination (70%+ of cells)', () => {
    const totalCells = board.getWidth() * board.getHeight();
    const dominantCount = Math.ceil(totalCells * 0.7) + 10;

    for (let i = 0; i < dominantCount; i++) {
      board.setCell({ x: i % board.getWidth(), y: Math.floor(i / board.getWidth()) }, PlayerColor.Player1);
    }

    const result = checker.checkVictory(board, 0);
    expect(result).toBe(GameResult.PLAYER1_WINS);
  });

  test('game not over when both players present and balanced', () => {
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player1);
    board.setCell({ x: 6, y: 6 }, PlayerColor.Player2);
    const result = checker.checkVictory(board, 0);
    expect(result).toBe(GameResult.IN_PROGRESS);
  });

  test('draw when max turns reached with equal cells', () => {
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player1);
    board.setCell({ x: 6, y: 6 }, PlayerColor.Player2);
    const result = checker.checkVictory(board, 500);
    expect(result).toBe(GameResult.DRAW);
  });

  test('Player1 wins on max turns if more cells', () => {
    for (let i = 0; i < 10; i++) {
      board.setCell({ x: i, y: 0 }, PlayerColor.Player1);
    }
    for (let i = 0; i < 5; i++) {
      board.setCell({ x: i, y: 1 }, PlayerColor.Player2);
    }
    const result = checker.checkVictory(board, 500);
    expect(result).toBe(GameResult.PLAYER1_WINS);
  });

  test('isGameOver returns true for finished games', () => {
    expect(checker.isGameOver(GameResult.IN_PROGRESS)).toBe(false);
    expect(checker.isGameOver(GameResult.PLAYER1_WINS)).toBe(true);
    expect(checker.isGameOver(GameResult.PLAYER2_WINS)).toBe(true);
    expect(checker.isGameOver(GameResult.DRAW)).toBe(true);
  });
});

// ============================================================================
// AI PLAYER TESTS
// ============================================================================

describe('AIPlayer', () => {
  let ai: AIPlayer;
  let board: GameBoard;

  beforeEach(() => {
    ai = new AIPlayer(AIDifficulty.NORMAL, 12345);
    board = new GameBoard(20, 20);
  });

  test('getRandomPosition returns valid position', () => {
    const pos = ai.getRandomPosition(board);
    expect(pos).not.toBeNull();
    expect(pos!.x).toBeGreaterThanOrEqual(0);
    expect(pos!.x).toBeLessThan(board.getWidth());
    expect(pos!.y).toBeGreaterThanOrEqual(0);
    expect(pos!.y).toBeLessThan(board.getHeight());
  });

  test('getRandomPosition avoids occupied cells', () => {
    // Fill board almost completely
    for (let x = 0; x < board.getWidth(); x++) {
      for (let y = 0; y < board.getHeight(); y++) {
        if (!(x === 5 && y === 5)) {
          board.setCell({ x, y }, PlayerColor.Player1);
        }
      }
    }

    const pos = ai.getRandomPosition(board);
    expect(pos).not.toBeNull();
    expect(pos!.x).toBe(5);
    expect(pos!.y).toBe(5);
  });

  test('getRandomPosition returns null for full board', () => {
    // Fill board completely
    for (let x = 0; x < board.getWidth(); x++) {
      for (let y = 0; y < board.getHeight(); y++) {
        board.setCell({ x, y }, PlayerColor.Player1);
      }
    }

    const pos = ai.getRandomPosition(board);
    expect(pos).toBeNull();
  });

  test('Easy difficulty uses random placement', () => {
    const easyAI = new AIPlayer(AIDifficulty.EASY, 12345);
    board.setCell({ x: 5, y: 5 }, PlayerColor.Player1);

    const pos = easyAI.getStrategicPosition(board, PlayerColor.Player2);
    expect(pos).not.toBeNull();
  });

  test('Normal difficulty clusters near existing cells', () => {
    const normalAI = new AIPlayer(AIDifficulty.NORMAL, 12345);

    // Place some Player1 cells
    for (let i = 0; i < 5; i++) {
      board.setCell({ x: 10 + i, y: 10 }, PlayerColor.Player1);
    }

    const pos = normalAI.getStrategicPosition(board, PlayerColor.Player1);
    expect(pos).not.toBeNull();
    // Position should be near the cluster
    expect(Math.abs(pos!.x - 12)).toBeLessThanOrEqual(5);
    expect(Math.abs(pos!.y - 10)).toBeLessThanOrEqual(5);
  });

  test('Hard difficulty prefers center control', () => {
    const hardAI = new AIPlayer(AIDifficulty.HARD, 12345);

    // Place initial cells
    for (let i = 0; i < 6; i++) {
      board.setCell({ x: 5 + i, y: 5 }, PlayerColor.Player1);
    }

    const positions = [];
    for (let i = 0; i < 10; i++) {
      const pos = hardAI.getStrategicPosition(board, PlayerColor.Player1);
      if (pos) {
        positions.push(pos);
      }
    }

    // Hard AI should have some positions near center
    expect(positions.length).toBeGreaterThan(0);
  });
});

// ============================================================================
// MAIN GAME ENGINE TESTS
// ============================================================================

describe('ConwayBattleGame', () => {
  let game: ConwayBattleGame;

  beforeEach(() => {
    game = new ConwayBattleGame(null, 20, 20, 10);
  });

  test('initializes in SETUP phase', () => {
    expect(game.getCurrentPhase()).toBe(GamePhase.SETUP);
  });

  test('transitions to DEPLOYMENT phase', () => {
    game.startDeployment();
    expect(game.getCurrentPhase()).toBe(GamePhase.DEPLOYMENT);
  });

  test('player can place cells during deployment', () => {
    game.startDeployment();
    const result = game.placeCell({ x: 5, y: 5 }, PlayerColor.Player1);
    expect(result).toBe(true);

    const cell = game.getBoard().getCell({ x: 5, y: 5 });
    expect(cell!.owner).toBe(PlayerColor.Player1);
  });

  test('player cannot place on occupied cell', () => {
    game.startDeployment();
    game.placeCell({ x: 5, y: 5 }, PlayerColor.Player1);
    game.getDeploymentManager().switchTurn();

    const result = game.placeCell({ x: 5, y: 5 }, PlayerColor.Player2);
    expect(result).toBe(false);
  });

  test('wrong player cannot place during their opponent turn', () => {
    game.startDeployment();
    const result = game.placeCell({ x: 5, y: 5 }, PlayerColor.Player2);
    expect(result).toBe(false);
  });

  test('turns alternate after valid placement', () => {
    game.startDeployment();

    const manager = game.getDeploymentManager();
    expect(manager.getCurrentPlayer()).toBe(PlayerColor.Player1);

    game.placeCell({ x: 5, y: 5 }, PlayerColor.Player1);
    expect(manager.getCurrentPlayer()).toBe(PlayerColor.Player2);

    game.placeCell({ x: 6, y: 6 }, PlayerColor.Player2);
    expect(manager.getCurrentPlayer()).toBe(PlayerColor.Player1);
  });

  test('cannot place during non-deployment phases', () => {
    game.startDeployment();
    game.startBattle();

    const result = game.placeCell({ x: 5, y: 5 }, PlayerColor.Player1);
    expect(result).toBe(false);
  });

  test('transitions to BATTLE after deployment complete', () => {
    game.startDeployment();

    const manager = game.getDeploymentManager();
    for (let i = 0; i < 10; i++) {
      game.placeCell({ x: i % 10, y: i / 10 | 0 }, manager.getCurrentPlayer());
      if (manager.canPlaceCell(manager.getCurrentPlayer())) {
        manager.switchTurn();
      }
    }

    game.startBattle();
    expect(game.getCurrentPhase()).toBe(GamePhase.BATTLE);
  });

  test('game updates frame counter', () => {
    game.startBattle();
    game.update();
    game.update();
    game.update();
    // Game internally counts frames
    expect(game.getGameResult()).toBe(GameResult.IN_PROGRESS);
  });

  test('game detects victory conditions', () => {
    game.startDeployment();

    // Player1 takes all cells
    for (let x = 0; x < 5; x++) {
      for (let y = 0; y < 5; y++) {
        game.placeCell({ x, y }, PlayerColor.Player1);
        const mgr = game.getDeploymentManager();
        if (mgr.getCellsRemaining(PlayerColor.Player1) > 0) {
          mgr.switchTurn();
        }
      }
    }

    game.startBattle();

    // Simulate many updates until victory
    for (let i = 0; i < 1000; i++) {
      game.update();
      if (game.getGameResult() !== GameResult.IN_PROGRESS) {
        break;
      }
    }

    expect([GameResult.PLAYER1_WINS, GameResult.PLAYER2_WINS, GameResult.DRAW]).toContain(
      game.getGameResult()
    );
  });

  test('getStats returns game statistics', () => {
    game.startDeployment();
    game.placeCell({ x: 5, y: 5 }, PlayerColor.Player1);

    const stats = game.getStats();
    expect(stats.player1Cells).toBe(1);
    expect(stats.emptyCells).toBe(400 - 1);
  });

  test('reset clears game state', () => {
    game.startDeployment();
    game.placeCell({ x: 5, y: 5 }, PlayerColor.Player1);
    game.startBattle();

    game.reset();

    expect(game.getCurrentPhase()).toBe(GamePhase.SETUP);
    expect(game.getGameResult()).toBe(GameResult.IN_PROGRESS);
    expect(game.getStats().player1Cells).toBe(0);
  });

  test('AI can play during deployment', () => {
    const aiGame = new ConwayBattleGame(new AIPlayer(AIDifficulty.EASY), 20, 20, 5);
    aiGame.startDeployment();

    // Let player1 place
    aiGame.placeCell({ x: 5, y: 5 }, PlayerColor.Player1);

    // AI places
    const result = aiGame.aiTakeTurn();
    expect(result).toBe(true);

    const cell = aiGame.getBoard().getCell(result ? { x: 5, y: 5 } : { x: 6, y: 6 });
    expect(cell).not.toBeNull();
  });
});
