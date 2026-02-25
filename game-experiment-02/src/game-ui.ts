/**
 * Game UI and Canvas Renderer
 */

import { GameManager } from './game-manager';
import { AICommand, CellType, ExecutionStep } from './maze-types';

const CELL_SIZE = 50;
const WALL_COLOR = '#333';
const EMPTY_COLOR = '#fff';
const START_COLOR = '#4caf50';
const GOAL_COLOR = '#f44336';
const AI_COLOR = '#2196f3';
const KEY_COLOR = '#ffc107';
const DOOR_COLOR = '#9c27b0';
const TELEPORTER_COLOR = '#00bcd4';
const MARKED_COLOR = '#e0e0e0';

export class GameUI {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private gameManager: GameManager;
  private running: boolean = false;

  constructor(canvasId: string) {
    this.canvas = document.getElementById(canvasId) as HTMLCanvasElement;
    this.ctx = this.canvas.getContext('2d') as CanvasRenderingContext2D;
    this.gameManager = new GameManager();
    this.setupEventListeners();
    this.render();
  }

  private setupEventListeners(): void {
    const levelSelect = document.getElementById('levelSelect') as HTMLSelectElement;
    const runBtn = document.getElementById('runBtn') as HTMLButtonElement;
    const resetBtn = document.getElementById('resetBtn') as HTMLButtonElement;

    levelSelect.addEventListener('change', (e) => {
      const levelId = parseInt((e.target as HTMLSelectElement).value);
      this.loadLevel(levelId);
    });

    runBtn.addEventListener('click', () => this.runProgram());
    resetBtn.addEventListener('click', () => this.resetLevel());
  }

  private loadLevel(levelId: number): void {
    this.gameManager.loadLevel(levelId);
    const level = this.gameManager.getCurrentLevel();

    if (level) {
      const levelInfo = document.getElementById('levelInfo');
      const levelName = document.getElementById('levelName');
      const levelDesc = document.getElementById('levelDesc');
      const levelDiff = document.getElementById('levelDiff');

      if (levelName) levelName.textContent = `Level ${level.id} - ${level.name}`;
      if (levelDesc) levelDesc.textContent = level.description;

      if (levelDiff) {
        levelDiff.textContent = level.difficulty;
        levelDiff.className = `difficulty ${level.difficulty.toLowerCase()}`;
      }
    }

    this.updateStats();
    this.render();
    this.clearLog();
    this.addLog('Level loaded. Ready to execute program.');
  }

  private runProgram(): void {
    if (this.running) return;
    this.running = true;

    const programInput = document.getElementById('program') as HTMLTextAreaElement;
    const commands = this.parseProgram(programInput.value);

    this.clearLog();
    this.addLog(`Executing ${commands.length} commands...`);

    // Execute with animation
    let index = 0;
    const executeNext = () => {
      if (index < commands.length && !this.gameManager.getCurrentAI()?.isFinished()) {
        const command = commands[index];
        const success = this.gameManager.executeCommand(command);

        if (success) {
          this.addLog(`✓ ${command}`);
        } else {
          this.addLog(`✗ ${command} - Failed`);
        }

        this.updateStats();
        this.render();
        index++;
        setTimeout(executeNext, 200); // Animation delay
      } else {
        this.onExecutionComplete();
        this.running = false;
      }
    };

    executeNext();
  }

  private parseProgram(input: string): AICommand[] {
    const commands: AICommand[] = [];
    const lines = input.split('\n').map((line) => line.trim().toUpperCase());

    for (const line of lines) {
      if (line === 'FORWARD') commands.push(AICommand.Forward);
      else if (line === 'TURN_LEFT') commands.push(AICommand.TurnLeft);
      else if (line === 'TURN_RIGHT') commands.push(AICommand.TurnRight);
      else if (line === 'SENSE_WALL') commands.push(AICommand.SenseWall);
      else if (line === 'MARK_PATH') commands.push(AICommand.MarkPath);
      else if (line === 'PICKUP_KEY') commands.push(AICommand.PickupKey);
      else if (line === 'USE_DOOR') commands.push(AICommand.UseDoor);
      else if (line === 'WAIT') commands.push(AICommand.Wait);
    }

    return commands;
  }

