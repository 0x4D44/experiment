import { GameState } from '../types/Physics';
import { Vector2D } from '../utils/Vector2D';

/**
 * Canvas-based renderer for Gravity Golf
 */
export class CanvasRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private width: number;
  private height: number;

  constructor(canvasElement: HTMLCanvasElement, width: number = 800, height: number = 600) {
    this.canvas = canvasElement;
    this.width = width;
    this.height = height;
    this.canvas.width = width;
    this.canvas.height = height;

    const ctx = this.canvas.getContext('2d');
    if (!ctx) {
      throw new Error('Could not get canvas 2D context');
    }
    this.ctx = ctx;
  }

  /**
   * Clear canvas
   */
  clear(): void {
    this.ctx.fillStyle = '#1a1a2e';
    this.ctx.fillRect(0, 0, this.width, this.height);
  }

  /**
   * Draw the game state
   */
  render(gameState: GameState): void {
    this.clear();

    // Draw gravity wells
    this.drawGravityWells(gameState);

    // Draw obstacles
    this.drawObstacles(gameState);

    // Draw trajectory prediction (optional, when ball stopped)
    if (gameState.ballStopped && !gameState.inHole) {
      this.drawTrajectoryGuide(gameState);
    }

    // Draw ball
    this.drawBall(gameState);

    // Draw hole
    this.drawHole(gameState);

    // Draw UI overlay
    this.drawUI(gameState);
  }

  /**
   * Draw gravity wells
   */
  private drawGravityWells(gameState: GameState): void {
    for (const well of gameState.gravityWells) {
      // Draw well circle
      this.ctx.strokeStyle = well.strength > 0 ? '#4a90ff' : '#ff4a4a';
      this.ctx.lineWidth = 2;
      this.ctx.beginPath();
      this.ctx.arc(well.position.x, well.position.y, well.radius, 0, Math.PI * 2);
      this.ctx.stroke();

      // Draw well center
      this.ctx.fillStyle = well.strength > 0 ? '#6ab7ff' : '#ff6b6b';
      this.ctx.fillRect(well.position.x - 4, well.position.y - 4, 8, 8);

      // Draw label
      const label = well.strength > 0 ? 'A' : 'R'; // Attractive or Repulsive
      this.ctx.fillStyle = '#ffffff';
      this.ctx.font = 'bold 10px Arial';
      this.ctx.fillText(label, well.position.x + 6, well.position.y - 6);
    }
  }

  /**
   * Draw obstacles
   */
  private drawObstacles(gameState: GameState): void {
    for (const obstacle of gameState.obstacles) {
      switch (obstacle.type) {
        case 'wall':
          this.drawWall(obstacle.position, obstacle.radius);
          break;
        case 'blackhole':
          this.drawBlackHole(obstacle.position, obstacle.radius);
          break;
        case 'wormhole':
          this.drawWormhole(obstacle.position, obstacle.radius);
          break;
        case 'asteroid':
          this.drawAsteroid(obstacle.position, obstacle.radius);
          break;
      }
    }

    // Draw wormholes
    for (const wormhole of gameState.wormholes) {
      this.ctx.strokeStyle = '#ffaa00';
      this.ctx.lineWidth = 2;
      this.ctx.setLineDash([5, 5]);
      this.ctx.beginPath();
      this.ctx.moveTo(wormhole.entrance.x, wormhole.entrance.y);
      this.ctx.lineTo(wormhole.exit.x, wormhole.exit.y);
      this.ctx.stroke();
      this.ctx.setLineDash([]);
    }
  }

  /**
   * Draw wall obstacle
   */
  private drawWall(position: Vector2D, radius: number): void {
    this.ctx.fillStyle = '#888888';
    this.ctx.fillRect(position.x - radius, position.y - radius, radius * 2, radius * 2);
    this.ctx.strokeStyle = '#ffffff';
    this.ctx.lineWidth = 2;
    this.ctx.strokeRect(position.x - radius, position.y - radius, radius * 2, radius * 2);
  }

  /**
   * Draw black hole obstacle
   */
  private drawBlackHole(position: Vector2D, radius: number): void {
    // Outer glow
    const gradient = this.ctx.createRadialGradient(
      position.x,
      position.y,
      0,
      position.x,
      position.y,
      radius
    );
    gradient.addColorStop(0, 'rgba(255, 0, 0, 0.3)');
    gradient.addColorStop(1, 'rgba(255, 0, 0, 0)');
    this.ctx.fillStyle = gradient;
    this.ctx.beginPath();
    this.ctx.arc(position.x, position.y, radius, 0, Math.PI * 2);
    this.ctx.fill();

    // Inner black core
    this.ctx.fillStyle = '#000000';
    this.ctx.beginPath();
    this.ctx.arc(position.x, position.y, radius * 0.7, 0, Math.PI * 2);
    this.ctx.fill();

    // Danger label
    this.ctx.fillStyle = '#ff0000';
    this.ctx.font = 'bold 12px Arial';
    this.ctx.fillText('BH', position.x - 8, position.y + 4);
  }

  /**
   * Draw wormhole obstacle
   */
  private drawWormhole(position: Vector2D, radius: number): void {
    const gradient = this.ctx.createRadialGradient(
      position.x,
      position.y,
      0,
      position.x,
      position.y,
      radius
    );
    gradient.addColorStop(0, 'rgba(255, 165, 0, 0.5)');
    gradient.addColorStop(1, 'rgba(255, 165, 0, 0)');
    this.ctx.fillStyle = gradient;
    this.ctx.beginPath();
    this.ctx.arc(position.x, position.y, radius, 0, Math.PI * 2);
    this.ctx.fill();

    this.ctx.strokeStyle = '#ffaa00';
    this.ctx.lineWidth = 2;
    this.ctx.beginPath();
    this.ctx.arc(position.x, position.y, radius, 0, Math.PI * 2);
    this.ctx.stroke();

    // Portal label
    this.ctx.fillStyle = '#ffaa00';
    this.ctx.font = 'bold 10px Arial';
    this.ctx.fillText('P', position.x - 3, position.y + 3);
  }

  /**
   * Draw asteroid obstacle
   */
  private drawAsteroid(position: Vector2D, radius: number): void {
    this.ctx.fillStyle = '#aa6633';
    this.ctx.beginPath();
    this.ctx.arc(position.x, position.y, radius, 0, Math.PI * 2);
    this.ctx.fill();

    this.ctx.strokeStyle = '#ddaa55';
    this.ctx.lineWidth = 1;
    this.ctx.stroke();

    // Add some texture
    this.ctx.fillStyle = 'rgba(255, 255, 255, 0.2)';
    this.ctx.beginPath();
    this.ctx.arc(position.x - radius * 0.3, position.y - radius * 0.3, radius * 0.3, 0, Math.PI * 2);
    this.ctx.fill();
  }

  /**
   * Draw trajectory guide
   */
  private drawTrajectoryGuide(gameState: GameState): void {
    // Simple trajectory preview - draw a line showing potential direction
    const startPos = gameState.ball.position;
    const guideLength = 150;

    // Draw aiming line circle
    this.ctx.strokeStyle = 'rgba(100, 200, 255, 0.3)';
    this.ctx.lineWidth = 1;
    this.ctx.beginPath();
    this.ctx.arc(startPos.x, startPos.y, guideLength, 0, Math.PI * 2);
    this.ctx.stroke();

    // Draw guidance text
    this.ctx.fillStyle = 'rgba(100, 200, 255, 0.5)';
    this.ctx.font = '12px Arial';
    this.ctx.fillText('Click to aim and hit', startPos.x + 20, startPos.y - 30);
  }

  /**
   * Draw ball
   */
  private drawBall(gameState: GameState): void {
    const ball = gameState.ball;
    const gradient = this.ctx.createRadialGradient(
      ball.position.x - ball.radius / 3,
      ball.position.y - ball.radius / 3,
      0,
      ball.position.x,
      ball.position.y,
      ball.radius
    );
    gradient.addColorStop(0, '#ffff88');
    gradient.addColorStop(1, '#ffdd00');

    this.ctx.fillStyle = gradient;
    this.ctx.beginPath();
    this.ctx.arc(ball.position.x, ball.position.y, ball.radius, 0, Math.PI * 2);
    this.ctx.fill();

    // Add shine
    this.ctx.fillStyle = 'rgba(255, 255, 255, 0.4)';
    this.ctx.beginPath();
    this.ctx.arc(ball.position.x - 2, ball.position.y - 2, 2, 0, Math.PI * 2);
    this.ctx.fill();

    // Draw velocity indicator if moving
    if (gameState.ballInMotion && gameState.ball.velocity.magnitude() > 0) {
      const vel = gameState.ball.velocity;
      const scale = Math.min(vel.magnitude() / 50, 1);
      this.ctx.strokeStyle = 'rgba(255, 100, 100, 0.5)';
      this.ctx.lineWidth = 2;
      this.ctx.beginPath();
      this.ctx.moveTo(ball.position.x, ball.position.y);
      this.ctx.lineTo(ball.position.x + vel.x * scale * 0.3, ball.position.y + vel.y * scale * 0.3);
      this.ctx.stroke();
    }
  }

  /**
   * Draw hole
   */
  private drawHole(gameState: GameState): void {
    const hole = gameState.hole;
    const holeRadius = gameState.holeRadius;

    // Hole shadow/depth
    const gradient = this.ctx.createRadialGradient(hole.x, hole.y, 0, hole.x, hole.y, holeRadius);
    gradient.addColorStop(0, 'rgba(0, 200, 0, 0.3)');
    gradient.addColorStop(1, 'rgba(0, 100, 0, 0)');

    this.ctx.fillStyle = gradient;
    this.ctx.beginPath();
    this.ctx.arc(hole.x, hole.y, holeRadius, 0, Math.PI * 2);
    this.ctx.fill();

    // Hole outline
    this.ctx.strokeStyle = '#00ff00';
    this.ctx.lineWidth = 3;
    this.ctx.beginPath();
    this.ctx.arc(hole.x, hole.y, holeRadius, 0, Math.PI * 2);
    this.ctx.stroke();

    // Flag
    this.ctx.strokeStyle = '#ffffff';
    this.ctx.lineWidth = 1;
    this.ctx.beginPath();
    this.ctx.moveTo(hole.x, hole.y - holeRadius - 5);
    this.ctx.lineTo(hole.x, hole.y - holeRadius - 15);
    this.ctx.stroke();

    this.ctx.fillStyle = '#ff0000';
    this.ctx.fillRect(hole.x, hole.y - holeRadius - 15, 12, 6);
  }

  /**
   * Draw UI overlay
   */
  private drawUI(gameState: GameState): void {
    // Score display
    this.ctx.fillStyle = '#ffffff';
    this.ctx.font = 'bold 16px Arial';
    this.ctx.fillText(`Strokes: ${gameState.strokes}`, 10, 25);

    // Gravity modifiers remaining
    this.ctx.fillText(`Modifiers: ${gameState.maxGravityModifiers - gameState.gravityModifiersUsed}`, 10, 45);

    // Ball status
    let ballStatus = 'Ready';
    if (gameState.inHole) {
      ballStatus = 'IN HOLE!';
      this.ctx.fillStyle = '#00ff00';
    } else if (gameState.ballInMotion) {
      ballStatus = 'Rolling...';
      this.ctx.fillStyle = '#ffff00';
    } else {
      this.ctx.fillStyle = '#ffffff';
    }
    this.ctx.fillText(ballStatus, 10, 65);

    // Boundary border
    this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
    this.ctx.lineWidth = 2;
    this.ctx.strokeRect(0, 0, this.width, this.height);
  }

  /**
   * Draw trajectory preview
   */
  drawTrajectoryPreview(trajectory: Vector2D[]): void {
    if (trajectory.length < 2) return;

    this.ctx.strokeStyle = 'rgba(100, 255, 100, 0.5)';
    this.ctx.lineWidth = 2;
    this.ctx.beginPath();
    this.ctx.moveTo(trajectory[0].x, trajectory[0].y);

    for (let i = 1; i < trajectory.length; i++) {
      this.ctx.lineTo(trajectory[i].x, trajectory[i].y);
    }

    this.ctx.stroke();

    // Draw trajectory points
    this.ctx.fillStyle = 'rgba(100, 255, 100, 0.3)';
    for (let i = 0; i < trajectory.length; i += Math.ceil(trajectory.length / 10)) {
      this.ctx.beginPath();
      this.ctx.arc(trajectory[i].x, trajectory[i].y, 2, 0, Math.PI * 2);
      this.ctx.fill();
    }
  }
}
