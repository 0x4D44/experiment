/**
 * Core type definitions for Maze Runner AI game
 */

export type Direction = 'N' | 'E' | 'S' | 'W';

export enum CellType {
  Empty = 0,
  Wall = 1,
  Goal = 2,
  Door = 4,
  Key = 8,
  Teleporter = 16,
  StartPosition = 32,
}

export enum AICommand {
  Forward = 'FORWARD',
  TurnLeft = 'TURN_LEFT',
  TurnRight = 'TURN_RIGHT',
  SenseWall = 'SENSE_WALL',
  MarkPath = 'MARK_PATH',
  PickupKey = 'PICKUP_KEY',
  UseDoor = 'USE_DOOR',
  Wait = 'WAIT',
}

export interface Position {
  x: number;
  y: number;
}

export interface Cell {
  type: number; // Bitmask of CellType
  keyId?: number; // Which key this door/pickup requires
  teleportTarget?: Position; // Where teleporter goes
}

export interface MazeGrid {
  width: number;
  height: number;
  cells: Cell[][];
  startPos: Position;
  goalPos: Position;
}

export interface AIState {
  position: Position;
  direction: Direction;
  keysHeld: Set<number>;
  stepCount: number;
  markedCells: Set<string>; // Serialized positions
  finished: boolean;
  reachedGoal: boolean;
}

export interface GameLevel {
  id: number;
  name: string;
  description: string;
  maze: MazeGrid;
  maxSteps: number;
  maxTime: number; // seconds
  difficulty: 'Easy' | 'Medium' | 'Hard' | 'Expert';
}

export interface GameScore {
  levelId: number;
  stepsTaken: number;
  timeTaken: number;
  reachedGoal: boolean;
  efficiency: number; // 0-100
}

export interface ExecutionStep {
  command: AICommand;
  position: Position;
  direction: Direction;
  keysHeld: number[];
  success: boolean;
  error?: string;
}