  private onExecutionComplete(): void {
    const isComplete = this.gameManager.isLevelComplete();
    const status = document.getElementById('statusDiv') as HTMLDivElement;

    if (isComplete) {
      status.className = 'status success';
      status.textContent = '🎉 Level Complete! Goal Reached!';
      status.style.display = 'block';
      this.addLog('Goal reached! Execution complete.');
    } else {
      const steps = this.gameManager.getStepCount();
      const level = this.gameManager.getCurrentLevel();
      if (level && steps >= level.maxSteps) {
        status.className = 'status failure';
        status.textContent = '❌ Step limit exceeded';
      } else {
        status.className = 'status failure';
        status.textContent = '❌ Goal not reached';
      }
      status.style.display = 'block';
      this.addLog('Execution complete but goal not reached.');
    }
  }

  private resetLevel(): void {
    if (this.running) return;
    this.gameManager.resetLevel();
    const status = document.getElementById('statusDiv') as HTMLDivElement;
    status.style.display = 'none';
    this.clearLog();
    this.addLog('Level reset to start position.');
    this.updateStats();
    this.render();
  }

  private render(): void {
    const level = this.gameManager.getCurrentLevel();
    const aiState = this.gameManager.getCurrentAIState();

    if (!level || !aiState) return;

    const maze = level.maze;
    const width = maze.width * CELL_SIZE;
    const height = maze.height * CELL_SIZE;

    this.canvas.width = width;
    this.canvas.height = height;

    // Draw background
    this.ctx.fillStyle = EMPTY_COLOR;
    this.ctx.fillRect(0, 0, width, height);

    // Draw cells
    for (let y = 0; y < maze.height; y++) {
      for (let x = 0; x < maze.width; x++) {
        const cell = maze.cells[y][x];
        const px = x * CELL_SIZE;
        const py = y * CELL_SIZE;

        // Draw marked cells
        if (aiState.markedCells.has(`${x},${y}`)) {
          this.ctx.fillStyle = MARKED_COLOR;
          this.ctx.fillRect(px, py, CELL_SIZE, CELL_SIZE);
        }

        // Draw cell type
        if ((cell.type & CellType.Wall) !== 0) {
          this.ctx.fillStyle = WALL_COLOR;
          this.ctx.fillRect(px, py, CELL_SIZE, CELL_SIZE);
        } else if ((cell.type & CellType.Goal) !== 0) {
          this.ctx.fillStyle = GOAL_COLOR;
          this.ctx.fillRect(px + 5, py + 5, CELL_SIZE - 10, CELL_SIZE - 10);
          this.drawText('★', px + CELL_SIZE / 2, py + CELL_SIZE / 2, 'white', '20px');
        } else if ((cell.type & CellType.StartPosition) !== 0) {
          this.ctx.fillStyle = START_COLOR;
          this.ctx.fillRect(px + 5, py + 5, CELL_SIZE - 10, CELL_SIZE - 10);
          this.drawText('●', px + CELL_SIZE / 2, py + CELL_SIZE / 2, 'white', '20px');
        } else if ((cell.type & CellType.Key) !== 0) {
          this.ctx.fillStyle = KEY_COLOR;
          this.ctx.beginPath();
          this.ctx.arc(px + CELL_SIZE / 2, py + CELL_SIZE / 2, 8, 0, Math.PI * 2);
          this.ctx.fill();
          this.drawText(`K${cell.keyId}`, px + CELL_SIZE / 2, py + CELL_SIZE / 2, 'black', '10px');
        } else if ((cell.type & CellType.Door) !== 0) {
          this.ctx.fillStyle = DOOR_COLOR;
          this.ctx.fillRect(px + 10, py + 10, CELL_SIZE - 20, CELL_SIZE - 20);
          this.drawText(`D${cell.keyId}`, px + CELL_SIZE / 2, py + CELL_SIZE / 2, 'white', '10px');
        } else if ((cell.type & CellType.Teleporter) !== 0) {
          this.ctx.fillStyle = TELEPORTER_COLOR;
          this.ctx.beginPath();
          this.ctx.arc(px + CELL_SIZE / 2, py + CELL_SIZE / 2, 10, 0, Math.PI * 2);
          this.ctx.fill();
          this.drawText('⊗', px + CELL_SIZE / 2, py + CELL_SIZE / 2, 'white', '14px');
        }

        // Draw grid
        this.ctx.strokeStyle = '#ddd';
        this.ctx.lineWidth = 1;
        this.ctx.strokeRect(px, py, CELL_SIZE, CELL_SIZE);
      }
    }

    // Draw AI
    const aiPx = aiState.position.x * CELL_SIZE;
    const aiPy = aiState.position.y * CELL_SIZE;
    this.ctx.fillStyle = AI_COLOR;
    this.ctx.beginPath();
    this.ctx.arc(aiPx + CELL_SIZE / 2, aiPy + CELL_SIZE / 2, 12, 0, Math.PI * 2);
    this.ctx.fill();

    // Draw direction indicator
    this.drawDirectionArrow(
      aiPx + CELL_SIZE / 2,
      aiPy + CELL_SIZE / 2,
      aiState.direction,
      'white'
    );
  }

