/**
 * Particle Playground - Canvas Renderer
 * Handles all visual rendering of the particle physics simulation
 */

import { Particle, ParticleType, Vector2D, PhysicsEngine, Attractor, GoalZone, Barrier, Portal } from './particle-physics';
import { GameMode } from './particle-game';

export class ParticleRenderer {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D;
  width: number;
  height: number;
  showTrails: boolean;
  showDebug: boolean;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d')!;
    this.width = canvas.width;
    this.height = canvas.height;
    this.showTrails = true;
    this.showDebug = false;
  }

  /**
   * Clear canvas
   */
  clear(): void {
    this.ctx.fillStyle = '#000000';
    this.ctx.fillRect(0, 0, this.width, this.height);
  }

  /**
   * Draw background grid
   */
  drawGrid(): void {
    this.ctx.strokeStyle = '#111111';
    this.ctx.lineWidth = 1;

    const gridSize = 50;

    // Vertical lines
    for (let x = 0; x < this.width; x += gridSize) {
      this.ctx.beginPath();
      this.ctx.moveTo(x, 0);
      this.ctx.lineTo(x, this.height);
      this.ctx.stroke();
    }

    // Horizontal lines
    for (let y = 0; y < this.height; y += gridSize) {
      this.ctx.beginPath();
      this.ctx.moveTo(0, y);
      this.ctx.lineTo(this.width, y);
      this.ctx.stroke();
    }
  }

  /**
   * Get color for particle type
   */
  getParticleColor(type: ParticleType): string {
    switch (type) {
      case ParticleType.NEUTRAL:
        return '#808080';
      case ParticleType.POSITIVE:
        return '#FF4444';
      case ParticleType.NEGATIVE:
        return '#4444FF';
      default:
        return '#FFFFFF';
    }
  }

  /**
   * Draw particle
   */
  drawParticle(particle: Particle): void {
    const color = this.getParticleColor(particle.type);

    // Draw trail if enabled
    if (this.showTrails && particle.trail.length > 1) {
      this.ctx.strokeStyle = color;
      this.ctx.globalAlpha = 0.3;
      this.ctx.lineWidth = 1;
      this.ctx.beginPath();
      this.ctx.moveTo(particle.trail[0].x, particle.trail[0].y);

      for (let i = 1; i < particle.trail.length; i++) {
        this.ctx.lineTo(particle.trail[i].x, particle.trail[i].y);
      }

      this.ctx.stroke();
      this.ctx.globalAlpha = 1.0;
    }

    // Draw particle
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.arc(particle.position.x, particle.position.y, particle.radius, 0, Math.PI * 2);
    this.ctx.fill();

    // Draw outline
    this.ctx.strokeStyle = '#FFFFFF';
    this.ctx.lineWidth = 1;
    this.ctx.stroke();
  }

  /**
   * Draw all particles
   */
  drawParticles(particles: Particle[]): void {
    for (const particle of particles) {
      this.drawParticle(particle);
    }
  }

  /**
   * Draw attractor
   */
  drawAttractor(attractor: Attractor): void {
    if (attractor.isAttractor) {
      this.ctx.fillStyle = 'rgba(255, 200, 0, 0.15)';
      this.ctx.strokeStyle = '#FFD700';
    } else {
      this.ctx.fillStyle = 'rgba(100, 200, 255, 0.15)';
      this.ctx.strokeStyle = '#64C8FF';
    }

    this.ctx.lineWidth = 2;
    this.ctx.beginPath();
    this.ctx.arc(attractor.position.x, attractor.position.y, attractor.radius, 0, Math.PI * 2);
    this.ctx.fill();
    this.ctx.stroke();

    // Draw center point
    this.ctx.fillStyle = this.ctx.strokeStyle;
    this.ctx.beginPath();
    this.ctx.arc(attractor.position.x, attractor.position.y, 4, 0, Math.PI * 2);
    this.ctx.fill();
  }

  /**
   * Draw all attractors
   */
  drawAttractors(attractors: Attractor[]): void {
    for (const attractor of attractors) {
      this.drawAttractor(attractor);
    }
  }

  /**
   * Draw barrier
   */
  drawBarrier(barrier: Barrier): void {
    this.ctx.strokeStyle = '#FF6B6B';
    this.ctx.lineWidth = barrier.thickness;
    this.ctx.lineCap = 'round';
    this.ctx.beginPath();
    this.ctx.moveTo(barrier.start.x, barrier.start.y);
    this.ctx.lineTo(barrier.end.x, barrier.end.y);
    this.ctx.stroke();
  }

  /**
   * Draw all barriers
   */
  drawBarriers(barriers: Barrier[]): void {
    for (const barrier of barriers) {
      this.drawBarrier(barrier);
    }
  }

  /**
   * Draw portal
   */
  drawPortal(portal: Portal): void {
    this.ctx.fillStyle = portal.color;
    this.ctx.globalAlpha = 0.6;
    this.ctx.beginPath();
    this.ctx.arc(portal.position.x, portal.position.y, portal.radius, 0, Math.PI * 2);
    this.ctx.fill();
    this.ctx.globalAlpha = 1.0;

    // Draw glowing outline
    this.ctx.strokeStyle = portal.color;
    this.ctx.lineWidth = 2;
    this.ctx.stroke();

    // Draw destination indicator
    this.ctx.strokeStyle = portal.color;
    this.ctx.lineWidth = 1;
    this.ctx.globalAlpha = 0.5;
    this.ctx.beginPath();
    this.ctx.moveTo(portal.position.x, portal.position.y);
    this.ctx.lineTo(portal.destinationPosition.x, portal.destinationPosition.y);
    this.ctx.stroke();
    this.ctx.globalAlpha = 1.0;
  }

  /**
   * Draw all portals
   */
  drawPortals(portals: Portal[]): void {
    for (const portal of portals) {
      this.drawPortal(portal);
    }
  }

  /**
   * Draw goal zone
   */
  drawGoal(goal: GoalZone): void {
    const isComplete = goal.isComplete();

    this.ctx.fillStyle = isComplete ? 'rgba(0, 255, 0, 0.2)' : 'rgba(255, 255, 0, 0.1)';
    this.ctx.strokeStyle = isComplete ? '#00FF00' : '#FFFF00';
    this.ctx.lineWidth = 3;

    this.ctx.beginPath();
    this.ctx.arc(goal.position.x, goal.position.y, goal.radius, 0, Math.PI * 2);
    this.ctx.fill();
    this.ctx.stroke();

    // Draw inner circle
    this.ctx.strokeStyle = isComplete ? '#00FF00' : '#FFFF00';
    this.ctx.lineWidth = 1;
    this.ctx.beginPath();
    this.ctx.arc(goal.position.x, goal.position.y, goal.radius - 5, 0, Math.PI * 2);
    this.ctx.stroke();

    // Draw label
    this.ctx.fillStyle = isComplete ? '#00FF00' : '#FFFF00';
    this.ctx.font = 'bold 12px Arial';
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'middle';
    this.ctx.fillText(`${goal.particlesInZone.length}/${goal.requiredParticles}`, goal.position.x, goal.position.y);
  }

  /**
   * Draw all goals
   */
  drawGoals(goals: GoalZone[]): void {
    for (const goal of goals) {
      this.drawGoal(goal);
    }
  }

  /**
   * Draw physics engine state
   */
  drawPhysicsState(physics: PhysicsEngine, goals: GoalZone[]): void {
    this.clear();
    this.drawGrid();
    this.drawAttractors(physics.attractors);
    this.drawBarriers(physics.barriers);
    this.drawPortals(physics.portals);
    this.drawGoals(goals);
    this.drawParticles(physics.particles);
  }

  /**
   * Draw HUD
   */
  drawHUD(
    levelName: string,
    levelNumber: number,
    totalLevels: number,
    time: number,
    timeLimit: number,
    isPaused: boolean,
    isComplete: boolean,
    isFailed: boolean,
    mode: GameMode,
  ): void {
    const padding = 15;

    // Level info
    this.ctx.fillStyle = '#FFFFFF';
    this.ctx.font = 'bold 18px Arial';
    this.ctx.textAlign = 'left';
    this.ctx.fillText(`Level ${levelNumber + 1}/${totalLevels}: ${levelName}`, padding, padding + 20);

    // Mode
    this.ctx.font = 'bold 14px Arial';
    this.ctx.fillStyle = mode === GameMode.PUZZLE ? '#FFD700' : '#00FF00';
    this.ctx.fillText(mode === GameMode.PUZZLE ? 'PUZZLE MODE' : 'SANDBOX MODE', padding, this.height - padding - 5);

    // Time
    if (timeLimit > 0) {
      this.ctx.fillStyle = '#FFFFFF';
      this.ctx.font = 'bold 16px Arial';
      this.ctx.textAlign = 'right';
      const timeStr = `${Math.floor(time)}s / ${timeLimit}s`;
      this.ctx.fillText(timeStr, this.width - padding, padding + 20);

      // Time warning if low
      if (timeLimit - time < 10) {
        this.ctx.fillStyle = '#FF4444';
        this.ctx.fillRect(this.width - 150, padding + 30, 140, 20);
        this.ctx.fillStyle = '#FFFFFF';
        this.ctx.font = 'bold 14px Arial';
        this.ctx.fillText('TIME RUNNING OUT', this.width - 80, padding + 43);
      }
    }

    // Pause indicator
    if (isPaused) {
      this.ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
      this.ctx.fillRect(0, 0, this.width, this.height);

      this.ctx.fillStyle = '#FFFF00';
      this.ctx.font = 'bold 48px Arial';
      this.ctx.textAlign = 'center';
      this.ctx.textBaseline = 'middle';
      this.ctx.fillText('PAUSED', this.width / 2, this.height / 2 - 40);

      this.ctx.font = '18px Arial';
      this.ctx.fillStyle = '#FFFFFF';
      this.ctx.fillText('Press SPACE to resume', this.width / 2, this.height / 2 + 20);
    }

    // Level complete
    if (isComplete) {
      this.ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
      this.ctx.fillRect(0, 0, this.width, this.height);

      this.ctx.fillStyle = '#00FF00';
      this.ctx.font = 'bold 48px Arial';
      this.ctx.textAlign = 'center';
      this.ctx.textBaseline = 'middle';
      this.ctx.fillText('LEVEL COMPLETE!', this.width / 2, this.height / 2 - 40);

      this.ctx.font = '18px Arial';
      this.ctx.fillStyle = '#FFFFFF';
      this.ctx.fillText('Press N for next level or R to replay', this.width / 2, this.height / 2 + 20);
    }

    // Level failed
    if (isFailed) {
      this.ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
      this.ctx.fillRect(0, 0, this.width, this.height);

      this.ctx.fillStyle = '#FF4444';
      this.ctx.font = 'bold 48px Arial';
      this.ctx.textAlign = 'center';
      this.ctx.textBaseline = 'middle';
      this.ctx.fillText('TIME LIMIT EXCEEDED!', this.width / 2, this.height / 2 - 40);

      this.ctx.font = '18px Arial';
      this.ctx.fillStyle = '#FFFFFF';
      this.ctx.fillText('Press R to retry', this.width / 2, this.height / 2 + 20);
    }
  }

  /**
   * Draw debug information
   */
  drawDebugInfo(physics: PhysicsEngine, fps: number): void {
    if (!this.showDebug) return;

    this.ctx.fillStyle = '#00FF00';
    this.ctx.font = '12px monospace';
    this.ctx.textAlign = 'left';
    this.ctx.textBaseline = 'top';

    let y = 10;
    const x = 10;
    const lineHeight = 16;

    this.ctx.fillText(`FPS: ${fps.toFixed(1)}`, x, y);
    y += lineHeight;

    this.ctx.fillText(`Particles: ${physics.particles.length}`, x, y);
    y += lineHeight;

    this.ctx.fillText(`Attractors: ${physics.attractors.length}`, x, y);
    y += lineHeight;

    this.ctx.fillText(`Barriers: ${physics.barriers.length}`, x, y);
    y += lineHeight;

    if (physics.particles.length > 0) {
      const p = physics.particles[0];
      this.ctx.fillText(`P1 Pos: (${p.position.x.toFixed(1)}, ${p.position.y.toFixed(1)})`, x, y);
      y += lineHeight;

      this.ctx.fillText(`P1 Vel: (${p.velocity.x.toFixed(2)}, ${p.velocity.y.toFixed(2)})`, x, y);
      y += lineHeight;

      this.ctx.fillText(`P1 Speed: ${p.velocity.magnitude().toFixed(2)}`, x, y);
    }
  }

  /**
   * Resize canvas
   */
  resize(width: number, height: number): void {
    this.width = width;
    this.height = height;
    this.canvas.width = width;
    this.canvas.height = height;
  }
}
