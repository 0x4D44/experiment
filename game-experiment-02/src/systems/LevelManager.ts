import { LevelConfig, BlockConfig } from '../types/index';
export class LevelManager {
  private currentLevel = 1;
  private maxLevel = 5;
  generateLevel(levelNumber: number): LevelConfig {
    const baseBlockSpeed = 200;
    const ballSpeed = baseBlockSpeed + levelNumber * 50;
    const paddleWidth = 100 - levelNumber * 5;
    const blocks: BlockConfig[] = this.generateBlocks(levelNumber);
    return { number: levelNumber, blocks, ballSpeed, paddleWidth: Math.max(paddleWidth, 60), bossHealth: levelNumber >= 3 ? 50 + levelNumber * 10 : undefined };
  }
  private generateBlocks(levelNumber: number): BlockConfig[] {
    const blocks: BlockConfig[] = [];
    const cols = 8;
    const rows = 3 + Math.floor(levelNumber / 2);
    const blockWidth = 90;
    const blockHeight = 20;
    const padding = 5;
    const startX = 10;
    const startY = 60;
    for (let row = 0; row < rows; row++) {
      for (let col = 0; col < cols; col++) {
        const x = startX + col * (blockWidth + padding);
        const y = startY + row * (blockHeight + padding);
        if ((row + col) % 3 !== 0 || Math.random() > 0.95) {
          blocks.push({
            x, y, width: blockWidth, height: blockHeight,
            health: Math.min(1 + Math.floor(levelNumber / 2), 5),
            points: 10 + levelNumber * 5,
            color: this.getBlockColor(row, levelNumber),
          });
        }
      }
    }
    return blocks;
  }
  private getBlockColor(row: number, level: number): number {
    const colors = [0xff6b6b, 0x4ecdc4, 0x45b7d1, 0xf9ca24, 0x6c5ce7, 0xa29bfe];
    return colors[(row + level) % colors.length];
  }
  hasWon(levelNumber: number): boolean { return levelNumber > this.maxLevel; }
  getNextLevel(currentLevel: number): number { return Math.min(currentLevel + 1, this.maxLevel + 1); }
  getMaxLevel(): number { return this.maxLevel; }
}
