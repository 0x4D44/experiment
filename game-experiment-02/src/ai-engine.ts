/**
 * AI interpreter and execution engine
 */

import { AICommand, AIState, Direction, Position, ExecutionStep, CellType } from './maze-types';
import { Maze } from './maze-grid';

export class AIEngine {
  private state: AIState;
  private maze: Maze;
  private maxSteps: number;
  private executionLog: ExecutionStep[] = [];

  constructor(maze: Maze, startPos: Position, maxSteps: number = 500) {
    this.maze = maze;
    this.maxSteps = maxSteps;
    this.state = {
      position: { ...startPos },
      direction: 'N', // Facing north
      keysHeld: new Set(),
      stepCount: 0,
      markedCells: new Set(),
      finished: false,
      reachedGoal: false,
    };
  }

  /**
   * Execute a single command
   */
  public executeCommand(command: AICommand): ExecutionStep {
    const step: ExecutionStep = {
      command,
      position: { ...this.state.position },
      direction: this.state.direction,
      keysHeld: Array.from(this.state.keysHeld),
      success: false,
    };

    // Check if already finished
    if (this.state.finished) {
      step.error = 'Execution already finished';
      return step;
    }

    // Check step limit
    if (this.state.stepCount >= this.maxSteps) {
      step.error = 'Step limit exceeded';
      this.state.finished = true;
      return step;
    }

    try {
      switch (command) {
        case AICommand.Forward:
          this.executeForward();
          break;
        case AICommand.TurnLeft:
          this.executeTurnLeft();
          break;
        case AICommand.TurnRight:
          this.executeTurnRight();
          break;
        case AICommand.SenseWall:
          // This just returns success; actual sensing is done differently
          break;
        case AICommand.MarkPath:
          this.executeMarkPath();
          break;
        case AICommand.PickupKey:
          this.executePickupKey();
          break;
        case AICommand.UseDoor:
          this.executeUseDoor();
          break;
        case AICommand.Wait:
          // Just increment step count
          break;
      }

      // Check if reached goal
      const goalPos = this.maze.getGoal();
      if (this.state.position.x === goalPos.x && this.state.position.y === goalPos.y) {
        this.state.reachedGoal = true;
        this.state.finished = true;
      }

      this.state.stepCount++;
      step.success = true;
      step.position = { ...this.state.position };
      step.direction = this.state.direction;
      step.keysHeld = Array.from(this.state.keysHeld);
    } catch (error) {
      step.error = String(error);
      this.state.finished = true;
    }

    this.executionLog.push(step);
    return step;
  }

  /**
   * Execute forward movement
   */
  private executeForward(): void {
    const nextPos = this.getNextPosition();

    if (!this.maze.isValidPosition(nextPos)) {
      throw new Error('Cannot move outside maze boundaries');
    }

    // Check for walls
    if (this.maze.isWall(nextPos)) {
      throw new Error('Cannot move through wall');
    }

    // Check for doors
    if (this.maze.hasType(nextPos, CellType.Door)) {
      const cell = this.maze.getCell(nextPos);
      if (cell && cell.keyId !== undefined && !this.state.keysHeld.has(cell.keyId)) {
        throw new Error(`Need key ${cell.keyId} to open door`);
      }
    }

    // Check for teleporter
    if (this.maze.hasType(nextPos, CellType.Teleporter)) {
      const cell = this.maze.getCell(nextPos);
      if (cell && cell.teleportTarget) {
        this.state.position = { ...cell.teleportTarget };
        return;
      }
    }

    this.state.position = nextPos;
  }

  /**
   * Execute turn left
   */
  private executeTurnLeft(): void {
    const turns = { N: 'W', W: 'S', S: 'E', E: 'N' } as Record<Direction, Direction>;
    this.state.direction = turns[this.state.direction];
  }

  /**
   * Execute turn right
   */
  private executeTurnRight(): void {
    const turns = { N: 'E', E: 'S', S: 'W', W: 'N' } as Record<Direction, Direction>;
    this.state.direction = turns[this.state.direction];
  }

  /**
   * Mark current cell
   */
  private executeMarkPath(): void {
    const key = `${this.state.position.x},${this.state.position.y}`;
    this.state.markedCells.add(key);
  }

  /**
   * Pickup a key
   */
  private executePickupKey(): void {
    const cell = this.maze.getCell(this.state.position);
    if (cell && this.maze.hasType(this.state.position, CellType.Key) && cell.keyId !== undefined) {
      this.state.keysHeld.add(cell.keyId);
    } else {
      throw new Error('No key at current position');
    }
  }

  /**
   * Use door
   */
  private executeUseDoor(): void {
    // Doors are checked during movement, this is a no-op
    // Could be used for interactive door mechanics in future
  }

  /**
   * Sense wall in current direction
   */
  public senseWallAhead(): boolean {
    const nextPos = this.getNextPosition();
    return this.maze.isWall(nextPos) || !this.maze.isValidPosition(nextPos);
  }

  /**
   * Get next position based on current direction
   */
  private getNextPosition(): Position {
    const { x, y } = this.state.position;
    switch (this.state.direction) {
      case 'N':
        return { x, y: y - 1 };
      case 'E':
        return { x: x + 1, y };
      case 'S':
        return { x, y: y + 1 };
      case 'W':
        return { x: x - 1, y };
    }
  }

  /**
   * Get current state
   */
  public getState(): Readonly<AIState> {
    return Object.freeze({ ...this.state });
  }

  /**
   * Get execution log
   */
  public getLog(): readonly ExecutionStep[] {
    return this.executionLog;
  }

  /**
   * Reset to initial state
   */
  public reset(): void {
    const startPos = this.maze.getStart();
    this.state = {
      position: { ...startPos },
      direction: 'N',
      keysHeld: new Set(),
      stepCount: 0,
      markedCells: new Set(),
      finished: false,
      reachedGoal: false,
    };
    this.executionLog = [];
  }

  /**
   * Check if AI reached the goal
   */
  public isGoalReached(): boolean {
    return this.state.reachedGoal;
  }

  /**
   * Check if execution is finished
   */
  public isFinished(): boolean {
    return this.state.finished;
  }

  /**
   * Get steps taken
   */
  public getStepCount(): number {
    return this.state.stepCount;
  }
}
