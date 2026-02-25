/**
 * Pulse Navigator - Audio Echolocation Rhythm Game
 * Navigate through darkness using only sonar pulses and rhythm
 */

import * as readline from 'readline';
import { GameController, GameState } from './game/GameController.js';

class GameInterface {
  private gameController: GameController;
  private updateInterval: NodeJS.Timeout | null = null;
  private rl: readline.Interface;

  constructor() {
    this.gameController = new GameController();
    this.rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    // Setup key handling
    this.setupKeyHandling();
  }

  /**
   * Setup keyboard input handling
   */
  private setupKeyHandling(): void {
    if (process.stdin.isTTY) {
      readline.emitKeypressEvents(process.stdin);
      process.stdin.setRawMode(true);
    }

    process.stdin.on('keypress', (_str, key) => {
      if (key.ctrl && key.name === 'c') {
        this.cleanup();
        process.exit();
      }

      this.handleKeyPress(key.name);
    });
  }

  /**
   * Handle key press
   */
  private handleKeyPress(keyName: string): void {
    const state = this.gameController.getGameState();

    if (state === GameState.MENU) {
      if (keyName === 'return' || keyName === 'space') {
        this.startGame();
      }
    } else if (state === GameState.PLAYING) {
      switch (keyName) {
        case 'space':
          this.gameController.handleSonarPulse();
          this.displayStatus();
          break;
        case 'w':
        case 'up':
          this.gameController.queueMovement('up');
          break;
        case 's':
        case 'down':
          this.gameController.queueMovement('down');
          break;
        case 'a':
        case 'left':
          this.gameController.queueMovement('left');
          break;
        case 'd':
        case 'right':
          this.gameController.queueMovement('right');
          break;
        case 'p':
          this.gameController.pauseGame();
          this.displayStatus();
          break;
      }
    } else if (state === GameState.PAUSED) {
      if (keyName === 'p') {
        this.gameController.resumeGame();
        this.displayStatus();
      }
    } else if (state === GameState.GAME_OVER || state === GameState.VICTORY) {
      if (keyName === 'r') {
        this.startGame();
      } else if (keyName === 'q') {
        this.cleanup();
        process.exit();
      }
    }
  }

  /**
   * Start the game
   */
  private async startGame(): Promise<void> {
    console.clear();
    console.log('Initializing audio engine...\n');

    try {
      await this.gameController.initialize();
      this.gameController.startGame();

      // Start update loop
      this.updateInterval = setInterval(() => {
        this.gameController.update();
        this.checkGameOver();
      }, 1000 / 60); // 60 FPS

      this.displayStatus();
    } catch (error) {
      console.error('Failed to initialize game:', error);
      this.displayMenu();
    }
  }

  /**
   * Check if game is over
   */
  private checkGameOver(): void {
    const state = this.gameController.getGameState();

    if (state === GameState.GAME_OVER || state === GameState.VICTORY) {
      if (this.updateInterval) {
        clearInterval(this.updateInterval);
        this.updateInterval = null;
      }
      this.displayGameOver();
    }
  }

  /**
   * Display main menu
   */
  displayMenu(): void {
    console.clear();
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║                    PULSE NAVIGATOR                         ║');
    console.log('║            Audio Echolocation Rhythm Game                  ║');
    console.log('╚════════════════════════════════════════════════════════════╝');
    console.log('');
    console.log('MISSION:');
    console.log('  Navigate through complete darkness using only sonar pulses.');
    console.log('  Find all objectives while maintaining your rhythm.');
    console.log('');
    console.log('CONTROLS:');
    console.log('  SPACE    - Send sonar pulse (maintain rhythm!)');
    console.log('  W/A/S/D  - Move (up/left/down/right)');
    console.log('  P        - Pause');
    console.log('  Ctrl+C   - Quit');
    console.log('');
    console.log('MECHANICS:');
    console.log('  • Press SPACE rhythmically to send sonar pulses');
    console.log('  • Perfect rhythm = more energy and higher scores');
    console.log('  • Better rhythm = longer sonar range');
    console.log('  • Listen for echoes to navigate');
    console.log('  • Movement costs energy');
    console.log('  • Find all objectives to win!');
    console.log('');
    console.log('AUDIO CUES:');
    console.log('  • High pitch echoes = objectives (your goal!)');
    console.log('  • Medium pitch echoes = obstacles (avoid!)');
    console.log('  • Low pitch echoes = walls (boundaries)');
    console.log('  • Stereo panning = direction of objects');
    console.log('');
    console.log('Press ENTER or SPACE to start...');
  }

