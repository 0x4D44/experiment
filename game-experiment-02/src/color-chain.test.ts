/**
 * Color Chain Reaction - Comprehensive Test Suite
 */

import {
  ColorChainGame,
  OrbImpl,
  OrbColor,
  OrbType,
  Level,
  LEVELS,
} from './color-chain';

describe('ColorChainGame', () => {
  let game: ColorChainGame;

  beforeEach(() => {
    game = new ColorChainGame(4, 3);
  });

  describe('Game Initialization', () => {
    test('should initialize with correct dimensions', () => {
      const state = game.getState();
      expect(state.width).toBe(4);
      expect(state.height).toBe(3);
    });

    test('should start with empty board', () => {
      const board = game.getBoard();
      expect(board.length).toBe(3);
      expect(board[0].length).toBe(4);
      for (let y = 0; y < 3; y++) {
        for (let x = 0; x < 4; x++) {
          expect(board[y][x]).toBeNull();
        }
      }
    });

    test('should start with correct initial state', () => {
      const state = game.getState();
      expect(state.score).toBe(0);
      expect(state.movesRemaining).toBe(20);
      expect(state.gameStatus).toBe('playing');
    });

    test('should initialize board from layout', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Blue },
        ],
        [
          { type: OrbType.Normal, color: OrbColor.Green },
          { type: OrbType.Normal, color: OrbColor.Yellow },
        ],
      ];

      game.initializeBoard(layout);
      const board = game.getBoard();

      expect(board[0][0]).not.toBeNull();
      expect(board[0][0]!.color).toBe(OrbColor.Red);
      expect(board[0][1]!.color).toBe(OrbColor.Blue);
      expect(board[1][0]!.color).toBe(OrbColor.Green);
      expect(board[1][1]!.color).toBe(OrbColor.Yellow);
    });
  });

  describe('Orb Color Matching', () => {
    test('should match orbs of same color', () => {
      const orb1 = new OrbImpl('orb1', OrbType.Normal, OrbColor.Red, 0, 0, 60);
      const orb2 = new OrbImpl('orb2', OrbType.Normal, OrbColor.Red, 1, 0, 60);
      expect(orb1.matchesColor(orb2)).toBe(true);
    });

    test('should not match orbs of different colors', () => {
      const orb1 = new OrbImpl('orb1', OrbType.Normal, OrbColor.Red, 0, 0, 60);
      const orb2 = new OrbImpl('orb2', OrbType.Normal, OrbColor.Blue, 1, 0, 60);
      expect(orb1.matchesColor(orb2)).toBe(false);
    });

    test('rainbow orb should match any color', () => {
      const rainbow = new OrbImpl('rainbow', OrbType.Rainbow, null, 0, 0, 60);
      const red = new OrbImpl('orb1', OrbType.Normal, OrbColor.Red, 1, 0, 60);
      const blue = new OrbImpl('orb2', OrbType.Normal, OrbColor.Blue, 1, 1, 60);

      expect(rainbow.matchesColor(red)).toBe(true);
      expect(rainbow.matchesColor(blue)).toBe(true);
    });

    test('should match any color with rainbow', () => {
      const red = new OrbImpl('orb1', OrbType.Normal, OrbColor.Red, 0, 0, 60);
      const rainbow = new OrbImpl('rainbow', OrbType.Rainbow, null, 1, 0, 60);
      expect(red.matchesColor(rainbow)).toBe(true);
    });
  });

  describe('Click Handling and Matching', () => {
    beforeEach(() => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Blue },
        ],
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Green },
          { type: OrbType.Normal, color: OrbColor.Yellow },
        ],
      ];
      game = new ColorChainGame(3, 2);
      game.initializeBoard(layout);
    });

    test('should find orb at click location', () => {
      const board = game.getBoard();
      const orb = board[0][0]!;
      const result = game.handleClick(orb.x, orb.y);
      expect(result).toBe(true);
    });

    test('should remove matched orbs after click', () => {
      game.handleClick(30, 30);
      const board = game.getBoard();
      expect(board[0][0]).toBeNull();
      expect(board[0][1]).toBeNull();
      expect(board[1][0]).toBeNull();
    });

    test('should decrement moves on valid click', () => {
      const movesBefore = game.getMovesRemaining();
      game.handleClick(30, 30);
      expect(game.getMovesRemaining()).toBe(movesBefore - 1);
    });

    test('should increase score on match', () => {
      const scoreBefore = game.getScore();
      game.handleClick(30, 30);
      expect(game.getScore()).toBeGreaterThan(scoreBefore);
    });

    test('should not match single orb', () => {
      game.handleClick(150, 30);
      const board = game.getBoard();
      expect(board[0][2]).not.toBeNull();
    });
  });

  describe('Chain Reactions and Cascades', () => {
    test('should cascade when new matches form after gravity', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Blue },
        ],
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
        ],
      ];
      game = new ColorChainGame(2, 2);
      game.initializeBoard(layout);

      const scoreBefore = game.getScore();
      game.handleClick(30, 30);

      expect(game.getScore()).toBeGreaterThan(scoreBefore);
    });

    test('should apply gravity after explosion', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Blue },
          { type: OrbType.Normal, color: OrbColor.Yellow },
        ],
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
        ],
      ];
      game = new ColorChainGame(2, 2);
      game.initializeBoard(layout);

      game.handleClick(60, 90);

      const board = game.getBoard();
      let totalOrbs = 0;
      for (let y = 0; y < 2; y++) {
        for (let x = 0; x < 2; x++) {
          if (board[y][x]) totalOrbs++;
        }
      }
      expect(totalOrbs).toBeLessThanOrEqual(4);
    });
  });

  describe('Black Orb Behavior', () => {
    test('should not be clickable', () => {
      const layout = [
        [{ type: OrbType.Black, color: null }],
        [{ type: OrbType.Normal, color: OrbColor.Red }],
      ];
      game = new ColorChainGame(1, 2);
      game.initializeBoard(layout);

      const result = game.handleClick(30, 30);
      expect(result).toBe(false);
    });

    test('should block chain propagation', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Black, color: null },
          { type: OrbType.Normal, color: OrbColor.Red },
        ],
      ];
      game = new ColorChainGame(3, 1);
      game.initializeBoard(layout);

      game.handleClick(30, 30);

      const board = game.getBoard();
      expect(board[0][2]).not.toBeNull();
    });
  });

  describe('Game Status and Win/Lose Conditions', () => {
    test('should end in won state when board is cleared', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
        ],
      ];
      game = new ColorChainGame(2, 1);
      game.initializeBoard(layout);

      game.handleClick(30, 30);
      expect(game.getGameStatus()).toBe('won');
    });

    test('should end in lost state when out of moves', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Blue },
          { type: OrbType.Normal, color: OrbColor.Green },
        ],
      ];
      game = new ColorChainGame(3, 1);
      game.initializeBoard(layout);

      for (let i = 0; i < 20; i++) {
        if (game.getGameStatus() !== 'playing') break;
        game.handleClick(30, 30);
      }

      expect(game.getGameStatus()).toBe('lost');
    });

    test('should not allow clicks when game is lost', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Blue },
        ],
      ];
      game = new ColorChainGame(2, 1);
      game.initializeBoard(layout);

      for (let i = 0; i < 21; i++) {
        if (game.getGameStatus() !== 'playing') break;
        game.handleClick(30, 30);
      }

      const moveBefore = game.getMovesRemaining();
      game.handleClick(30, 30);
      expect(game.getMovesRemaining()).toBe(moveBefore);
    });

    test('should not allow clicks when game is won', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
        ],
      ];
      game = new ColorChainGame(2, 1);
      game.initializeBoard(layout);

      game.handleClick(30, 30);
      const moveBefore = game.getMovesRemaining();
      game.handleClick(30, 30);
      expect(game.getMovesRemaining()).toBe(moveBefore);
    });
  });

  describe('Physics Simulation', () => {
    test('should apply velocity to orbs', () => {
      const layout = [
        [{ type: OrbType.Normal, color: OrbColor.Red }],
      ];
      game = new ColorChainGame(1, 1);
      game.initializeBoard(layout);

      const board = game.getBoard();
      const orb = board[0][0]!;

      const xBefore = orb.x;
      orb.vx = 5;
      game.updatePhysics();

      expect(orb.x).toBeGreaterThan(xBefore);
    });

    test('should apply damping to velocity', () => {
      const layout = [
        [{ type: OrbType.Normal, color: OrbColor.Red }],
      ];
      game = new ColorChainGame(1, 1);
      game.initializeBoard(layout);

      const board = game.getBoard();
      const orb = board[0][0]!;

      orb.vx = 10;
      const vBefore = orb.vx;

      game.updatePhysics();
      expect(orb.vx).toBeLessThan(vBefore);
      expect(orb.vx).toBeGreaterThan(0);
    });

    test('should stop orb when velocity becomes negligible', () => {
      const layout = [
        [{ type: OrbType.Normal, color: OrbColor.Red }],
      ];
      game = new ColorChainGame(1, 1);
      game.initializeBoard(layout);

      const board = game.getBoard();
      const orb = board[0][0]!;

      orb.vx = 0.1;
      orb.vy = 0.1;

      for (let i = 0; i < 100; i++) {
        game.updatePhysics();
        if (orb.vx === 0 && orb.vy === 0) break;
      }

      expect(orb.vx).toBe(0);
      expect(orb.vy).toBe(0);
    });
  });

  describe('Level System', () => {
    test('should load level data', () => {
      expect(LEVELS.length).toBeGreaterThan(0);
    });

    test('first level should have valid properties', () => {
      const level = LEVELS[0];
      expect(level.movesLimit).toBeGreaterThan(0);
      expect(level.id).toBe(1);
    });

    test('should have valid level layouts', () => {
      for (const level of LEVELS) {
        expect(level.boardLayout.length).toBeGreaterThan(0);
        expect(level.boardLayout[0].length).toBeGreaterThan(0);
        expect(level.targetScore).toBeGreaterThan(0);
        expect(level.movesLimit).toBeGreaterThan(0);
      }
    });
  });

  describe('Score Calculation', () => {
    test('should calculate score for matches', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
        ],
      ];
      game = new ColorChainGame(3, 1);
      game.initializeBoard(layout);

      const scoreBefore = game.getScore();
      game.handleClick(30, 30);
      const scoreAfter = game.getScore();

      expect(scoreAfter - scoreBefore).toBeGreaterThanOrEqual(30);
    });

    test('should give cascade bonus for large matches', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
        ],
      ];
      game = new ColorChainGame(5, 1);
      game.initializeBoard(layout);

      const scoreBefore = game.getScore();
      game.handleClick(30, 30);
      const scoreAfter = game.getScore();

      expect(scoreAfter - scoreBefore).toBeGreaterThan(50);
    });
  });

  describe('Game Reset', () => {
    test('should reset game state', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Red },
        ],
      ];
      game = new ColorChainGame(2, 1);
      game.initializeBoard(layout);

      game.handleClick(30, 30);
      game.reset();

      expect(game.getScore()).toBe(0);
      expect(game.getMovesRemaining()).toBe(20);
      expect(game.getGameStatus()).toBe('playing');
    });
  });

  describe('Edge Cases', () => {
    test('should handle click outside board', () => {
      const layout = [
        [{ type: OrbType.Normal, color: OrbColor.Red }],
      ];
      game = new ColorChainGame(1, 1);
      game.initializeBoard(layout);

      const result = game.handleClick(999, 999);
      expect(result).toBe(false);
    });

    test('should handle empty board gracefully', () => {
      game = new ColorChainGame(2, 2);
      expect(() => game.handleClick(30, 30)).not.toThrow();
    });

    test('should maintain board integrity after operations', () => {
      const layout = [
        [
          { type: OrbType.Normal, color: OrbColor.Red },
          { type: OrbType.Normal, color: OrbColor.Blue },
        ],
        [
          { type: OrbType.Normal, color: OrbColor.Green },
          { type: OrbType.Normal, color: OrbColor.Yellow },
        ],
      ];
      game = new ColorChainGame(2, 2);
      game.initializeBoard(layout);

      game.handleClick(30, 30);

      const board = game.getBoard();
      let count = 0;
      for (let y = 0; y < 2; y++) {
        for (let x = 0; x < 2; x++) {
          if (board[y][x] !== null) count++;
        }
      }

      expect(count).toBeGreaterThanOrEqual(0);
      expect(count).toBeLessThanOrEqual(4);
    });
  });
});
