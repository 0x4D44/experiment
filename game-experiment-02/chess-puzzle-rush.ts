/**
 * Chess Puzzle Rush - Core Game Engine
 * A fast-paced chess puzzle game with time pressure and score multipliers
 */

// ============================================================================
// TYPE DEFINITIONS
// ============================================================================

export enum PieceType {
  Pawn = 'P',
  Knight = 'N',
  Bishop = 'B',
  Rook = 'R',
  Queen = 'Q',
  King = 'K',
}

export enum Color {
  White = 'w',
  Black = 'b',
}

export interface Piece {
  type: PieceType;
  color: Color;
}

export interface Square {
  piece: Piece | null;
  file: number; // 0-7 (a-h)
  rank: number; // 0-7 (1-8)
}

export interface Move {
  from: { file: number; rank: number };
  to: { file: number; rank: number };
  promotion?: PieceType;
}

export interface GameState {
  board: (Piece | null)[][];
  toMove: Color;
  canCastleWhiteKingside: boolean;
  canCastleWhiteQueenside: boolean;
  canCastleBlackKingside: boolean;
  canCastleBlackQueenside: boolean;
  enPassantFile: number | null;
  halfMoveClock: number;
  fullMoveNumber: number;
}

export interface Puzzle {
  id: string;
  name: string;
  fen: string;
  solution: Move[];
  difficulty: 1 | 2 | 3; // 1 = checkmate in 1, 2 = in 2, 3 = in 3
  category: 'standard' | 'knights-only' | 'pawns-only';
}

export interface GameScore {
  puzzlesCompleted: number;
  totalScore: number;
  currentCombo: number;
  currentPuzzleScore: number;
}

// ============================================================================
// CHESS ENGINE
// ============================================================================

export class ChessEngine {
  private gameState: GameState;
  private moveHistory: Move[] = [];

  constructor(fen: string = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1') {
    this.gameState = this.parseFEN(fen);
  }

  /**
   * Parse FEN notation into GameState
   */
  private parseFEN(fen: string): GameState {
    const parts = fen.split(' ');
    const boardPart = parts[0];
    const toMove = (parts[1] === 'b' ? Color.Black : Color.White);
    const castlingRights = parts[2] || '-';
    const enPassantStr = parts[3] || '-';
    const halfMoveClock = parseInt(parts[4] || '0', 10);
    const fullMoveNumber = parseInt(parts[5] || '1', 10);

    // Parse board
    const board: (Piece | null)[][] = [];
    const ranks = boardPart.split('/');
    for (let i = 0; i < 8; i++) {
      const rank: (Piece | null)[] = [];
      const rankStr = ranks[i];
      for (let j = 0; j < rankStr.length; j++) {
        const ch = rankStr[j];
        if (/\d/.test(ch)) {
          for (let k = 0; k < parseInt(ch, 10); k++) {
            rank.push(null);
          }
        } else {
          const piece: Piece = {
            type: ch.toUpperCase() as PieceType,
            color: ch === ch.toUpperCase() ? Color.White : Color.Black,
          };
          rank.push(piece);
        }
      }
      board.push(rank);
    }

    // Parse en passant
    const enPassantFile = enPassantStr !== '-' ? enPassantStr.charCodeAt(0) - 97 : null;

    return {
      board,
      toMove,
      canCastleWhiteKingside: castlingRights.includes('K'),
      canCastleWhiteQueenside: castlingRights.includes('Q'),
      canCastleBlackKingside: castlingRights.includes('k'),
      canCastleBlackQueenside: castlingRights.includes('q'),
      enPassantFile,
      halfMoveClock,
      fullMoveNumber,
    };
  }

  /**
   * Generate FEN from current game state
   */
  getFEN(): string {
    let boardStr = '';
    for (let rank = 0; rank < 8; rank++) {
      let emptyCount = 0;
      for (let file = 0; file < 8; file++) {
        const piece = this.gameState.board[rank][file];
        if (piece) {
          if (emptyCount > 0) {
            boardStr += emptyCount;
            emptyCount = 0;
          }
          const notation = piece.color === Color.White ? piece.type : piece.type.toLowerCase();
          boardStr += notation;
        } else {
          emptyCount++;
        }
      }
      if (emptyCount > 0) {
        boardStr += emptyCount;
      }
      if (rank < 7) boardStr += '/';
    }

    let castling = '';
    if (this.gameState.canCastleWhiteKingside) castling += 'K';
    if (this.gameState.canCastleWhiteQueenside) castling += 'Q';
    if (this.gameState.canCastleBlackKingside) castling += 'k';
    if (this.gameState.canCastleBlackQueenside) castling += 'q';
    if (castling === '') castling = '-';

    const enPassant =
      this.gameState.enPassantFile !== null
        ? String.fromCharCode(97 + this.gameState.enPassantFile) +
          (this.gameState.toMove === Color.White ? '6' : '3')
        : '-';

    return `${boardStr} ${this.gameState.toMove} ${castling} ${enPassant} ${this.gameState.halfMoveClock} ${this.gameState.fullMoveNumber}`;
  }

