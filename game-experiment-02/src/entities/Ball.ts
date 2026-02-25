import { Vector2 } from '../utils/collision';
export class Ball {
  public x: number = 0;
  public y: number = 0;
  public radius: number = 6;
  public velocity: Vector2 = { x: 0, y: 0 };
  public speed: number = 300;
  public active: boolean = true;
  public isStuck: boolean = false;
  constructor(x: number, y: number, radius: number = 6) {
    this.x = x;
    this.y = y;
    this.radius = radius;
  }
  update(deltaTime: number): void {
    if (!this.active || this.isStuck) return;
    this.x += this.velocity.x * deltaTime;
    this.y += this.velocity.y * deltaTime;
  }
  setVelocity(vx: number, vy: number): void {
    const length = Math.sqrt(vx * vx + vy * vy);
    if (length > 0) {
      this.velocity = { x: (vx / length) * this.speed, y: (vy / length) * this.speed };
    }
  }
  stick(x: number, y: number): void {
    this.isStuck = true;
    this.x = x;
    this.y = y;
    this.velocity = { x: 0, y: 0 };
  }
  launch(angle: number): void {
    this.isStuck = false;
    this.velocity = { x: Math.cos(angle) * this.speed, y: Math.sin(angle) * this.speed };
  }
  reset(x: number, y: number): void {
    this.x = x;
    this.y = y;
    this.velocity = { x: 0, y: 0 };
    this.isStuck = true;
    this.active = true;
  }
  isOutOfBounds(gameHeight: number): boolean {
    return this.y > gameHeight;
  }
}
