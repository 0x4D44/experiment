/**
 * Particle Playground - Main Application
 * Entry point for the particle physics game
 */

import { ParticleGame, GameMode } from './particle-game';
import { ParticleRenderer } from './particle-renderer';
import { ParticleType, Vector2D } from './particle-physics';

/**
 * Main application class
 */
class ParticlePlaygroundApp {
  game: ParticleGame;
  renderer: ParticleRenderer;
  canvas: HTMLCanvasElement;
  gameLoop: number | null;
  fps: number;
  deltaTime: number;
  lastFrameTime: number;

  // UI Elements
  canvas_el!: HTMLCanvasElement;
  levelSelect!: HTMLSelectElement;
  modeGameBtn!: HTMLButtonElement;
  modeSandboxBtn!: HTMLButtonElement;
  pausePlayBtn!: HTMLButtonElement;
  prevLevelBtn!: HTMLButtonElement;
  nextLevelBtn!: HTMLButtonElement;
  resetLevelBtn!: HTMLButtonElement;
  clearAllBtn!: HTMLButtonElement;
  toggleTrailsBtn!: HTMLButtonElement;
  toggleDebugBtn!: HTMLButtonElement;
  spawnNeutralBtn!: HTMLButtonElement;
  spawnPositiveBtn!: HTMLButtonElement;
  spawnNegativeBtn!: HTMLButtonElement;
  clearParticlesBtn!: HTMLButtonElement;
  addAttractorBtn!: HTMLButtonElement;
  addRepulsorBtn!: HTMLButtonElement;
  toggleGravityBtn!: HTMLButtonElement;
  puzzleControls!: HTMLElement;
  sandboxControls!: HTMLElement;

  // State
  nextSpawnType: ParticleType | null;
  nextAttractorMode: boolean | null;
  currentGravity: boolean;

  constructor() {
    this.canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
    this.game = new ParticleGame(this.canvas.width, this.canvas.height);
    this.renderer = new ParticleRenderer(this.canvas);
    this.gameLoop = null;
    this.fps = 0;
    this.deltaTime = 0;
    this.lastFrameTime = Date.now();

    this.nextSpawnType = null;
    this.nextAttractorMode = null;
    this.currentGravity = true;

    this.initializeUI();
    this.setupEventListeners();
    this.start();
  }

  /**
   * Initialize UI elements
   */
  private initializeUI(): void {
    this.canvas_el = document.getElementById('gameCanvas') as HTMLCanvasElement;
    this.levelSelect = document.getElementById('levelSelect') as HTMLSelectElement;
    this.modeGameBtn = document.getElementById('modeGameBtn') as HTMLButtonElement;
    this.modeSandboxBtn = document.getElementById('modeSandboxBtn') as HTMLButtonElement;
    this.pausePlayBtn = document.getElementById('pausePlayBtn') as HTMLButtonElement;
    this.prevLevelBtn = document.getElementById('prevLevelBtn') as HTMLButtonElement;
    this.nextLevelBtn = document.getElementById('nextLevelBtn') as HTMLButtonElement;
    this.resetLevelBtn = document.getElementById('resetLevelBtn') as HTMLButtonElement;
    this.clearAllBtn = document.getElementById('clearAllBtn') as HTMLButtonElement;
    this.toggleTrailsBtn = document.getElementById('toggleTrailsBtn') as HTMLButtonElement;
    this.toggleDebugBtn = document.getElementById('toggleDebugBtn') as HTMLButtonElement;
    this.spawnNeutralBtn = document.getElementById('spawnNeutralBtn') as HTMLButtonElement;
    this.spawnPositiveBtn = document.getElementById('spawnPositiveBtn') as HTMLButtonElement;
    this.spawnNegativeBtn = document.getElementById('spawnNegativeBtn') as HTMLButtonElement;
    this.clearParticlesBtn = document.getElementById('clearParticlesBtn') as HTMLButtonElement;
    this.addAttractorBtn = document.getElementById('addAttractorBtn') as HTMLButtonElement;
    this.addRepulsorBtn = document.getElementById('addRepulsorBtn') as HTMLButtonElement;
    this.toggleGravityBtn = document.getElementById('toggleGravityBtn') as HTMLButtonElement;
    this.puzzleControls = document.getElementById('puzzleControls') as HTMLElement;
    this.sandboxControls = document.getElementById('sandboxControls') as HTMLElement;

    this.populateLevelSelect();
  }

  /**
   * Populate level selector
   */
  private populateLevelSelect(): void {
    this.levelSelect.innerHTML = '';
    for (let i = 0; i < this.game.levels.length; i++) {
      const option = document.createElement('option');
      option.value = String(i);
      option.textContent = `${i + 1}. ${this.game.levels[i].name}`;
      this.levelSelect.appendChild(option);
    }
  }