  /**
   * Get all valid moves for current position
   */
  getValidMoves(forColor?: Color): Move[] {
    const color = forColor || this.gameState.toMove;
    const moves: Move[] = [];

    for (let rank = 0; rank < 8; rank++) {
      for (let file = 0; file < 8; file++) {
        const piece = this.gameState.board[rank][file];
        if (piece && piece.color === color) {
          const pieceMoves = this.getPieceMoves(file, rank);
          moves.push(...pieceMoves);
        }
      }
    }

    return moves.filter(move => !this.isKingInCheckAfterMove(move, color));
  }

  /**
   * Get pseudo-legal moves for a piece at given square
   */
  private getPieceMoves(file: number, rank: number): Move[] {
    const piece = this.gameState.board[rank][file];
    if (!piece) return [];

    const moves: Move[] = [];

    switch (piece.type) {
      case PieceType.Pawn:
        moves.push(...this.getPawnMoves(file, rank, piece.color));
        break;
      case PieceType.Knight:
        moves.push(...this.getKnightMoves(file, rank, piece.color));
        break;
      case PieceType.Bishop:
        moves.push(...this.getBishopMoves(file, rank, piece.color));
        break;
      case PieceType.Rook:
        moves.push(...this.getRookMoves(file, rank, piece.color));
        break;
      case PieceType.Queen:
        moves.push(...this.getQueenMoves(file, rank, piece.color));
        break;
      case PieceType.King:
        moves.push(...this.getKingMoves(file, rank, piece.color));
        break;
    }

    return moves;
  }

  private getPawnMoves(file: number, rank: number, color: Color): Move[] {
    const moves: Move[] = [];
    const direction = color === Color.White ? 1 : -1;
    const startRank = color === Color.White ? 1 : 6;
    const promotionRank = color === Color.White ? 7 : 0;

    // Forward move
    const nextRank = rank + direction;
    if (nextRank >= 0 && nextRank <= 7) {
      if (!this.gameState.board[nextRank][file]) {
        if (nextRank === promotionRank) {
          moves.push({ from: { file, rank }, to: { file, rank: nextRank }, promotion: PieceType.Queen });
          moves.push({ from: { file, rank }, to: { file, rank: nextRank }, promotion: PieceType.Rook });
          moves.push({ from: { file, rank }, to: { file, rank: nextRank }, promotion: PieceType.Bishop });
          moves.push({ from: { file, rank }, to: { file, rank: nextRank }, promotion: PieceType.Knight });
        } else {
          moves.push({ from: { file, rank }, to: { file, rank: nextRank } });
        }

        // Double move from start
        if (rank === startRank && !this.gameState.board[rank + 2 * direction][file]) {
          moves.push({ from: { file, rank }, to: { file, rank: rank + 2 * direction } });
        }
      }

      // Captures
      for (const captureFile of [file - 1, file + 1]) {
        if (captureFile >= 0 && captureFile <= 7) {
          const target = this.gameState.board[nextRank][captureFile];
          if (target && target.color !== color) {
            if (nextRank === promotionRank) {
              moves.push({ from: { file, rank }, to: { file: captureFile, rank: nextRank }, promotion: PieceType.Queen });
              moves.push({ from: { file, rank }, to: { file: captureFile, rank: nextRank }, promotion: PieceType.Rook });
              moves.push({ from: { file, rank }, to: { file: captureFile, rank: nextRank }, promotion: PieceType.Bishop });
              moves.push({ from: { file, rank }, to: { file: captureFile, rank: nextRank }, promotion: PieceType.Knight });
            } else {
              moves.push({ from: { file, rank }, to: { file: captureFile, rank: nextRank } });
            }
          }

          // En passant
          if (
            this.gameState.enPassantFile === captureFile &&
            nextRank === (color === Color.White ? 5 : 2)
          ) {
            moves.push({ from: { file, rank }, to: { file: captureFile, rank: nextRank } });
          }
        }
      }
    }

    return moves;
  }

