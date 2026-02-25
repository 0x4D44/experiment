/**
 * Hex Commander UI - Canvas-based rendering
 * Handles game rendering and user input
 */

import { Game, GamePhase, Unit, PlayerSide, AxialCoord, TerrainType, UnitType, coordEqual, getTerrain } from './hex-commander';
import SimpleAI from './hex-commander-ai';

const HEX_SIZE = 30; // Radius of hexagon
const HEX_WIDTH = HEX_SIZE * 2;
const HEX_HEIGHT = HEX_SIZE * Math.sqrt(3);

interface HexPixelPos {
  x: number;
  y: number;
}

export class HexCommanderUI {
  private game: Game;
  private ai: SimpleAI;
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private selectedUnit: Unit | null = null;
  private validMoves: AxialCoord[] = [];
  private gameRunning = false;

  constructor(canvasId: string) {
    this.canvas = document.getElementById(canvasId) as HTMLCanvasElement;
    if (!this.canvas) {
      throw new Error(`Canvas with id ${canvasId} not found`);
    }

    const ctx = this.canvas.getContext('2d');
    if (!ctx) {
      throw new Error('Failed to get canvas context');
    }
    this.ctx = ctx;

    // Set canvas size
    this.canvas.width = 1200;
    this.canvas.height = 900;

    this.game = new Game(12, 12);
    this.ai = new SimpleAI(this.game);
    this.gameRunning = true;

    // Setup event listeners
    this.canvas.addEventListener('click', (e) => this.handleClick(e));

    // Start game loop
    this.gameLoop();
  }

  private coordToPixel(coord: AxialCoord): HexPixelPos {
    const col = coord.q;
    const row = coord.r;

    const x = HEX_SIZE * (3 / 2 * col);
    const y = HEX_SIZE * (Math.sqrt(3) / 2 * col + Math.sqrt(3) * row);

    return { x: x + 50, y: y + 50 };
  }

  private pixelToCoord(x: number, y: number): AxialCoord | null {
    // Approximate conversion back to hex coordinates
    const adjustedX = x - 50;
    const adjustedY = y - 50;

    const q = (2 / 3 * adjustedX) / HEX_SIZE;
    const r = (-1 / 3 * adjustedX + Math.sqrt(3) / 3 * adjustedY) / HEX_SIZE;

    const roundedQ = Math.round(q);
    const roundedR = Math.round(r);

    return { q: roundedQ, r: roundedR };
  }

  private drawHexagon(x: number, y: number, color: string): void {
    this.ctx.fillStyle = color;
    this.ctx.strokeStyle = '#333';
    this.ctx.lineWidth = 2;

    this.ctx.beginPath();
    for (let i = 0; i < 6; i++) {
      const angle = (Math.PI / 3) * i;
      const hx = x + HEX_SIZE * Math.cos(angle);
      const hy = y + HEX_SIZE * Math.sin(angle);
      if (i === 0) {
        this.ctx.moveTo(hx, hy);
      } else {
        this.ctx.lineTo(hx, hy);
      }
    }
    this.ctx.closePath();
    this.ctx.fill();
    this.ctx.stroke();
  }

  private getTerrainColor(terrain: TerrainType): string {
    switch (terrain) {
      case TerrainType.Plain:
        return '#90EE90';
      case TerrainType.Forest:
        return '#228B22';
      case TerrainType.Mountain:
        return '#A9A9A9';
      case TerrainType.Water:
        return '#1E90FF';
      case TerrainType.ResourceNode:
        return '#FFD700';
      default:
        return '#CCCCCC';
    }
  }

  private getUnitColor(unit: Unit): string {
    if (unit.owner === PlayerSide.Player1) {
      return '#FF6B6B';
    } else {
      return '#4169E1';
    }
  }

  private drawUnit(unit: Unit, x: number, y: number): void {
    const color = this.getUnitColor(unit);
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.arc(x, y, HEX_SIZE * 0.4, 0, Math.PI * 2);
    this.ctx.fill();

    // Draw unit type indicator
    this.ctx.fillStyle = '#FFF';
    this.ctx.font = 'bold 12px Arial';
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'middle';

    let label = '';
    switch (unit.type) {
      case UnitType.Infantry:
        label = 'I';
        break;
      case UnitType.Cavalry:
        label = 'C';
        break;
      case UnitType.Archer:
        label = 'A';
        break;
    }

    this.ctx.fillText(label, x, y);

    // Draw health bar
    const healthPercent = unit.health / unit.maxHealth;
    const barWidth = HEX_SIZE * 0.6;
    const barX = x - barWidth / 2;
    const barY = y + HEX_SIZE * 0.5;

    this.ctx.fillStyle = '#CCC';
    this.ctx.fillRect(barX, barY, barWidth, 4);

    this.ctx.fillStyle = healthPercent > 0.5 ? '#00AA00' : healthPercent > 0.25 ? '#FFAA00' : '#AA0000';
    this.ctx.fillRect(barX, barY, barWidth * healthPercent, 4);
  }

  private drawValidMoves(): void {
    this.ctx.fillStyle = 'rgba(200, 200, 255, 0.3)';
    this.ctx.strokeStyle = '#4169E1';
    this.ctx.lineWidth = 1;

    for (const move of this.validMoves) {
      const pos = this.coordToPixel(move);
      this.drawHexagon(pos.x, pos.y, 'rgba(200, 200, 255, 0.3)');
    }
  }

