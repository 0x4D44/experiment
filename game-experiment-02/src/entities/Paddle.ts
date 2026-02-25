import { Rectangle } from '../utils/collision';
import { PowerUpType } from '../types/index';
export class Paddle {
  public x: number = 0;
  public y: number = 0;
  public width: number = 100;
  public height: number = 15;
  public speed: number = 400;
  public velocityX: number = 0;
  public isLaser: boolean = false;
  public isSticky: boolean = false;
  public isShield: boolean = false;
  constructor(x: number, y: number, width: number = 100) {
    this.x = x;
    this.y = y;
    this.width = width;
  }
  update(deltaTime: number, gameWidth: number): void {
    this.x += this.velocityX * deltaTime;
    if (this.x < 0) this.x = 0;
    if (this.x + this.width > gameWidth) this.x = gameWidth - this.width;
  }
  moveLeft(): void { this.velocityX = -this.speed; }
  moveRight(): void { this.velocityX = this.speed; }
  stop(): void { this.velocityX = 0; }
  getBounds(): Rectangle { return { x: this.x, y: this.y, width: this.width, height: this.height }; }
  applyPowerUp(type: PowerUpType): void {
    switch (type) {
      case PowerUpType.LASER_PADDLE: this.isLaser = true; break;
      case PowerUpType.STICKY_PADDLE: this.isSticky = true; break;
      case PowerUpType.EXPAND_PADDLE: this.width = Math.min(this.width * 1.5, 200); break;
      case PowerUpType.SHIELD: this.isShield = true; break;
    }
  }
  removePowerUp(type: PowerUpType): void {
    switch (type) {
      case PowerUpType.LASER_PADDLE: this.isLaser = false; break;
      case PowerUpType.STICKY_PADDLE: this.isSticky = false; break;
      case PowerUpType.SHIELD: this.isShield = false; break;
    }
  }
  reset(x: number, y: number): void {
    this.x = x;
    this.y = y;
    this.width = 100;
    this.velocityX = 0;
    this.isLaser = false;
    this.isSticky = false;
    this.isShield = false;
  }
}
