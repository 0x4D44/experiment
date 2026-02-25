import { PowerUpType, PowerUpConfig } from '../types/index';
export class PowerUp {
  public x: number;
  public y: number;
  public type: PowerUpType;
  public active: boolean = true;
  public width: number = 20;
  public height: number = 20;
  public velocityY: number = 150;
  public duration: number;
  constructor(config: PowerUpConfig) {
    this.x = config.x;
    this.y = config.y;
    this.type = config.type;
    this.duration = config.duration || 10000;
  }
  update(deltaTime: number): void { this.y += this.velocityY * deltaTime; }
  isOffScreen(gameHeight: number): boolean { return this.y > gameHeight; }
  getBounds() { return { x: this.x, y: this.y, width: this.width, height: this.height }; }
  getColor(): number {
    switch (this.type) {
      case PowerUpType.MULTI_BALL: return 0xff6b6b;
      case PowerUpType.LASER_PADDLE: return 0xffd93d;
      case PowerUpType.STICKY_PADDLE: return 0x95e1d3;
      case PowerUpType.SLOW_MO: return 0x6c5ce7;
      case PowerUpType.EXPAND_PADDLE: return 0x00b894;
      case PowerUpType.SHIELD: return 0x74b9ff;
    }
  }
}
