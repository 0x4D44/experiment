/**
 * Main game controller that coordinates all game systems
 */

import { TimingEngine, HitAccuracy } from '../timing/TimingEngine.js';
import { AudioEngine } from '../audio/AudioEngine.js';
import { GameWorld } from './GameWorld.js';
import { RhythmScorer } from './RhythmScorer.js';

export enum GameState {
  MENU = 'MENU',
  PLAYING = 'PLAYING',
  PAUSED = 'PAUSED',
  GAME_OVER = 'GAME_OVER',
  VICTORY = 'VICTORY',
}

export interface GameStatus {
  state: GameState;
  score: number;
  combo: number;
  maxCombo: number;
  energy: number;
  energyPercentage: number;
  objectivesRemaining: number;
  tempo: number | null;
  consistency: number;
  averageDeviation: number;
  grade: string;
}

export class GameController {
  private timingEngine: TimingEngine;
  private audioEngine: AudioEngine;
  private gameWorld: GameWorld;
  private rhythmScorer: RhythmScorer;
  private gameState: GameState = GameState.MENU;
  private lastUpdateTime: number = 0;
  private moveSpeed: number = 2.0; // units per second
  private movementQueue: Array<{ dx: number; dy: number }> = [];

  constructor() {
    this.timingEngine = new TimingEngine();
    this.audioEngine = new AudioEngine();
    this.gameWorld = new GameWorld();
    this.rhythmScorer = new RhythmScorer();
  }

  /**
   * Initialize the game
   */
  async initialize(): Promise<void> {
    await this.audioEngine.initialize();
    this.lastUpdateTime = performance.now();
  }

  /**
   * Start a new game
   */
  startGame(): void {
    this.gameState = GameState.PLAYING;
    this.timingEngine.reset();
    this.rhythmScorer.reset();
    this.gameWorld = new GameWorld();
    this.lastUpdateTime = performance.now();
    this.movementQueue = [];
  }

  /**
   * Update game state (call this in game loop)
   */
  update(): void {
    if (this.gameState !== GameState.PLAYING) {
      return;
    }

    const currentTime = performance.now();
    const deltaTime = (currentTime - this.lastUpdateTime) / 1000; // Convert to seconds
    this.lastUpdateTime = currentTime;

    // Recharge energy over time
    this.rhythmScorer.rechargeEnergy(deltaTime);

    // Process movement queue
    this.processMovement(deltaTime);

    // Check win condition
    if (this.gameWorld.getRemainingObjectives() === 0) {
      this.gameState = GameState.VICTORY;
      this.audioEngine.playObjectiveReached();
    }

    // Check lose condition (out of energy)
    if (this.rhythmScorer.getEnergyPercentage() <= 0) {
      this.gameState = GameState.GAME_OVER;
      this.audioEngine.playNegativeFeedback();
    }
  }

  /**
   * Process queued movements
   */
  private processMovement(deltaTime: number): void {
    if (this.movementQueue.length === 0) {
      return;
    }

    const movement = this.movementQueue.shift()!;
    const distance = this.moveSpeed * deltaTime;

    // Cost energy for movement
    const energyCost = distance * 2;
    if (!this.rhythmScorer.consumeEnergyForMovement(energyCost)) {
      // Not enough energy
      this.audioEngine.playNegativeFeedback();
      return;
    }

    // Try to move
    const success = this.gameWorld.movePlayer(movement, distance);

    if (!success) {
      // Collision
      this.audioEngine.playCollisionWarning();
    }
  }

  /**
   * Handle sonar pulse input
   */
  handleSonarPulse(): void {
    if (this.gameState !== GameState.PLAYING) {
      return;
    }

    // Register beat with timing engine
    const beatEvent = this.timingEngine.registerBeat();

    // Process beat for scoring
    const result = this.rhythmScorer.processBeat(beatEvent);

    // Play sonar pulse sound
    const intensity = this.rhythmScorer.getEnergyPercentage();
    this.audioEngine.playSonarPulse(intensity);

    // Get echoes from nearby objects
    const sonarRange = result.sonarRange;
    const echoes = this.gameWorld.getEchoes(sonarRange);

    // Play echo sounds
    this.audioEngine.playEchoes(echoes);

    // Play feedback based on accuracy
    if (beatEvent.accuracy === HitAccuracy.PERFECT) {
      // Play positive feedback for high combo
      if (this.rhythmScorer.getCombo() > 0 && this.rhythmScorer.getCombo() % 10 === 0) {
        this.audioEngine.playPositiveFeedback();
      }
    } else if (beatEvent.accuracy === HitAccuracy.MISS) {
      this.audioEngine.playNegativeFeedback();
    }

    // Check if objective reached
    if (this.gameWorld.checkObjectiveReached()) {
      this.audioEngine.playObjectiveReached();
      this.rhythmScorer.processBeat({
        timestamp: performance.now(),
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      }); // Bonus for reaching objective
    }
  }

  /**
   * Queue movement in a direction
   */
  queueMovement(direction: 'up' | 'down' | 'left' | 'right'): void {
    if (this.gameState !== GameState.PLAYING) {
      return;
    }

    let dx = 0;
    let dy = 0;

    switch (direction) {
      case 'up':
        dy = -1;
        break;
      case 'down':
        dy = 1;
        break;
      case 'left':
        dx = -1;
        break;
      case 'right':
        dx = 1;
        break;
    }

    this.movementQueue.push({ dx, dy });
  }

  /**
   * Get current game status
   */
  getStatus(): GameStatus {
    const stats = this.timingEngine.getStatistics();

    return {
      state: this.gameState,
      score: this.rhythmScorer.getScore(),
      combo: this.rhythmScorer.getCombo(),
      maxCombo: this.rhythmScorer.getMaxCombo(),
      energy: this.rhythmScorer.getEnergyState().current,
      energyPercentage: this.rhythmScorer.getEnergyPercentage(),
      objectivesRemaining: this.gameWorld.getRemainingObjectives(),
      tempo: stats.currentTempo,
      consistency: stats.consistency,
      averageDeviation: stats.averageDeviation,
      grade: this.rhythmScorer.getGrade(),
    };
  }

  /**
   * Get timing statistics
   */
  getTimingStatistics(): ReturnType<TimingEngine['getStatistics']> {
    return this.timingEngine.getStatistics();
  }

  /**
   * Pause game
   */
  pauseGame(): void {
    if (this.gameState === GameState.PLAYING) {
      this.gameState = GameState.PAUSED;
    }
  }

  /**
   * Resume game
   */
  resumeGame(): void {
    if (this.gameState === GameState.PAUSED) {
      this.gameState = GameState.PLAYING;
      this.lastUpdateTime = performance.now();
    }
  }

  /**
   * Get current game state
   */
  getGameState(): GameState {
    return this.gameState;
  }

  /**
   * Get player position (for debugging/visualization)
   */
  getPlayerPosition(): { x: number; y: number } {
    return this.gameWorld.getPlayerPosition();
  }

  /**
   * Get all game objects (for debugging/visualization)
   */
  getWorldObjects(): ReturnType<GameWorld['getAllObjects']> {
    return this.gameWorld.getAllObjects();
  }

  /**
   * Cleanup resources
   */
  async cleanup(): Promise<void> {
    await this.audioEngine.cleanup();
  }
}
