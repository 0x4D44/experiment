/**
 * Main game state and flow management
 */

import { GameLevel, GameScore, AICommand } from './maze-types';
import { LevelManager } from './level-manager';
import { Maze } from './maze-grid';
import { AIEngine } from './ai-engine';

export class GameManager {
  private levelManager: LevelManager;
  private currentLevelId: number = 1;
  private currentLevel: GameLevel | null = null;
  private currentMaze: Maze | null = null;
  private currentAI: AIEngine | null = null;
  private scores: Map<number, GameScore> = new Map();
  private startTime: number = 0;

  constructor() {
    this.levelManager = new LevelManager();
    this.loadLevel(1);
  }

  /**
   * Load a specific level
   */
  public loadLevel(levelId: number): boolean {
    const level = this.levelManager.getLevel(levelId);
    if (!level) {
      return false;
    }

    this.currentLevelId = levelId;
    this.currentLevel = level;

    // Reconstruct maze from grid
    const grid = level.maze;
    this.currentMaze = new Maze(grid.width, grid.height);

    // Copy all cells
    for (let y = 0; y < grid.height; y++) {
      for (let x = 0; x < grid.width; x++) {
        const srcCell = grid.cells[y][x];
        const pos = { x, y };

        // Copy cell types
        if (srcCell.type !== 0) {
          this.currentMaze.getCell(pos)!.type = srcCell.type;
          if (srcCell.keyId !== undefined) {
            this.currentMaze.getCell(pos)!.keyId = srcCell.keyId;
          }
          if (srcCell.teleportTarget) {
            this.currentMaze.getCell(pos)!.teleportTarget = { ...srcCell.teleportTarget };
          }
        }
      }
    }

    // Create AI engine
    this.currentAI = new AIEngine(this.currentMaze, grid.startPos, level.maxSteps);
    this.startTime = Date.now();

    return true;
  }

  /**
   * Execute a single command on the current AI
   */
  public executeCommand(command: AICommand): boolean {
    if (!this.currentAI || !this.currentLevel) {
      return false;
    }

    const step = this.currentAI.executeCommand(command);
    return step.success;
  }

  /**
   * Execute multiple commands
   */
  public executeProgram(commands: AICommand[]): void {
    if (!this.currentAI || !this.currentLevel) {
      return;
    }

    for (const command of commands) {
      if (this.currentAI.isFinished()) {
        break;
      }
      this.executeCommand(command);
    }
  }

  /**
   * Reset current level
   */
  public resetLevel(): void {
    if (this.currentAI) {
      this.currentAI.reset();
      this.startTime = Date.now();
    }
  }

  /**
   * Get current level
   */
  public getCurrentLevel(): GameLevel | null {
    return this.currentLevel;
  }

  /**
   * Get current AI state
   */
  public getCurrentAIState() {
    return this.currentAI?.getState();
  }

  /**
   * Get current level ID
   */
  public getCurrentLevelId(): number {
    return this.currentLevelId;
  }

  /**
   * Get all levels
   */
  public getAllLevels(): GameLevel[] {
    return this.levelManager.getAllLevels();
  }

  /**
   * Check if current level is completed
   */
  public isLevelComplete(): boolean {
    return this.currentAI?.isGoalReached() ?? false;
  }

  /**
   * Get current score
   */
  public getCurrentScore(): GameScore | null {
    if (!this.currentAI || !this.currentLevel) {
      return null;
    }

    const timeTaken = (Date.now() - this.startTime) / 1000;
    const stepsTaken = this.currentAI.getStepCount();
    const reachedGoal = this.currentAI.isGoalReached();

    // Calculate efficiency score (0-100)
    let efficiency = 0;
    if (reachedGoal) {
      const maxSteps = this.currentLevel.maxSteps;
      const maxTime = this.currentLevel.maxTime;
      const stepRatio = Math.min(stepsTaken / maxSteps, 1.0);
      const timeRatio = Math.min(timeTaken / maxTime, 1.0);
      efficiency = Math.max(0, 100 - (stepRatio * 50 + timeRatio * 50));
    }

    return {
      levelId: this.currentLevel.id,
      stepsTaken,
      timeTaken,
      reachedGoal,
      efficiency,
    };
  }

  /**
   * Save score for level
   */
  public saveScore(levelId: number, score: GameScore): void {
    // Only save if better than previous
    const existing = this.scores.get(levelId);
    if (!existing || score.efficiency > existing.efficiency) {
      this.scores.set(levelId, score);
    }
  }

  /**
   * Get score for level
   */
  public getScore(levelId: number): GameScore | undefined {
    return this.scores.get(levelId);
  }

  /**
   * Get all scores
   */
  public getAllScores(): Map<number, GameScore> {
    return new Map(this.scores);
  }

  /**
   * Get AI execution log
   */
  public getExecutionLog() {
    return this.currentAI?.getLog();
  }

  /**
   * Sense wall ahead in current AI
   */
  public senseWallAhead(): boolean {
    return this.currentAI?.senseWallAhead() ?? false;
  }

  /**
   * Get total steps taken
   */
  public getStepCount(): number {
    return this.currentAI?.getStepCount() ?? 0;
  }

  /**
   * Get total levels
   */
  public getTotalLevels(): number {
    return this.levelManager.getTotalLevels();
  }
}
