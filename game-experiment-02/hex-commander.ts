/**
 * Hex Commander - Turn-Based Strategy Game
 * A hexagonal grid-based strategy game with units, terrain, and fog of war
 */

// ============================================================================
// COORDINATE SYSTEM - Axial Coordinates for Hex Grids
// ============================================================================

export interface AxialCoord {
  q: number;
  r: number;
}

export function coord(q: number, r: number): AxialCoord {
  return { q, r };
}

export function coordEqual(a: AxialCoord, b: AxialCoord): boolean {
  return a.q === b.q && a.r === b.r;
}

export function coordDistance(a: AxialCoord, b: AxialCoord): number {
  return (Math.abs(a.q - b.q) + Math.abs(a.r - b.r) + Math.abs(a.q + a.r - b.q - b.r)) / 2;
}

export function coordNeighbors(c: AxialCoord): AxialCoord[] {
  const directions: AxialCoord[] = [
    { q: 1, r: 0 },
    { q: 1, r: -1 },
    { q: 0, r: -1 },
    { q: -1, r: 0 },
    { q: -1, r: 1 },
    { q: 0, r: 1 },
  ];
  return directions.map((d) => ({ q: c.q + d.q, r: c.r + d.r }));
}

// ============================================================================
// TERRAIN SYSTEM
// ============================================================================

export enum TerrainType {
  Plain = 'plain',
  Forest = 'forest',
  Mountain = 'mountain',
  Water = 'water',
  ResourceNode = 'resource_node',
}

export interface Terrain {
  type: TerrainType;
  movementCost: number;
  defenseBonus: number;
}

const TERRAIN_PROPERTIES: Record<TerrainType, Terrain> = {
  [TerrainType.Plain]: { type: TerrainType.Plain, movementCost: 1, defenseBonus: 0 },
  [TerrainType.Forest]: { type: TerrainType.Forest, movementCost: 2, defenseBonus: 2 },
  [TerrainType.Mountain]: { type: TerrainType.Mountain, movementCost: 3, defenseBonus: 3 },
  [TerrainType.Water]: { type: TerrainType.Water, movementCost: Infinity, defenseBonus: 0 },
  [TerrainType.ResourceNode]: { type: TerrainType.ResourceNode, movementCost: 1, defenseBonus: 1 },
};

export function getTerrain(type: TerrainType): Terrain {
  return TERRAIN_PROPERTIES[type];
}

// ============================================================================
// UNIT SYSTEM
// ============================================================================

export enum UnitType {
  Infantry = 'infantry',
  Cavalry = 'cavalry',
  Archer = 'archer',
}

export enum PlayerSide {
  Player1 = 'player1',
  Player2 = 'player2',
}

export interface UnitStats {
  type: UnitType;
  health: number;
  maxHealth: number;
  attack: number;
  defense: number;
  movement: number;
  vision: number;
  cost: number;
}

const UNIT_TEMPLATES: Record<UnitType, Omit<UnitStats, 'health' | 'maxHealth'>> = {
  [UnitType.Infantry]: {
    type: UnitType.Infantry,
    attack: 5,
    defense: 3,
    movement: 3,
    vision: 3,
    cost: 100,
  },
  [UnitType.Cavalry]: {
    type: UnitType.Cavalry,
    attack: 4,
    defense: 2,
    movement: 5,
    vision: 4,
    cost: 150,
  },
  [UnitType.Archer]: {
    type: UnitType.Archer,
    attack: 6,
    defense: 1,
    movement: 3,
    vision: 5,
    cost: 120,
  },
};

export interface Unit extends UnitStats {
  id: string;
  owner: PlayerSide;
  position: AxialCoord;
  moved: boolean;
  attacked: boolean;
}

let unitIdCounter = 0;

export function createUnit(type: UnitType, owner: PlayerSide, position: AxialCoord): Unit {
  const template = UNIT_TEMPLATES[type];
  const health = 10;
  return {
    id: `unit_${++unitIdCounter}`,
    ...template,
    owner,
    position,
    health,
    maxHealth: health,
    moved: false,
    attacked: false,
  };
}

// ============================================================================
// MAP SYSTEM
// ============================================================================

export interface MapTile {
  coord: AxialCoord;
  terrain: TerrainType;
  unit?: Unit;
  resourceOwner?: PlayerSide; // For resource nodes
}

