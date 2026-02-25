import { Vector2D } from '../utils/Vector2D';
import { GameState, HoleConfig } from '../types/Physics';
import { PhysicsEngine } from './PhysicsEngine';
import { HOLE_CONFIGURATIONS } from '../entities/HoleConfigurations';

export class GameManager {
  private physics: PhysicsEngine;
  private currentHoleIndex: number = 0;
  private gameState: GameState | null = null;
  private currentHoleConfig: HoleConfig | null = null;
  private totalScore: number = 0;
  private holeScores: number[] = [];
  private gameStarted: boolean = false;
  private gameOver: boolean = false;

  constructor() {
    this.physics = new PhysicsEngine({
      timeStep: 0.016,
      damping: 0.98,
      groundFriction: 0.05,
      gravityConstant: 500,
      maxVelocity: 500,
      stopThreshold: 0.1,
    });
  }

  /**
   * Start a new game
   */
  startGame(): void {
    this.gameStarted = true;
    this.gameOver = false;
    this.totalScore = 0;
    this.holeScores = [];
    this.currentHoleIndex = 0;
    this.loadHole(0);
  }

  /**
   * Load a specific hole
   */
  loadHole(holeIndex: number): void {
    if (holeIndex < 0 || holeIndex >= HOLE_CONFIGURATIONS.length) {
      this.gameOver = true;
      return;
    }

    this.currentHoleIndex = holeIndex;
    this.currentHoleConfig = HOLE_CONFIGURATIONS[holeIndex];

    // Initialize game state
    this.gameState = {
      ball: {
        position: this.currentHoleConfig.ballStartPos.clone(),
        velocity: new Vector2D(0, 0),
        mass: 1,
        radius: 5,
      },
      gravityWells: this.currentHoleConfig.initialGravityWells.map((w) => ({
        ...w,
        position: w.position.clone(),
      })),
      obstacles: this.currentHoleConfig.obstacles,
      wormholes: this.currentHoleConfig.wormholes,
      hole: this.currentHoleConfig.holePos.clone(),
      holeRadius: 15,
      groundFriction: 0.05,
      damping: 0.98,
      ballStopped: true,
      strokes: 0,
      gravityModifiersUsed: 0,
      maxGravityModifiers: this.currentHoleConfig.maxGravityModifiers,
      inHole: false,
      ballInMotion: false,
    };
  }

  /**
   * Get current game state
   */
  getGameState(): GameState {
    if (!this.gameState) {
      throw new Error('Game not initialized');
    }
    return this.gameState;
  }

  /**
   * Get current hole config
   */
  getCurrentHoleConfig(): HoleConfig {
    if (!this.currentHoleConfig) {
      throw new Error('No hole loaded');
    }
    return this.currentHoleConfig;
  }

  /**
   * Update game physics
   */
  update(): void {
    if (!this.gameState || !this.gameStarted || this.gameState.inHole) {
      return;
    }

    this.physics.update(this.gameState);

    // Check if ball has come to rest after hitting
    if (this.gameState.ballStopped && this.gameState.strokes > 0) {
      // Ball is resting, ready for next shot
    }
  }

  /**
   * Hit the ball
   */
  hitBall(angle: number, power: number): void {
    if (!this.gameState || this.gameState.inHole) {
      return;
    }

    const direction = Vector2D.fromAngle(angle);
    this.physics.applyImpulse(this.gameState, direction, power);
  }

  /**
   * Place a gravity modifier
   */
  placeGravityModifier(position: Vector2D, isRepulsive: boolean = false): boolean {
    if (!this.gameState) {
      return false;
    }

    const strength = isRepulsive ? -300 : 300;
    const radius = 150;

    return this.physics.placeGravityWell(this.gameState, position, strength, radius);
  }

  /**
   * Check if hole is complete
   */
  isHoleComplete(): boolean {
    return this.gameState?.inHole ?? false;
  }

