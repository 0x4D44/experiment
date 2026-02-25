/**
 * Color Chain Reaction - Core Game Engine
 * TypeScript implementation with physics simulation and chain reaction logic
 */

export enum OrbColor {
  Red = 'red',
  Blue = 'blue',
  Green = 'green',
  Yellow = 'yellow',
  Purple = 'purple',
}

export enum OrbType {
  Normal = 'normal',
  Rainbow = 'rainbow',
  Black = 'black',
  Mirror = 'mirror',
}

export interface Orb {
  id: string;
  type: OrbType;
  color: OrbColor | null;
  gridX: number;
  gridY: number;
  x: number;
  y: number;
  vx: number;
  vy: number;
  radius: number;
  isExploding: boolean;
  matchesColor(other: Orb): boolean;
  applyVelocity(): void;
  stop(): void;
}

export interface GameState {
  board: (Orb | null)[][];
  width: number;
  height: number;
  score: number;
  movesRemaining: number;
  levelId: number;
  gameStatus: 'playing' | 'won' | 'lost';
  totalMatches: number;
}

export interface Level {
  id: number;
  name: string;
  boardLayout: Array<{ type: OrbType; color: OrbColor | null }[]>;
  targetScore: number;
  movesLimit: number;
  width: number;
  height: number;
}

export class OrbImpl implements Orb {
  id: string;
  type: OrbType;
  color: OrbColor | null;
  gridX: number;
  gridY: number;
  x: number;
  y: number;
  vx: number = 0;
  vy: number = 0;
  radius: number = 15;
  isExploding: boolean = false;

  constructor(
    id: string,
    type: OrbType,
    color: OrbColor | null,
    gridX: number,
    gridY: number,
    cellSize: number
  ) {
    this.id = id;
    this.type = type;
    this.color = color;
    this.gridX = gridX;
    this.gridY = gridY;
    this.x = gridX * cellSize + cellSize / 2;
    this.y = gridY * cellSize + cellSize / 2;
  }

  matchesColor(other: Orb): boolean {
    if (this.type === OrbType.Rainbow || other.type === OrbType.Rainbow) {
      return true;
    }
    if (this.type === OrbType.Normal && other.type === OrbType.Normal) {
      return this.color === other.color;
    }
    return false;
  }

  applyVelocity(): void {
    this.x += this.vx;
    this.y += this.vy;
    this.vx *= 0.95;
    this.vy *= 0.95;
  }

  stop(): void {
    this.vx = 0;
    this.vy = 0;
  }
}

export class ColorChainGame {
  private state: GameState;
  private cellSize: number = 60;
  private orbCounter: number = 0;
  private explosionThreshold: number = 30;

  constructor(width: number, height: number) {
    this.state = {
      board: Array(height)
        .fill(null)
        .map(() => Array(width).fill(null)),
      width,
      height,
      score: 0,
      movesRemaining: 20,
      levelId: 1,
      gameStatus: 'playing',
      totalMatches: 0,
    };
  }

  getState(): GameState {
    return { ...this.state };
  }

  getBoard(): (Orb | null)[][] {
    return this.state.board.map((row) => [...row]);
  }

  getScore(): number {
    return this.state.score;
  }

  getMovesRemaining(): number {
    return this.state.movesRemaining;
  }

  getGameStatus(): 'playing' | 'won' | 'lost' {
    return this.state.gameStatus;
  }

  initializeBoard(layout: Array<{ type: OrbType; color: OrbColor | null }[]>): void {
    this.state.board = Array(this.state.height)
      .fill(null)
      .map(() => Array(this.state.width).fill(null));

    for (let y = 0; y < layout.length; y++) {
      for (let x = 0; x < layout[y].length; x++) {
        const { type, color } = layout[y][x];
        if (type !== null) {
          const orb = new OrbImpl(
            `orb-${this.orbCounter++}`,
            type,
            color,
            x,
            y,
            this.cellSize
          );
          this.state.board[y][x] = orb;
        }
      }
    }
  }

  handleClick(pixelX: number, pixelY: number): boolean {
    if (this.state.gameStatus !== 'playing') {
      return false;
    }

    const clickedOrb = this.findOrbAtPixel(pixelX, pixelY);
    if (!clickedOrb || clickedOrb.type === OrbType.Black) {
      return false;
    }

    this.triggerChainReaction(clickedOrb);
    this.state.movesRemaining--;
    this.updateGameStatus();
    return true;
  }