export class GameMap {
  private tiles: Map<string, MapTile> = new Map();
  width: number;
  height: number;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    this.initializeMap();
  }

  private coordKey(c: AxialCoord): string {
    return `${c.q},${c.r}`;
  }

  private initializeMap(): void {
    for (let q = 0; q < this.width; q++) {
      for (let r = 0; r < this.height; r++) {
        const c = coord(q, r);
        const terrain = this.randomTerrain(q, r);
        this.tiles.set(this.coordKey(c), {
          coord: c,
          terrain,
        });
      }
    }
  }

  private randomTerrain(q: number, r: number): TerrainType {
    const rand = Math.random();
    // Biased toward plains
    if (rand < 0.5) return TerrainType.Plain;
    if (rand < 0.7) return TerrainType.Forest;
    if (rand < 0.85) return TerrainType.Mountain;
    if (rand < 0.95) return TerrainType.Water;
    return TerrainType.ResourceNode;
  }

  getTile(c: AxialCoord): MapTile | undefined {
    return this.tiles.get(this.coordKey(c));
  }

  isValid(c: AxialCoord): boolean {
    return this.getTile(c) !== undefined;
  }

  placeUnit(unit: Unit, position: AxialCoord): boolean {
    const tile = this.getTile(position);
    if (!tile || tile.unit || tile.terrain === TerrainType.Water) {
      return false;
    }
    unit.position = position;
    tile.unit = unit;
    return true;
  }

  removeUnit(c: AxialCoord): Unit | undefined {
    const tile = this.getTile(c);
    if (!tile) return undefined;
    const unit = tile.unit;
    tile.unit = undefined;
    return unit;
  }

  getUnitsForPlayer(player: PlayerSide): Unit[] {
    const units: Unit[] = [];
    this.tiles.forEach((tile) => {
      if (tile.unit && tile.unit.owner === player) {
        units.push(tile.unit);
      }
    });
    return units;
  }

  getAllTiles(): MapTile[] {
    return Array.from(this.tiles.values());
  }
}

// ============================================================================
// GAME STATE AND LOGIC
// ============================================================================

export enum GamePhase {
  PlayerTurn = 'player_turn',
  AITurn = 'ai_turn',
  GameOver = 'game_over',
}

export interface GameState {
  phase: GamePhase;
  currentPlayer: PlayerSide;
  turn: number;
  map: GameMap;
  player1Resources: number;
  player2Resources: number;
  player1VictoryPoints: number;
  player2VictoryPoints: number;
  gameOverWinner?: PlayerSide;
}

export class Game {
  state: GameState;

  constructor(mapWidth: number = 12, mapHeight: number = 12) {
    this.state = {
      phase: GamePhase.PlayerTurn,
      currentPlayer: PlayerSide.Player1,
      turn: 1,
      map: new GameMap(mapWidth, mapHeight),
      player1Resources: 500,
      player2Resources: 500,
      player1VictoryPoints: 0,
      player2VictoryPoints: 0,
    };
    this.initializeUnits();
  }

  private initializeUnits(): void {
    const map = this.state.map;

    // Player 1 units (top left)
    const p1infantry = createUnit(UnitType.Infantry, PlayerSide.Player1, coord(1, 1));
    const p1cavalry = createUnit(UnitType.Cavalry, PlayerSide.Player1, coord(0, 2));
    const p1archer = createUnit(UnitType.Archer, PlayerSide.Player1, coord(2, 0));

    map.placeUnit(p1infantry, p1infantry.position);
    map.placeUnit(p1cavalry, p1cavalry.position);
    map.placeUnit(p1archer, p1archer.position);

    // Player 2 units (bottom right)
    const p2infantry = createUnit(UnitType.Infantry, PlayerSide.Player2, coord(10, 10));
    const p2cavalry = createUnit(UnitType.Cavalry, PlayerSide.Player2, coord(11, 9));
    const p2archer = createUnit(UnitType.Archer, PlayerSide.Player2, coord(9, 11));

    map.placeUnit(p2infantry, p2infantry.position);
    map.placeUnit(p2cavalry, p2cavalry.position);
    map.placeUnit(p2archer, p2archer.position);
  }