  /**
   * Get current score for hole (strokes)
   */
  getCurrentStrokeCount(): number {
    return this.gameState?.strokes ?? 0;
  }

  /**
   * Get score relative to par
   */
  getScoreRelativeToPar(holeIndex?: number): number {
    const index = holeIndex ?? this.currentHoleIndex;
    const hole = HOLE_CONFIGURATIONS[index];
    const strokes = this.holeScores[index] ?? this.gameState?.strokes ?? 0;
    return strokes - hole.par;
  }

  /**
   * Finish current hole
   */
  finishHole(): void {
    if (!this.gameState || !this.currentHoleConfig) {
      return;
    }

    this.holeScores[this.currentHoleIndex] = this.gameState.strokes;
    this.totalScore += this.gameState.strokes;
  }

  /**
   * Move to next hole
   */
  nextHole(): void {
    this.finishHole();

    if (this.currentHoleIndex + 1 < HOLE_CONFIGURATIONS.length) {
      this.loadHole(this.currentHoleIndex + 1);
    } else {
      this.gameOver = true;
    }
  }

  /**
   * Restart current hole
   */
  restartHole(): void {
    this.loadHole(this.currentHoleIndex);
  }

  /**
   * Get trajectory prediction
   */
  getTrajectoryPreview(angle: number, power: number, steps: number = 100): Vector2D[] {
    if (!this.gameState) {
      return [];
    }

    // Clone state and apply impulse
    const testState = JSON.parse(JSON.stringify(this.gameState));
    testState.ball.position = this.gameState.ball.position.clone();
    testState.ball.velocity = this.gameState.ball.velocity.clone();

    // Convert to proper objects
    testState.ball.position = new Vector2D(testState.ball.position.x, testState.ball.position.y);
    testState.ball.velocity = new Vector2D(testState.ball.velocity.x, testState.ball.velocity.y);
    testState.hole = new Vector2D(testState.hole.x, testState.hole.y);

    testState.gravityWells = testState.gravityWells.map((w: any) => ({
      ...w,
      position: new Vector2D(w.position.x, w.position.y),
    }));

    // Apply impulse
    const direction = Vector2D.fromAngle(angle);
    this.physics.applyImpulse(testState, direction, power);

    // Get trajectory
    return this.physics.predictTrajectory(testState, steps).map((p) => p.position);
  }

  /**
   * Get total game score
   */
  getTotalScore(): number {
    return this.totalScore;
  }

  /**
   * Get total par for completed holes
   */
  getTotalPar(): number {
    let totalPar = 0;
    for (let i = 0; i < this.holeScores.length; i++) {
      totalPar += HOLE_CONFIGURATIONS[i].par;
    }
    return totalPar;
  }

  /**
   * Check if game is over
   */
  isGameOver(): boolean {
    return this.gameOver;
  }

  /**
   * Get current hole number (1-indexed)
   */
  getCurrentHoleNumber(): number {
    return this.currentHoleIndex + 1;
  }

  /**
   * Get total holes
   */
  getTotalHoles(): number {
    return HOLE_CONFIGURATIONS.length;
  }

  /**
   * Get all holes
   */
  getAllHoles(): HoleConfig[] {
    return HOLE_CONFIGURATIONS;
  }

  /**
   * Get hole scores
   */
  getHoleScores(): number[] {
    return this.holeScores;
  }

  /**
   * Check if ball is in motion
   */
  isBallInMotion(): boolean {
    return this.gameState?.ballInMotion ?? false;
  }

  /**
   * Check if ball has stopped
   */
  isBallStopped(): boolean {
    return this.gameState?.ballStopped ?? true;
  }

  /**
   * Get gravity modifiers remaining
   */
  getGravityModifiersRemaining(): number {
    if (!this.gameState) return 0;
    return this.gameState.maxGravityModifiers - this.gameState.gravityModifiersUsed;
  }

  /**
   * Get gravity wells on current hole
   */
  getGravityWells() {
    return this.gameState?.gravityWells ?? [];
  }
}