  /**
   * Setup event listeners
   */
  private setupEventListeners(): void {
    // Mode buttons
    this.modeGameBtn.addEventListener('click', () => this.setMode(GameMode.PUZZLE));
    this.modeSandboxBtn.addEventListener('click', () => this.setMode(GameMode.SANDBOX));

    // Puzzle controls
    this.levelSelect.addEventListener('change', (e) => {
      const levelIndex = parseInt((e.target as HTMLSelectElement).value);
      this.game.loadLevel(levelIndex);
    });

    this.prevLevelBtn.addEventListener('click', () => this.game.previousLevel());
    this.nextLevelBtn.addEventListener('click', () => this.game.nextLevel());
    this.resetLevelBtn.addEventListener('click', () => this.game.resetLevel());

    // Playback controls
    this.pausePlayBtn.addEventListener('click', () => this.togglePause());
    this.clearAllBtn.addEventListener('click', () => this.game.physics.clearParticles());

    // Display controls
    this.toggleTrailsBtn.addEventListener('click', () => this.toggleTrails());
    this.toggleDebugBtn.addEventListener('click', () => this.toggleDebug());

    // Sandbox controls
    this.spawnNeutralBtn.addEventListener('click', () => {
      this.nextSpawnType = ParticleType.NEUTRAL;
      this.updateSpawnButtonStates();
    });

    this.spawnPositiveBtn.addEventListener('click', () => {
      this.nextSpawnType = ParticleType.POSITIVE;
      this.updateSpawnButtonStates();
    });

    this.spawnNegativeBtn.addEventListener('click', () => {
      this.nextSpawnType = ParticleType.NEGATIVE;
      this.updateSpawnButtonStates();
    });

    this.clearParticlesBtn.addEventListener('click', () => this.game.physics.clearParticles());

    this.addAttractorBtn.addEventListener('click', () => {
      this.nextAttractorMode = true;
      this.updateAttractorButtonStates();
    });

    this.addRepulsorBtn.addEventListener('click', () => {
      this.nextAttractorMode = false;
      this.updateAttractorButtonStates();
    });

    this.toggleGravityBtn.addEventListener('click', () => this.toggleGravity());

    // Canvas click for sandbox
    this.canvas_el.addEventListener('click', (e) => this.handleCanvasClick(e));

    // Keyboard controls
    document.addEventListener('keydown', (e) => this.handleKeyDown(e));
  }

  /**
   * Handle canvas click
   */
  private handleCanvasClick(e: MouseEvent): void {
    const rect = this.canvas_el.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const pos = new Vector2D(x, y);

    if (this.game.mode === GameMode.SANDBOX) {
      // Spawn particle
      if (this.nextSpawnType !== null) {
        this.game.spawnParticle(pos, this.nextSpawnType);
        this.nextSpawnType = null;
        this.updateSpawnButtonStates();
      }

      // Add attractor
      if (this.nextAttractorMode !== null) {
        this.game.addAttractorInSandbox(pos, this.nextAttractorMode);
        this.nextAttractorMode = null;
        this.updateAttractorButtonStates();
      }
    }
  }

  /**
   * Handle keyboard input
   */
  private handleKeyDown(e: KeyboardEvent): void {
    switch (e.key.toLowerCase()) {
      case ' ':
        e.preventDefault();
        this.togglePause();
        break;
      case 'n':
        this.game.nextLevel();
        break;
      case 'p':
        this.game.previousLevel();
        break;
      case 'r':
        this.game.resetLevel();
        break;
      case 'c':
        this.game.physics.clearParticles();
        break;
      case 't':
        this.toggleTrails();
        break;
      case 'd':
        this.toggleDebug();
        break;
      case '1':
        this.nextSpawnType = ParticleType.NEUTRAL;
        this.updateSpawnButtonStates();
        break;
      case '2':
        this.nextSpawnType = ParticleType.POSITIVE;
        this.updateSpawnButtonStates();
        break;
      case '3':
        this.nextSpawnType = ParticleType.NEGATIVE;
        this.updateSpawnButtonStates();
        break;
      case 'a':
        this.nextAttractorMode = true;
        this.updateAttractorButtonStates();
        break;
      case 'x':
        this.nextAttractorMode = false;
        this.updateAttractorButtonStates();
        break;
    }
  }

  /**
   * Set game mode
   */
  private setMode(mode: GameMode): void {
    this.game.mode = mode;
    this.game.physics.clearParticles();
    this.game.physics.clearAll();

    if (mode === GameMode.PUZZLE) {
      this.modeGameBtn.classList.add('active');
      this.modeSandboxBtn.classList.remove('active');
      this.puzzleControls.classList.remove('hidden');
      this.sandboxControls.classList.add('hidden');
      this.game.loadLevel(0);
    } else {
      this.modeGameBtn.classList.remove('active');
      this.modeSandboxBtn.classList.add('active');
      this.puzzleControls.classList.add('hidden');
      this.sandboxControls.classList.remove('hidden');
    }
  }