  private drawGame(): void {
    // Clear canvas
    this.ctx.fillStyle = '#F0F0F0';
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

    // Draw tiles
    const tiles = this.game.state.map.getAllTiles();
    for (const tile of tiles) {
      const pos = this.coordToPixel(tile.coord);
      const color = this.getTerrainColor(tile.terrain);
      this.drawHexagon(pos.x, pos.y, color);

      // Draw resource indicator for resource nodes
      if (tile.terrain === TerrainType.ResourceNode && tile.resourceOwner) {
        this.ctx.fillStyle = tile.resourceOwner === PlayerSide.Player1 ? 'rgba(255, 0, 0, 0.2)' : 'rgba(0, 0, 255, 0.2)';
        this.ctx.beginPath();
        this.ctx.arc(pos.x, pos.y, HEX_SIZE * 0.3, 0, Math.PI * 2);
        this.ctx.fill();
      }
    }

    // Draw valid moves if unit selected
    if (this.validMoves.length > 0) {
      this.drawValidMoves();
    }

    // Draw units
    for (const tile of tiles) {
      if (tile.unit) {
        const pos = this.coordToPixel(tile.coord);
        this.drawUnit(tile.unit, pos.x, pos.y);

        // Highlight selected unit
        if (this.selectedUnit && coordEqual(this.selectedUnit.position, tile.unit.position)) {
          this.ctx.strokeStyle = '#FFF';
          this.ctx.lineWidth = 3;
          this.ctx.beginPath();
          for (let i = 0; i < 6; i++) {
            const angle = (Math.PI / 3) * i;
            const hx = pos.x + HEX_SIZE * Math.cos(angle);
            const hy = pos.y + HEX_SIZE * Math.sin(angle);
            if (i === 0) {
              this.ctx.moveTo(hx, hy);
            } else {
              this.ctx.lineTo(hx, hy);
            }
          }
          this.ctx.closePath();
          this.ctx.stroke();
        }
      }
    }

    // Draw UI
    this.drawUI();
  }

  private drawUI(): void {
    this.ctx.fillStyle = '#000';
    this.ctx.font = '14px Arial';
    this.ctx.textAlign = 'left';

    const yStart = 10;
    const lineHeight = 20;

    this.ctx.fillText(`Turn: ${this.game.state.turn}`, 10, yStart);
    this.ctx.fillText(`Player 1 Resources: ${this.game.state.player1Resources}`, 10, yStart + lineHeight);
    this.ctx.fillText(`Player 2 Resources: ${this.game.state.player2Resources}`, 10, yStart + lineHeight * 2);
    this.ctx.fillText(`Current: ${this.game.state.currentPlayer}`, 10, yStart + lineHeight * 3);
    this.ctx.fillText(this.game.getGameStatus(), 10, yStart + lineHeight * 4);

    // Draw controls info
    if (this.game.state.phase === GamePhase.PlayerTurn) {
      this.ctx.fillText('Click unit to select, then valid square to move', 10, this.canvas.height - 40);
      this.ctx.fillText('Right-click to attack, Space to end turn', 10, this.canvas.height - 20);
    }
  }

  private handleClick(e: MouseEvent): void {
    if (this.game.state.phase !== GamePhase.PlayerTurn) return;

    const rect = this.canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const coord = this.pixelToCoord(x, y);
    if (!coord) return;

    const tile = this.game.state.map.getTile(coord);
    if (!tile) return;

    // If clicking on a unit, select it
    if (tile.unit && tile.unit.owner === PlayerSide.Player1) {
      this.selectUnit(tile.unit);
    } else if (this.selectedUnit && this.validMoves.some((m) => coordEqual(m, coord))) {
      // If clicking on a valid move, move the unit
      this.game.moveUnit(this.selectedUnit, coord);
      this.selectedUnit = null;
      this.validMoves = [];
    } else if (this.selectedUnit && e.button === 2) {
      // Right-click to attack
      if (tile.unit) {
        this.game.attackUnit(this.selectedUnit, tile.unit);
      }
    }
  }

  private selectUnit(unit: Unit): void {
    this.selectedUnit = unit;

    // Calculate valid moves
    this.validMoves = [];
    const tiles = this.game.state.map.getAllTiles();
    for (const tile of tiles) {
      if (this.game.canMoveUnit(unit, tile.coord)) {
        this.validMoves.push(tile.coord);
      }
    }
  }

  private gameLoop = (): void => {
    this.drawGame();

    // Handle AI turn
    if (this.gameRunning && this.game.state.phase === GamePhase.AITurn) {
      setTimeout(() => {
        this.ai.takeTurn();
      }, 1000);
    }

    // Handle game over
    if (this.game.state.phase === GamePhase.GameOver) {
      this.gameRunning = false;
    }

    requestAnimationFrame(this.gameLoop);
  };

  public endTurn(): void {
    if (this.game.state.phase === GamePhase.PlayerTurn) {
      this.selectedUnit = null;
      this.validMoves = [];
      this.game.endTurn();
    }
  }
}

export default HexCommanderUI;