  private findOrbAtPixel(pixelX: number, pixelY: number): Orb | null {
    for (let y = 0; y < this.state.board.length; y++) {
      for (let x = 0; x < this.state.board[y].length; x++) {
        const orb = this.state.board[y][x];
        if (orb) {
          const dx = orb.x - pixelX;
          const dy = orb.y - pixelY;
          const distance = Math.sqrt(dx * dx + dy * dy);
          if (distance <= orb.radius) {
            return orb;
          }
        }
      }
    }
    return null;
  }

  private triggerChainReaction(startOrb: Orb): void {
    let totalMatched = 0;

    let hasNewMatches = true;
    while (hasNewMatches) {
      const matchedOrbs = this.findMatches(startOrb);
      if (matchedOrbs.length === 0) {
        break;
      }

      this.explodeOrbs(matchedOrbs);
      totalMatched += matchedOrbs.length;
      this.removeOrbs(matchedOrbs);
      this.applyGravity();

      const newStartOrb = this.findNewStartOrb();
      if (newStartOrb) {
        startOrb = newStartOrb;
        hasNewMatches = true;
      } else {
        hasNewMatches = false;
      }
    }

    const scoreGain = this.calculateScore(totalMatched);
    this.state.score += scoreGain;
    this.state.totalMatches += totalMatched;
  }

  private findMatches(startOrb: Orb): Orb[] {
    const matched: Orb[] = [];
    const visited = new Set<string>();
    const queue: Orb[] = [startOrb];
    visited.add(startOrb.id);

    while (queue.length > 0) {
      const current = queue.shift()!;
      matched.push(current);

      const adjacent = this.getAdjacentOrbs(current);
      for (const orb of adjacent) {
        if (!visited.has(orb.id) && current.matchesColor(orb)) {
          visited.add(orb.id);
          queue.push(orb);
        }
      }
    }

    return matched.length >= 2 ? matched : [];
  }

  private getAdjacentOrbs(orb: Orb): Orb[] {
    const adjacent: Orb[] = [];
    const directions = [
      [-1, 0],
      [1, 0],
      [0, -1],
      [0, 1],
    ];

    for (const [dx, dy] of directions) {
      const nx = orb.gridX + dx;
      const ny = orb.gridY + dy;
      if (
        nx >= 0 &&
        nx < this.state.width &&
        ny >= 0 &&
        ny < this.state.height
      ) {
        const neighbor = this.state.board[ny][nx];
        if (neighbor && neighbor.type !== OrbType.Black) {
          adjacent.push(neighbor);
        }
      }
    }
    return adjacent;
  }

  private explodeOrbs(orbs: Orb[]): void {
    let centerX = 0;
    let centerY = 0;
    for (const orb of orbs) {
      centerX += orb.x;
      centerY += orb.y;
    }
    centerX /= orbs.length;
    centerY /= orbs.length;

    const explosionRadius = 100;
    for (let y = 0; y < this.state.board.length; y++) {
      for (let x = 0; x < this.state.board[y].length; x++) {
        const orb = this.state.board[y][x];
        if (orb && !orbs.includes(orb)) {
          const dx = orb.x - centerX;
          const dy = orb.y - centerY;
          const distance = Math.sqrt(dx * dx + dy * dy);

          if (distance < explosionRadius && distance > 0) {
            const force = 5 * (1 - distance / explosionRadius);
            orb.vx += (dx / distance) * force;
            orb.vy += (dy / distance) * force;
          }
        }
      }
    }

    for (const orb of orbs) {
      orb.isExploding = true;
    }
  }

  private removeOrbs(orbs: Orb[]): void {
    for (const orb of orbs) {
      this.state.board[orb.gridY][orb.gridX] = null;
    }
  }

  private applyGravity(): void {
    for (let y = this.state.height - 1; y > 0; y--) {
      for (let x = 0; x < this.state.width; x++) {
        const orb = this.state.board[y][x];
        if (!orb) {
          for (let checkY = y - 1; checkY >= 0; checkY--) {
            if (this.state.board[checkY][x]) {
              const movingOrb = this.state.board[checkY][x]!;
              this.state.board[checkY][x] = null;
              this.state.board[y][x] = movingOrb;
              movingOrb.gridY = y;
              movingOrb.y = y * this.cellSize + this.cellSize / 2;
              break;
            }
          }
        }
      }
    }
  }

