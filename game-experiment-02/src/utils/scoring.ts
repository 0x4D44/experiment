const COMBO_RESET_TIME = 3000;
export class ScoringSystem {
  private baseBlockPoints = 10;
  private comboBlocksDestroyed = 0;
  private comboStartTime = 0;
  calculateBlockPoints(blockHealth: number, blockMaxHealth: number, combo: number): number {
    const healthMultiplier = blockMaxHealth / (blockMaxHealth - blockHealth + 1);
    const basePoints = this.baseBlockPoints * healthMultiplier;
    const comboMultiplier = 1 + Math.min(combo * 0.2, 2);
    return Math.floor(basePoints * comboMultiplier);
  }
  getComboMultiplier(combo: number): number {
    return 1 + Math.min(combo * 0.2, 2);
  }
  updateCombo(blockDestroyed: boolean, currentTime: number): number {
    if (blockDestroyed) {
      if (this.comboStartTime === 0 || currentTime - this.comboStartTime > COMBO_RESET_TIME) {
        this.comboBlocksDestroyed = 1;
        this.comboStartTime = currentTime;
      } else {
        this.comboBlocksDestroyed++;
      }
    } else {
      if (this.comboStartTime > 0 && currentTime - this.comboStartTime > COMBO_RESET_TIME) {
        this.comboBlocksDestroyed = 0;
        this.comboStartTime = 0;
      }
    }
    return this.comboBlocksDestroyed;
  }
  resetCombo(): void {
    this.comboBlocksDestroyed = 0;
    this.comboStartTime = 0;
  }
}
