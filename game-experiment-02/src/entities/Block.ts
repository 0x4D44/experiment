import { Rectangle } from '../utils/collision';
import { BlockConfig } from '../types/index';
export class Block {
  public x: number;
  public y: number;
  public width: number;
  public height: number;
  public health: number;
  public maxHealth: number;
  public points: number;
  public color: number;
  public active: boolean = true;
  constructor(config: BlockConfig) {
    this.x = config.x;
    this.y = config.y;
    this.width = config.width;
    this.height = config.height;
    this.health = config.health;
    this.maxHealth = config.health;
    this.points = config.points;
    this.color = config.color;
  }
  takeDamage(amount: number = 1): boolean {
    this.health -= amount;
    if (this.health <= 0) {
      this.active = false;
      return true;
    }
    return false;
  }
  getBounds(): Rectangle { return { x: this.x, y: this.y, width: this.width, height: this.height }; }
  getHealthPercentage(): number { return Math.max(0, this.health / this.maxHealth); }
  getCurrentColor(): number {
    const healthPercent = this.getHealthPercentage();
    const factor = 0.3 + healthPercent * 0.7;
    const r = Math.floor(((this.color >> 16) & 0xff) * factor);
    const g = Math.floor(((this.color >> 8) & 0xff) * factor);
    const b = Math.floor((this.color & 0xff) * factor);
    return (r << 16) | (g << 8) | b;
  }
  reset(): void {
    this.health = this.maxHealth;
    this.active = true;
  }
}