  private findNewStartOrb(): Orb | null {
    for (let y = 0; y < this.state.board.length; y++) {
      for (let x = 0; x < this.state.board[y].length; x++) {
        const orb = this.state.board[y][x];
        if (orb) {
          const adjacent = this.getAdjacentOrbs(orb);
          for (const adj of adjacent) {
            if (orb.matchesColor(adj)) {
              return orb;
            }
          }
        }
      }
    }
    return null;
  }

  private calculateScore(matchedCount: number): number {
    const baseScore = matchedCount * 10;
    const cascadeBonus = matchedCount > 4 ? (matchedCount - 4) * 5 : 0;
    return baseScore + cascadeBonus;
  }

  private updateGameStatus(): void {
    const isEmpty = this.isBoardEmpty();
    const outOfMoves = this.state.movesRemaining <= 0;

    if (isEmpty) {
      this.state.gameStatus = 'won';
    } else if (outOfMoves && !isEmpty) {
      this.state.gameStatus = 'lost';
    }
  }

  private isBoardEmpty(): boolean {
    for (let y = 0; y < this.state.board.length; y++) {
      for (let x = 0; x < this.state.board[y].length; x++) {
        if (this.state.board[y][x] !== null) {
          return false;
        }
      }
    }
    return true;
  }

  reset(): void {
    this.state.score = 0;
    this.state.movesRemaining = 20;
    this.state.gameStatus = 'playing';
    this.state.totalMatches = 0;
    this.state.board = Array(this.state.height)
      .fill(null)
      .map(() => Array(this.state.width).fill(null));
  }

  updatePhysics(): void {
    for (let y = 0; y < this.state.board.length; y++) {
      for (let x = 0; x < this.state.board[y].length; x++) {
        const orb = this.state.board[y][x];
        if (orb && (orb.vx !== 0 || orb.vy !== 0)) {
          orb.applyVelocity();

          const minDist = 3;
          const targetX = orb.gridX * this.cellSize + this.cellSize / 2;
          const targetY = orb.gridY * this.cellSize + this.cellSize / 2;

          if (
            Math.abs(orb.x - targetX) < minDist &&
            Math.abs(orb.y - targetY) < minDist
          ) {
            orb.x = targetX;
            orb.y = targetY;
            orb.stop();
          }
        }
      }
    }
  }
}

export const LEVELS: Level[] = [
  {
    id: 1,
    name: 'Getting Started',
    boardLayout: [
      [
        { type: OrbType.Normal, color: OrbColor.Red },
        { type: OrbType.Normal, color: OrbColor.Red },
        { type: OrbType.Normal, color: OrbColor.Blue },
        { type: OrbType.Normal, color: OrbColor.Blue },
      ],
      [
        { type: OrbType.Normal, color: OrbColor.Green },
        { type: OrbType.Normal, color: OrbColor.Green },
        { type: OrbType.Normal, color: OrbColor.Yellow },
        { type: OrbType.Normal, color: OrbColor.Yellow },
      ],
      [
        { type: OrbType.Normal, color: OrbColor.Red },
        { type: OrbType.Normal, color: OrbColor.Blue },
        { type: OrbType.Normal, color: OrbColor.Green },
        { type: OrbType.Normal, color: OrbColor.Yellow },
      ],
    ],
    targetScore: 150,
    movesLimit: 10,
    width: 4,
    height: 3,
  },
  {
    id: 2,
    name: 'Chain Reaction',
    boardLayout: [
      [
        { type: OrbType.Normal, color: OrbColor.Red },
        { type: OrbType.Normal, color: OrbColor.Red },
        { type: OrbType.Normal, color: OrbColor.Red },
      ],
      [
        { type: OrbType.Normal, color: OrbColor.Blue },
        { type: OrbType.Normal, color: OrbColor.Blue },
        { type: OrbType.Normal, color: OrbColor.Blue },
      ],
    ],
    targetScore: 100,
    movesLimit: 5,
    width: 3,
    height: 2,
  },
];