  private getKnightMoves(file: number, rank: number, color: Color): Move[] {
    const moves: Move[] = [];
    const offsets = [
      [-2, -1], [-2, 1], [-1, -2], [-1, 2],
      [1, -2], [1, 2], [2, -1], [2, 1],
    ];

    for (const [df, dr] of offsets) {
      const newFile = file + df;
      const newRank = rank + dr;
      if (newFile >= 0 && newFile <= 7 && newRank >= 0 && newRank <= 7) {
        const target = this.gameState.board[newRank][newFile];
        if (!target || target.color !== color) {
          moves.push({ from: { file, rank }, to: { file: newFile, rank: newRank } });
        }
      }
    }

    return moves;
  }

  private getBishopMoves(file: number, rank: number, color: Color): Move[] {
    const moves: Move[] = [];
    const directions = [[-1, -1], [-1, 1], [1, -1], [1, 1]];

    for (const [df, dr] of directions) {
      for (let i = 1; i < 8; i++) {
        const newFile = file + df * i;
        const newRank = rank + dr * i;
        if (newFile < 0 || newFile > 7 || newRank < 0 || newRank > 7) break;

        const target = this.gameState.board[newRank][newFile];
        if (!target) {
          moves.push({ from: { file, rank }, to: { file: newFile, rank: newRank } });
        } else if (target.color !== color) {
          moves.push({ from: { file, rank }, to: { file: newFile, rank: newRank } });
          break;
        } else {
          break;
        }
      }
    }

    return moves;
  }

  private getRookMoves(file: number, rank: number, color: Color): Move[] {
    const moves: Move[] = [];
    const directions = [[-1, 0], [1, 0], [0, -1], [0, 1]];

    for (const [df, dr] of directions) {
      for (let i = 1; i < 8; i++) {
        const newFile = file + df * i;
        const newRank = rank + dr * i;
        if (newFile < 0 || newFile > 7 || newRank < 0 || newRank > 7) break;

        const target = this.gameState.board[newRank][newFile];
        if (!target) {
          moves.push({ from: { file, rank }, to: { file: newFile, rank: newRank } });
        } else if (target.color !== color) {
          moves.push({ from: { file, rank }, to: { file: newFile, rank: newRank } });
          break;
        } else {
          break;
        }
      }
    }

    return moves;
  }

  private getQueenMoves(file: number, rank: number, color: Color): Move[] {
    return [
      ...this.getRookMoves(file, rank, color),
      ...this.getBishopMoves(file, rank, color),
    ];
  }

  private getKingMoves(file: number, rank: number, color: Color): Move[] {
    const moves: Move[] = [];
    const offsets = [
      [-1, -1], [-1, 0], [-1, 1],
      [0, -1], [0, 1],
      [1, -1], [1, 0], [1, 1],
    ];

    for (const [df, dr] of offsets) {
      const newFile = file + df;
      const newRank = rank + dr;
      if (newFile >= 0 && newFile <= 7 && newRank >= 0 && newRank <= 7) {
        const target = this.gameState.board[newRank][newFile];
        if (!target || target.color !== color) {
          moves.push({ from: { file, rank }, to: { file: newFile, rank: newRank } });
        }
      }
    }

    // Castling
    if (color === Color.White && rank === 0) {
      if (
        this.gameState.canCastleWhiteKingside &&
        !this.gameState.board[0][5] &&
        !this.gameState.board[0][6]
      ) {
        moves.push({ from: { file: 4, rank: 0 }, to: { file: 6, rank: 0 } });
      }
      if (
        this.gameState.canCastleWhiteQueenside &&
        !this.gameState.board[0][1] &&
        !this.gameState.board[0][2] &&
        !this.gameState.board[0][3]
      ) {
        moves.push({ from: { file: 4, rank: 0 }, to: { file: 2, rank: 0 } });
      }
    } else if (color === Color.Black && rank === 7) {
      if (
        this.gameState.canCastleBlackKingside &&
        !this.gameState.board[7][5] &&
        !this.gameState.board[7][6]
      ) {
        moves.push({ from: { file: 4, rank: 7 }, to: { file: 6, rank: 7 } });
      }
      if (
        this.gameState.canCastleBlackQueenside &&
        !this.gameState.board[7][1] &&
        !this.gameState.board[7][2] &&
        !this.gameState.board[7][3]
      ) {
        moves.push({ from: { file: 4, rank: 7 }, to: { file: 2, rank: 7 } });
      }
    }

    return moves;
  }