  private drawText(
    text: string,
    x: number,
    y: number,
    color: string,
    font: string
  ): void {
    this.ctx.fillStyle = color;
    this.ctx.font = font;
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'middle';
    this.ctx.fillText(text, x, y);
  }

  private drawDirectionArrow(
    x: number,
    y: number,
    direction: string,
    color: string
  ): void {
    this.ctx.strokeStyle = color;
    this.ctx.lineWidth = 2;
    const arrowSize = 8;

    switch (direction) {
      case 'N':
        this.ctx.beginPath();
        this.ctx.moveTo(x, y - arrowSize);
        this.ctx.lineTo(x - 3, y);
        this.ctx.lineTo(x, y - 3);
        this.ctx.lineTo(x + 3, y);
        this.ctx.closePath();
        this.ctx.stroke();
        break;
      case 'E':
        this.ctx.beginPath();
        this.ctx.moveTo(x + arrowSize, y);
        this.ctx.lineTo(x, y - 3);
        this.ctx.lineTo(x + 3, y);
        this.ctx.lineTo(x, y + 3);
        this.ctx.closePath();
        this.ctx.stroke();
        break;
      case 'S':
        this.ctx.beginPath();
        this.ctx.moveTo(x, y + arrowSize);
        this.ctx.lineTo(x - 3, y);
        this.ctx.lineTo(x, y + 3);
        this.ctx.lineTo(x + 3, y);
        this.ctx.closePath();
        this.ctx.stroke();
        break;
      case 'W':
        this.ctx.beginPath();
        this.ctx.moveTo(x - arrowSize, y);
        this.ctx.lineTo(x, y - 3);
        this.ctx.lineTo(x - 3, y);
        this.ctx.lineTo(x, y + 3);
        this.ctx.closePath();
        this.ctx.stroke();
        break;
    }
  }

  private updateStats(): void {
    const stepCount = document.getElementById('stepCount');
    const posCount = document.getElementById('posCount');
    const effCount = document.getElementById('effCount');
    const timeCount = document.getElementById('timeCount');

    const aiState = this.gameManager.getCurrentAIState();
    const score = this.gameManager.getCurrentScore();

    if (stepCount && aiState) {
      stepCount.textContent = aiState.stepCount.toString();
    }

    if (posCount && aiState) {
      posCount.textContent = `(${aiState.position.x}, ${aiState.position.y})`;
    }

    if (score && effCount) {
      effCount.textContent = `${Math.round(score.efficiency)}%`;
    }

    if (timeCount && score) {
      timeCount.textContent = `${score.timeTaken.toFixed(1)}s`;
    }
  }

  private addLog(message: string): void {
    const logDiv = document.getElementById('logDiv') as HTMLDivElement;
    const entry = document.createElement('div');
    entry.className = 'log-entry';
    if (message.includes('✗') || message.includes('Goal not reached') || message.includes('Step limit')) {
      entry.className += ' error';
    } else if (message.includes('✓') || message.includes('Goal reached')) {
      entry.className += ' success';
    }
    entry.textContent = message;
    logDiv.appendChild(entry);
    logDiv.scrollTop = logDiv.scrollHeight;
  }

  private clearLog(): void {
    const logDiv = document.getElementById('logDiv') as HTMLDivElement;
    logDiv.innerHTML = '';
  }
}

// Initialize game when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  new GameUI('gameCanvas');
});