  canMoveUnit(unit: Unit, targetCoord: AxialCoord): boolean {
    if (unit.moved) return false;
    if (!this.state.map.isValid(targetCoord)) return false;

    const distance = coordDistance(unit.position, targetCoord);
    if (distance > unit.movement) return false;

    const tile = this.state.map.getTile(targetCoord);
    if (!tile || tile.unit) return false;
    if (tile.terrain === TerrainType.Water) return false;

    return true;
  }

  moveUnit(unit: Unit, targetCoord: AxialCoord): boolean {
    if (!this.canMoveUnit(unit, targetCoord)) return false;

    this.state.map.removeUnit(unit.position);
    this.state.map.placeUnit(unit, targetCoord);
    unit.moved = true;
    return true;
  }

  canAttackUnit(attacker: Unit, defender: Unit): boolean {
    if (attacker.attacked) return false;
    if (attacker.owner === defender.owner) return false;

    const distance = coordDistance(attacker.position, defender.position);
    const range = attacker.type === UnitType.Archer ? 3 : 1;
    return distance <= range;
  }

  attackUnit(attacker: Unit, defender: Unit): number {
    if (!this.canAttackUnit(attacker, defender)) return 0;

    const tile = this.state.map.getTile(defender.position);
    const terrainBonus = tile ? getTerrain(tile.terrain).defenseBonus : 0;

    const damage = Math.max(1, attacker.attack - (defender.defense + terrainBonus));
    defender.health -= damage;
    attacker.attacked = true;

    if (defender.health <= 0) {
      this.state.map.removeUnit(defender.position);
    }

    return damage;
  }

  endTurn(): void {
    // Reset unit states
    const current = this.state.currentPlayer;
    const units = this.state.map.getUnitsForPlayer(current);
    units.forEach((u) => {
      u.moved = false;
      u.attacked = false;
    });

    // Collect resources from resource nodes
    this.collectResources();

    // Update victory points from held resource nodes
    this.updateVictoryPoints();

    // Switch turns
    if (this.state.currentPlayer === PlayerSide.Player1) {
      this.state.currentPlayer = PlayerSide.Player2;
      this.state.phase = GamePhase.AITurn;
    } else {
      this.state.currentPlayer = PlayerSide.Player1;
      this.state.phase = GamePhase.PlayerTurn;
      this.state.turn++;
    }

    // Check win condition
    this.checkWinCondition();
  }

  private collectResources(): void {
    const resources = this.state.currentPlayer === PlayerSide.Player1 ? 10 : 10;
    if (this.state.currentPlayer === PlayerSide.Player1) {
      this.state.player1Resources += resources;
    } else {
      this.state.player2Resources += resources;
    }
  }

  private updateVictoryPoints(): void {
    // 1 point per resource node held
    let p1Points = 0;
    let p2Points = 0;

    this.state.map.getAllTiles().forEach((tile) => {
      if (tile.terrain === TerrainType.ResourceNode && tile.unit) {
        if (tile.unit.owner === PlayerSide.Player1) {
          p1Points++;
        } else {
          p2Points++;
        }
      }
    });

    this.state.player1VictoryPoints = p1Points;
    this.state.player2VictoryPoints = p2Points;
  }

  private checkWinCondition(): void {
    const p1Units = this.state.map.getUnitsForPlayer(PlayerSide.Player1).length;
    const p2Units = this.state.map.getUnitsForPlayer(PlayerSide.Player2).length;

    if (p1Units === 0) {
      this.state.phase = GamePhase.GameOver;
      this.state.gameOverWinner = PlayerSide.Player2;
    } else if (p2Units === 0) {
      this.state.phase = GamePhase.GameOver;
      this.state.gameOverWinner = PlayerSide.Player1;
    } else if (this.state.turn >= 50) {
      // Turn limit for draw prevention
      this.state.phase = GamePhase.GameOver;
      if (this.state.player1VictoryPoints > this.state.player2VictoryPoints) {
        this.state.gameOverWinner = PlayerSide.Player1;
      } else if (this.state.player2VictoryPoints > this.state.player1VictoryPoints) {
        this.state.gameOverWinner = PlayerSide.Player2;
      }
    }
  }

  getGameStatus(): string {
    if (this.state.phase === GamePhase.GameOver) {
      if (this.state.gameOverWinner) {
        return `Game Over! ${this.state.gameOverWinner} wins!`;
      }
      return 'Game Over! Draw!';
    }
    return `Turn ${this.state.turn} - ${this.state.currentPlayer}'s turn`;
  }
}

export default Game;