  /**
   * Check if king is in check
   */
  isKingInCheck(color: Color): boolean {
    const kingPos = this.findKing(color);
    if (!kingPos) return false;

    const opponentColor = color === Color.White ? Color.Black : Color.White;
    const opponentMoves = this.getValidMovesForColor(opponentColor, true);

    return opponentMoves.some(
      move => move.to.file === kingPos.file && move.to.rank === kingPos.rank
    );
  }

  /**
   * Check if a move leaves king in check
   */
  private isKingInCheckAfterMove(move: Move, color: Color): boolean {
    const savedBoard = this.cloneBoard();
    const savedState = { ...this.gameState };

    this.makeMoveSilent(move);
    const inCheck = this.isKingInCheck(color);

    this.gameState = savedState;
    this.gameState.board = savedBoard;

    return inCheck;
  }

  /**
   * Get valid moves without checking for king safety
   */
  private getValidMovesForColor(color: Color, pseudoLegal: boolean = false): Move[] {
    const moves: Move[] = [];

    for (let rank = 0; rank < 8; rank++) {
      for (let file = 0; file < 8; file++) {
        const piece = this.gameState.board[rank][file];
        if (piece && piece.color === color) {
          const pieceMoves = this.getPieceMoves(file, rank);
          moves.push(...pieceMoves);
        }
      }
    }

    return moves;
  }

  /**
   * Find king position for a color
   */
  private findKing(color: Color): { file: number; rank: number } | null {
    for (let rank = 0; rank < 8; rank++) {
      for (let file = 0; file < 8; file++) {
        const piece = this.gameState.board[rank][file];
        if (piece && piece.type === PieceType.King && piece.color === color) {
          return { file, rank };
        }
      }
    }
    return null;
  }

  /**
   * Clone the board
   */
  private cloneBoard(): (Piece | null)[][] {
    return this.gameState.board.map(rank => [...rank]);
  }

  /**
   * Make a move without validation
   */
  private makeMoveSilent(move: Move): void {
    const piece = this.gameState.board[move.from.rank][move.from.file];
    if (!piece) return;

    const isCapture = !!this.gameState.board[move.to.rank][move.to.file];
    const isPawnMove = piece.type === PieceType.Pawn;

    // Handle en passant
    let enPassantFile: number | null = null;
    if (isPawnMove && Math.abs(move.from.rank - move.to.rank) === 2) {
      enPassantFile = move.from.file;
    }

    // Move piece
    this.gameState.board[move.to.rank][move.to.file] = piece;
    this.gameState.board[move.from.rank][move.from.file] = null;

    // Handle promotion
    if (move.promotion) {
      piece.type = move.promotion;
    }

    // Handle en passant capture
    if (
      isPawnMove &&
      move.from.file !== move.to.file &&
      !isCapture &&
      this.gameState.enPassantFile === move.to.file
    ) {
      this.gameState.board[move.from.rank][move.to.file] = null;
    }

    // Handle castling
    if (piece.type === PieceType.King) {
      if (piece.color === Color.White) {
        this.gameState.canCastleWhiteKingside = false;
        this.gameState.canCastleWhiteQueenside = false;
        if (move.from.file === 4 && move.to.file === 6) {
          const rook = this.gameState.board[0][7];
          if (rook) {
            this.gameState.board[0][5] = rook;
            this.gameState.board[0][7] = null;
          }
        } else if (move.from.file === 4 && move.to.file === 2) {
          const rook = this.gameState.board[0][0];
          if (rook) {
            this.gameState.board[0][3] = rook;
            this.gameState.board[0][0] = null;
          }
        }
      } else {
        this.gameState.canCastleBlackKingside = false;
        this.gameState.canCastleBlackQueenside = false;
        if (move.from.file === 4 && move.to.file === 6) {
          const rook = this.gameState.board[7][7];
          if (rook) {
            this.gameState.board[7][5] = rook;
            this.gameState.board[7][7] = null;
          }
        } else if (move.from.file === 4 && move.to.file === 2) {
          const rook = this.gameState.board[7][0];
          if (rook) {
            this.gameState.board[7][3] = rook;
            this.gameState.board[7][0] = null;
          }
        }
      }
    }

    // Update castling rights for rook moves
    if (piece.type === PieceType.Rook) {
      if (piece.color === Color.White) {
        if (move.from.file === 0) this.gameState.canCastleWhiteQueenside = false;
        if (move.from.file === 7) this.gameState.canCastleWhiteKingside = false;
      } else {
        if (move.from.file === 0) this.gameState.canCastleBlackQueenside = false;
        if (move.from.file === 7) this.gameState.canCastleBlackKingside = false;
      }
    }

    this.gameState.enPassantFile = enPassantFile;

    if (isPawnMove || isCapture) {
      this.gameState.halfMoveClock = 0;
    } else {
      this.gameState.halfMoveClock++;
    }

    // Update to move AFTER updating clocks
    const wasWhiteToMove = this.gameState.toMove === Color.White;
    this.gameState.toMove = wasWhiteToMove ? Color.Black : Color.White;

    // Increment full move number after Black moves
    if (wasWhiteToMove) {
      // White just moved, now it's Black's turn - don't increment yet
    } else {
      // Black just moved, increment full move number
      this.gameState.fullMoveNumber++;
    }
  }