  /**
   * Toggle pause
   */
  private togglePause(): void {
    this.game.togglePause();
    this.pausePlayBtn.textContent = this.game.physics.isPaused ? 'Resume [SPACE]' : 'Pause [SPACE]';
  }

  /**
   * Toggle trails display
   */
  private toggleTrails(): void {
    this.renderer.showTrails = !this.renderer.showTrails;
    this.toggleTrailsBtn.textContent = `Trails: ${this.renderer.showTrails ? 'ON' : 'OFF'}`;
  }

  /**
   * Toggle debug display
   */
  private toggleDebug(): void {
    this.renderer.showDebug = !this.renderer.showDebug;
    this.toggleDebugBtn.textContent = `Debug: ${this.renderer.showDebug ? 'ON' : 'OFF'}`;
  }

  /**
   * Toggle gravity
   */
  private toggleGravity(): void {
    this.currentGravity = !this.currentGravity;
    if (this.currentGravity) {
      this.game.physics.setGravity(0, 0.05);
      this.toggleGravityBtn.textContent = 'Gravity: ON';
    } else {
      this.game.physics.setGravity(0, 0);
      this.toggleGravityBtn.textContent = 'Gravity: OFF';
    }
  }

  /**
   * Update spawn button states
   */
  private updateSpawnButtonStates(): void {
    this.spawnNeutralBtn.style.opacity = this.nextSpawnType === ParticleType.NEUTRAL ? '1' : '0.5';
    this.spawnPositiveBtn.style.opacity = this.nextSpawnType === ParticleType.POSITIVE ? '1' : '0.5';
    this.spawnNegativeBtn.style.opacity = this.nextSpawnType === ParticleType.NEGATIVE ? '1' : '0.5';
  }

  /**
   * Update attractor button states
   */
  private updateAttractorButtonStates(): void {
    this.addAttractorBtn.style.opacity = this.nextAttractorMode === true ? '1' : '0.5';
    this.addRepulsorBtn.style.opacity = this.nextAttractorMode === false ? '1' : '0.5';
  }

  /**
   * Update UI status displays
   */
  private updateStatusDisplay(): void {
    const statusLevel = document.getElementById('statusLevel');
    const statusTime = document.getElementById('statusTime');
    const statusGoal = document.getElementById('statusGoal');

    if (statusLevel) {
      statusLevel.textContent = `${this.game.currentLevel + 1}/${this.game.levels.length}`;
    }

    if (statusTime) {
      statusTime.textContent = `${this.game.time.toFixed(1)}s`;
    }

    if (statusGoal && this.game.goals.length > 0) {
      let totalParticles = 0;
      let requiredTotal = 0;

      for (const goal of this.game.goals) {
        totalParticles += goal.particlesInZone.length;
        requiredTotal += goal.requiredParticles;
      }

      statusGoal.textContent = `${totalParticles}/${requiredTotal}`;
    }

    // Update level select
    this.levelSelect.value = String(this.game.currentLevel);
  }

  /**
   * Start the game
   */
  private start(): void {
    this.game.start();
    this.gameLoop = requestAnimationFrame(() => this.update());
  }

  /**
   * Update and render
   */
  private update(): void {
    const now = Date.now();
    this.deltaTime = (now - this.lastFrameTime) / 1000;
    this.lastFrameTime = now;

    // Calculate FPS
    this.fps = 1 / this.deltaTime;

    // Update game
    this.game.update();

    // Render
    this.renderer.drawPhysicsState(this.game.physics, this.game.goals);
    this.renderer.drawHUD(
      this.game.levels[this.game.currentLevel].name,
      this.game.currentLevel,
      this.game.levels.length,
      this.game.time,
      this.game.timeLimit,
      this.game.physics.isPaused,
      this.game.isLevelComplete,
      this.game.isFailed,
      this.game.mode,
    );
    this.renderer.drawDebugInfo(this.game.physics, this.fps);

    // Update status display
    this.updateStatusDisplay();

    this.gameLoop = requestAnimationFrame(() => this.update());
  }

  /**
   * Destroy the game
   */
  public destroy(): void {
    if (this.gameLoop !== null) {
      cancelAnimationFrame(this.gameLoop);
    }
  }
}

// Initialize game when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => {
    (window as any).app = new ParticlePlaygroundApp();
  });
} else {
  (window as any).app = new ParticlePlaygroundApp();
}