  /**
   * Display game status
   */
  private displayStatus(): void {
    const status = this.gameController.getStatus();
    const stats = this.gameController.getTimingStatistics();

    console.clear();
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║                    PULSE NAVIGATOR                         ║');
    console.log('╚════════════════════════════════════════════════════════════╝');
    console.log('');

    if (status.state === GameState.PAUSED) {
      console.log('                        ⏸ PAUSED ⏸');
      console.log('');
      console.log('Press P to resume...');
      console.log('');
    }

    console.log('STATUS:');
    console.log(`  Objectives Remaining: ${status.objectivesRemaining}`);
    console.log(`  Score: ${status.score}`);
    console.log(`  Combo: ${status.combo}x (Max: ${status.maxCombo}x)`);
    console.log('');

    console.log('ENERGY:');
    const energyBar = this.createBar(status.energyPercentage, 40);
    console.log(`  ${energyBar} ${Math.floor(status.energy)}/100`);
    console.log('');

    console.log('RHYTHM STATS:');
    console.log(`  Tempo: ${status.tempo ? status.tempo.toFixed(1) + ' BPM' : 'Establishing...'}`);
    console.log(`  Consistency: ${(status.consistency * 100).toFixed(1)}%`);
    console.log(`  Avg Deviation: ±${status.averageDeviation.toFixed(1)}ms`);
    console.log('');

    console.log('TIMING BREAKDOWN:');
    console.log(`  Perfect: ${stats.perfectBeats}`);
    console.log(`  Good: ${stats.goodBeats}`);
    console.log(`  OK: ${stats.okBeats}`);
    console.log(`  Miss: ${stats.missedBeats}`);
    console.log(`  Total Beats: ${stats.totalBeats}`);
    console.log('');

    console.log('CONTROLS: SPACE=Pulse | WASD=Move | P=Pause');
    console.log('');
    console.log('TIP: Press SPACE rhythmically to maintain energy!');
  }

  /**
   * Display game over screen
   */
  private displayGameOver(): void {
    const status = this.gameController.getStatus();
    const stats = this.gameController.getTimingStatistics();

    console.clear();
    console.log('╔════════════════════════════════════════════════════════════╗');

    if (status.state === GameState.VICTORY) {
      console.log('║                      🎉 VICTORY! 🎉                        ║');
    } else {
      console.log('║                      GAME OVER                             ║');
    }

    console.log('╚════════════════════════════════════════════════════════════╝');
    console.log('');

    console.log('FINAL SCORE:');
    console.log(`  Score: ${status.score}`);
    console.log(`  Grade: ${status.grade}`);
    console.log(`  Max Combo: ${status.maxCombo}x`);
    console.log('');

    console.log('TIMING PERFORMANCE:');
    console.log(`  Total Beats: ${stats.totalBeats}`);
    console.log(`  Perfect: ${stats.perfectBeats} (${((stats.perfectBeats / stats.totalBeats) * 100).toFixed(1)}%)`);
    console.log(`  Good: ${stats.goodBeats} (${((stats.goodBeats / stats.totalBeats) * 100).toFixed(1)}%)`);
    console.log(`  OK: ${stats.okBeats} (${((stats.okBeats / stats.totalBeats) * 100).toFixed(1)}%)`);
    console.log(`  Miss: ${stats.missedBeats} (${((stats.missedBeats / stats.totalBeats) * 100).toFixed(1)}%)`);
    console.log('');

    console.log(`  Average Tempo: ${stats.currentTempo ? stats.currentTempo.toFixed(1) + ' BPM' : 'N/A'}`);
    console.log(`  Consistency: ${(stats.consistency * 100).toFixed(1)}%`);
    console.log(`  Average Deviation: ±${stats.averageDeviation.toFixed(1)}ms`);
    console.log('');

    console.log('Press R to play again | Q to quit');
  }

  /**
   * Create a progress bar
   */
  private createBar(percentage: number, length: number): string {
    const filled = Math.floor(percentage * length);
    const empty = length - filled;
    return '[' + '█'.repeat(filled) + '░'.repeat(empty) + ']';
  }

  /**
   * Cleanup resources
   */
  private async cleanup(): Promise<void> {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
    }

    await this.gameController.cleanup();
    this.rl.close();

    if (process.stdin.isTTY) {
      process.stdin.setRawMode(false);
    }
  }

  /**
   * Run the game
   */
  async run(): Promise<void> {
    this.displayMenu();
  }
}

// Start the game
const game = new GameInterface();
game.run().catch(console.error);