  /**
   * Make a move (with validation)
   */
  makeMove(move: Move): boolean {
    const validMoves = this.getValidMoves();
    const isValidMove = validMoves.some(
      m => m.from.file === move.from.file &&
           m.from.rank === move.from.rank &&
           m.to.file === move.to.file &&
           m.to.rank === move.to.rank &&
           m.promotion === move.promotion
    );

    if (!isValidMove) return false;

    this.makeMoveSilent(move);
    this.moveHistory.push(move);
    return true;
  }

  /**
   * Check if current side is in checkmate
   */
  isCheckmate(): boolean {
    if (!this.isKingInCheck(this.gameState.toMove)) return false;
    return this.getValidMoves().length === 0;
  }

  /**
   * Check if current side is in stalemate
   */
  isStalemate(): boolean {
    if (this.isKingInCheck(this.gameState.toMove)) return false;
    return this.getValidMoves().length === 0;
  }

  /**
   * Get the board state
   */
  getBoard(): (Piece | null)[][] {
    return this.gameState.board;
  }

  /**
   * Get who's turn it is
   */
  getToMove(): Color {
    return this.gameState.toMove;
  }

  /**
   * Get move history
   */
  getMoveHistory(): Move[] {
    return [...this.moveHistory];
  }

  /**
   * Reset to FEN
   */
  reset(fen: string): void {
    this.gameState = this.parseFEN(fen);
    this.moveHistory = [];
  }
}

// ============================================================================
// PUZZLE SYSTEM
// ============================================================================

export class PuzzleManager {
  private puzzles: Map<string, Puzzle> = new Map();
  private currentIndex: number = 0;

  constructor(puzzles: Puzzle[]) {
    for (const puzzle of puzzles) {
      this.puzzles.set(puzzle.id, puzzle);
    }
  }

  getPuzzle(id: string): Puzzle | undefined {
    return this.puzzles.get(id);
  }

  getAllPuzzles(): Puzzle[] {
    return Array.from(this.puzzles.values());
  }

  getPuzzlesByDifficulty(difficulty: 1 | 2 | 3): Puzzle[] {
    return Array.from(this.puzzles.values()).filter(p => p.difficulty === difficulty);
  }

  getPuzzlesByCategory(category: 'standard' | 'knights-only' | 'pawns-only'): Puzzle[] {
    return Array.from(this.puzzles.values()).filter(p => p.category === category);
  }

  /**
   * Verify if a sequence of moves solves the puzzle
   */
  verifySolution(puzzleId: string, moves: Move[]): boolean {
    const puzzle = this.puzzles.get(puzzleId);
    if (!puzzle) return false;

    const engine = new ChessEngine(puzzle.fen);

    for (let i = 0; i < puzzle.solution.length; i++) {
      const expectedMove = puzzle.solution[i];
      if (i < moves.length) {
        const userMove = moves[i];
        if (
          userMove.from.file !== expectedMove.from.file ||
          userMove.from.rank !== expectedMove.from.rank ||
          userMove.to.file !== expectedMove.to.file ||
          userMove.to.rank !== expectedMove.to.rank ||
          userMove.promotion !== expectedMove.promotion
        ) {
          return false;
        }
      } else {
        return false;
      }
      engine.makeMove(expectedMove);
    }

    return engine.isCheckmate();
  }

  /**
   * Get hint for a puzzle (reveal first move)
   */
  getHint(puzzleId: string): Move | undefined {
    const puzzle = this.puzzles.get(puzzleId);
    return puzzle?.solution[0];
  }
}

// ============================================================================
// PUZZLE DATA - Standard Puzzle Set (25+ puzzles)
// ============================================================================

export const STANDARD_PUZZLES: Puzzle[] = [
  // Checkmate in 1 puzzles
  {
    id: 'mate1-001',
    name: 'Back Rank Mate',
    fen: '6k1/5ppp/8/8/8/8/R7/K7 w - - 0 1',
    solution: [{ from: { file: 0, rank: 1 }, to: { file: 0, rank: 7 } }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-002',
    name: 'Smothered Mate',
    fen: '6k1/5ppp/8/8/8/8/5Q2/K7 w - - 0 1',
    solution: [{ from: { file: 5, rank: 1 }, to: { file: 5, rank: 7 } }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-003',
    name: 'Queen and King Mate',
    fen: '5k2/5Q2/6K1/8/8/8/8/8 w - - 0 1',
    solution: [{ from: { file: 5, rank: 6 }, to: { file: 5, rank: 7 } }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-004',
    name: 'Rook on 7th Rank',
    fen: '6k1/R6p/6P1/8/8/8/8/K7 w - - 0 1',
    solution: [{ from: { file: 0, rank: 6 }, to: { file: 6, rank: 6 } }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-005',
    name: 'Diagonal Checkmate',
    fen: '6k1/5B1p/6P1/8/8/8/8/K7 w - - 0 1',
    solution: [{ from: { file: 5, rank: 6 }, to: { file: 7, rank: 7 } }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-006',
    name: 'Knight Fork Mate',
    fen: '6k1/5ppp/8/5N2/8/8/8/K7 w - - 0 1',
    solution: [{ from: { file: 5, rank: 4 }, to: { file: 7, rank: 5 } }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-007',
    name: 'Pawn Promotion Mate',
    fen: '6k1/4P3/8/8/8/8/8/K7 w - - 0 1',
    solution: [{ from: { file: 4, rank: 6 }, to: { file: 4, rank: 7 }, promotion: PieceType.Queen }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-008',
    name: 'Corner Mate',
    fen: '7k/5Q2/6K1/8/8/8/8/8 w - - 0 1',
    solution: [{ from: { file: 5, rank: 6 }, to: { file: 7, rank: 6 } }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-009',
    name: 'Rook and Bishop Mate',
    fen: '6k1/5pp1/5B2/7R/8/8/8/K7 w - - 0 1',
    solution: [{ from: { file: 7, rank: 4 }, to: { file: 7, rank: 7 } }],
    difficulty: 1,
    category: 'standard',
  },
  {
    id: 'mate1-010',
    name: 'Two Rooks Mate',
    fen: '5rk1/5R2/5R2/8/8/8/8/K7 w - - 0 1',
    solution: [{ from: { file: 5, rank: 5 }, to: { file: 5, rank: 7 } }],
    difficulty: 1,
    category: 'standard',
  },

  // Checkmate in 2 puzzles
  {
    id: 'mate2-001',
    name: 'Quiet Mate in 2',
    fen: '6k1/5ppp/8/8/8/5Q2/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 2 }, to: { file: 5, rank: 6 } },
      { from: { file: 5, rank: 6 }, to: { file: 7, rank: 6 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-002',
    name: 'Forcing Mate in 2',
    fen: '6k1/4Q1pp/8/8/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 4, rank: 6 }, to: { file: 7, rank: 6 } },
      { from: { file: 7, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-003',
    name: 'Knight Discovery Mate',
    fen: '6k1/5ppp/4N3/4Q3/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 4, rank: 4 }, to: { file: 6, rank: 5 } },
      { from: { file: 6, rank: 5 }, to: { file: 7, rank: 6 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-004',
    name: 'Rook and Knight Mate',
    fen: '6k1/5ppp/6N1/7R/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 7, rank: 4 }, to: { file: 7, rank: 7 } },
      { from: { file: 7, rank: 7 }, to: { file: 7, rank: 6 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-005',
    name: 'Bishop Trap Mate',
    fen: '6k1/5ppp/5B2/6Q1/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 4 }, to: { file: 7, rank: 6 } },
      { from: { file: 7, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-006',
    name: 'Pawn and Queen Mate',
    fen: '6k1/5p1p/5Q2/8/4P3/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 5 }, to: { file: 5, rank: 7 } },
      { from: { file: 5, rank: 7 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-007',
    name: 'Skewer Pattern',
    fen: '5rk1/5ppp/5N2/6R1/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 7, rank: 4 }, to: { file: 7, rank: 6 } },
      { from: { file: 7, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-008',
    name: 'Intermezzo Tactic',
    fen: '6k1/5ppp/8/5Q2/8/6N1/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 4 }, to: { file: 7, rank: 6 } },
      { from: { file: 7, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-009',
    name: 'Double Rook Mate',
    fen: '6k1/5ppp/6R1/7R/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 7, rank: 5 }, to: { file: 7, rank: 6 } },
      { from: { file: 7, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 2,
    category: 'standard',
  },
  {
    id: 'mate2-010',
    name: 'Queen Sacrifice Mate',
    fen: '5rk1/5ppp/5Q2/8/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 5 }, to: { file: 7, rank: 7 } },
      { from: { file: 7, rank: 7 }, to: { file: 6, rank: 7 } },
    ],
    difficulty: 2,
    category: 'standard',
  },

  // Checkmate in 3 puzzles
  {
    id: 'mate3-001',
    name: 'Three Move Classic',
    fen: '6k1/5ppp/8/5Q2/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 4 }, to: { file: 5, rank: 7 } },
      { from: { file: 5, rank: 7 }, to: { file: 6, rank: 6 } },
      { from: { file: 6, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 3,
    category: 'standard',
  },
  {
    id: 'mate3-002',
    name: 'Rook Ladder Mate',
    fen: '5rk1/5ppp/6R1/8/8/8/R7/K7 w - - 0 1',
    solution: [
      { from: { file: 6, rank: 5 }, to: { file: 6, rank: 6 } },
      { from: { file: 6, rank: 6 }, to: { file: 6, rank: 7 } },
      { from: { file: 0, rank: 1 }, to: { file: 0, rank: 7 } },
    ],
    difficulty: 3,
    category: 'standard',
  },
  {
    id: 'mate3-003',
    name: 'Knight Check Mate',
    fen: '6k1/5ppp/8/5N2/5Q2/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 4 }, to: { file: 7, rank: 5 } },
      { from: { file: 7, rank: 5 }, to: { file: 7, rank: 6 } },
      { from: { file: 7, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 3,
    category: 'standard',
  },
  {
    id: 'mate3-004',
    name: 'Quiet Maneuver',
    fen: '6k1/5ppp/8/8/8/5Q2/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 2 }, to: { file: 1, rank: 2 } },
      { from: { file: 1, rank: 2 }, to: { file: 1, rank: 7 } },
      { from: { file: 1, rank: 7 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 3,
    category: 'standard',
  },
  {
    id: 'mate3-005',
    name: 'Patient Attack',
    fen: '5rk1/5ppp/6Q1/8/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 6, rank: 5 }, to: { file: 6, rank: 6 } },
      { from: { file: 6, rank: 6 }, to: { file: 7, rank: 6 } },
      { from: { file: 7, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 3,
    category: 'standard',
  },

  // Knight-only puzzles
  {
    id: 'knight-001',
    name: 'Knight Fork',
    fen: '6k1/5ppp/8/5N2/8/8/8/K7 w - - 0 1',
    solution: [{ from: { file: 5, rank: 4 }, to: { file: 7, rank: 5 } }],
    difficulty: 1,
    category: 'knights-only',
  },
  {
    id: 'knight-002',
    name: 'Knight Maneuver',
    fen: '6k1/5ppp/8/3N4/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 3, rank: 4 }, to: { file: 5, rank: 5 } },
      { from: { file: 5, rank: 5 }, to: { file: 7, rank: 6 } },
    ],
    difficulty: 2,
    category: 'knights-only',
  },
  {
    id: 'knight-003',
    name: 'Knight Dance',
    fen: '6k1/5ppp/8/2N5/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 2, rank: 4 }, to: { file: 4, rank: 5 } },
      { from: { file: 4, rank: 5 }, to: { file: 6, rank: 6 } },
      { from: { file: 6, rank: 6 }, to: { file: 7, rank: 7 } },
    ],
    difficulty: 3,
    category: 'knights-only',
  },

  // Pawns-only puzzles
  {
    id: 'pawn-001',
    name: 'Pawn Promotion',
    fen: '6k1/4P3/8/8/8/8/8/K7 w - - 0 1',
    solution: [{ from: { file: 4, rank: 6 }, to: { file: 4, rank: 7 }, promotion: PieceType.Queen }],
    difficulty: 1,
    category: 'pawns-only',
  },
  {
    id: 'pawn-002',
    name: 'Pawn Advance',
    fen: '6k1/5ppp/5P2/8/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 5, rank: 5 }, to: { file: 5, rank: 6 } },
      { from: { file: 5, rank: 6 }, to: { file: 5, rank: 7 }, promotion: PieceType.Queen },
    ],
    difficulty: 2,
    category: 'pawns-only',
  },
  {
    id: 'pawn-003',
    name: 'Passed Pawn',
    fen: '6k1/4Pppp/8/8/8/8/8/K7 w - - 0 1',
    solution: [
      { from: { file: 4, rank: 6 }, to: { file: 5, rank: 7 }, promotion: PieceType.Queen },
      { from: { file: 5, rank: 7 }, to: { file: 6, rank: 7 } },
    ],
    difficulty: 2,
    category: 'pawns-only',
  },
];

// ============================================================================
// GAME STATE MANAGER
// ============================================================================

export class GameManager {
  private engine: ChessEngine;
  private puzzleManager: PuzzleManager;
  private currentPuzzleId: string;
  private playerMoves: Move[] = [];
  private score: GameScore = {
    puzzlesCompleted: 0,
    totalScore: 0,
    currentCombo: 0,
    currentPuzzleScore: 0,
  };
  private usedHint: boolean = false;
  private startTime: number = 0;

  constructor(puzzleManager: PuzzleManager, startPuzzleId: string) {
    this.puzzleManager = puzzleManager;
    this.currentPuzzleId = startPuzzleId;
    const puzzle = puzzleManager.getPuzzle(startPuzzleId);
    if (!puzzle) throw new Error('Puzzle not found');
    this.engine = new ChessEngine(puzzle.fen);
    this.startTime = Date.now();
  }

  /**
   * Attempt to make a move in the puzzle
   */
  makeMove(move: Move): boolean {
    if (!this.engine.makeMove(move)) {
      return false;
    }
    this.playerMoves.push(move);
    return true;
  }

  /**
   * Check if the puzzle is solved
   */
  isSolved(): boolean {
    if (this.playerMoves.length === 0) return false;
    return this.puzzleManager.verifySolution(this.currentPuzzleId, this.playerMoves);
  }

  /**
   * Calculate score for completed puzzle
   */
  calculateScore(): number {
    const puzzle = this.puzzleManager.getPuzzle(this.currentPuzzleId);
    if (!puzzle) return 0;

    const elapsed = (Date.now() - this.startTime) / 1000;
    const baseBonusTime = 60 - Math.min(50, elapsed);
    const timeBonus = Math.max(0, baseBonusTime * 10);
    const difficultyBonus = puzzle.difficulty * 50;
    const hintPenalty = this.usedHint ? 100 : 0;

    let comboMultiplier = 1 + this.score.currentCombo * 0.1;
    comboMultiplier = Math.min(2, comboMultiplier); // Cap at 2x

    return Math.floor((difficultyBonus + timeBonus - hintPenalty) * comboMultiplier);
  }

  /**
   * Complete the current puzzle
   */
  completePuzzle(): void {
    const puzzleScore = this.calculateScore();
    this.score.totalScore += puzzleScore;
    this.score.puzzlesCompleted++;
    this.score.currentPuzzleScore = puzzleScore;
    this.score.currentCombo++;
  }

  /**
   * Fail the current puzzle
   */
  failPuzzle(): void {
    this.score.currentCombo = 0;
    this.score.currentPuzzleScore = 0;
  }

  /**
   * Get hint
   */
  getHint(): Move | undefined {
    if (this.usedHint) return undefined;
    this.usedHint = true;
    return this.puzzleManager.getHint(this.currentPuzzleId);
  }

  /**
   * Get current game state
   */
  getBoard(): (Piece | null)[][] {
    return this.engine.getBoard();
  }

  /**
   * Get whose turn it is
   */
  getToMove(): Color {
    return this.engine.getToMove();
  }

  /**
   * Get current puzzle
   */
  getCurrentPuzzle(): Puzzle | undefined {
    return this.puzzleManager.getPuzzle(this.currentPuzzleId);
  }

  /**
   * Get score
   */
  getScore(): GameScore {
    return { ...this.score };
  }

  /**
   * Get valid moves
   */
  getValidMoves(): Move[] {
    return this.engine.getValidMoves();
  }

  /**
   * Get player moves so far
   */
  getPlayerMoves(): Move[] {
    return [...this.playerMoves];
  }
}
